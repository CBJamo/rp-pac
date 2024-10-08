#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart {
    ptr: *mut u8,
}
unsafe impl Send for Uart {}
unsafe impl Sync for Uart {}
impl Uart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data Register, UARTDR"]
    #[inline(always)]
    pub const fn uartdr(self) -> crate::common::Reg<regs::Uartdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Receive Status Register/Error Clear Register, UARTRSR/UARTECR"]
    #[inline(always)]
    pub const fn uartrsr(self) -> crate::common::Reg<regs::Uartrsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Flag Register, UARTFR"]
    #[inline(always)]
    pub const fn uartfr(self) -> crate::common::Reg<regs::Uartfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "IrDA Low-Power Counter Register, UARTILPR"]
    #[inline(always)]
    pub const fn uartilpr(self) -> crate::common::Reg<regs::Uartilpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Integer Baud Rate Register, UARTIBRD"]
    #[inline(always)]
    pub const fn uartibrd(self) -> crate::common::Reg<regs::Uartibrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Fractional Baud Rate Register, UARTFBRD"]
    #[inline(always)]
    pub const fn uartfbrd(self) -> crate::common::Reg<regs::Uartfbrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Line Control Register, UARTLCR_H"]
    #[inline(always)]
    pub const fn uartlcr_h(self) -> crate::common::Reg<regs::UartlcrH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Control Register, UARTCR"]
    #[inline(always)]
    pub const fn uartcr(self) -> crate::common::Reg<regs::Uartcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Interrupt FIFO Level Select Register, UARTIFLS"]
    #[inline(always)]
    pub const fn uartifls(self) -> crate::common::Reg<regs::Uartifls, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Interrupt Mask Set/Clear Register, UARTIMSC"]
    #[inline(always)]
    pub const fn uartimsc(self) -> crate::common::Reg<regs::Uartimsc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Raw Interrupt Status Register, UARTRIS"]
    #[inline(always)]
    pub const fn uartris(self) -> crate::common::Reg<regs::Uartris, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Masked Interrupt Status Register, UARTMIS"]
    #[inline(always)]
    pub const fn uartmis(self) -> crate::common::Reg<regs::Uartmis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Interrupt Clear Register, UARTICR"]
    #[inline(always)]
    pub const fn uarticr(self) -> crate::common::Reg<regs::Uarticr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "DMA Control Register, UARTDMACR"]
    #[inline(always)]
    pub const fn uartdmacr(self) -> crate::common::Reg<regs::Uartdmacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "UARTPeriphID0 Register"]
    #[inline(always)]
    pub const fn uartperiphid0(self) -> crate::common::Reg<regs::Uartperiphid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe0usize) as _) }
    }
    #[doc = "UARTPeriphID1 Register"]
    #[inline(always)]
    pub const fn uartperiphid1(self) -> crate::common::Reg<regs::Uartperiphid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe4usize) as _) }
    }
    #[doc = "UARTPeriphID2 Register"]
    #[inline(always)]
    pub const fn uartperiphid2(self) -> crate::common::Reg<regs::Uartperiphid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe8usize) as _) }
    }
    #[doc = "UARTPeriphID3 Register"]
    #[inline(always)]
    pub const fn uartperiphid3(self) -> crate::common::Reg<regs::Uartperiphid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fecusize) as _) }
    }
    #[doc = "UARTPCellID0 Register"]
    #[inline(always)]
    pub const fn uartpcellid0(self) -> crate::common::Reg<regs::Uartpcellid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff0usize) as _) }
    }
    #[doc = "UARTPCellID1 Register"]
    #[inline(always)]
    pub const fn uartpcellid1(self) -> crate::common::Reg<regs::Uartpcellid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff4usize) as _) }
    }
    #[doc = "UARTPCellID2 Register"]
    #[inline(always)]
    pub const fn uartpcellid2(self) -> crate::common::Reg<regs::Uartpcellid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff8usize) as _) }
    }
    #[doc = "UARTPCellID3 Register"]
    #[inline(always)]
    pub const fn uartpcellid3(self) -> crate::common::Reg<regs::Uartpcellid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs;
