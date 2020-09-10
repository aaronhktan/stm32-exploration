#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"]
    pub tr: TR,
    #[doc = "0x04 - date register"]
    pub dr: DR,
    #[doc = "0x08 - control register"]
    pub cr: CR,
    #[doc = "0x0c - initialization and status register"]
    pub isr: ISR,
    #[doc = "0x10 - prescaler register"]
    pub prer: PRER,
    _reserved5: [u8; 8usize],
    #[doc = "0x1c - alarm A register"]
    pub alrmar: ALRMAR,
    _reserved6: [u8; 4usize],
    #[doc = "0x24 - write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - sub second register"]
    pub ssr: SSR,
    #[doc = "0x2c - shift control register"]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - timestamp time register"]
    pub tstr: TSTR,
    #[doc = "0x34 - timestamp date register"]
    pub tsdr: TSDR,
    #[doc = "0x38 - time-stamp sub second register"]
    pub tsssr: TSSSR,
    #[doc = "0x3c - calibration register"]
    pub calr: CALR,
    #[doc = "0x40 - tamper and alternate function configuration register"]
    pub tafcr: TAFCR,
    #[doc = "0x44 - alarm A sub second register"]
    pub alrmassr: ALRMASSR,
    _reserved15: [u8; 8usize],
    #[doc = "0x50 - backup register"]
    pub bkp0r: BKP0R,
    #[doc = "0x54 - backup register"]
    pub bkp1r: BKP1R,
    #[doc = "0x58 - backup register"]
    pub bkp2r: BKP2R,
    #[doc = "0x5c - backup register"]
    pub bkp3r: BKP3R,
    #[doc = "0x60 - backup register"]
    pub bkp4r: BKP4R,
}
#[doc = "time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](tr) module"]
pub type TR = crate::Reg<u32, _TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR;
#[doc = "`read()` method returns [tr::R](tr::R) reader structure"]
impl crate::Readable for TR {}
#[doc = "`write(|w| ..)` method takes [tr::W](tr::W) writer structure"]
impl crate::Writable for TR {}
#[doc = "time register"]
pub mod tr;
#[doc = "date register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "date register"]
pub mod dr;
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "control register"]
pub mod cr;
#[doc = "initialization and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "initialization and status register"]
pub mod isr;
#[doc = "prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prer](prer) module"]
pub type PRER = crate::Reg<u32, _PRER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRER;
#[doc = "`read()` method returns [prer::R](prer::R) reader structure"]
impl crate::Readable for PRER {}
#[doc = "`write(|w| ..)` method takes [prer::W](prer::W) writer structure"]
impl crate::Writable for PRER {}
#[doc = "prescaler register"]
pub mod prer;
#[doc = "alarm A register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmar](alrmar) module"]
pub type ALRMAR = crate::Reg<u32, _ALRMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMAR;
#[doc = "`read()` method returns [alrmar::R](alrmar::R) reader structure"]
impl crate::Readable for ALRMAR {}
#[doc = "`write(|w| ..)` method takes [alrmar::W](alrmar::W) writer structure"]
impl crate::Writable for ALRMAR {}
#[doc = "alarm A register"]
pub mod alrmar;
#[doc = "write protection register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpr](wpr) module"]
pub type WPR = crate::Reg<u32, _WPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPR;
#[doc = "`write(|w| ..)` method takes [wpr::W](wpr::W) writer structure"]
impl crate::Writable for WPR {}
#[doc = "write protection register"]
pub mod wpr;
#[doc = "sub second register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssr](ssr) module"]
pub type SSR = crate::Reg<u32, _SSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSR;
#[doc = "`read()` method returns [ssr::R](ssr::R) reader structure"]
impl crate::Readable for SSR {}
#[doc = "sub second register"]
pub mod ssr;
#[doc = "shift control register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftr](shiftr) module"]
pub type SHIFTR = crate::Reg<u32, _SHIFTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTR;
#[doc = "`write(|w| ..)` method takes [shiftr::W](shiftr::W) writer structure"]
impl crate::Writable for SHIFTR {}
#[doc = "shift control register"]
pub mod shiftr;
#[doc = "timestamp time register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tstr](tstr) module"]
pub type TSTR = crate::Reg<u32, _TSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSTR;
#[doc = "`read()` method returns [tstr::R](tstr::R) reader structure"]
impl crate::Readable for TSTR {}
#[doc = "timestamp time register"]
pub mod tstr;
#[doc = "timestamp date register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsdr](tsdr) module"]
pub type TSDR = crate::Reg<u32, _TSDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSDR;
#[doc = "`read()` method returns [tsdr::R](tsdr::R) reader structure"]
impl crate::Readable for TSDR {}
#[doc = "timestamp date register"]
pub mod tsdr;
#[doc = "time-stamp sub second register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsssr](tsssr) module"]
pub type TSSSR = crate::Reg<u32, _TSSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSSSR;
#[doc = "`read()` method returns [tsssr::R](tsssr::R) reader structure"]
impl crate::Readable for TSSSR {}
#[doc = "time-stamp sub second register"]
pub mod tsssr;
#[doc = "calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calr](calr) module"]
pub type CALR = crate::Reg<u32, _CALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALR;
#[doc = "`read()` method returns [calr::R](calr::R) reader structure"]
impl crate::Readable for CALR {}
#[doc = "`write(|w| ..)` method takes [calr::W](calr::W) writer structure"]
impl crate::Writable for CALR {}
#[doc = "calibration register"]
pub mod calr;
#[doc = "tamper and alternate function configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tafcr](tafcr) module"]
pub type TAFCR = crate::Reg<u32, _TAFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAFCR;
#[doc = "`read()` method returns [tafcr::R](tafcr::R) reader structure"]
impl crate::Readable for TAFCR {}
#[doc = "`write(|w| ..)` method takes [tafcr::W](tafcr::W) writer structure"]
impl crate::Writable for TAFCR {}
#[doc = "tamper and alternate function configuration register"]
pub mod tafcr;
#[doc = "alarm A sub second register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmassr](alrmassr) module"]
pub type ALRMASSR = crate::Reg<u32, _ALRMASSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMASSR;
#[doc = "`read()` method returns [alrmassr::R](alrmassr::R) reader structure"]
impl crate::Readable for ALRMASSR {}
#[doc = "`write(|w| ..)` method takes [alrmassr::W](alrmassr::W) writer structure"]
impl crate::Writable for ALRMASSR {}
#[doc = "alarm A sub second register"]
pub mod alrmassr;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp0r](bkp0r) module"]
pub type BKP0R = crate::Reg<u32, _BKP0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP0R;
#[doc = "`read()` method returns [bkp0r::R](bkp0r::R) reader structure"]
impl crate::Readable for BKP0R {}
#[doc = "`write(|w| ..)` method takes [bkp0r::W](bkp0r::W) writer structure"]
impl crate::Writable for BKP0R {}
#[doc = "backup register"]
pub mod bkp0r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp1r](bkp1r) module"]
pub type BKP1R = crate::Reg<u32, _BKP1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP1R;
#[doc = "`read()` method returns [bkp1r::R](bkp1r::R) reader structure"]
impl crate::Readable for BKP1R {}
#[doc = "`write(|w| ..)` method takes [bkp1r::W](bkp1r::W) writer structure"]
impl crate::Writable for BKP1R {}
#[doc = "backup register"]
pub mod bkp1r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp2r](bkp2r) module"]
pub type BKP2R = crate::Reg<u32, _BKP2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP2R;
#[doc = "`read()` method returns [bkp2r::R](bkp2r::R) reader structure"]
impl crate::Readable for BKP2R {}
#[doc = "`write(|w| ..)` method takes [bkp2r::W](bkp2r::W) writer structure"]
impl crate::Writable for BKP2R {}
#[doc = "backup register"]
pub mod bkp2r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp3r](bkp3r) module"]
pub type BKP3R = crate::Reg<u32, _BKP3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP3R;
#[doc = "`read()` method returns [bkp3r::R](bkp3r::R) reader structure"]
impl crate::Readable for BKP3R {}
#[doc = "`write(|w| ..)` method takes [bkp3r::W](bkp3r::W) writer structure"]
impl crate::Writable for BKP3R {}
#[doc = "backup register"]
pub mod bkp3r;
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp4r](bkp4r) module"]
pub type BKP4R = crate::Reg<u32, _BKP4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP4R;
#[doc = "`read()` method returns [bkp4r::R](bkp4r::R) reader structure"]
impl crate::Readable for BKP4R {}
#[doc = "`write(|w| ..)` method takes [bkp4r::W](bkp4r::W) writer structure"]
impl crate::Writable for BKP4R {}
#[doc = "backup register"]
pub mod bkp4r;
