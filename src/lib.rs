// SPDX-FileCopyrightText: Copyright The arm-generic-timer Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_std]
#![doc = include_str!("../README.md")]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::time::Duration;

use bitflags::bitflags;
use safe_mmio::{
    field, field_shared,
    fields::{ReadPure, ReadPureWrite},
    UniqueMmioPointer,
};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// Counter Control Register
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub struct CntCr(u32);

/// Counter Status Register
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub struct CntSr(u32);

/// Counter Identification Register.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub struct CntId(u32);

/// Counter-timer Access Control Register.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub struct CntAcr(u32);

/// Timer feature bits, defined at I5.7.16 CNTTIDR, Counter-timer Timer ID Register description.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub struct Features(u8);

/// Counter-timer EL0 Access Control Register.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub struct CntEl0Acr(u32);

/// Common control register of the physical and virtual timers. Defined at I5.7.10 CNTP_CTL,
/// Counter-timer Physical Timer Control and at CNTV_CTL, Counter-timer Virtual Timer Control.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub struct TimerControl(u32);

bitflags! {
    impl CntCr: u32 {
        /// Scaling is enabled. This bit depends on the presence of FEAT_CNTSC.
        const SCEN = 1 << 2;
        /// Halt-on-debug.
        const HDBG = 1 << 1;
        /// System counter enabled.
        const EN = 1 << 0;
    }

    impl CntSr: u32 {
        /// Halt-on-debug.
        const HDBG = 1 << 1;
    }

    impl CntAcr: u32 {
        /// Read/write access to the EL1 Physical Timer registers
        const RWPT = 1 << 5;
        /// Read/write access to the Virtual Timer registers
        const RWVT = 1 << 4;
        /// Read-only access to CNTVOFF
        const RVOFF = 1 << 3;
        /// Read-only access to CNTFRQ
        const RFRQ = 1 << 2;
        /// Read-only access to CNTVCT
        const RVCT = 1 << 1;
        /// Read-only access to CNTPCT
        const RPCT = 1 << 0;
    }

    impl Features: u8 {
        /// Frame<n> has a second view, CNTEL0Base<n>.
        const CNTEL0BASE = 1 << 2;
        /// Frame<n> has virtual capability. The virtual time and offset registers are implemented.
        const VIRTUAL = 1 << 1;
        /// Frame<n> is implemented.
        const IMPLEMENTED = 1 << 0;
    }

    impl CntEl0Acr: u32 {
        /// Second view read access control for CNTP_CVAL, CNTP_TVAL, and CNTP_CTL.
        const EL0PTEN = 1 << 9;
        /// Second view read access control for CNTV_CVAL, CNTV_TVAL, and CNTV_CTL.
        const EL0VTEN = 1 << 8;
        /// Second view read access control for CNTVCT and CNTFRQ.
        const EL0VCTEN = 1 << 1;
        /// Second view read access control for CNTPCT and CNTFRQ.
        const EL0PCTEN = 1 << 0;
    }

    impl TimerControl: u32 {
        /// Timer condition is met.
        const ISTATUS = 1 << 2;
        /// Timer interrupt is masked.
        const IMASK = 1 << 1;
        /// Timer enabled.
        const ENABLE = 1 << 0;
    }
}

impl CntCr {
    const FCREQ_MASK: u32 = 0x0000_03ff;
    const FCREQ_SHIFT: u32 = 8;

    /// Write FCREQ field of the register.
    pub fn set_fcreq(&mut self, index: usize) {
        let mut value = self.0 & !(Self::FCREQ_MASK << Self::FCREQ_SHIFT);
        value |= ((index as u32) & Self::FCREQ_MASK) << Self::FCREQ_SHIFT;
        self.0 = value;
    }
}

impl CntSr {
    const FCACK_MASK: u32 = 0x0000_03ff;
    const FCACK_SHIFT: u32 = 8;

    /// Read FCACK field of the register.
    pub fn fcack(&self) -> usize {
        ((self.0 >> Self::FCACK_SHIFT) & Self::FCACK_MASK) as usize
    }
}

impl CntId {
    const CNTSC_MASK: u32 = 0b1111;
    const CNTSC_IMPLEMENTED: u32 = 0b0001;

    /// Indicates whether Counter Scaling is implemented.
    pub fn scaling_implemented(&self) -> bool {
        self.0 & Self::CNTSC_MASK == Self::CNTSC_IMPLEMENTED
    }
}

/// Table I2-1 CNTControlBase memory map
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct CntControlBase {
    /// 0x000 Counter Control Register
    cntcr: ReadPureWrite<CntCr>,
    /// 0x004 Counter Status Register
    cntsr: ReadPure<CntSr>,
    /// 0x008 Counter Count Value register
    cntcv: ReadPureWrite<u64>,
    /// 0x010 Counter Counter Scale register
    cntscr: ReadPureWrite<u32>,
    reserved_14: [u32; 2],
    /// 0x01c Counter ID register
    cntid: ReadPure<CntId>,
    /// 0x020 Counter Frequency IDs
    cntfid: [ReadPureWrite<u32>; 40],
    /// 0x0c0 Implementation defined
    impdef_0c0: [u32; 16],
    reserved_100: [u32; 948],
    /// 0xfd0 Counter ID registers
    counter_id: [ReadPure<u32>; 12],
}

/// Table I2-2 CNTReadBase memory map
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct CntReadBase {
    /// 0x000 Counter Count Value register
    cntcv: ReadPure<u64>,
    reserved_8: [u32; 1010],
    /// 0xfd0 Counter ID registers
    counter_id: [ReadPure<u32>; 12],
}

/// Table I2-3 CNTCTLBase memory map
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct CntCtlBase {
    /// 0x000 Counter-timer Frequency
    cntfrq: ReadPureWrite<u32>,
    /// 0x004 Counter-timer Non-secure Access Register
    cntnsar: ReadPureWrite<u32>,
    /// 0x008 Counter-timer Timer ID Register
    cnttidr: ReadPure<u32>,
    reserved_00c: [u32; 13],
    /// 0x040 Counter-timer Access Control Registers
    cntacr: [ReadPureWrite<CntAcr>; 8],
    reserved_060: [u32; 8],
    /// 0x080 Counter-timer Virtual Offsets
    cntvoff: [ReadPureWrite<u64>; 8],
    reserved_0c0: [u32; 16],
    /// 0x100 Implementation defined
    impdef_100: [u32; 448],
    reserved_800: [u32; 496],
    impdef_fc0: [u32; 4],
    /// 0xfd0 Counter ID registers
    counter_id: [ReadPure<u32>; 12],
}

/// Repeated subset of register that describe a physical or virtual timer in the CntBase or
/// CntEl0Base blocks.
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct TimerRegs {
    /// 0x000 Counter-timer Timer CompareValue
    cval: ReadPureWrite<u64>,
    /// 0x008 Counter-timer Timer TimerValue
    tval: ReadPureWrite<u32>,
    /// 0x00c Counter-timer Timer Control
    ctl: ReadPureWrite<TimerControl>,
}

/// Table I2-4 CNTBaseN memory map
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct CntBase {
    /// 0x000 Counter-timer Physical Count
    cntpct: ReadPure<u64>,
    /// 0x008 Counter-timer Virtual Count
    cntvct: ReadPure<u64>,
    /// 0x010 Counter-timer Frequency
    cntfrq: ReadPure<u32>,
    /// 0x014 Counter-timer EL0 Access Control Register
    cntel0acr: ReadPureWrite<CntEl0Acr>,
    /// 0x018 Counter-timer Virtual Offset
    cntvoff: ReadPure<u64>,
    /// 0x020-0x02c Physical timer block
    cntp: TimerRegs,
    /// 0x030-0x03c Virtual timer block
    cntv: TimerRegs,
    reserved: [u32; 996],
    /// 0xfd0 Counter ID registers
    counter_id: [ReadPure<u32>; 12],
}

/// CntEl0Base frame is identical to the CntBase frame, except that CNTVOFF, CNTEL0ACR registers are
/// never visible and CNTEL0ACR of the corresponding CntBase controls the access of the physical and
/// virtual timer registers.
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct CntEl0Base {
    /// 0x000 Counter-timer Physical Count
    cntpct: ReadPure<u64>,
    /// 0x008 Counter-timer Virtual Count
    cntvct: ReadPure<u64>,
    /// 0x010 Counter-timer Frequency
    cntfrq: ReadPure<u32>,
    reserved_014: [u32; 3],
    /// 0x020-0x02c Physical timer block
    cntp: TimerRegs,
    /// 0x030-0x03c Virtual timer block
    cntv: TimerRegs,
    reserved: [u32; 996],
    /// 0xfd0 Counter ID registers
    counter_id: [ReadPure<u32>; 12],
}

/// Driver for the CNTControlBase block.
pub struct GenericTimerControl<'a> {
    regs: UniqueMmioPointer<'a, CntControlBase>,
}

impl<'a> GenericTimerControl<'a> {
    /// Creates new instance.
    pub fn new(regs: UniqueMmioPointer<'a, CntControlBase>) -> Self {
        Self { regs }
    }

    /// Enables or disables the timer.
    pub fn set_enable(&mut self, enable: bool) {
        let mut cntcr = field!(self.regs, cntcr).read();
        cntcr.set(CntCr::EN, enable);
        field!(self.regs, cntcr).write(cntcr);
    }

    /// Sets the number of the entry in the Frequency modes table to select.
    pub fn request_frequency(&mut self, index: usize) {
        let mut cntcr = field!(self.regs, cntcr).read();
        cntcr.set_fcreq(index);
        field!(self.regs, cntcr).write(cntcr);
    }

    /// Gets currently selected entry index in the Frequency modes table.
    pub fn frequency_index(&self) -> usize {
        field_shared!(self.regs, cntsr).read().fcack()
    }

    /// Gets timer count value.
    pub fn count(&self) -> u64 {
        field_shared!(self.regs, cntcv).read()
    }

    /// Sets timer count value.
    pub fn set_count(&mut self, count: u64) {
        field!(self.regs, cntcv).write(count);
    }

    /// Checks whether scaling is implemented by the timer.
    pub fn scaling_implemented(&self) -> bool {
        field_shared!(self.regs, cntid).read().scaling_implemented()
    }

    /// Gets scale value.
    pub fn scale(&self) -> u32 {
        field_shared!(self.regs, cntscr).read()
    }

    /// Sets scale and enable scaling.
    pub fn enable_scaling(&mut self, scale: u32) {
        field!(self.regs, cntscr).write(scale);
        let cntcr = field!(self.regs, cntcr).read();
        field!(self.regs, cntcr).write(cntcr | CntCr::SCEN);
    }

    /// Disables scaling.
    pub fn disable_scaling(&mut self) {
        let cntcr = field!(self.regs, cntcr).read();
        field!(self.regs, cntcr).write(cntcr - CntCr::SCEN);
        field!(self.regs, cntscr).write(0);
    }

    /// Indicates the base frequency of the system counter in Hz.
    pub fn base_frequency(&self) -> u32 {
        field_shared!(self.regs, cntfid).get(0).unwrap().read()
    }

    /// Gets frequency mode of the given index in Hz. The availablity of the frequency mode is
    /// implementation defined.
    pub fn frequency_mode(&self, index: usize) -> Option<u32> {
        let frequency = field_shared!(self.regs, cntfid).get(index).unwrap().read();

        if frequency != 0 {
            Some(frequency)
        } else {
            None
        }
    }

    /// Sets frequency mode of the given index. The availablity of the frequency mode is
    /// implementation defined.
    pub fn set_frequency_mode(&mut self, index: usize, frequency: u32) {
        field!(self.regs, cntfid)
            .get(index)
            .unwrap()
            .write(frequency)
    }
}

/// Driver for the CNTCTLBase block.
pub struct GenericTimerCtl<'a> {
    regs: UniqueMmioPointer<'a, CntCtlBase>,
}

impl<'a> GenericTimerCtl<'a> {
    /// Creates new instance.
    pub fn new(regs: UniqueMmioPointer<'a, CntCtlBase>) -> Self {
        Self { regs }
    }

    /// Gets counter frequency in Hz.
    pub fn frequency(&self) -> u32 {
        field_shared!(self.regs, cntfrq).read()
    }

    /// Sets counter frequency in Hz.
    pub fn set_frequency(&mut self, frequency: u32) {
        field!(self.regs, cntfrq).write(frequency);
    }

    /// Gets non-secure access state.
    pub fn non_secure_access(&self, index: usize) -> bool {
        assert!(index < 8);

        let cntnsar = field_shared!(self.regs, cntnsar).read();
        cntnsar & (1 << index) != 0
    }

    /// Provides the highest-level control of whether frames CNTBaseN and CNTEL0BaseN are accessible
    /// by Non-secure accesses.
    pub fn set_non_secure_access(&mut self, index: usize, enable: bool) {
        assert!(index < 8);

        let mut cntnsar = field_shared!(self.regs, cntnsar).read();
        if enable {
            cntnsar |= 1 << index;
        } else {
            cntnsar &= !(1 << index);
        }
        field!(self.regs, cntnsar).write(cntnsar);
    }

    /// Queries features of the timer.
    pub fn features(&self, index: usize) -> Features {
        assert!(index < 8);

        let cnttidr = field_shared!(self.regs, cnttidr).read();
        Features::from_bits_truncate((cnttidr >> (index * 8)) as u8)
    }

    /// Gets current top-level access controls for the elements of a timer frame.
    pub fn access_control(&self, index: usize) -> CntAcr {
        field_shared!(self.regs, cntacr).get(index).unwrap().read()
    }

    /// Sets top-level access controls for the elements of a timer frame.
    pub fn set_access_control(&mut self, index: usize, cntacr: CntAcr) {
        field!(self.regs, cntacr).get(index).unwrap().write(cntacr);
    }

    /// Gets the 64-bit virtual offset for frame CNTBase.
    pub fn virtual_offset(&self, index: usize) -> u64 {
        field_shared!(self.regs, cntvoff).get(index).unwrap().read()
    }

    /// Sets the 64-bit virtual offset for frame CNTBase. This is the offset between real time
    /// and virtual time.
    pub fn set_virtual_offset(&mut self, index: usize, offset: u64) {
        field!(self.regs, cntvoff).get(index).unwrap().write(offset);
    }
}

/// Driver for the physical or virtual timer instance of the CNTBase block.
pub struct Timer<'a> {
    regs: UniqueMmioPointer<'a, TimerRegs>,
    frequency: u32,
}

impl<'a> Timer<'a> {
    /// Creates new instance.
    pub fn new(regs: UniqueMmioPointer<'a, TimerRegs>, frequency: u32) -> Self {
        Self { regs, frequency }
    }

    /// Sets up timer to generate an interrupt after the given duration.
    ///
    /// # Safety
    ///
    /// The system must be prepared to take an interrupt. The vector table has to be set and the
    /// interrupt controller must be configured properly.
    pub unsafe fn generate_interrupt_after(&mut self, duration: Duration) {
        self.set_deadline(duration);
        self.set_control(TimerControl::ENABLE);
    }

    /// Disables the timer and masks the interrupt.
    pub fn cancel_interrupt(&mut self) {
        self.set_control(TimerControl::IMASK);
    }

    /// Blocking waits for a duration.
    pub fn wait(&mut self, duration: Duration) {
        self.set_deadline(duration);
        self.set_control(TimerControl::ENABLE | TimerControl::IMASK);

        while !self.control().contains(TimerControl::ISTATUS) {
            core::hint::spin_loop();
        }
    }

    /// Sets the compare register to trigger after the given duration.
    fn set_deadline(&mut self, duration: Duration) {
        let increment = self.frequency as u64 * duration.as_micros() as u64 / 1_000_000;

        let value = field!(self.regs, cval).read();
        field!(self.regs, cval).write(value + increment);
    }

    /// Reads CTL register.
    fn control(&self) -> TimerControl {
        field_shared!(self.regs, ctl).read()
    }

    /// Sets CTL register.
    fn set_control(&mut self, control: TimerControl) {
        field!(self.regs, ctl).write(control)
    }
}

/// Driver for the CNTBase timer block.
pub struct GenericTimerCnt<'a> {
    regs: UniqueMmioPointer<'a, CntBase>,
}

impl<'a> GenericTimerCnt<'a> {
    /// Creates new instance.
    pub fn new(regs: UniqueMmioPointer<'a, CntBase>) -> Self {
        Self { regs }
    }

    /// Gets physical count.
    pub fn physical_count(&self) -> u64 {
        field_shared!(self.regs, cntpct).read()
    }

    /// Gets virtual count.
    pub fn virtual_count(&self) -> u64 {
        field_shared!(self.regs, cntvct).read()
    }

    /// Gets frequency in Hz.
    pub fn frequency(&self) -> u32 {
        field_shared!(self.regs, cntfrq).read()
    }

    /// Gets second view access rights.
    pub fn el0_access(&self) -> CntEl0Acr {
        field_shared!(self.regs, cntel0acr).read()
    }

    /// Sets second view access rights.
    pub fn set_el0_access(&mut self, value: CntEl0Acr) {
        field!(self.regs, cntel0acr).write(value)
    }

    /// Gets the 64-bit virtual offset for frame CNTBase.
    pub fn virtual_offset(&self) -> u64 {
        field_shared!(self.regs, cntvoff).read()
    }

    /// Gets physical timer.
    pub fn physical_timer(&mut self) -> Timer<'_> {
        let frequency = self.frequency();
        Timer::new(field!(self.regs, cntp), frequency)
    }

    /// Gets virtual timer.
    pub fn virtual_timer(&mut self) -> Timer<'_> {
        let frequency = self.frequency();
        Timer::new(field!(self.regs, cntv), frequency)
    }
}

/// Driver for the CNTEL0Base timer block.
pub struct GenericTimerCntEl0<'a> {
    regs: UniqueMmioPointer<'a, CntEl0Base>,
}

impl<'a> GenericTimerCntEl0<'a> {
    /// Creates new instance.
    pub fn new(regs: UniqueMmioPointer<'a, CntEl0Base>) -> Self {
        Self { regs }
    }

    /// Gets physical count.
    pub fn physical_count(&self) -> u64 {
        field_shared!(self.regs, cntpct).read()
    }

    /// Gets virtual count.
    pub fn virtual_count(&self) -> u64 {
        field_shared!(self.regs, cntvct).read()
    }

    /// Gets frequency in Hz.
    pub fn frequency(&self) -> u32 {
        field_shared!(self.regs, cntfrq).read()
    }

    /// Gets physical timer.
    pub fn physical_timer(&mut self) -> Timer<'_> {
        let frequency = self.frequency();
        Timer::new(field!(self.regs, cntp), frequency)
    }

    /// Gets virtual timer.
    pub fn virtual_timer(&mut self) -> Timer<'_> {
        let frequency = self.frequency();
        Timer::new(field!(self.regs, cntv), frequency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes() {
        assert_eq!(0x1000, core::mem::size_of::<CntControlBase>());
        assert_eq!(0x1000, core::mem::size_of::<CntReadBase>());
        assert_eq!(0x1000, core::mem::size_of::<CntCtlBase>());
        assert_eq!(0x1000, core::mem::size_of::<CntBase>());
        assert_eq!(0x1000, core::mem::size_of::<CntEl0Base>());
    }
}
