#[doc = "Reader of register CIER"]
pub type R = crate::R<u32, super::CIER>;
#[doc = "Reader of field `CSSLSE`"]
pub type CSSLSE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSIRDYIE`"]
pub type MSIRDYIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLRDYIE`"]
pub type PLLRDYIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSERDYIE`"]
pub type HSERDYIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSI16RDYIE`"]
pub type HSI16RDYIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSERDYIE`"]
pub type LSERDYIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSIRDYIE`"]
pub type LSIRDYIE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 7 - LSE CSS interrupt flag"]
    #[inline(always)]
    pub fn csslse(&self) -> CSSLSE_R {
        CSSLSE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI16 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi16rdyie(&self) -> HSI16RDYIE_R {
        HSI16RDYIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 0x01) != 0)
    }
}
