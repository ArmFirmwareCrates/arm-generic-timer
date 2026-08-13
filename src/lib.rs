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
    fn timer_value(&self) -> u32;
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
        while start.wrapping_sub(self.timer.timer_value()) < increment {
            spin_loop();
        }
    }

    /// Returns the downcounter value as a duration.
    pub fn remaining_time(&self) -> Duration {
        util::ticks_to_duration(u64::from(self.timer.timer_value()), self.timer.frequency())
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

    struct MockTimer<'a> {
        enabled: bool,
        frequency: u32,
        timer_values: &'a [u32],
        value_index: Cell<usize>,
    }

    impl<'a> MockTimer<'a> {
        /// Value representing an arbitrary `u32` returned by querying `TVAL` when the timer is not
        /// enabled.
        pub const UNKNOWN_TVAL: u32 = 0x1234_BCDE;

        pub fn new(frequency: u32, timer_values: &'a [u32]) -> Self {
            Self {
                enabled: false,
                frequency,
                timer_values,
                value_index: Cell::new(0),
            }
        }
    }

    impl<'a> Drop for MockTimer<'a> {
        fn drop(&mut self) {
            assert!(
                self.timer_values.len() == self.value_index.get(),
                "Not all timer values have been used: {:?}",
                &self.timer_values[self.value_index.get()..]
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

        fn timer_value(&self) -> u32 {
            if !self.enabled {
                return Self::UNKNOWN_TVAL;
            }

            let index = self.value_index.get();
            self.value_index.update(|i| i + 1);

            self.timer_values[index]
        }
    }

    #[test]
    fn wait() {
        let mock = MockTimer::new(1000, &[7000, 5000, 3000, 2000]);

        let mut timer = Timer::new(mock);
        timer.enable();

        timer.wait(Duration::from_secs(5));
    }

    #[test]
    fn wait_overflow() {
        let mock = MockTimer::new(1000, &[2000, 1000, 2001]);

        let mut timer = Timer::new(mock);
        timer.enable();
        timer.wait(Duration::from_secs(u64::MAX));
    }

    #[test]
    fn disabled_timer() {
        let mock = MockTimer::new(1, &[]);
        let timer = Timer::new(mock);

        assert_eq!(
            timer.remaining_time(),
            Duration::from_secs(MockTimer::UNKNOWN_TVAL as u64)
        );
    }
}
