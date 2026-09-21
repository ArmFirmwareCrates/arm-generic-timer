// SPDX-FileCopyrightText: Copyright The arm-generic-timer Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! System register based timer driver implementations.
//!
//! See D24.10 Generic Timer registers.

use crate::{CounterInterface, TimerInterface};
use arm_sysregs::el0::{
    accessors::{
        read_cntfrq_el0, read_cntp_ctl_el0, read_cntp_cval_el0, read_cntp_tval_el0,
        read_cntpct_el0, read_cntv_ctl_el0, read_cntv_cval_el0, read_cntv_tval_el0,
        read_cntvct_el0, write_cntp_ctl_el0, write_cntp_cval_el0, write_cntp_tval_el0,
        write_cntv_ctl_el0, write_cntv_cval_el0, write_cntv_tval_el0,
    },
    registers::{CntpCtlEl0, CntpCvalEl0, CntpTvalEl0, CntvCtlEl0, CntvCvalEl0, CntvTvalEl0},
};
#[cfg(feature = "el1")]
use arm_sysregs::el1::{
    accessors::{
        read_cntps_ctl_el1, read_cntps_cval_el1, read_cntps_tval_el1, write_cntps_ctl_el1,
        write_cntps_cval_el1, write_cntps_tval_el1,
    },
    registers::{CntpsCtlEl1, CntpsCvalEl1, CntpsTvalEl1},
};
#[cfg(feature = "el2")]
use arm_sysregs::el2::{
    accessors::{
        read_cnthp_ctl_el2, read_cnthp_cval_el2, read_cnthp_tval_el2, read_cnthps_ctl_el2,
        read_cnthps_cval_el2, read_cnthps_tval_el2, read_cnthv_ctl_el2, read_cnthv_cval_el2,
        read_cnthv_tval_el2, read_cnthvs_ctl_el2, read_cnthvs_cval_el2, read_cnthvs_tval_el2,
        write_cnthp_ctl_el2, write_cnthp_cval_el2, write_cnthp_tval_el2, write_cnthps_ctl_el2,
        write_cnthps_cval_el2, write_cnthps_tval_el2, write_cnthv_ctl_el2, write_cnthv_cval_el2,
        write_cnthv_tval_el2, write_cnthvs_ctl_el2, write_cnthvs_cval_el2, write_cnthvs_tval_el2,
    },
    registers::{
        CnthpCtlEl2, CnthpCvalEl2, CnthpTvalEl2, CnthpsCtlEl2, CnthpsCvalEl2, CnthpsTvalEl2,
        CnthvCtlEl2, CnthvCvalEl2, CnthvTvalEl2, CnthvsCtlEl2, CnthvsCvalEl2, CnthvsTvalEl2,
    },
};

/// Physical Secure Timer
///
/// Uses `CNTPS_*` system registers.
#[cfg(feature = "el1")]
pub struct PhysicalSecureTimer(());

#[cfg(feature = "el1")]
impl PhysicalSecureTimer {
    /// Creates an instance of the Physical Secure Timer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no other instance exists and that there is
    /// no concurrent access to the CNTPS_* system registers.
    pub const unsafe fn new() -> Self {
        Self(())
    }
}

#[cfg(feature = "el1")]
impl TimerInterface for PhysicalSecureTimer {
    fn enable(&mut self) {
        let control = read_cntps_ctl_el1();
        write_cntps_ctl_el1(control | CntpsCtlEl1::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> i32 {
        read_cntps_tval_el1().timervalue() as i32
    }

    fn enable_interrupt(&mut self, enabled: bool) {
        let mut control = read_cntps_ctl_el1();
        control.set(CntpsCtlEl1::IMASK, !enabled);

        write_cntps_ctl_el1(control);
    }

    fn compare_value(&self) -> u64 {
        read_cntps_cval_el1().comparevalue()
    }

    fn set_timer_value(&mut self, timer_value: i32) {
        write_cntps_tval_el1(CntpsTvalEl1::empty().with_timervalue(timer_value));
    }

    fn set_compare_value(&mut self, compare_value: u64) {
        write_cntps_cval_el1(CntpsCvalEl1::empty().with_comparevalue(compare_value));
    }
}

/// Hypervisor Physical Timer
///
/// Uses `CNTHP_*` system registers.
#[cfg(feature = "el2")]
pub struct HypervisorPhysicalTimer(());

#[cfg(feature = "el2")]
impl HypervisorPhysicalTimer {
    /// Creates an instance of the Hypervisor Physical Timer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no other instance exists and that there is
    /// no concurrent access to the CNTHP_* system registers.
    pub const unsafe fn new() -> Self {
        Self(())
    }
}

#[cfg(feature = "el2")]
impl TimerInterface for HypervisorPhysicalTimer {
    fn enable(&mut self) {
        let control = read_cnthp_ctl_el2();
        write_cnthp_ctl_el2(control | CnthpCtlEl2::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> i32 {
        read_cnthp_tval_el2().timervalue() as i32
    }

    fn enable_interrupt(&mut self, enabled: bool) {
        let mut control = read_cnthp_ctl_el2();
        control.set(CnthpCtlEl2::IMASK, !enabled);

        write_cnthp_ctl_el2(control);
    }

    fn compare_value(&self) -> u64 {
        read_cnthp_cval_el2().comparevalue()
    }

    fn set_timer_value(&mut self, timer_value: i32) {
        write_cnthp_tval_el2(CnthpTvalEl2::empty().with_timervalue(timer_value));
    }

    fn set_compare_value(&mut self, compare_value: u64) {
        write_cnthp_cval_el2(CnthpCvalEl2::empty().with_comparevalue(compare_value));
    }
}

/// Secure EL2 Physical Timer
///
/// Uses `CNTHPS_*` system registers.
#[cfg(feature = "el2")]
pub struct SecureEl2PhysicalTimer(());

#[cfg(feature = "el2")]
impl SecureEl2PhysicalTimer {
    /// Creates an instance of the Secure EL2 Physical Timer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no other instance exists and that there is
    /// no concurrent access to the CNTHPS_* system registers.
    pub const unsafe fn new() -> Self {
        Self(())
    }
}

#[cfg(feature = "el2")]
impl TimerInterface for SecureEl2PhysicalTimer {
    fn enable(&mut self) {
        let control = read_cnthps_ctl_el2();
        write_cnthps_ctl_el2(control | CnthpsCtlEl2::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> i32 {
        read_cnthps_tval_el2().timervalue() as i32
    }

    fn enable_interrupt(&mut self, enabled: bool) {
        let mut control = read_cnthps_ctl_el2();
        control.set(CnthpsCtlEl2::IMASK, !enabled);

        write_cnthps_ctl_el2(control);
    }

    fn compare_value(&self) -> u64 {
        read_cnthps_cval_el2().comparevalue()
    }

    fn set_timer_value(&mut self, timer_value: i32) {
        write_cnthps_tval_el2(CnthpsTvalEl2::empty().with_timervalue(timer_value));
    }

    fn set_compare_value(&mut self, compare_value: u64) {
        write_cnthps_cval_el2(CnthpsCvalEl2::empty().with_comparevalue(compare_value));
    }
}

/// EL2 Virtual Timer
///
/// Uses `CNTHV_*` system registers
#[cfg(feature = "el2")]
pub struct El2VirtualTimer(());

#[cfg(feature = "el2")]
impl El2VirtualTimer {
    /// Creates an instance of the EL2 Virtual Timer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no other instance exists and that there is
    /// no concurrent access to the CNTHV_* system registers.
    pub const unsafe fn new() -> Self {
        Self(())
    }
}

#[cfg(feature = "el2")]
impl TimerInterface for El2VirtualTimer {
    fn enable(&mut self) {
        let control = read_cnthv_ctl_el2();
        write_cnthv_ctl_el2(control | CnthvCtlEl2::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> i32 {
        read_cnthv_tval_el2().timervalue() as i32
    }

    fn enable_interrupt(&mut self, enabled: bool) {
        let mut control = read_cnthv_ctl_el2();
        control.set(CnthvCtlEl2::IMASK, !enabled);

        write_cnthv_ctl_el2(control);
    }

    fn compare_value(&self) -> u64 {
        read_cnthv_cval_el2().comparevalue()
    }

    fn set_timer_value(&mut self, timer_value: i32) {
        write_cnthv_tval_el2(CnthvTvalEl2::empty().with_timervalue(timer_value));
    }

    fn set_compare_value(&mut self, compare_value: u64) {
        write_cnthv_cval_el2(CnthvCvalEl2::empty().with_comparevalue(compare_value));
    }
}

/// Secure EL2 Virtual Timer
///
/// Uses `CNTHVS_*` system registers
#[cfg(feature = "el2")]
pub struct SecureEl2VirtualTimer(());

#[cfg(feature = "el2")]
impl SecureEl2VirtualTimer {
    /// Creates an instance of the Secure EL2 Virtual Timer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no other instance exists and that there is
    /// no concurrent access to the CNTHVS_* system registers.
    pub const unsafe fn new() -> Self {
        Self(())
    }
}

#[cfg(feature = "el2")]
impl TimerInterface for SecureEl2VirtualTimer {
    fn enable(&mut self) {
        let control = read_cnthvs_ctl_el2();
        write_cnthvs_ctl_el2(control | CnthvsCtlEl2::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> i32 {
        read_cnthvs_tval_el2().timervalue() as i32
    }

    fn enable_interrupt(&mut self, enabled: bool) {
        let mut control = read_cnthvs_ctl_el2();
        control.set(CnthvsCtlEl2::IMASK, !enabled);

        write_cnthvs_ctl_el2(control);
    }

    fn compare_value(&self) -> u64 {
        read_cnthvs_cval_el2().comparevalue()
    }

    fn set_timer_value(&mut self, timer_value: i32) {
        write_cnthvs_tval_el2(CnthvsTvalEl2::empty().with_timervalue(timer_value));
    }

    fn set_compare_value(&mut self, compare_value: u64) {
        write_cnthvs_cval_el2(CnthvsCvalEl2::empty().with_comparevalue(compare_value));
    }
}

/// Physical Timer
///
/// Uses `CNTP_*` system registers.
pub struct PhysicalTimer(());

impl PhysicalTimer {
    /// Creates an instance of the Physical Timer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no other instance exists and that there is
    /// no concurrent access to the CNTP_* system registers.
    pub const unsafe fn new() -> Self {
        Self(())
    }
}

impl TimerInterface for PhysicalTimer {
    fn enable(&mut self) {
        let control = read_cntp_ctl_el0();
        write_cntp_ctl_el0(control | CntpCtlEl0::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> i32 {
        read_cntp_tval_el0().timervalue() as i32
    }

    fn enable_interrupt(&mut self, enabled: bool) {
        let mut control = read_cntp_ctl_el0();
        control.set(CntpCtlEl0::IMASK, !enabled);

        write_cntp_ctl_el0(control);
    }

    fn compare_value(&self) -> u64 {
        read_cntp_cval_el0().comparevalue()
    }

    fn set_timer_value(&mut self, timer_value: i32) {
        write_cntp_tval_el0(CntpTvalEl0::empty().with_timervalue(timer_value));
    }

    fn set_compare_value(&mut self, compare_value: u64) {
        write_cntp_cval_el0(CntpCvalEl0::empty().with_comparevalue(compare_value));
    }
}

/// Virtual Timer
///
/// Uses `CNTV_*` system registers.
pub struct VirtualTimer(());

impl VirtualTimer {
    /// Creates an instance of the Virtual Timer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no other instance exists and that there is
    /// no concurrent access to the CNTV_* system registers.
    pub const unsafe fn new() -> Self {
        Self(())
    }
}

impl TimerInterface for VirtualTimer {
    fn enable(&mut self) {
        let control = read_cntv_ctl_el0();
        write_cntv_ctl_el0(control | CntvCtlEl0::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> i32 {
        read_cntv_tval_el0().timervalue() as i32
    }

    fn enable_interrupt(&mut self, enabled: bool) {
        let mut control = read_cntv_ctl_el0();
        control.set(CntvCtlEl0::IMASK, !enabled);

        write_cntv_ctl_el0(control);
    }

    fn compare_value(&self) -> u64 {
        read_cntv_cval_el0().comparevalue()
    }

    fn set_timer_value(&mut self, timer_value: i32) {
        write_cntv_tval_el0(CntvTvalEl0::empty().with_timervalue(timer_value));
    }

    fn set_compare_value(&mut self, compare_value: u64) {
        write_cntv_cval_el0(CntvCvalEl0::empty().with_comparevalue(compare_value));
    }
}

/// Physical Counter.
pub struct PhysicalCounter;

impl CounterInterface for PhysicalCounter {
    fn counter_value(&self) -> u64 {
        read_cntpct_el0().physicalcount()
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }
}

/// Virtual Counter.
pub struct VirtualCounter;

impl CounterInterface for VirtualCounter {
    fn counter_value(&self) -> u64 {
        read_cntvct_el0().virtualcount()
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }
}
