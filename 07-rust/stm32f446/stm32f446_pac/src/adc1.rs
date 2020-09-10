#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - status register"]
    pub sr: SR,
    #[doc = "0x04 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x08 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x0c - sample time register 1"]
    pub smpr1: SMPR1,
    #[doc = "0x10 - sample time register 2"]
    pub smpr2: SMPR2,
    #[doc = "0x14 - injected channel data offset register x"]
    pub jofr1: JOFR1,
    #[doc = "0x18 - injected channel data offset register x"]
    pub jofr2: JOFR2,
    #[doc = "0x1c - injected channel data offset register x"]
    pub jofr3: JOFR3,
    #[doc = "0x20 - injected channel data offset register x"]
    pub jofr4: JOFR4,
    #[doc = "0x24 - watchdog higher threshold register"]
    pub htr: HTR,
    #[doc = "0x28 - watchdog lower threshold register"]
    pub ltr: LTR,
    #[doc = "0x2c - regular sequence register 1"]
    pub sqr1: SQR1,
    #[doc = "0x30 - regular sequence register 2"]
    pub sqr2: SQR2,
    #[doc = "0x34 - regular sequence register 3"]
    pub sqr3: SQR3,
    #[doc = "0x38 - injected sequence register"]
    pub jsqr: JSQR,
    #[doc = "0x3c - injected data register x"]
    pub jdr1: JDR1,
    #[doc = "0x40 - injected data register x"]
    pub jdr2: JDR2,
    #[doc = "0x44 - injected data register x"]
    pub jdr3: JDR3,
    #[doc = "0x48 - injected data register x"]
    pub jdr4: JDR4,
    #[doc = "0x4c - regular data register"]
    pub dr: DR,
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "status register"]
pub mod sr;
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "control register 1"]
pub mod cr1;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "control register 2"]
pub mod cr2;
#[doc = "sample time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr1](smpr1) module"]
pub type SMPR1 = crate::Reg<u32, _SMPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPR1;
#[doc = "`read()` method returns [smpr1::R](smpr1::R) reader structure"]
impl crate::Readable for SMPR1 {}
#[doc = "`write(|w| ..)` method takes [smpr1::W](smpr1::W) writer structure"]
impl crate::Writable for SMPR1 {}
#[doc = "sample time register 1"]
pub mod smpr1;
#[doc = "sample time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr2](smpr2) module"]
pub type SMPR2 = crate::Reg<u32, _SMPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPR2;
#[doc = "`read()` method returns [smpr2::R](smpr2::R) reader structure"]
impl crate::Readable for SMPR2 {}
#[doc = "`write(|w| ..)` method takes [smpr2::W](smpr2::W) writer structure"]
impl crate::Writable for SMPR2 {}
#[doc = "sample time register 2"]
pub mod smpr2;
#[doc = "injected channel data offset register x\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jofr1](jofr1) module"]
pub type JOFR1 = crate::Reg<u32, _JOFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JOFR1;
#[doc = "`read()` method returns [jofr1::R](jofr1::R) reader structure"]
impl crate::Readable for JOFR1 {}
#[doc = "`write(|w| ..)` method takes [jofr1::W](jofr1::W) writer structure"]
impl crate::Writable for JOFR1 {}
#[doc = "injected channel data offset register x"]
pub mod jofr1;
#[doc = "injected channel data offset register x\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jofr2](jofr2) module"]
pub type JOFR2 = crate::Reg<u32, _JOFR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JOFR2;
#[doc = "`read()` method returns [jofr2::R](jofr2::R) reader structure"]
impl crate::Readable for JOFR2 {}
#[doc = "`write(|w| ..)` method takes [jofr2::W](jofr2::W) writer structure"]
impl crate::Writable for JOFR2 {}
#[doc = "injected channel data offset register x"]
pub mod jofr2;
#[doc = "injected channel data offset register x\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jofr3](jofr3) module"]
pub type JOFR3 = crate::Reg<u32, _JOFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JOFR3;
#[doc = "`read()` method returns [jofr3::R](jofr3::R) reader structure"]
impl crate::Readable for JOFR3 {}
#[doc = "`write(|w| ..)` method takes [jofr3::W](jofr3::W) writer structure"]
impl crate::Writable for JOFR3 {}
#[doc = "injected channel data offset register x"]
pub mod jofr3;
#[doc = "injected channel data offset register x\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jofr4](jofr4) module"]
pub type JOFR4 = crate::Reg<u32, _JOFR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JOFR4;
#[doc = "`read()` method returns [jofr4::R](jofr4::R) reader structure"]
impl crate::Readable for JOFR4 {}
#[doc = "`write(|w| ..)` method takes [jofr4::W](jofr4::W) writer structure"]
impl crate::Writable for JOFR4 {}
#[doc = "injected channel data offset register x"]
pub mod jofr4;
#[doc = "watchdog higher threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htr](htr) module"]
pub type HTR = crate::Reg<u32, _HTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HTR;
#[doc = "`read()` method returns [htr::R](htr::R) reader structure"]
impl crate::Readable for HTR {}
#[doc = "`write(|w| ..)` method takes [htr::W](htr::W) writer structure"]
impl crate::Writable for HTR {}
#[doc = "watchdog higher threshold register"]
pub mod htr;
#[doc = "watchdog lower threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltr](ltr) module"]
pub type LTR = crate::Reg<u32, _LTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LTR;
#[doc = "`read()` method returns [ltr::R](ltr::R) reader structure"]
impl crate::Readable for LTR {}
#[doc = "`write(|w| ..)` method takes [ltr::W](ltr::W) writer structure"]
impl crate::Writable for LTR {}
#[doc = "watchdog lower threshold register"]
pub mod ltr;
#[doc = "regular sequence register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr1](sqr1) module"]
pub type SQR1 = crate::Reg<u32, _SQR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR1;
#[doc = "`read()` method returns [sqr1::R](sqr1::R) reader structure"]
impl crate::Readable for SQR1 {}
#[doc = "`write(|w| ..)` method takes [sqr1::W](sqr1::W) writer structure"]
impl crate::Writable for SQR1 {}
#[doc = "regular sequence register 1"]
pub mod sqr1;
#[doc = "regular sequence register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr2](sqr2) module"]
pub type SQR2 = crate::Reg<u32, _SQR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR2;
#[doc = "`read()` method returns [sqr2::R](sqr2::R) reader structure"]
impl crate::Readable for SQR2 {}
#[doc = "`write(|w| ..)` method takes [sqr2::W](sqr2::W) writer structure"]
impl crate::Writable for SQR2 {}
#[doc = "regular sequence register 2"]
pub mod sqr2;
#[doc = "regular sequence register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr3](sqr3) module"]
pub type SQR3 = crate::Reg<u32, _SQR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR3;
#[doc = "`read()` method returns [sqr3::R](sqr3::R) reader structure"]
impl crate::Readable for SQR3 {}
#[doc = "`write(|w| ..)` method takes [sqr3::W](sqr3::W) writer structure"]
impl crate::Writable for SQR3 {}
#[doc = "regular sequence register 3"]
pub mod sqr3;
#[doc = "injected sequence register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jsqr](jsqr) module"]
pub type JSQR = crate::Reg<u32, _JSQR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JSQR;
#[doc = "`read()` method returns [jsqr::R](jsqr::R) reader structure"]
impl crate::Readable for JSQR {}
#[doc = "`write(|w| ..)` method takes [jsqr::W](jsqr::W) writer structure"]
impl crate::Writable for JSQR {}
#[doc = "injected sequence register"]
pub mod jsqr;
#[doc = "injected data register x\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr1](jdr1) module"]
pub type JDR1 = crate::Reg<u32, _JDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR1;
#[doc = "`read()` method returns [jdr1::R](jdr1::R) reader structure"]
impl crate::Readable for JDR1 {}
#[doc = "injected data register x"]
pub mod jdr1;
#[doc = "injected data register x\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr2](jdr2) module"]
pub type JDR2 = crate::Reg<u32, _JDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR2;
#[doc = "`read()` method returns [jdr2::R](jdr2::R) reader structure"]
impl crate::Readable for JDR2 {}
#[doc = "injected data register x"]
pub mod jdr2;
#[doc = "injected data register x\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr3](jdr3) module"]
pub type JDR3 = crate::Reg<u32, _JDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR3;
#[doc = "`read()` method returns [jdr3::R](jdr3::R) reader structure"]
impl crate::Readable for JDR3 {}
#[doc = "injected data register x"]
pub mod jdr3;
#[doc = "injected data register x\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr4](jdr4) module"]
pub type JDR4 = crate::Reg<u32, _JDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR4;
#[doc = "`read()` method returns [jdr4::R](jdr4::R) reader structure"]
impl crate::Readable for JDR4 {}
#[doc = "injected data register x"]
pub mod jdr4;
#[doc = "regular data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "regular data register"]
pub mod dr;
