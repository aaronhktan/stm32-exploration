#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IDCODE"]
    pub dbgmcu_idcode: DBGMCU_IDCODE,
    #[doc = "0x04 - Control Register"]
    pub dbgmcu_cr: DBGMCU_CR,
    #[doc = "0x08 - Debug MCU APB1 Freeze registe"]
    pub dbgmcu_apb1_fz: DBGMCU_APB1_FZ,
    #[doc = "0x0c - Debug MCU APB2 Freeze registe"]
    pub dbgmcu_apb2_fz: DBGMCU_APB2_FZ,
}
#[doc = "IDCODE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgmcu_idcode](dbgmcu_idcode) module"]
pub type DBGMCU_IDCODE = crate::Reg<u32, _DBGMCU_IDCODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGMCU_IDCODE;
#[doc = "`read()` method returns [dbgmcu_idcode::R](dbgmcu_idcode::R) reader structure"]
impl crate::Readable for DBGMCU_IDCODE {}
#[doc = "IDCODE"]
pub mod dbgmcu_idcode;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgmcu_cr](dbgmcu_cr) module"]
pub type DBGMCU_CR = crate::Reg<u32, _DBGMCU_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGMCU_CR;
#[doc = "`read()` method returns [dbgmcu_cr::R](dbgmcu_cr::R) reader structure"]
impl crate::Readable for DBGMCU_CR {}
#[doc = "`write(|w| ..)` method takes [dbgmcu_cr::W](dbgmcu_cr::W) writer structure"]
impl crate::Writable for DBGMCU_CR {}
#[doc = "Control Register"]
pub mod dbgmcu_cr;
#[doc = "Debug MCU APB1 Freeze registe\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgmcu_apb1_fz](dbgmcu_apb1_fz) module"]
pub type DBGMCU_APB1_FZ = crate::Reg<u32, _DBGMCU_APB1_FZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGMCU_APB1_FZ;
#[doc = "`read()` method returns [dbgmcu_apb1_fz::R](dbgmcu_apb1_fz::R) reader structure"]
impl crate::Readable for DBGMCU_APB1_FZ {}
#[doc = "`write(|w| ..)` method takes [dbgmcu_apb1_fz::W](dbgmcu_apb1_fz::W) writer structure"]
impl crate::Writable for DBGMCU_APB1_FZ {}
#[doc = "Debug MCU APB1 Freeze registe"]
pub mod dbgmcu_apb1_fz;
#[doc = "Debug MCU APB2 Freeze registe\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgmcu_apb2_fz](dbgmcu_apb2_fz) module"]
pub type DBGMCU_APB2_FZ = crate::Reg<u32, _DBGMCU_APB2_FZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGMCU_APB2_FZ;
#[doc = "`read()` method returns [dbgmcu_apb2_fz::R](dbgmcu_apb2_fz::R) reader structure"]
impl crate::Readable for DBGMCU_APB2_FZ {}
#[doc = "`write(|w| ..)` method takes [dbgmcu_apb2_fz::W](dbgmcu_apb2_fz::W) writer structure"]
impl crate::Writable for DBGMCU_APB2_FZ {}
#[doc = "Debug MCU APB2 Freeze registe"]
pub mod dbgmcu_apb2_fz;
