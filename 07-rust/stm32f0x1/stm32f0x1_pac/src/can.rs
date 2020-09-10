#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN_MCR"]
    pub can_mcr: CAN_MCR,
    #[doc = "0x04 - CAN_MSR"]
    pub can_msr: CAN_MSR,
    #[doc = "0x08 - CAN_TSR"]
    pub can_tsr: CAN_TSR,
    #[doc = "0x0c - CAN_RF0R"]
    pub can_rf0r: CAN_RF0R,
    #[doc = "0x10 - CAN_RF1R"]
    pub can_rf1r: CAN_RF1R,
    #[doc = "0x14 - CAN_IER"]
    pub can_ier: CAN_IER,
    #[doc = "0x18 - CAN_ESR"]
    pub can_esr: CAN_ESR,
    #[doc = "0x1c - CAN BTR"]
    pub can_btr: CAN_BTR,
    _reserved8: [u8; 352usize],
    #[doc = "0x180 - CAN_TI0R"]
    pub can_ti0r: CAN_TI0R,
    #[doc = "0x184 - CAN_TDT0R"]
    pub can_tdt0r: CAN_TDT0R,
    #[doc = "0x188 - CAN_TDL0R"]
    pub can_tdl0r: CAN_TDL0R,
    #[doc = "0x18c - CAN_TDH0R"]
    pub can_tdh0r: CAN_TDH0R,
    #[doc = "0x190 - CAN_TI1R"]
    pub can_ti1r: CAN_TI1R,
    #[doc = "0x194 - CAN_TDT1R"]
    pub can_tdt1r: CAN_TDT1R,
    #[doc = "0x198 - CAN_TDL1R"]
    pub can_tdl1r: CAN_TDL1R,
    #[doc = "0x19c - CAN_TDH1R"]
    pub can_tdh1r: CAN_TDH1R,
    #[doc = "0x1a0 - CAN_TI2R"]
    pub can_ti2r: CAN_TI2R,
    #[doc = "0x1a4 - CAN_TDT2R"]
    pub can_tdt2r: CAN_TDT2R,
    #[doc = "0x1a8 - CAN_TDL2R"]
    pub can_tdl2r: CAN_TDL2R,
    #[doc = "0x1ac - CAN_TDH2R"]
    pub can_tdh2r: CAN_TDH2R,
    #[doc = "0x1b0 - CAN_RI0R"]
    pub can_ri0r: CAN_RI0R,
    #[doc = "0x1b4 - CAN_RDT0R"]
    pub can_rdt0r: CAN_RDT0R,
    #[doc = "0x1b8 - CAN_RDL0R"]
    pub can_rdl0r: CAN_RDL0R,
    #[doc = "0x1bc - CAN_RDH0R"]
    pub can_rdh0r: CAN_RDH0R,
    #[doc = "0x1c0 - CAN_RI1R"]
    pub can_ri1r: CAN_RI1R,
    #[doc = "0x1c4 - CAN_RDT1R"]
    pub can_rdt1r: CAN_RDT1R,
    #[doc = "0x1c8 - CAN_RDL1R"]
    pub can_rdl1r: CAN_RDL1R,
    #[doc = "0x1cc - CAN_RDH1R"]
    pub can_rdh1r: CAN_RDH1R,
    _reserved28: [u8; 48usize],
    #[doc = "0x200 - CAN_FMR"]
    pub can_fmr: CAN_FMR,
    #[doc = "0x204 - CAN_FM1R"]
    pub can_fm1r: CAN_FM1R,
    _reserved30: [u8; 4usize],
    #[doc = "0x20c - CAN_FS1R"]
    pub can_fs1r: CAN_FS1R,
    _reserved31: [u8; 4usize],
    #[doc = "0x214 - CAN_FFA1R"]
    pub can_ffa1r: CAN_FFA1R,
    _reserved32: [u8; 4usize],
    #[doc = "0x21c - CAN_FA1R"]
    pub can_fa1r: CAN_FA1R,
    _reserved33: [u8; 32usize],
    #[doc = "0x240 - Filter bank 0 register 1"]
    pub f0r1: F0R1,
    #[doc = "0x244 - Filter bank 0 register 2"]
    pub f0r2: F0R2,
    #[doc = "0x248 - Filter bank 1 register 1"]
    pub f1r1: F1R1,
    #[doc = "0x24c - Filter bank 1 register 2"]
    pub f1r2: F1R2,
    #[doc = "0x250 - Filter bank 2 register 1"]
    pub f2r1: F2R1,
    #[doc = "0x254 - Filter bank 2 register 2"]
    pub f2r2: F2R2,
    #[doc = "0x258 - Filter bank 3 register 1"]
    pub f3r1: F3R1,
    #[doc = "0x25c - Filter bank 3 register 2"]
    pub f3r2: F3R2,
    #[doc = "0x260 - Filter bank 4 register 1"]
    pub f4r1: F4R1,
    #[doc = "0x264 - Filter bank 4 register 2"]
    pub f4r2: F4R2,
    #[doc = "0x268 - Filter bank 5 register 1"]
    pub f5r1: F5R1,
    #[doc = "0x26c - Filter bank 5 register 2"]
    pub f5r2: F5R2,
    #[doc = "0x270 - Filter bank 6 register 1"]
    pub f6r1: F6R1,
    #[doc = "0x274 - Filter bank 6 register 2"]
    pub f6r2: F6R2,
    #[doc = "0x278 - Filter bank 7 register 1"]
    pub f7r1: F7R1,
    #[doc = "0x27c - Filter bank 7 register 2"]
    pub f7r2: F7R2,
    #[doc = "0x280 - Filter bank 8 register 1"]
    pub f8r1: F8R1,
    #[doc = "0x284 - Filter bank 8 register 2"]
    pub f8r2: F8R2,
    #[doc = "0x288 - Filter bank 9 register 1"]
    pub f9r1: F9R1,
    #[doc = "0x28c - Filter bank 9 register 2"]
    pub f9r2: F9R2,
    #[doc = "0x290 - Filter bank 10 register 1"]
    pub f10r1: F10R1,
    #[doc = "0x294 - Filter bank 10 register 2"]
    pub f10r2: F10R2,
    #[doc = "0x298 - Filter bank 11 register 1"]
    pub f11r1: F11R1,
    #[doc = "0x29c - Filter bank 11 register 2"]
    pub f11r2: F11R2,
    #[doc = "0x2a0 - Filter bank 4 register 1"]
    pub f12r1: F12R1,
    #[doc = "0x2a4 - Filter bank 12 register 2"]
    pub f12r2: F12R2,
    #[doc = "0x2a8 - Filter bank 13 register 1"]
    pub f13r1: F13R1,
    #[doc = "0x2ac - Filter bank 13 register 2"]
    pub f13r2: F13R2,
    #[doc = "0x2b0 - Filter bank 14 register 1"]
    pub f14r1: F14R1,
    #[doc = "0x2b4 - Filter bank 14 register 2"]
    pub f14r2: F14R2,
    #[doc = "0x2b8 - Filter bank 15 register 1"]
    pub f15r1: F15R1,
    #[doc = "0x2bc - Filter bank 15 register 2"]
    pub f15r2: F15R2,
    #[doc = "0x2c0 - Filter bank 16 register 1"]
    pub f16r1: F16R1,
    #[doc = "0x2c4 - Filter bank 16 register 2"]
    pub f16r2: F16R2,
    #[doc = "0x2c8 - Filter bank 17 register 1"]
    pub f17r1: F17R1,
    #[doc = "0x2cc - Filter bank 17 register 2"]
    pub f17r2: F17R2,
    #[doc = "0x2d0 - Filter bank 18 register 1"]
    pub f18r1: F18R1,
    #[doc = "0x2d4 - Filter bank 18 register 2"]
    pub f18r2: F18R2,
    #[doc = "0x2d8 - Filter bank 19 register 1"]
    pub f19r1: F19R1,
    #[doc = "0x2dc - Filter bank 19 register 2"]
    pub f19r2: F19R2,
    #[doc = "0x2e0 - Filter bank 20 register 1"]
    pub f20r1: F20R1,
    #[doc = "0x2e4 - Filter bank 20 register 2"]
    pub f20r2: F20R2,
    #[doc = "0x2e8 - Filter bank 21 register 1"]
    pub f21r1: F21R1,
    #[doc = "0x2ec - Filter bank 21 register 2"]
    pub f21r2: F21R2,
    #[doc = "0x2f0 - Filter bank 22 register 1"]
    pub f22r1: F22R1,
    #[doc = "0x2f4 - Filter bank 22 register 2"]
    pub f22r2: F22R2,
    #[doc = "0x2f8 - Filter bank 23 register 1"]
    pub f23r1: F23R1,
    #[doc = "0x2fc - Filter bank 23 register 2"]
    pub f23r2: F23R2,
    #[doc = "0x300 - Filter bank 24 register 1"]
    pub f24r1: F24R1,
    #[doc = "0x304 - Filter bank 24 register 2"]
    pub f24r2: F24R2,
    #[doc = "0x308 - Filter bank 25 register 1"]
    pub f25r1: F25R1,
    #[doc = "0x30c - Filter bank 25 register 2"]
    pub f25r2: F25R2,
    #[doc = "0x310 - Filter bank 26 register 1"]
    pub f26r1: F26R1,
    #[doc = "0x314 - Filter bank 26 register 2"]
    pub f26r2: F26R2,
    #[doc = "0x318 - Filter bank 27 register 1"]
    pub f27r1: F27R1,
    #[doc = "0x31c - Filter bank 27 register 2"]
    pub f27r2: F27R2,
}
#[doc = "CAN_MCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_mcr](can_mcr) module"]
pub type CAN_MCR = crate::Reg<u32, _CAN_MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_MCR;
#[doc = "`read()` method returns [can_mcr::R](can_mcr::R) reader structure"]
impl crate::Readable for CAN_MCR {}
#[doc = "`write(|w| ..)` method takes [can_mcr::W](can_mcr::W) writer structure"]
impl crate::Writable for CAN_MCR {}
#[doc = "CAN_MCR"]
pub mod can_mcr;
#[doc = "CAN_MSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_msr](can_msr) module"]
pub type CAN_MSR = crate::Reg<u32, _CAN_MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_MSR;
#[doc = "`read()` method returns [can_msr::R](can_msr::R) reader structure"]
impl crate::Readable for CAN_MSR {}
#[doc = "`write(|w| ..)` method takes [can_msr::W](can_msr::W) writer structure"]
impl crate::Writable for CAN_MSR {}
#[doc = "CAN_MSR"]
pub mod can_msr;
#[doc = "CAN_TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tsr](can_tsr) module"]
pub type CAN_TSR = crate::Reg<u32, _CAN_TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TSR;
#[doc = "`read()` method returns [can_tsr::R](can_tsr::R) reader structure"]
impl crate::Readable for CAN_TSR {}
#[doc = "`write(|w| ..)` method takes [can_tsr::W](can_tsr::W) writer structure"]
impl crate::Writable for CAN_TSR {}
#[doc = "CAN_TSR"]
pub mod can_tsr;
#[doc = "CAN_RF0R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_rf0r](can_rf0r) module"]
pub type CAN_RF0R = crate::Reg<u32, _CAN_RF0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_RF0R;
#[doc = "`read()` method returns [can_rf0r::R](can_rf0r::R) reader structure"]
impl crate::Readable for CAN_RF0R {}
#[doc = "`write(|w| ..)` method takes [can_rf0r::W](can_rf0r::W) writer structure"]
impl crate::Writable for CAN_RF0R {}
#[doc = "CAN_RF0R"]
pub mod can_rf0r;
#[doc = "CAN_RF1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_rf1r](can_rf1r) module"]
pub type CAN_RF1R = crate::Reg<u32, _CAN_RF1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_RF1R;
#[doc = "`read()` method returns [can_rf1r::R](can_rf1r::R) reader structure"]
impl crate::Readable for CAN_RF1R {}
#[doc = "`write(|w| ..)` method takes [can_rf1r::W](can_rf1r::W) writer structure"]
impl crate::Writable for CAN_RF1R {}
#[doc = "CAN_RF1R"]
pub mod can_rf1r;
#[doc = "CAN_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ier](can_ier) module"]
pub type CAN_IER = crate::Reg<u32, _CAN_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_IER;
#[doc = "`read()` method returns [can_ier::R](can_ier::R) reader structure"]
impl crate::Readable for CAN_IER {}
#[doc = "`write(|w| ..)` method takes [can_ier::W](can_ier::W) writer structure"]
impl crate::Writable for CAN_IER {}
#[doc = "CAN_IER"]
pub mod can_ier;
#[doc = "CAN_ESR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_esr](can_esr) module"]
pub type CAN_ESR = crate::Reg<u32, _CAN_ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_ESR;
#[doc = "`read()` method returns [can_esr::R](can_esr::R) reader structure"]
impl crate::Readable for CAN_ESR {}
#[doc = "`write(|w| ..)` method takes [can_esr::W](can_esr::W) writer structure"]
impl crate::Writable for CAN_ESR {}
#[doc = "CAN_ESR"]
pub mod can_esr;
#[doc = "CAN BTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_btr](can_btr) module"]
pub type CAN_BTR = crate::Reg<u32, _CAN_BTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_BTR;
#[doc = "`read()` method returns [can_btr::R](can_btr::R) reader structure"]
impl crate::Readable for CAN_BTR {}
#[doc = "`write(|w| ..)` method takes [can_btr::W](can_btr::W) writer structure"]
impl crate::Writable for CAN_BTR {}
#[doc = "CAN BTR"]
pub mod can_btr;
#[doc = "CAN_TI0R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ti0r](can_ti0r) module"]
pub type CAN_TI0R = crate::Reg<u32, _CAN_TI0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TI0R;
#[doc = "`read()` method returns [can_ti0r::R](can_ti0r::R) reader structure"]
impl crate::Readable for CAN_TI0R {}
#[doc = "`write(|w| ..)` method takes [can_ti0r::W](can_ti0r::W) writer structure"]
impl crate::Writable for CAN_TI0R {}
#[doc = "CAN_TI0R"]
pub mod can_ti0r;
#[doc = "CAN_TDT0R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tdt0r](can_tdt0r) module"]
pub type CAN_TDT0R = crate::Reg<u32, _CAN_TDT0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TDT0R;
#[doc = "`read()` method returns [can_tdt0r::R](can_tdt0r::R) reader structure"]
impl crate::Readable for CAN_TDT0R {}
#[doc = "`write(|w| ..)` method takes [can_tdt0r::W](can_tdt0r::W) writer structure"]
impl crate::Writable for CAN_TDT0R {}
#[doc = "CAN_TDT0R"]
pub mod can_tdt0r;
#[doc = "CAN_TDL0R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tdl0r](can_tdl0r) module"]
pub type CAN_TDL0R = crate::Reg<u32, _CAN_TDL0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TDL0R;
#[doc = "`read()` method returns [can_tdl0r::R](can_tdl0r::R) reader structure"]
impl crate::Readable for CAN_TDL0R {}
#[doc = "`write(|w| ..)` method takes [can_tdl0r::W](can_tdl0r::W) writer structure"]
impl crate::Writable for CAN_TDL0R {}
#[doc = "CAN_TDL0R"]
pub mod can_tdl0r;
#[doc = "CAN_TDH0R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tdh0r](can_tdh0r) module"]
pub type CAN_TDH0R = crate::Reg<u32, _CAN_TDH0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TDH0R;
#[doc = "`read()` method returns [can_tdh0r::R](can_tdh0r::R) reader structure"]
impl crate::Readable for CAN_TDH0R {}
#[doc = "`write(|w| ..)` method takes [can_tdh0r::W](can_tdh0r::W) writer structure"]
impl crate::Writable for CAN_TDH0R {}
#[doc = "CAN_TDH0R"]
pub mod can_tdh0r;
#[doc = "CAN_TI1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ti1r](can_ti1r) module"]
pub type CAN_TI1R = crate::Reg<u32, _CAN_TI1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TI1R;
#[doc = "`read()` method returns [can_ti1r::R](can_ti1r::R) reader structure"]
impl crate::Readable for CAN_TI1R {}
#[doc = "`write(|w| ..)` method takes [can_ti1r::W](can_ti1r::W) writer structure"]
impl crate::Writable for CAN_TI1R {}
#[doc = "CAN_TI1R"]
pub mod can_ti1r;
#[doc = "CAN_TDT1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tdt1r](can_tdt1r) module"]
pub type CAN_TDT1R = crate::Reg<u32, _CAN_TDT1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TDT1R;
#[doc = "`read()` method returns [can_tdt1r::R](can_tdt1r::R) reader structure"]
impl crate::Readable for CAN_TDT1R {}
#[doc = "`write(|w| ..)` method takes [can_tdt1r::W](can_tdt1r::W) writer structure"]
impl crate::Writable for CAN_TDT1R {}
#[doc = "CAN_TDT1R"]
pub mod can_tdt1r;
#[doc = "CAN_TDL1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tdl1r](can_tdl1r) module"]
pub type CAN_TDL1R = crate::Reg<u32, _CAN_TDL1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TDL1R;
#[doc = "`read()` method returns [can_tdl1r::R](can_tdl1r::R) reader structure"]
impl crate::Readable for CAN_TDL1R {}
#[doc = "`write(|w| ..)` method takes [can_tdl1r::W](can_tdl1r::W) writer structure"]
impl crate::Writable for CAN_TDL1R {}
#[doc = "CAN_TDL1R"]
pub mod can_tdl1r;
#[doc = "CAN_TDH1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tdh1r](can_tdh1r) module"]
pub type CAN_TDH1R = crate::Reg<u32, _CAN_TDH1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TDH1R;
#[doc = "`read()` method returns [can_tdh1r::R](can_tdh1r::R) reader structure"]
impl crate::Readable for CAN_TDH1R {}
#[doc = "`write(|w| ..)` method takes [can_tdh1r::W](can_tdh1r::W) writer structure"]
impl crate::Writable for CAN_TDH1R {}
#[doc = "CAN_TDH1R"]
pub mod can_tdh1r;
#[doc = "CAN_TI2R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ti2r](can_ti2r) module"]
pub type CAN_TI2R = crate::Reg<u32, _CAN_TI2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TI2R;
#[doc = "`read()` method returns [can_ti2r::R](can_ti2r::R) reader structure"]
impl crate::Readable for CAN_TI2R {}
#[doc = "`write(|w| ..)` method takes [can_ti2r::W](can_ti2r::W) writer structure"]
impl crate::Writable for CAN_TI2R {}
#[doc = "CAN_TI2R"]
pub mod can_ti2r;
#[doc = "CAN_TDT2R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tdt2r](can_tdt2r) module"]
pub type CAN_TDT2R = crate::Reg<u32, _CAN_TDT2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TDT2R;
#[doc = "`read()` method returns [can_tdt2r::R](can_tdt2r::R) reader structure"]
impl crate::Readable for CAN_TDT2R {}
#[doc = "`write(|w| ..)` method takes [can_tdt2r::W](can_tdt2r::W) writer structure"]
impl crate::Writable for CAN_TDT2R {}
#[doc = "CAN_TDT2R"]
pub mod can_tdt2r;
#[doc = "CAN_TDL2R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tdl2r](can_tdl2r) module"]
pub type CAN_TDL2R = crate::Reg<u32, _CAN_TDL2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TDL2R;
#[doc = "`read()` method returns [can_tdl2r::R](can_tdl2r::R) reader structure"]
impl crate::Readable for CAN_TDL2R {}
#[doc = "`write(|w| ..)` method takes [can_tdl2r::W](can_tdl2r::W) writer structure"]
impl crate::Writable for CAN_TDL2R {}
#[doc = "CAN_TDL2R"]
pub mod can_tdl2r;
#[doc = "CAN_TDH2R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tdh2r](can_tdh2r) module"]
pub type CAN_TDH2R = crate::Reg<u32, _CAN_TDH2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_TDH2R;
#[doc = "`read()` method returns [can_tdh2r::R](can_tdh2r::R) reader structure"]
impl crate::Readable for CAN_TDH2R {}
#[doc = "`write(|w| ..)` method takes [can_tdh2r::W](can_tdh2r::W) writer structure"]
impl crate::Writable for CAN_TDH2R {}
#[doc = "CAN_TDH2R"]
pub mod can_tdh2r;
#[doc = "CAN_RI0R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ri0r](can_ri0r) module"]
pub type CAN_RI0R = crate::Reg<u32, _CAN_RI0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_RI0R;
#[doc = "`read()` method returns [can_ri0r::R](can_ri0r::R) reader structure"]
impl crate::Readable for CAN_RI0R {}
#[doc = "CAN_RI0R"]
pub mod can_ri0r;
#[doc = "CAN_RDT0R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_rdt0r](can_rdt0r) module"]
pub type CAN_RDT0R = crate::Reg<u32, _CAN_RDT0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_RDT0R;
#[doc = "`read()` method returns [can_rdt0r::R](can_rdt0r::R) reader structure"]
impl crate::Readable for CAN_RDT0R {}
#[doc = "CAN_RDT0R"]
pub mod can_rdt0r;
#[doc = "CAN_RDL0R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_rdl0r](can_rdl0r) module"]
pub type CAN_RDL0R = crate::Reg<u32, _CAN_RDL0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_RDL0R;
#[doc = "`read()` method returns [can_rdl0r::R](can_rdl0r::R) reader structure"]
impl crate::Readable for CAN_RDL0R {}
#[doc = "CAN_RDL0R"]
pub mod can_rdl0r;
#[doc = "CAN_RDH0R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_rdh0r](can_rdh0r) module"]
pub type CAN_RDH0R = crate::Reg<u32, _CAN_RDH0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_RDH0R;
#[doc = "`read()` method returns [can_rdh0r::R](can_rdh0r::R) reader structure"]
impl crate::Readable for CAN_RDH0R {}
#[doc = "CAN_RDH0R"]
pub mod can_rdh0r;
#[doc = "CAN_RI1R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ri1r](can_ri1r) module"]
pub type CAN_RI1R = crate::Reg<u32, _CAN_RI1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_RI1R;
#[doc = "`read()` method returns [can_ri1r::R](can_ri1r::R) reader structure"]
impl crate::Readable for CAN_RI1R {}
#[doc = "CAN_RI1R"]
pub mod can_ri1r;
#[doc = "CAN_RDT1R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_rdt1r](can_rdt1r) module"]
pub type CAN_RDT1R = crate::Reg<u32, _CAN_RDT1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_RDT1R;
#[doc = "`read()` method returns [can_rdt1r::R](can_rdt1r::R) reader structure"]
impl crate::Readable for CAN_RDT1R {}
#[doc = "CAN_RDT1R"]
pub mod can_rdt1r;
#[doc = "CAN_RDL1R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_rdl1r](can_rdl1r) module"]
pub type CAN_RDL1R = crate::Reg<u32, _CAN_RDL1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_RDL1R;
#[doc = "`read()` method returns [can_rdl1r::R](can_rdl1r::R) reader structure"]
impl crate::Readable for CAN_RDL1R {}
#[doc = "CAN_RDL1R"]
pub mod can_rdl1r;
#[doc = "CAN_RDH1R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_rdh1r](can_rdh1r) module"]
pub type CAN_RDH1R = crate::Reg<u32, _CAN_RDH1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_RDH1R;
#[doc = "`read()` method returns [can_rdh1r::R](can_rdh1r::R) reader structure"]
impl crate::Readable for CAN_RDH1R {}
#[doc = "CAN_RDH1R"]
pub mod can_rdh1r;
#[doc = "CAN_FMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_fmr](can_fmr) module"]
pub type CAN_FMR = crate::Reg<u32, _CAN_FMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_FMR;
#[doc = "`read()` method returns [can_fmr::R](can_fmr::R) reader structure"]
impl crate::Readable for CAN_FMR {}
#[doc = "`write(|w| ..)` method takes [can_fmr::W](can_fmr::W) writer structure"]
impl crate::Writable for CAN_FMR {}
#[doc = "CAN_FMR"]
pub mod can_fmr;
#[doc = "CAN_FM1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_fm1r](can_fm1r) module"]
pub type CAN_FM1R = crate::Reg<u32, _CAN_FM1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_FM1R;
#[doc = "`read()` method returns [can_fm1r::R](can_fm1r::R) reader structure"]
impl crate::Readable for CAN_FM1R {}
#[doc = "`write(|w| ..)` method takes [can_fm1r::W](can_fm1r::W) writer structure"]
impl crate::Writable for CAN_FM1R {}
#[doc = "CAN_FM1R"]
pub mod can_fm1r;
#[doc = "CAN_FS1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_fs1r](can_fs1r) module"]
pub type CAN_FS1R = crate::Reg<u32, _CAN_FS1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_FS1R;
#[doc = "`read()` method returns [can_fs1r::R](can_fs1r::R) reader structure"]
impl crate::Readable for CAN_FS1R {}
#[doc = "`write(|w| ..)` method takes [can_fs1r::W](can_fs1r::W) writer structure"]
impl crate::Writable for CAN_FS1R {}
#[doc = "CAN_FS1R"]
pub mod can_fs1r;
#[doc = "CAN_FFA1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ffa1r](can_ffa1r) module"]
pub type CAN_FFA1R = crate::Reg<u32, _CAN_FFA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_FFA1R;
#[doc = "`read()` method returns [can_ffa1r::R](can_ffa1r::R) reader structure"]
impl crate::Readable for CAN_FFA1R {}
#[doc = "`write(|w| ..)` method takes [can_ffa1r::W](can_ffa1r::W) writer structure"]
impl crate::Writable for CAN_FFA1R {}
#[doc = "CAN_FFA1R"]
pub mod can_ffa1r;
#[doc = "CAN_FA1R\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_fa1r](can_fa1r) module"]
pub type CAN_FA1R = crate::Reg<u32, _CAN_FA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAN_FA1R;
#[doc = "`read()` method returns [can_fa1r::R](can_fa1r::R) reader structure"]
impl crate::Readable for CAN_FA1R {}
#[doc = "`write(|w| ..)` method takes [can_fa1r::W](can_fa1r::W) writer structure"]
impl crate::Writable for CAN_FA1R {}
#[doc = "CAN_FA1R"]
pub mod can_fa1r;
#[doc = "Filter bank 0 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f0r1](f0r1) module"]
pub type F0R1 = crate::Reg<u32, _F0R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F0R1;
#[doc = "`read()` method returns [f0r1::R](f0r1::R) reader structure"]
impl crate::Readable for F0R1 {}
#[doc = "`write(|w| ..)` method takes [f0r1::W](f0r1::W) writer structure"]
impl crate::Writable for F0R1 {}
#[doc = "Filter bank 0 register 1"]
pub mod f0r1;
#[doc = "Filter bank 0 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f0r2](f0r2) module"]
pub type F0R2 = crate::Reg<u32, _F0R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F0R2;
#[doc = "`read()` method returns [f0r2::R](f0r2::R) reader structure"]
impl crate::Readable for F0R2 {}
#[doc = "`write(|w| ..)` method takes [f0r2::W](f0r2::W) writer structure"]
impl crate::Writable for F0R2 {}
#[doc = "Filter bank 0 register 2"]
pub mod f0r2;
#[doc = "Filter bank 1 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f1r1](f1r1) module"]
pub type F1R1 = crate::Reg<u32, _F1R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F1R1;
#[doc = "`read()` method returns [f1r1::R](f1r1::R) reader structure"]
impl crate::Readable for F1R1 {}
#[doc = "`write(|w| ..)` method takes [f1r1::W](f1r1::W) writer structure"]
impl crate::Writable for F1R1 {}
#[doc = "Filter bank 1 register 1"]
pub mod f1r1;
#[doc = "Filter bank 1 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f1r2](f1r2) module"]
pub type F1R2 = crate::Reg<u32, _F1R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F1R2;
#[doc = "`read()` method returns [f1r2::R](f1r2::R) reader structure"]
impl crate::Readable for F1R2 {}
#[doc = "`write(|w| ..)` method takes [f1r2::W](f1r2::W) writer structure"]
impl crate::Writable for F1R2 {}
#[doc = "Filter bank 1 register 2"]
pub mod f1r2;
#[doc = "Filter bank 2 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f2r1](f2r1) module"]
pub type F2R1 = crate::Reg<u32, _F2R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F2R1;
#[doc = "`read()` method returns [f2r1::R](f2r1::R) reader structure"]
impl crate::Readable for F2R1 {}
#[doc = "`write(|w| ..)` method takes [f2r1::W](f2r1::W) writer structure"]
impl crate::Writable for F2R1 {}
#[doc = "Filter bank 2 register 1"]
pub mod f2r1;
#[doc = "Filter bank 2 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f2r2](f2r2) module"]
pub type F2R2 = crate::Reg<u32, _F2R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F2R2;
#[doc = "`read()` method returns [f2r2::R](f2r2::R) reader structure"]
impl crate::Readable for F2R2 {}
#[doc = "`write(|w| ..)` method takes [f2r2::W](f2r2::W) writer structure"]
impl crate::Writable for F2R2 {}
#[doc = "Filter bank 2 register 2"]
pub mod f2r2;
#[doc = "Filter bank 3 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f3r1](f3r1) module"]
pub type F3R1 = crate::Reg<u32, _F3R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F3R1;
#[doc = "`read()` method returns [f3r1::R](f3r1::R) reader structure"]
impl crate::Readable for F3R1 {}
#[doc = "`write(|w| ..)` method takes [f3r1::W](f3r1::W) writer structure"]
impl crate::Writable for F3R1 {}
#[doc = "Filter bank 3 register 1"]
pub mod f3r1;
#[doc = "Filter bank 3 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f3r2](f3r2) module"]
pub type F3R2 = crate::Reg<u32, _F3R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F3R2;
#[doc = "`read()` method returns [f3r2::R](f3r2::R) reader structure"]
impl crate::Readable for F3R2 {}
#[doc = "`write(|w| ..)` method takes [f3r2::W](f3r2::W) writer structure"]
impl crate::Writable for F3R2 {}
#[doc = "Filter bank 3 register 2"]
pub mod f3r2;
#[doc = "Filter bank 4 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f4r1](f4r1) module"]
pub type F4R1 = crate::Reg<u32, _F4R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F4R1;
#[doc = "`read()` method returns [f4r1::R](f4r1::R) reader structure"]
impl crate::Readable for F4R1 {}
#[doc = "`write(|w| ..)` method takes [f4r1::W](f4r1::W) writer structure"]
impl crate::Writable for F4R1 {}
#[doc = "Filter bank 4 register 1"]
pub mod f4r1;
#[doc = "Filter bank 4 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f4r2](f4r2) module"]
pub type F4R2 = crate::Reg<u32, _F4R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F4R2;
#[doc = "`read()` method returns [f4r2::R](f4r2::R) reader structure"]
impl crate::Readable for F4R2 {}
#[doc = "`write(|w| ..)` method takes [f4r2::W](f4r2::W) writer structure"]
impl crate::Writable for F4R2 {}
#[doc = "Filter bank 4 register 2"]
pub mod f4r2;
#[doc = "Filter bank 5 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f5r1](f5r1) module"]
pub type F5R1 = crate::Reg<u32, _F5R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F5R1;
#[doc = "`read()` method returns [f5r1::R](f5r1::R) reader structure"]
impl crate::Readable for F5R1 {}
#[doc = "`write(|w| ..)` method takes [f5r1::W](f5r1::W) writer structure"]
impl crate::Writable for F5R1 {}
#[doc = "Filter bank 5 register 1"]
pub mod f5r1;
#[doc = "Filter bank 5 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f5r2](f5r2) module"]
pub type F5R2 = crate::Reg<u32, _F5R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F5R2;
#[doc = "`read()` method returns [f5r2::R](f5r2::R) reader structure"]
impl crate::Readable for F5R2 {}
#[doc = "`write(|w| ..)` method takes [f5r2::W](f5r2::W) writer structure"]
impl crate::Writable for F5R2 {}
#[doc = "Filter bank 5 register 2"]
pub mod f5r2;
#[doc = "Filter bank 6 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f6r1](f6r1) module"]
pub type F6R1 = crate::Reg<u32, _F6R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F6R1;
#[doc = "`read()` method returns [f6r1::R](f6r1::R) reader structure"]
impl crate::Readable for F6R1 {}
#[doc = "`write(|w| ..)` method takes [f6r1::W](f6r1::W) writer structure"]
impl crate::Writable for F6R1 {}
#[doc = "Filter bank 6 register 1"]
pub mod f6r1;
#[doc = "Filter bank 6 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f6r2](f6r2) module"]
pub type F6R2 = crate::Reg<u32, _F6R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F6R2;
#[doc = "`read()` method returns [f6r2::R](f6r2::R) reader structure"]
impl crate::Readable for F6R2 {}
#[doc = "`write(|w| ..)` method takes [f6r2::W](f6r2::W) writer structure"]
impl crate::Writable for F6R2 {}
#[doc = "Filter bank 6 register 2"]
pub mod f6r2;
#[doc = "Filter bank 7 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f7r1](f7r1) module"]
pub type F7R1 = crate::Reg<u32, _F7R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F7R1;
#[doc = "`read()` method returns [f7r1::R](f7r1::R) reader structure"]
impl crate::Readable for F7R1 {}
#[doc = "`write(|w| ..)` method takes [f7r1::W](f7r1::W) writer structure"]
impl crate::Writable for F7R1 {}
#[doc = "Filter bank 7 register 1"]
pub mod f7r1;
#[doc = "Filter bank 7 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f7r2](f7r2) module"]
pub type F7R2 = crate::Reg<u32, _F7R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F7R2;
#[doc = "`read()` method returns [f7r2::R](f7r2::R) reader structure"]
impl crate::Readable for F7R2 {}
#[doc = "`write(|w| ..)` method takes [f7r2::W](f7r2::W) writer structure"]
impl crate::Writable for F7R2 {}
#[doc = "Filter bank 7 register 2"]
pub mod f7r2;
#[doc = "Filter bank 8 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f8r1](f8r1) module"]
pub type F8R1 = crate::Reg<u32, _F8R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F8R1;
#[doc = "`read()` method returns [f8r1::R](f8r1::R) reader structure"]
impl crate::Readable for F8R1 {}
#[doc = "`write(|w| ..)` method takes [f8r1::W](f8r1::W) writer structure"]
impl crate::Writable for F8R1 {}
#[doc = "Filter bank 8 register 1"]
pub mod f8r1;
#[doc = "Filter bank 8 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f8r2](f8r2) module"]
pub type F8R2 = crate::Reg<u32, _F8R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F8R2;
#[doc = "`read()` method returns [f8r2::R](f8r2::R) reader structure"]
impl crate::Readable for F8R2 {}
#[doc = "`write(|w| ..)` method takes [f8r2::W](f8r2::W) writer structure"]
impl crate::Writable for F8R2 {}
#[doc = "Filter bank 8 register 2"]
pub mod f8r2;
#[doc = "Filter bank 9 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f9r1](f9r1) module"]
pub type F9R1 = crate::Reg<u32, _F9R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F9R1;
#[doc = "`read()` method returns [f9r1::R](f9r1::R) reader structure"]
impl crate::Readable for F9R1 {}
#[doc = "`write(|w| ..)` method takes [f9r1::W](f9r1::W) writer structure"]
impl crate::Writable for F9R1 {}
#[doc = "Filter bank 9 register 1"]
pub mod f9r1;
#[doc = "Filter bank 9 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f9r2](f9r2) module"]
pub type F9R2 = crate::Reg<u32, _F9R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F9R2;
#[doc = "`read()` method returns [f9r2::R](f9r2::R) reader structure"]
impl crate::Readable for F9R2 {}
#[doc = "`write(|w| ..)` method takes [f9r2::W](f9r2::W) writer structure"]
impl crate::Writable for F9R2 {}
#[doc = "Filter bank 9 register 2"]
pub mod f9r2;
#[doc = "Filter bank 10 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f10r1](f10r1) module"]
pub type F10R1 = crate::Reg<u32, _F10R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F10R1;
#[doc = "`read()` method returns [f10r1::R](f10r1::R) reader structure"]
impl crate::Readable for F10R1 {}
#[doc = "`write(|w| ..)` method takes [f10r1::W](f10r1::W) writer structure"]
impl crate::Writable for F10R1 {}
#[doc = "Filter bank 10 register 1"]
pub mod f10r1;
#[doc = "Filter bank 10 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f10r2](f10r2) module"]
pub type F10R2 = crate::Reg<u32, _F10R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F10R2;
#[doc = "`read()` method returns [f10r2::R](f10r2::R) reader structure"]
impl crate::Readable for F10R2 {}
#[doc = "`write(|w| ..)` method takes [f10r2::W](f10r2::W) writer structure"]
impl crate::Writable for F10R2 {}
#[doc = "Filter bank 10 register 2"]
pub mod f10r2;
#[doc = "Filter bank 11 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f11r1](f11r1) module"]
pub type F11R1 = crate::Reg<u32, _F11R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F11R1;
#[doc = "`read()` method returns [f11r1::R](f11r1::R) reader structure"]
impl crate::Readable for F11R1 {}
#[doc = "`write(|w| ..)` method takes [f11r1::W](f11r1::W) writer structure"]
impl crate::Writable for F11R1 {}
#[doc = "Filter bank 11 register 1"]
pub mod f11r1;
#[doc = "Filter bank 11 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f11r2](f11r2) module"]
pub type F11R2 = crate::Reg<u32, _F11R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F11R2;
#[doc = "`read()` method returns [f11r2::R](f11r2::R) reader structure"]
impl crate::Readable for F11R2 {}
#[doc = "`write(|w| ..)` method takes [f11r2::W](f11r2::W) writer structure"]
impl crate::Writable for F11R2 {}
#[doc = "Filter bank 11 register 2"]
pub mod f11r2;
#[doc = "Filter bank 4 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f12r1](f12r1) module"]
pub type F12R1 = crate::Reg<u32, _F12R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F12R1;
#[doc = "`read()` method returns [f12r1::R](f12r1::R) reader structure"]
impl crate::Readable for F12R1 {}
#[doc = "`write(|w| ..)` method takes [f12r1::W](f12r1::W) writer structure"]
impl crate::Writable for F12R1 {}
#[doc = "Filter bank 4 register 1"]
pub mod f12r1;
#[doc = "Filter bank 12 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f12r2](f12r2) module"]
pub type F12R2 = crate::Reg<u32, _F12R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F12R2;
#[doc = "`read()` method returns [f12r2::R](f12r2::R) reader structure"]
impl crate::Readable for F12R2 {}
#[doc = "`write(|w| ..)` method takes [f12r2::W](f12r2::W) writer structure"]
impl crate::Writable for F12R2 {}
#[doc = "Filter bank 12 register 2"]
pub mod f12r2;
#[doc = "Filter bank 13 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f13r1](f13r1) module"]
pub type F13R1 = crate::Reg<u32, _F13R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F13R1;
#[doc = "`read()` method returns [f13r1::R](f13r1::R) reader structure"]
impl crate::Readable for F13R1 {}
#[doc = "`write(|w| ..)` method takes [f13r1::W](f13r1::W) writer structure"]
impl crate::Writable for F13R1 {}
#[doc = "Filter bank 13 register 1"]
pub mod f13r1;
#[doc = "Filter bank 13 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f13r2](f13r2) module"]
pub type F13R2 = crate::Reg<u32, _F13R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F13R2;
#[doc = "`read()` method returns [f13r2::R](f13r2::R) reader structure"]
impl crate::Readable for F13R2 {}
#[doc = "`write(|w| ..)` method takes [f13r2::W](f13r2::W) writer structure"]
impl crate::Writable for F13R2 {}
#[doc = "Filter bank 13 register 2"]
pub mod f13r2;
#[doc = "Filter bank 14 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f14r1](f14r1) module"]
pub type F14R1 = crate::Reg<u32, _F14R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F14R1;
#[doc = "`read()` method returns [f14r1::R](f14r1::R) reader structure"]
impl crate::Readable for F14R1 {}
#[doc = "`write(|w| ..)` method takes [f14r1::W](f14r1::W) writer structure"]
impl crate::Writable for F14R1 {}
#[doc = "Filter bank 14 register 1"]
pub mod f14r1;
#[doc = "Filter bank 14 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f14r2](f14r2) module"]
pub type F14R2 = crate::Reg<u32, _F14R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F14R2;
#[doc = "`read()` method returns [f14r2::R](f14r2::R) reader structure"]
impl crate::Readable for F14R2 {}
#[doc = "`write(|w| ..)` method takes [f14r2::W](f14r2::W) writer structure"]
impl crate::Writable for F14R2 {}
#[doc = "Filter bank 14 register 2"]
pub mod f14r2;
#[doc = "Filter bank 15 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f15r1](f15r1) module"]
pub type F15R1 = crate::Reg<u32, _F15R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F15R1;
#[doc = "`read()` method returns [f15r1::R](f15r1::R) reader structure"]
impl crate::Readable for F15R1 {}
#[doc = "`write(|w| ..)` method takes [f15r1::W](f15r1::W) writer structure"]
impl crate::Writable for F15R1 {}
#[doc = "Filter bank 15 register 1"]
pub mod f15r1;
#[doc = "Filter bank 15 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f15r2](f15r2) module"]
pub type F15R2 = crate::Reg<u32, _F15R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F15R2;
#[doc = "`read()` method returns [f15r2::R](f15r2::R) reader structure"]
impl crate::Readable for F15R2 {}
#[doc = "`write(|w| ..)` method takes [f15r2::W](f15r2::W) writer structure"]
impl crate::Writable for F15R2 {}
#[doc = "Filter bank 15 register 2"]
pub mod f15r2;
#[doc = "Filter bank 16 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f16r1](f16r1) module"]
pub type F16R1 = crate::Reg<u32, _F16R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F16R1;
#[doc = "`read()` method returns [f16r1::R](f16r1::R) reader structure"]
impl crate::Readable for F16R1 {}
#[doc = "`write(|w| ..)` method takes [f16r1::W](f16r1::W) writer structure"]
impl crate::Writable for F16R1 {}
#[doc = "Filter bank 16 register 1"]
pub mod f16r1;
#[doc = "Filter bank 16 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f16r2](f16r2) module"]
pub type F16R2 = crate::Reg<u32, _F16R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F16R2;
#[doc = "`read()` method returns [f16r2::R](f16r2::R) reader structure"]
impl crate::Readable for F16R2 {}
#[doc = "`write(|w| ..)` method takes [f16r2::W](f16r2::W) writer structure"]
impl crate::Writable for F16R2 {}
#[doc = "Filter bank 16 register 2"]
pub mod f16r2;
#[doc = "Filter bank 17 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f17r1](f17r1) module"]
pub type F17R1 = crate::Reg<u32, _F17R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F17R1;
#[doc = "`read()` method returns [f17r1::R](f17r1::R) reader structure"]
impl crate::Readable for F17R1 {}
#[doc = "`write(|w| ..)` method takes [f17r1::W](f17r1::W) writer structure"]
impl crate::Writable for F17R1 {}
#[doc = "Filter bank 17 register 1"]
pub mod f17r1;
#[doc = "Filter bank 17 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f17r2](f17r2) module"]
pub type F17R2 = crate::Reg<u32, _F17R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F17R2;
#[doc = "`read()` method returns [f17r2::R](f17r2::R) reader structure"]
impl crate::Readable for F17R2 {}
#[doc = "`write(|w| ..)` method takes [f17r2::W](f17r2::W) writer structure"]
impl crate::Writable for F17R2 {}
#[doc = "Filter bank 17 register 2"]
pub mod f17r2;
#[doc = "Filter bank 18 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f18r1](f18r1) module"]
pub type F18R1 = crate::Reg<u32, _F18R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F18R1;
#[doc = "`read()` method returns [f18r1::R](f18r1::R) reader structure"]
impl crate::Readable for F18R1 {}
#[doc = "`write(|w| ..)` method takes [f18r1::W](f18r1::W) writer structure"]
impl crate::Writable for F18R1 {}
#[doc = "Filter bank 18 register 1"]
pub mod f18r1;
#[doc = "Filter bank 18 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f18r2](f18r2) module"]
pub type F18R2 = crate::Reg<u32, _F18R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F18R2;
#[doc = "`read()` method returns [f18r2::R](f18r2::R) reader structure"]
impl crate::Readable for F18R2 {}
#[doc = "`write(|w| ..)` method takes [f18r2::W](f18r2::W) writer structure"]
impl crate::Writable for F18R2 {}
#[doc = "Filter bank 18 register 2"]
pub mod f18r2;
#[doc = "Filter bank 19 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f19r1](f19r1) module"]
pub type F19R1 = crate::Reg<u32, _F19R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F19R1;
#[doc = "`read()` method returns [f19r1::R](f19r1::R) reader structure"]
impl crate::Readable for F19R1 {}
#[doc = "`write(|w| ..)` method takes [f19r1::W](f19r1::W) writer structure"]
impl crate::Writable for F19R1 {}
#[doc = "Filter bank 19 register 1"]
pub mod f19r1;
#[doc = "Filter bank 19 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f19r2](f19r2) module"]
pub type F19R2 = crate::Reg<u32, _F19R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F19R2;
#[doc = "`read()` method returns [f19r2::R](f19r2::R) reader structure"]
impl crate::Readable for F19R2 {}
#[doc = "`write(|w| ..)` method takes [f19r2::W](f19r2::W) writer structure"]
impl crate::Writable for F19R2 {}
#[doc = "Filter bank 19 register 2"]
pub mod f19r2;
#[doc = "Filter bank 20 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f20r1](f20r1) module"]
pub type F20R1 = crate::Reg<u32, _F20R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F20R1;
#[doc = "`read()` method returns [f20r1::R](f20r1::R) reader structure"]
impl crate::Readable for F20R1 {}
#[doc = "`write(|w| ..)` method takes [f20r1::W](f20r1::W) writer structure"]
impl crate::Writable for F20R1 {}
#[doc = "Filter bank 20 register 1"]
pub mod f20r1;
#[doc = "Filter bank 20 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f20r2](f20r2) module"]
pub type F20R2 = crate::Reg<u32, _F20R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F20R2;
#[doc = "`read()` method returns [f20r2::R](f20r2::R) reader structure"]
impl crate::Readable for F20R2 {}
#[doc = "`write(|w| ..)` method takes [f20r2::W](f20r2::W) writer structure"]
impl crate::Writable for F20R2 {}
#[doc = "Filter bank 20 register 2"]
pub mod f20r2;
#[doc = "Filter bank 21 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f21r1](f21r1) module"]
pub type F21R1 = crate::Reg<u32, _F21R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F21R1;
#[doc = "`read()` method returns [f21r1::R](f21r1::R) reader structure"]
impl crate::Readable for F21R1 {}
#[doc = "`write(|w| ..)` method takes [f21r1::W](f21r1::W) writer structure"]
impl crate::Writable for F21R1 {}
#[doc = "Filter bank 21 register 1"]
pub mod f21r1;
#[doc = "Filter bank 21 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f21r2](f21r2) module"]
pub type F21R2 = crate::Reg<u32, _F21R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F21R2;
#[doc = "`read()` method returns [f21r2::R](f21r2::R) reader structure"]
impl crate::Readable for F21R2 {}
#[doc = "`write(|w| ..)` method takes [f21r2::W](f21r2::W) writer structure"]
impl crate::Writable for F21R2 {}
#[doc = "Filter bank 21 register 2"]
pub mod f21r2;
#[doc = "Filter bank 22 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f22r1](f22r1) module"]
pub type F22R1 = crate::Reg<u32, _F22R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F22R1;
#[doc = "`read()` method returns [f22r1::R](f22r1::R) reader structure"]
impl crate::Readable for F22R1 {}
#[doc = "`write(|w| ..)` method takes [f22r1::W](f22r1::W) writer structure"]
impl crate::Writable for F22R1 {}
#[doc = "Filter bank 22 register 1"]
pub mod f22r1;
#[doc = "Filter bank 22 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f22r2](f22r2) module"]
pub type F22R2 = crate::Reg<u32, _F22R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F22R2;
#[doc = "`read()` method returns [f22r2::R](f22r2::R) reader structure"]
impl crate::Readable for F22R2 {}
#[doc = "`write(|w| ..)` method takes [f22r2::W](f22r2::W) writer structure"]
impl crate::Writable for F22R2 {}
#[doc = "Filter bank 22 register 2"]
pub mod f22r2;
#[doc = "Filter bank 23 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f23r1](f23r1) module"]
pub type F23R1 = crate::Reg<u32, _F23R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F23R1;
#[doc = "`read()` method returns [f23r1::R](f23r1::R) reader structure"]
impl crate::Readable for F23R1 {}
#[doc = "`write(|w| ..)` method takes [f23r1::W](f23r1::W) writer structure"]
impl crate::Writable for F23R1 {}
#[doc = "Filter bank 23 register 1"]
pub mod f23r1;
#[doc = "Filter bank 23 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f23r2](f23r2) module"]
pub type F23R2 = crate::Reg<u32, _F23R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F23R2;
#[doc = "`read()` method returns [f23r2::R](f23r2::R) reader structure"]
impl crate::Readable for F23R2 {}
#[doc = "`write(|w| ..)` method takes [f23r2::W](f23r2::W) writer structure"]
impl crate::Writable for F23R2 {}
#[doc = "Filter bank 23 register 2"]
pub mod f23r2;
#[doc = "Filter bank 24 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f24r1](f24r1) module"]
pub type F24R1 = crate::Reg<u32, _F24R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F24R1;
#[doc = "`read()` method returns [f24r1::R](f24r1::R) reader structure"]
impl crate::Readable for F24R1 {}
#[doc = "`write(|w| ..)` method takes [f24r1::W](f24r1::W) writer structure"]
impl crate::Writable for F24R1 {}
#[doc = "Filter bank 24 register 1"]
pub mod f24r1;
#[doc = "Filter bank 24 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f24r2](f24r2) module"]
pub type F24R2 = crate::Reg<u32, _F24R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F24R2;
#[doc = "`read()` method returns [f24r2::R](f24r2::R) reader structure"]
impl crate::Readable for F24R2 {}
#[doc = "`write(|w| ..)` method takes [f24r2::W](f24r2::W) writer structure"]
impl crate::Writable for F24R2 {}
#[doc = "Filter bank 24 register 2"]
pub mod f24r2;
#[doc = "Filter bank 25 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f25r1](f25r1) module"]
pub type F25R1 = crate::Reg<u32, _F25R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F25R1;
#[doc = "`read()` method returns [f25r1::R](f25r1::R) reader structure"]
impl crate::Readable for F25R1 {}
#[doc = "`write(|w| ..)` method takes [f25r1::W](f25r1::W) writer structure"]
impl crate::Writable for F25R1 {}
#[doc = "Filter bank 25 register 1"]
pub mod f25r1;
#[doc = "Filter bank 25 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f25r2](f25r2) module"]
pub type F25R2 = crate::Reg<u32, _F25R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F25R2;
#[doc = "`read()` method returns [f25r2::R](f25r2::R) reader structure"]
impl crate::Readable for F25R2 {}
#[doc = "`write(|w| ..)` method takes [f25r2::W](f25r2::W) writer structure"]
impl crate::Writable for F25R2 {}
#[doc = "Filter bank 25 register 2"]
pub mod f25r2;
#[doc = "Filter bank 26 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f26r1](f26r1) module"]
pub type F26R1 = crate::Reg<u32, _F26R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F26R1;
#[doc = "`read()` method returns [f26r1::R](f26r1::R) reader structure"]
impl crate::Readable for F26R1 {}
#[doc = "`write(|w| ..)` method takes [f26r1::W](f26r1::W) writer structure"]
impl crate::Writable for F26R1 {}
#[doc = "Filter bank 26 register 1"]
pub mod f26r1;
#[doc = "Filter bank 26 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f26r2](f26r2) module"]
pub type F26R2 = crate::Reg<u32, _F26R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F26R2;
#[doc = "`read()` method returns [f26r2::R](f26r2::R) reader structure"]
impl crate::Readable for F26R2 {}
#[doc = "`write(|w| ..)` method takes [f26r2::W](f26r2::W) writer structure"]
impl crate::Writable for F26R2 {}
#[doc = "Filter bank 26 register 2"]
pub mod f26r2;
#[doc = "Filter bank 27 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f27r1](f27r1) module"]
pub type F27R1 = crate::Reg<u32, _F27R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F27R1;
#[doc = "`read()` method returns [f27r1::R](f27r1::R) reader structure"]
impl crate::Readable for F27R1 {}
#[doc = "`write(|w| ..)` method takes [f27r1::W](f27r1::W) writer structure"]
impl crate::Writable for F27R1 {}
#[doc = "Filter bank 27 register 1"]
pub mod f27r1;
#[doc = "Filter bank 27 register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f27r2](f27r2) module"]
pub type F27R2 = crate::Reg<u32, _F27R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F27R2;
#[doc = "`read()` method returns [f27r2::R](f27r2::R) reader structure"]
impl crate::Readable for F27R2 {}
#[doc = "`write(|w| ..)` method takes [f27r2::W](f27r2::W) writer structure"]
impl crate::Writable for F27R2 {}
#[doc = "Filter bank 27 register 2"]
pub mod f27r2;
