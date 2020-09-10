#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - master control register"]
    pub mcr: MCR,
    #[doc = "0x04 - master status register"]
    pub msr: MSR,
    #[doc = "0x08 - transmit status register"]
    pub tsr: TSR,
    #[doc = "0x0c - receive FIFO 0 register"]
    pub rf0r: RF0R,
    #[doc = "0x10 - receive FIFO 1 register"]
    pub rf1r: RF1R,
    #[doc = "0x14 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x18 - interrupt enable register"]
    pub esr: ESR,
    #[doc = "0x1c - bit timing register"]
    pub btr: BTR,
    _reserved8: [u8; 352usize],
    #[doc = "0x180 - TX mailbox identifier register"]
    pub ti0r: TI0R,
    #[doc = "0x184 - mailbox data length control and time stamp register"]
    pub tdt0r: TDT0R,
    #[doc = "0x188 - mailbox data low register"]
    pub tdl0r: TDL0R,
    #[doc = "0x18c - mailbox data high register"]
    pub tdh0r: TDH0R,
    #[doc = "0x190 - mailbox identifier register"]
    pub ti1r: TI1R,
    #[doc = "0x194 - mailbox data length control and time stamp register"]
    pub tdt1r: TDT1R,
    #[doc = "0x198 - mailbox data low register"]
    pub tdl1r: TDL1R,
    #[doc = "0x19c - mailbox data high register"]
    pub tdh1r: TDH1R,
    #[doc = "0x1a0 - mailbox identifier register"]
    pub ti2r: TI2R,
    #[doc = "0x1a4 - mailbox data length control and time stamp register"]
    pub tdt2r: TDT2R,
    #[doc = "0x1a8 - mailbox data low register"]
    pub tdl2r: TDL2R,
    #[doc = "0x1ac - mailbox data high register"]
    pub tdh2r: TDH2R,
    #[doc = "0x1b0 - receive FIFO mailbox identifier register"]
    pub ri0r: RI0R,
    #[doc = "0x1b4 - mailbox data high register"]
    pub rdt0r: RDT0R,
    #[doc = "0x1b8 - mailbox data high register"]
    pub rdl0r: RDL0R,
    #[doc = "0x1bc - receive FIFO mailbox data high register"]
    pub rdh0r: RDH0R,
    #[doc = "0x1c0 - mailbox data high register"]
    pub ri1r: RI1R,
    #[doc = "0x1c4 - mailbox data high register"]
    pub rdt1r: RDT1R,
    #[doc = "0x1c8 - mailbox data high register"]
    pub rdl1r: RDL1R,
    #[doc = "0x1cc - mailbox data high register"]
    pub rdh1r: RDH1R,
    _reserved28: [u8; 48usize],
    #[doc = "0x200 - filter master register"]
    pub fmr: FMR,
    #[doc = "0x204 - filter mode register"]
    pub fm1r: FM1R,
    _reserved30: [u8; 4usize],
    #[doc = "0x20c - filter scale register"]
    pub fs1r: FS1R,
    _reserved31: [u8; 4usize],
    #[doc = "0x214 - filter FIFO assignment register"]
    pub ffa1r: FFA1R,
    _reserved32: [u8; 4usize],
    #[doc = "0x21c - filter activation register"]
    pub fa1r: FA1R,
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
#[doc = "master control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "master control register"]
pub mod mcr;
#[doc = "master status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](msr) module"]
pub type MSR = crate::Reg<u32, _MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR;
#[doc = "`read()` method returns [msr::R](msr::R) reader structure"]
impl crate::Readable for MSR {}
#[doc = "`write(|w| ..)` method takes [msr::W](msr::W) writer structure"]
impl crate::Writable for MSR {}
#[doc = "master status register"]
pub mod msr;
#[doc = "transmit status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](tsr) module"]
pub type TSR = crate::Reg<u32, _TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSR;
#[doc = "`read()` method returns [tsr::R](tsr::R) reader structure"]
impl crate::Readable for TSR {}
#[doc = "`write(|w| ..)` method takes [tsr::W](tsr::W) writer structure"]
impl crate::Writable for TSR {}
#[doc = "transmit status register"]
pub mod tsr;
#[doc = "receive FIFO 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf0r](rf0r) module"]
pub type RF0R = crate::Reg<u32, _RF0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF0R;
#[doc = "`read()` method returns [rf0r::R](rf0r::R) reader structure"]
impl crate::Readable for RF0R {}
#[doc = "`write(|w| ..)` method takes [rf0r::W](rf0r::W) writer structure"]
impl crate::Writable for RF0R {}
#[doc = "receive FIFO 0 register"]
pub mod rf0r;
#[doc = "receive FIFO 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf1r](rf1r) module"]
pub type RF1R = crate::Reg<u32, _RF1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RF1R;
#[doc = "`read()` method returns [rf1r::R](rf1r::R) reader structure"]
impl crate::Readable for RF1R {}
#[doc = "`write(|w| ..)` method takes [rf1r::W](rf1r::W) writer structure"]
impl crate::Writable for RF1R {}
#[doc = "receive FIFO 1 register"]
pub mod rf1r;
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr](esr) module"]
pub type ESR = crate::Reg<u32, _ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESR;
#[doc = "`read()` method returns [esr::R](esr::R) reader structure"]
impl crate::Readable for ESR {}
#[doc = "`write(|w| ..)` method takes [esr::W](esr::W) writer structure"]
impl crate::Writable for ESR {}
#[doc = "interrupt enable register"]
pub mod esr;
#[doc = "bit timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr](btr) module"]
pub type BTR = crate::Reg<u32, _BTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR;
#[doc = "`read()` method returns [btr::R](btr::R) reader structure"]
impl crate::Readable for BTR {}
#[doc = "`write(|w| ..)` method takes [btr::W](btr::W) writer structure"]
impl crate::Writable for BTR {}
#[doc = "bit timing register"]
pub mod btr;
#[doc = "TX mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ti0r](ti0r) module"]
pub type TI0R = crate::Reg<u32, _TI0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TI0R;
#[doc = "`read()` method returns [ti0r::R](ti0r::R) reader structure"]
impl crate::Readable for TI0R {}
#[doc = "`write(|w| ..)` method takes [ti0r::W](ti0r::W) writer structure"]
impl crate::Writable for TI0R {}
#[doc = "TX mailbox identifier register"]
pub mod ti0r;
#[doc = "mailbox data length control and time stamp register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdt0r](tdt0r) module"]
pub type TDT0R = crate::Reg<u32, _TDT0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDT0R;
#[doc = "`read()` method returns [tdt0r::R](tdt0r::R) reader structure"]
impl crate::Readable for TDT0R {}
#[doc = "`write(|w| ..)` method takes [tdt0r::W](tdt0r::W) writer structure"]
impl crate::Writable for TDT0R {}
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt0r;
#[doc = "mailbox data low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdl0r](tdl0r) module"]
pub type TDL0R = crate::Reg<u32, _TDL0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDL0R;
#[doc = "`read()` method returns [tdl0r::R](tdl0r::R) reader structure"]
impl crate::Readable for TDL0R {}
#[doc = "`write(|w| ..)` method takes [tdl0r::W](tdl0r::W) writer structure"]
impl crate::Writable for TDL0R {}
#[doc = "mailbox data low register"]
pub mod tdl0r;
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdh0r](tdh0r) module"]
pub type TDH0R = crate::Reg<u32, _TDH0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDH0R;
#[doc = "`read()` method returns [tdh0r::R](tdh0r::R) reader structure"]
impl crate::Readable for TDH0R {}
#[doc = "`write(|w| ..)` method takes [tdh0r::W](tdh0r::W) writer structure"]
impl crate::Writable for TDH0R {}
#[doc = "mailbox data high register"]
pub mod tdh0r;
#[doc = "mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ti1r](ti1r) module"]
pub type TI1R = crate::Reg<u32, _TI1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TI1R;
#[doc = "`read()` method returns [ti1r::R](ti1r::R) reader structure"]
impl crate::Readable for TI1R {}
#[doc = "`write(|w| ..)` method takes [ti1r::W](ti1r::W) writer structure"]
impl crate::Writable for TI1R {}
#[doc = "mailbox identifier register"]
pub mod ti1r;
#[doc = "mailbox data length control and time stamp register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdt1r](tdt1r) module"]
pub type TDT1R = crate::Reg<u32, _TDT1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDT1R;
#[doc = "`read()` method returns [tdt1r::R](tdt1r::R) reader structure"]
impl crate::Readable for TDT1R {}
#[doc = "`write(|w| ..)` method takes [tdt1r::W](tdt1r::W) writer structure"]
impl crate::Writable for TDT1R {}
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt1r;
#[doc = "mailbox data low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdl1r](tdl1r) module"]
pub type TDL1R = crate::Reg<u32, _TDL1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDL1R;
#[doc = "`read()` method returns [tdl1r::R](tdl1r::R) reader structure"]
impl crate::Readable for TDL1R {}
#[doc = "`write(|w| ..)` method takes [tdl1r::W](tdl1r::W) writer structure"]
impl crate::Writable for TDL1R {}
#[doc = "mailbox data low register"]
pub mod tdl1r;
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdh1r](tdh1r) module"]
pub type TDH1R = crate::Reg<u32, _TDH1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDH1R;
#[doc = "`read()` method returns [tdh1r::R](tdh1r::R) reader structure"]
impl crate::Readable for TDH1R {}
#[doc = "`write(|w| ..)` method takes [tdh1r::W](tdh1r::W) writer structure"]
impl crate::Writable for TDH1R {}
#[doc = "mailbox data high register"]
pub mod tdh1r;
#[doc = "mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ti2r](ti2r) module"]
pub type TI2R = crate::Reg<u32, _TI2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TI2R;
#[doc = "`read()` method returns [ti2r::R](ti2r::R) reader structure"]
impl crate::Readable for TI2R {}
#[doc = "`write(|w| ..)` method takes [ti2r::W](ti2r::W) writer structure"]
impl crate::Writable for TI2R {}
#[doc = "mailbox identifier register"]
pub mod ti2r;
#[doc = "mailbox data length control and time stamp register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdt2r](tdt2r) module"]
pub type TDT2R = crate::Reg<u32, _TDT2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDT2R;
#[doc = "`read()` method returns [tdt2r::R](tdt2r::R) reader structure"]
impl crate::Readable for TDT2R {}
#[doc = "`write(|w| ..)` method takes [tdt2r::W](tdt2r::W) writer structure"]
impl crate::Writable for TDT2R {}
#[doc = "mailbox data length control and time stamp register"]
pub mod tdt2r;
#[doc = "mailbox data low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdl2r](tdl2r) module"]
pub type TDL2R = crate::Reg<u32, _TDL2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDL2R;
#[doc = "`read()` method returns [tdl2r::R](tdl2r::R) reader structure"]
impl crate::Readable for TDL2R {}
#[doc = "`write(|w| ..)` method takes [tdl2r::W](tdl2r::W) writer structure"]
impl crate::Writable for TDL2R {}
#[doc = "mailbox data low register"]
pub mod tdl2r;
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdh2r](tdh2r) module"]
pub type TDH2R = crate::Reg<u32, _TDH2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDH2R;
#[doc = "`read()` method returns [tdh2r::R](tdh2r::R) reader structure"]
impl crate::Readable for TDH2R {}
#[doc = "`write(|w| ..)` method takes [tdh2r::W](tdh2r::W) writer structure"]
impl crate::Writable for TDH2R {}
#[doc = "mailbox data high register"]
pub mod tdh2r;
#[doc = "receive FIFO mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ri0r](ri0r) module"]
pub type RI0R = crate::Reg<u32, _RI0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RI0R;
#[doc = "`read()` method returns [ri0r::R](ri0r::R) reader structure"]
impl crate::Readable for RI0R {}
#[doc = "receive FIFO mailbox identifier register"]
pub mod ri0r;
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdt0r](rdt0r) module"]
pub type RDT0R = crate::Reg<u32, _RDT0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDT0R;
#[doc = "`read()` method returns [rdt0r::R](rdt0r::R) reader structure"]
impl crate::Readable for RDT0R {}
#[doc = "mailbox data high register"]
pub mod rdt0r;
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdl0r](rdl0r) module"]
pub type RDL0R = crate::Reg<u32, _RDL0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDL0R;
#[doc = "`read()` method returns [rdl0r::R](rdl0r::R) reader structure"]
impl crate::Readable for RDL0R {}
#[doc = "mailbox data high register"]
pub mod rdl0r;
#[doc = "receive FIFO mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdh0r](rdh0r) module"]
pub type RDH0R = crate::Reg<u32, _RDH0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDH0R;
#[doc = "`read()` method returns [rdh0r::R](rdh0r::R) reader structure"]
impl crate::Readable for RDH0R {}
#[doc = "receive FIFO mailbox data high register"]
pub mod rdh0r;
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ri1r](ri1r) module"]
pub type RI1R = crate::Reg<u32, _RI1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RI1R;
#[doc = "`read()` method returns [ri1r::R](ri1r::R) reader structure"]
impl crate::Readable for RI1R {}
#[doc = "mailbox data high register"]
pub mod ri1r;
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdt1r](rdt1r) module"]
pub type RDT1R = crate::Reg<u32, _RDT1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDT1R;
#[doc = "`read()` method returns [rdt1r::R](rdt1r::R) reader structure"]
impl crate::Readable for RDT1R {}
#[doc = "mailbox data high register"]
pub mod rdt1r;
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdl1r](rdl1r) module"]
pub type RDL1R = crate::Reg<u32, _RDL1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDL1R;
#[doc = "`read()` method returns [rdl1r::R](rdl1r::R) reader structure"]
impl crate::Readable for RDL1R {}
#[doc = "mailbox data high register"]
pub mod rdl1r;
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdh1r](rdh1r) module"]
pub type RDH1R = crate::Reg<u32, _RDH1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDH1R;
#[doc = "`read()` method returns [rdh1r::R](rdh1r::R) reader structure"]
impl crate::Readable for RDH1R {}
#[doc = "mailbox data high register"]
pub mod rdh1r;
#[doc = "filter master register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmr](fmr) module"]
pub type FMR = crate::Reg<u32, _FMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMR;
#[doc = "`read()` method returns [fmr::R](fmr::R) reader structure"]
impl crate::Readable for FMR {}
#[doc = "`write(|w| ..)` method takes [fmr::W](fmr::W) writer structure"]
impl crate::Writable for FMR {}
#[doc = "filter master register"]
pub mod fmr;
#[doc = "filter mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm1r](fm1r) module"]
pub type FM1R = crate::Reg<u32, _FM1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FM1R;
#[doc = "`read()` method returns [fm1r::R](fm1r::R) reader structure"]
impl crate::Readable for FM1R {}
#[doc = "`write(|w| ..)` method takes [fm1r::W](fm1r::W) writer structure"]
impl crate::Writable for FM1R {}
#[doc = "filter mode register"]
pub mod fm1r;
#[doc = "filter scale register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs1r](fs1r) module"]
pub type FS1R = crate::Reg<u32, _FS1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS1R;
#[doc = "`read()` method returns [fs1r::R](fs1r::R) reader structure"]
impl crate::Readable for FS1R {}
#[doc = "`write(|w| ..)` method takes [fs1r::W](fs1r::W) writer structure"]
impl crate::Writable for FS1R {}
#[doc = "filter scale register"]
pub mod fs1r;
#[doc = "filter FIFO assignment register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffa1r](ffa1r) module"]
pub type FFA1R = crate::Reg<u32, _FFA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FFA1R;
#[doc = "`read()` method returns [ffa1r::R](ffa1r::R) reader structure"]
impl crate::Readable for FFA1R {}
#[doc = "`write(|w| ..)` method takes [ffa1r::W](ffa1r::W) writer structure"]
impl crate::Writable for FFA1R {}
#[doc = "filter FIFO assignment register"]
pub mod ffa1r;
#[doc = "filter activation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fa1r](fa1r) module"]
pub type FA1R = crate::Reg<u32, _FA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FA1R;
#[doc = "`read()` method returns [fa1r::R](fa1r::R) reader structure"]
impl crate::Readable for FA1R {}
#[doc = "`write(|w| ..)` method takes [fa1r::W](fa1r::W) writer structure"]
impl crate::Writable for FA1R {}
#[doc = "filter activation register"]
pub mod fa1r;
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
