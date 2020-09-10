#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configuration register 1"]
    pub syscfg_cfgr1: SYSCFG_CFGR1,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub syscfg_exticr1: SYSCFG_EXTICR1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub syscfg_exticr2: SYSCFG_EXTICR2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub syscfg_exticr3: SYSCFG_EXTICR3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub syscfg_exticr4: SYSCFG_EXTICR4,
    #[doc = "0x18 - configuration register 2"]
    pub syscfg_cfgr2: SYSCFG_CFGR2,
    #[doc = "0x1c - control and status register"]
    pub comp_csr: COMP_CSR,
}
#[doc = "configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cfgr1](syscfg_cfgr1) module"]
pub type SYSCFG_CFGR1 = crate::Reg<u32, _SYSCFG_CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_CFGR1;
#[doc = "`read()` method returns [syscfg_cfgr1::R](syscfg_cfgr1::R) reader structure"]
impl crate::Readable for SYSCFG_CFGR1 {}
#[doc = "`write(|w| ..)` method takes [syscfg_cfgr1::W](syscfg_cfgr1::W) writer structure"]
impl crate::Writable for SYSCFG_CFGR1 {}
#[doc = "configuration register 1"]
pub mod syscfg_cfgr1;
#[doc = "external interrupt configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_exticr1](syscfg_exticr1) module"]
pub type SYSCFG_EXTICR1 = crate::Reg<u32, _SYSCFG_EXTICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_EXTICR1;
#[doc = "`read()` method returns [syscfg_exticr1::R](syscfg_exticr1::R) reader structure"]
impl crate::Readable for SYSCFG_EXTICR1 {}
#[doc = "`write(|w| ..)` method takes [syscfg_exticr1::W](syscfg_exticr1::W) writer structure"]
impl crate::Writable for SYSCFG_EXTICR1 {}
#[doc = "external interrupt configuration register 1"]
pub mod syscfg_exticr1;
#[doc = "external interrupt configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_exticr2](syscfg_exticr2) module"]
pub type SYSCFG_EXTICR2 = crate::Reg<u32, _SYSCFG_EXTICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_EXTICR2;
#[doc = "`read()` method returns [syscfg_exticr2::R](syscfg_exticr2::R) reader structure"]
impl crate::Readable for SYSCFG_EXTICR2 {}
#[doc = "`write(|w| ..)` method takes [syscfg_exticr2::W](syscfg_exticr2::W) writer structure"]
impl crate::Writable for SYSCFG_EXTICR2 {}
#[doc = "external interrupt configuration register 2"]
pub mod syscfg_exticr2;
#[doc = "external interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_exticr3](syscfg_exticr3) module"]
pub type SYSCFG_EXTICR3 = crate::Reg<u32, _SYSCFG_EXTICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_EXTICR3;
#[doc = "`read()` method returns [syscfg_exticr3::R](syscfg_exticr3::R) reader structure"]
impl crate::Readable for SYSCFG_EXTICR3 {}
#[doc = "`write(|w| ..)` method takes [syscfg_exticr3::W](syscfg_exticr3::W) writer structure"]
impl crate::Writable for SYSCFG_EXTICR3 {}
#[doc = "external interrupt configuration register 3"]
pub mod syscfg_exticr3;
#[doc = "external interrupt configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_exticr4](syscfg_exticr4) module"]
pub type SYSCFG_EXTICR4 = crate::Reg<u32, _SYSCFG_EXTICR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_EXTICR4;
#[doc = "`read()` method returns [syscfg_exticr4::R](syscfg_exticr4::R) reader structure"]
impl crate::Readable for SYSCFG_EXTICR4 {}
#[doc = "`write(|w| ..)` method takes [syscfg_exticr4::W](syscfg_exticr4::W) writer structure"]
impl crate::Writable for SYSCFG_EXTICR4 {}
#[doc = "external interrupt configuration register 4"]
pub mod syscfg_exticr4;
#[doc = "configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cfgr2](syscfg_cfgr2) module"]
pub type SYSCFG_CFGR2 = crate::Reg<u32, _SYSCFG_CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_CFGR2;
#[doc = "`read()` method returns [syscfg_cfgr2::R](syscfg_cfgr2::R) reader structure"]
impl crate::Readable for SYSCFG_CFGR2 {}
#[doc = "`write(|w| ..)` method takes [syscfg_cfgr2::W](syscfg_cfgr2::W) writer structure"]
impl crate::Writable for SYSCFG_CFGR2 {}
#[doc = "configuration register 2"]
pub mod syscfg_cfgr2;
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_csr](comp_csr) module"]
pub type COMP_CSR = crate::Reg<u32, _COMP_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_CSR;
#[doc = "`read()` method returns [comp_csr::R](comp_csr::R) reader structure"]
impl crate::Readable for COMP_CSR {}
#[doc = "`write(|w| ..)` method takes [comp_csr::W](comp_csr::W) writer structure"]
impl crate::Writable for COMP_CSR {}
#[doc = "control and status register"]
pub mod comp_csr;
