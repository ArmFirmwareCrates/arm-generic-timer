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
        let ticks =
            u128::from(self.timer.frequency()).saturating_mul(duration.as_micros()) / 1_000_000;
        let increment = u32::try_from(ticks).unwrap_or(u32::MAX);

        let start = self.timer.timer_value();

        // The timer is a down-counter
        while start.wrapping_sub(self.timer.timer_value()) < increment {
            spin_loop();
        }
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
        let nanoseconds =
            u128::from(self.counter_value()) * 1_000_000_000 / u128::from(self.counter.frequency());
        let seconds = (nanoseconds / 1_000_000_000).try_into().unwrap();
        // Can't overflow as the result must always be less than 1_000_000_000, which fits in a u32.
        let subsecond_nanoseconds = (nanoseconds % 1_000_000_000) as u32;
        Duration::new(seconds, subsecond_nanoseconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;

    struct MockTimer<'a> {
        frequency: u32,
        timer_values: &'a [u32],
        value_index: Cell<usize>,
    }

    impl<'a> MockTimer<'a> {
        pub fn new(frequency: u32, timer_values: &'a [u32]) -> Self {
            Self {
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
        fn enable(&mut self) {}

        fn frequency(&self) -> u32 {
            self.frequency
        }

        fn timer_value(&self) -> u32 {
            let index = self.value_index.get();
            self.value_index.update(|i| i + 1);

            self.timer_values[index]
        }
    }

    #[test]
    fn wait() {
        let mock = MockTimer::new(1000, &[7000, 5000, 3000, 2000]);

        let timer = Timer::new(mock);
        timer.wait(Duration::from_secs(5));
    }

    #[test]
    fn wait_overflow() {
        let mock = MockTimer::new(1000, &[2000, 1000, 2001]);

        let timer = Timer::new(mock);
        timer.wait(Duration::from_secs(u64::MAX));
    }
}
