// SPDX-FileCopyrightText: Copyright The arm-generic-timer Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_std]
#![doc = include_str!("../README.md")]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "embedded-hal")]
mod embedded_hal;
#[cfg(feature = "embedded-hal-timer")]
mod embedded_hal_timer;
pub mod memory_mapped;
#[cfg(any(test, feature = "fakes", target_arch = "aarch64"))]
pub mod sysreg;
mod util;

use core::{hint::spin_loop, time::Duration};

/// Interface for accessing common timer registers.
pub trait TimerInterface {
    /// Enables timer
    fn enable(&mut self);

    /// Returns the frequency in Hz.
    fn frequency(&self) -> u32;

    /// Returns the down-counter value.
    fn timer_value(&self) -> i32;

    /// Returns the compare value.
    fn compare_value(&self) -> u64;

    /// Sets timer value. This programs the compare value relative to the physical counter, which
    /// affects the value returned by [`TimerInterface::compare_value()`].
    /// Setting a negative value will trigger an interrupt, if enabled.
    fn set_timer_value(&mut self, timer_value: i32);

    /// Sets the compare value. This also affects the down-counter value queried using
    /// [`TimerInterface::timer_value()`].
    fn set_compare_value(&mut self, compare_value: u64);

    /// Unmasks / masks interrupts for the counter.
    fn enable_interrupt(&mut self, enabled: bool);
}

/// Generic timer object allowing blocking wait and interrupt enablement.
pub struct Timer<T: TimerInterface> {
    timer: T,
}

impl<T: TimerInterface> Timer<T> {
    /// Creates new instance.
    pub fn new(timer: T) -> Self {
        Self { timer }
    }

    /// Enables timer.
    pub fn enable(&mut self) {
        self.timer.enable();
    }

    /// Blocking waits for a duration or maximal possible timer. The timer must be enabled before
    /// calling wait.
    pub fn wait(&self, duration: Duration) {
        let ticks = util::duration_to_ticks(duration, self.timer.frequency());
        let increment = u32::try_from(ticks).unwrap_or(u32::MAX);

        let start = self.timer.timer_value();

        // The timer is a down-counter
        while (start.wrapping_sub(self.timer.timer_value()) as u32) < increment {
            spin_loop();
        }
    }

    /// Returns the downcounter value as a duration.
    pub fn remaining_time(&self) -> Duration {
        util::ticks_to_duration(
            u64::try_from(self.timer.timer_value()).unwrap_or(0),
            self.timer.frequency(),
        )
    }

    /// Sets the remaining timer duration.
    ///
    /// If the duration converted to ticks overflows `i32`, TVAL will be set to `i32::MAX` ticks.
    pub fn set_remaining_time(&mut self, duration: Duration) {
        let ticks = i32::try_from(util::duration_to_ticks(duration, self.timer.frequency()))
            .unwrap_or(i32::MAX);

        self.timer.set_timer_value(ticks);
    }

    /// Unmasks interrupts for this timer.
    /// Interrupts and output signals will only be emitted if [`Self::enable()`] has been called.
    pub fn enable_interrupt(&mut self) {
        self.timer.enable_interrupt(true);
    }

    /// Masks interrupts for this timer.
    pub fn disable_interrupt(&mut self) {
        self.timer.enable_interrupt(false);
    }
}

/// Interface for accessing common counter registers.
pub trait CounterInterface {
    /// Returns the up-counter value.
    fn counter_value(&self) -> u64;

    /// Returns the counter frequency in Hz.
    fn frequency(&self) -> u32;
}

/// An up-counter keeping track of elapsed time.
pub struct Counter<C: CounterInterface> {
    counter: C,
    offset: u64,
}

impl<C: CounterInterface> Counter<C> {
    /// Creates a new instance.
    pub fn new(counter: C) -> Self {
        Self { counter, offset: 0 }
    }

    /// Returns the counter value in ticks.
    fn counter_value(&self) -> u64 {
        self.counter.counter_value() - self.offset
    }

    /// Returns the counter value.
    pub fn elapsed_time(&self) -> Duration {
        util::ticks_to_duration(self.counter_value(), self.counter.frequency())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;

    /// Mock timer for use in unit tests.
    ///
    /// Queries of `timer_value` are based on `counter_values` and `compare_value`. On every query,
    /// the next available `counter_value` is used, until the list is exhausted.
    struct MockTimer<'a> {
        enabled: bool,
        frequency: u32,
        counter_values: &'a [u64],
        compare_value: u64,
        value_index: Cell<usize>,
    }

    impl<'a> MockTimer<'a> {
        /// Value representing an arbitrary `i32` returned by querying `TVAL` when the timer is not
        /// enabled.
        pub const UNKNOWN_TVAL: i32 = 0x1234_BCDE;

        /// Creates a new mock timer. `counter_values` are the mock values for physical counts.
        pub fn new(frequency: u32, counter_values: &'a [u64], compare_value: u64) -> Self {
            Self {
                enabled: false,
                frequency,
                counter_values,
                compare_value,
                value_index: Cell::new(0),
            }
        }

        fn next_counter_value(&self) -> u64 {
            let index = self.value_index.get();
            self.value_index.update(|i| i + 1);

            *self
                .counter_values
                .get(index)
                .expect("Mock counter query out of bounds")
        }
    }

    impl<'a> Drop for MockTimer<'a> {
        fn drop(&mut self) {
            assert!(
                self.counter_values.len() <= self.value_index.get(),
                "Not all timer values have been used: {:?}",
                &self.counter_values[self.value_index.get()..]
            );
        }
    }

    impl<'a> TimerInterface for MockTimer<'a> {
        fn enable(&mut self) {
            self.enabled = true;
        }

        fn frequency(&self) -> u32 {
            self.frequency
        }

        fn timer_value(&self) -> i32 {
            if !self.enabled {
                return Self::UNKNOWN_TVAL;
            }

            let cval = self.compare_value;
            let physical_count = self.next_counter_value();

            // intentionally truncated to 32 bits:
            // TimerValue is the low 32 bits of `ZeroExtend{64}((CVAL - PhysicalCountInt)[31:0])`
            cval.wrapping_sub(physical_count) as i32
        }

        fn compare_value(&self) -> u64 {
            self.compare_value
        }

        fn set_timer_value(&mut self, timer_value: i32) {
            self.compare_value = self
                .counter_values
                .get(self.value_index.get())
                .unwrap()
                .wrapping_add(timer_value as u64);
        }

        fn set_compare_value(&mut self, compare_value: u64) {
            self.compare_value = compare_value
        }

        fn enable_interrupt(&mut self, _enabled: bool) {
            unimplemented!()
        }
    }

    #[test]
    fn wait() {
        let mock = MockTimer::new(1000, &[0, 2000, 4000, 5000], 0);

        let mut timer = Timer::new(mock);
        timer.enable();

        timer.wait(Duration::from_secs(5));
    }

    #[test]
    fn wait_overflow() {
        let mock = MockTimer::new(1000, &[1, 1000, 0], 0);

        let mut timer = Timer::new(mock);
        timer.enable();
        timer.wait(Duration::from_secs(u64::MAX));
    }

    #[test]
    fn set_remaining_time() {
        let mock = MockTimer::new(1000, &[5000], 0);
        let mut timer = Timer::new(mock);
        timer.enable();

        timer.set_remaining_time(Duration::from_secs(2));
        assert_eq!(timer.timer.compare_value(), 7000);

        assert_eq!(timer.remaining_time(), Duration::from_secs(2));
    }

    #[test]
    fn remaining_time_cval() {
        let mock = MockTimer::new(1000, &[1000, 2000], 3000);
        let mut timer = Timer::new(mock);
        timer.enable();

        assert_eq!(timer.remaining_time(), Duration::from_secs(2));
        assert_eq!(timer.remaining_time(), Duration::from_secs(1));
    }

    #[test]
    fn disabled_timer() {
        let mock = MockTimer::new(1, &[], 0);
        let timer = Timer::new(mock);

        assert_eq!(
            timer.remaining_time(),
            Duration::from_secs(MockTimer::UNKNOWN_TVAL as u64)
        );
    }

    #[test]
    fn remaining_time() {
        let compare_value = u32::MAX as u64;
        let counter_values = [
            compare_value - i32::MAX.unsigned_abs() as u64,
            compare_value - 1000,
            compare_value,
            compare_value + i32::MIN.unsigned_abs() as u64,
            compare_value + 1,
        ];
        let mock = MockTimer::new(1, &counter_values, compare_value);
        let mut timer = Timer::new(mock);
        timer.enable();

        // TVAL = i32::MAX
        assert_eq!(timer.remaining_time(), Duration::from_secs(i32::MAX as u64));

        // TVAL = 1000
        assert_eq!(timer.remaining_time(), Duration::from_secs(1000));

        // TVAL = 0
        assert_eq!(timer.remaining_time(), Duration::from_secs(0));

        // TVAL = i32::MIN
        assert_eq!(timer.remaining_time(), Duration::from_secs(0));

        // TVAL = -1
        assert_eq!(timer.remaining_time(), Duration::from_secs(0));
    }

    #[test]
    fn wait_negative() {
        let counter_values = [0, i32::MIN.unsigned_abs() as u64];
        let mock = MockTimer::new(1, &counter_values, 0);
        let mut timer = Timer::new(mock);
        timer.enable();

        // wait() will take the "current" timer value, 0 as the start time
        // It will then consume i32::MIN from the MockTimer, after i32::MIN seconds.
        let wait_duration = -(i32::MIN as i64) as u64;

        timer.wait(Duration::from_secs(wait_duration));
    }

    #[test]
    fn wait_around_zero() {
        let mock = MockTimer::new(1, &[0, 1, 2, 3], 1);
        let mut timer = Timer::new(mock);
        timer.enable();

        timer.wait(Duration::from_secs(3));
    }

    #[test]
    fn wait_extremities() {
        let compare_value = i32::MAX as u64;
        let counter_values = [
            0,
            compare_value,
            compare_value + i32::MIN.unsigned_abs() as u64,
        ];
        let mock = MockTimer::new(1, &counter_values, compare_value);
        let mut timer = Timer::new(mock);
        timer.enable();

        let wait_duration = u32::MAX as u64 + 1;

        timer.wait(Duration::from_secs(wait_duration));
    }
}
