#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSCFG configuration register 1"]
    pub cfgr1: CFGR1,
    #[doc = "0x04 - SYSCFG configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: EXTICR1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: EXTICR2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: EXTICR3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: EXTICR4,
    #[doc = "0x18 - Comparator 1 control and status register"]
    pub comp1_ctrl: COMP1_CTRL,
    #[doc = "0x1c - Comparator 2 control and status register"]
    pub comp2_ctrl: COMP2_CTRL,
    #[doc = "0x20 - SYSCFG configuration register 3"]
    pub cfgr3: CFGR3,
}
#[doc = "SYSCFG configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](cfgr1) module"]
pub type CFGR1 = crate::Reg<u32, _CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR1;
#[doc = "`read()` method returns [cfgr1::R](cfgr1::R) reader structure"]
impl crate::Readable for CFGR1 {}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](cfgr1::W) writer structure"]
impl crate::Writable for CFGR1 {}
#[doc = "SYSCFG configuration register 1"]
pub mod cfgr1;
#[doc = "SYSCFG configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](cfgr2) module"]
pub type CFGR2 = crate::Reg<u32, _CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR2;
#[doc = "`read()` method returns [cfgr2::R](cfgr2::R) reader structure"]
impl crate::Readable for CFGR2 {}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](cfgr2::W) writer structure"]
impl crate::Writable for CFGR2 {}
#[doc = "SYSCFG configuration register 2"]
pub mod cfgr2;
#[doc = "external interrupt configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr1](exticr1) module"]
pub type EXTICR1 = crate::Reg<u32, _EXTICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR1;
#[doc = "`read()` method returns [exticr1::R](exticr1::R) reader structure"]
impl crate::Readable for EXTICR1 {}
#[doc = "`write(|w| ..)` method takes [exticr1::W](exticr1::W) writer structure"]
impl crate::Writable for EXTICR1 {}
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "external interrupt configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](exticr2) module"]
pub type EXTICR2 = crate::Reg<u32, _EXTICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR2;
#[doc = "`read()` method returns [exticr2::R](exticr2::R) reader structure"]
impl crate::Readable for EXTICR2 {}
#[doc = "`write(|w| ..)` method takes [exticr2::W](exticr2::W) writer structure"]
impl crate::Writable for EXTICR2 {}
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "external interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](exticr3) module"]
pub type EXTICR3 = crate::Reg<u32, _EXTICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR3;
#[doc = "`read()` method returns [exticr3::R](exticr3::R) reader structure"]
impl crate::Readable for EXTICR3 {}
#[doc = "`write(|w| ..)` method takes [exticr3::W](exticr3::W) writer structure"]
impl crate::Writable for EXTICR3 {}
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "external interrupt configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](exticr4) module"]
pub type EXTICR4 = crate::Reg<u32, _EXTICR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR4;
#[doc = "`read()` method returns [exticr4::R](exticr4::R) reader structure"]
impl crate::Readable for EXTICR4 {}
#[doc = "`write(|w| ..)` method takes [exticr4::W](exticr4::W) writer structure"]
impl crate::Writable for EXTICR4 {}
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "SYSCFG configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](cfgr3) module"]
pub type CFGR3 = crate::Reg<u32, _CFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR3;
#[doc = "`read()` method returns [cfgr3::R](cfgr3::R) reader structure"]
impl crate::Readable for CFGR3 {}
#[doc = "`write(|w| ..)` method takes [cfgr3::W](cfgr3::W) writer structure"]
impl crate::Writable for CFGR3 {}
#[doc = "SYSCFG configuration register 3"]
pub mod cfgr3;
#[doc = "Comparator 1 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_ctrl](comp1_ctrl) module"]
pub type COMP1_CTRL = crate::Reg<u32, _COMP1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP1_CTRL;
#[doc = "`read()` method returns [comp1_ctrl::R](comp1_ctrl::R) reader structure"]
impl crate::Readable for COMP1_CTRL {}
#[doc = "`write(|w| ..)` method takes [comp1_ctrl::W](comp1_ctrl::W) writer structure"]
impl crate::Writable for COMP1_CTRL {}
#[doc = "Comparator 1 control and status register"]
pub mod comp1_ctrl;
#[doc = "Comparator 2 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_ctrl](comp2_ctrl) module"]
pub type COMP2_CTRL = crate::Reg<u32, _COMP2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP2_CTRL;
#[doc = "`read()` method returns [comp2_ctrl::R](comp2_ctrl::R) reader structure"]
impl crate::Readable for COMP2_CTRL {}
#[doc = "`write(|w| ..)` method takes [comp2_ctrl::W](comp2_ctrl::W) writer structure"]
impl crate::Writable for COMP2_CTRL {}
#[doc = "Comparator 2 control and status register"]
pub mod comp2_ctrl;
