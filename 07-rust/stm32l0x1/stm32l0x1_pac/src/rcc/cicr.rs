#[doc = "Reader of register CICR"]
pub type R = crate::R<u32, super::CICR>;
#[doc = "Reader of field `CSSHSEC`"]
pub type CSSHSEC_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSSLSEC`"]
pub type CSSLSEC_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSIRDYC`"]
pub type MSIRDYC_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLRDYC`"]
pub type PLLRDYC_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSERDYC`"]
pub type HSERDYC_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSI16RDYC`"]
pub type HSI16RDYC_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSERDYC`"]
pub type LSERDYC_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSIRDYC`"]
pub type LSIRDYC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 8 - Clock Security System Interrupt clear"]
    #[inline(always)]
    pub fn csshsec(&self) -> CSSHSEC_R {
        CSSHSEC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LSE Clock Security System Interrupt clear"]
    #[inline(always)]
    pub fn csslsec(&self) -> CSSLSEC_R {
        CSSLSEC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MSI ready Interrupt clear"]
    #[inline(always)]
    pub fn msirdyc(&self) -> MSIRDYC_R {
        MSIRDYC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PLL ready Interrupt clear"]
    #[inline(always)]
    pub fn pllrdyc(&self) -> PLLRDYC_R {
        PLLRDYC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt clear"]
    #[inline(always)]
    pub fn hserdyc(&self) -> HSERDYC_R {
        HSERDYC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI16 ready Interrupt clear"]
    #[inline(always)]
    pub fn hsi16rdyc(&self) -> HSI16RDYC_R {
        HSI16RDYC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt clear"]
    #[inline(always)]
    pub fn lserdyc(&self) -> LSERDYC_R {
        LSERDYC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LSI ready Interrupt clear"]
    #[inline(always)]
    pub fn lsirdyc(&self) -> LSIRDYC_R {
        LSIRDYC_R::new((self.bits & 0x01) != 0)
    }
}
