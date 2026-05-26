// SPDX-FileCopyrightText: Copyright The arm-generic-timer Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! System register based timer driver implementations.
//!
//! See D24.10 Generic Timer registers.

use crate::TimerInterface;
#[cfg(feature = "el2")]
use arm_sysregs::{
    CnthpCtlEl2, CnthpsCtlEl2, CnthvCtlEl2, CnthvsCtlEl2, read_cnthp_ctl_el2, read_cnthp_tval_el2,
    read_cnthps_ctl_el2, read_cnthps_tval_el2, read_cnthv_ctl_el2, read_cnthv_tval_el2,
    read_cnthvs_ctl_el2, read_cnthvs_tval_el2, write_cnthp_ctl_el2, write_cnthps_ctl_el2,
    write_cnthv_ctl_el2, write_cnthvs_ctl_el2,
};
use arm_sysregs::{
    CntpCtlEl0, CntvCtlEl0, read_cntfrq_el0, read_cntp_ctl_el0, read_cntp_tval_el0,
    read_cntv_ctl_el0, read_cntv_tval_el0, write_cntp_ctl_el0, write_cntv_ctl_el0,
};
#[cfg(feature = "el1")]
use arm_sysregs::{CntpsCtlEl1, read_cntps_ctl_el1, read_cntps_tval_el1, write_cntps_ctl_el1};

/// Physical Secure Timer
///
/// Uses `CNTPS_*` system registers.
#[cfg(feature = "el1")]
pub struct PhysicalSecureTimer;

#[cfg(feature = "el1")]
impl TimerInterface for PhysicalSecureTimer {
    fn enable(&mut self) {
        let control = read_cntps_ctl_el1();
        write_cntps_ctl_el1(control | CntpsCtlEl1::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> u32 {
        read_cntps_tval_el1().timervalue()
    }
}

/// Hypervisor Physical Timer
///
/// Uses `CNTHP_*` system registers.
#[cfg(feature = "el2")]
pub struct HypervisorPhysicalTimer;

#[cfg(feature = "el2")]
impl TimerInterface for HypervisorPhysicalTimer {
    fn enable(&mut self) {
        let control = read_cnthp_ctl_el2();
        write_cnthp_ctl_el2(control | CnthpCtlEl2::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> u32 {
        read_cnthp_tval_el2().timervalue()
    }
}

/// Secure EL2 Physical Timer
///
/// Uses `CNTHPS_*` system registers.
#[cfg(feature = "el2")]
pub struct SecureEl2PhysicalTimer;

#[cfg(feature = "el2")]
impl TimerInterface for SecureEl2PhysicalTimer {
    fn enable(&mut self) {
        let control = read_cnthps_ctl_el2();
        write_cnthps_ctl_el2(control | CnthpsCtlEl2::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> u32 {
        read_cnthps_tval_el2().timervalue()
    }
}

/// EL2 Virtual Timer
///
/// Uses `CNTHV_*` system registers
#[cfg(feature = "el2")]
pub struct El2VirtualTimer;

#[cfg(feature = "el2")]
impl TimerInterface for El2VirtualTimer {
    fn enable(&mut self) {
        let control = read_cnthv_ctl_el2();
        write_cnthv_ctl_el2(control | CnthvCtlEl2::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> u32 {
        read_cnthv_tval_el2().timervalue()
    }
}

/// Secure EL2 Virtual Timer
///
/// Uses `CNTHVS_*` system registers
#[cfg(feature = "el2")]
pub struct SecureEl2VirtualTimer;

#[cfg(feature = "el2")]
impl TimerInterface for SecureEl2VirtualTimer {
    fn enable(&mut self) {
        let control = read_cnthvs_ctl_el2();
        write_cnthvs_ctl_el2(control | CnthvsCtlEl2::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> u32 {
        read_cnthvs_tval_el2().timervalue()
    }
}

/// Physical Timer
///
/// Uses `CNTP_*` system registers.
pub struct PhysicalTimer;

impl TimerInterface for PhysicalTimer {
    fn enable(&mut self) {
        let control = read_cntp_ctl_el0();
        write_cntp_ctl_el0(control | CntpCtlEl0::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> u32 {
        read_cntp_tval_el0().timervalue()
    }
}

/// Virtual Timer
///
/// Uses `CNTV_*` system registers.
pub struct VirtualTimer;

impl TimerInterface for VirtualTimer {
    fn enable(&mut self) {
        let control = read_cntv_ctl_el0();
        write_cntv_ctl_el0(control | CntvCtlEl0::ENABLE);
    }

    fn frequency(&self) -> u32 {
        read_cntfrq_el0().clockfreq()
    }

    fn timer_value(&self) -> u32 {
        read_cntv_tval_el0().timervalue()
    }
}
