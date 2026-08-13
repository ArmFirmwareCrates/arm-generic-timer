// SPDX-FileCopyrightText: Copyright The arm-generic-timer Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::time::Duration;

/// Converts timer ticks to a duration, based on a timer frequency.
pub fn ticks_to_duration(ticks: u64, freq: u32) -> Duration {
    let nanoseconds = u128::from(ticks) * 1_000_000_000 / u128::from(freq);
    let seconds = (nanoseconds / 1_000_000_000).try_into().unwrap();
    // Can't overflow as the result must always be less than 1_000_000_000, which fits in a u32.
    let subsecond_nanoseconds = (nanoseconds % 1_000_000_000) as u32;
    Duration::new(seconds, subsecond_nanoseconds)
}

/// Converts a duration to timer ticks, assuming the timer is running on a given frequency.
///
/// When the given duration and frequency duration would overflow `u64`, `u64::MAX` is returned.
pub fn duration_to_ticks(duration: Duration, freq: u32) -> u64 {
    let nanos = duration.as_nanos();

    u64::try_from(nanos.saturating_mul(u128::from(freq)) / 1_000_000_000).unwrap_or(u64::MAX)
}
