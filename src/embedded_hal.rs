// SPDX-FileCopyrightText: Copyright The arm-generic-timer Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Implementation of `embedded-hal` traits for timers.

use core::time::Duration;

use crate::{Timer, TimerInterface};
use embedded_hal::delay::DelayNs;

impl<T: TimerInterface> DelayNs for Timer<T> {
    fn delay_ns(&mut self, ns: u32) {
        self.wait(Duration::from_nanos(ns.into()));
    }

    fn delay_ms(&mut self, ms: u32) {
        self.wait(Duration::from_millis(ms.into()));
    }

    fn delay_us(&mut self, us: u32) {
        self.wait(Duration::from_micros(us.into()));
    }
}
