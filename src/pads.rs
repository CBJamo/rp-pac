#[derive(Copy, Clone)]
pub struct Pads(pub *mut u8);
unsafe impl Send for Pads {}
unsafe impl Sync for Pads {}
impl Pads {
    #[doc = "Voltage select. Per bank control"]
    pub fn voltage_select(self) -> crate::common::Reg<regs::VoltageSelect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio(self, n: usize) -> crate::common::Reg<regs::GpioCtrl, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize + n * 4usize)) }
    }
}
pub mod regs;
pub mod vals;
