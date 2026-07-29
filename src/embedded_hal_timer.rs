// SPDX-FileCopyrightText: Copyright The arm-generic-timer Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Implementation of `embedded-hal-timer` traits for timers.

use crate::{Counter, CounterInterface};
use embedded_hal_timer::OverflowError;

/// Armv8.0 specifies that the system counter must be at least 56 bits wide. It may be wider, but
/// there doesn't seem to be a way to check.
const MIN_MAX_TICKS: u64 = 0x00ff_ffff_ffff_ffff;

impl<T: CounterInterface> embedded_hal_timer::Timer for Counter<T> {
    fn start(&mut self) {
        self.offset = self.counter_value()
    }

    fn tickrate(&self) -> u64 {
        self.counter.frequency().into()
    }

    fn elapsed_ticks(&self) -> Result<u64, OverflowError> {
        // The counter is guaranteed not to roll over in less than 40 years, so assume that never
        // happens.
        Ok(self.counter_value())
    }

    fn elapsed_nanos(&self) -> Result<u64, OverflowError> {
        // TODO: Can we avoid overflow in the first multiplication without using u128 or losing
        // precision?
        (u128::from(self.elapsed_ticks()?) * 1_000_000_000 / u128::from(self.tickrate()))
            .try_into()
            .map_err(|_| OverflowError)
    }

    fn elapsed_micros(&self) -> Result<u64, OverflowError> {
        (u128::from(self.elapsed_ticks()?) * 1_000_000 / u128::from(self.tickrate()))
            .try_into()
            .map_err(|_| OverflowError)
    }

    fn elapsed_millis(&self) -> Result<u64, OverflowError> {
        (u128::from(self.elapsed_ticks()?) * 1000 / u128::from(self.tickrate()))
            .try_into()
            .map_err(|_| OverflowError)
    }

    fn elapsed_secs(&self) -> Result<u64, OverflowError> {
        Ok(self.elapsed_ticks()? / self.tickrate())
    }

    fn max_ticks(&self) -> u64 {
        MIN_MAX_TICKS - self.counter.counter_value()
    }

    fn max_nanos(&self) -> u64 {
        (u128::from(self.max_ticks()) * 1_000_000_000 / u128::from(self.tickrate()))
            .try_into()
            .unwrap_or(u64::MAX)
    }

    fn max_micros(&self) -> u64 {
        (u128::from(self.max_ticks()) * 1_000_000 / u128::from(self.tickrate()))
            .try_into()
            .unwrap_or(u64::MAX)
    }

    fn max_millis(&self) -> u64 {
        (u128::from(self.max_ticks()) * 1000 / u128::from(self.tickrate()))
            .try_into()
            .unwrap_or(u64::MAX)
    }

    fn max_secs(&self) -> u64 {
        self.max_ticks() / self.tickrate()
    }
}
