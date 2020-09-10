#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Reader of field `OVR3`"]
pub type OVR3_R = crate::R<bool, bool>;
#[doc = "Reader of field `STRT3`"]
pub type STRT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `JSTRT3`"]
pub type JSTRT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `JEOC3`"]
pub type JEOC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC3`"]
pub type EOC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `AWD3`"]
pub type AWD3_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVR2`"]
pub type OVR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `STRT2`"]
pub type STRT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `JSTRT2`"]
pub type JSTRT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `JEOC2`"]
pub type JEOC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC2`"]
pub type EOC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `AWD2`"]
pub type AWD2_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVR1`"]
pub type OVR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `STRT1`"]
pub type STRT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `JSTRT1`"]
pub type JSTRT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `JEOC1`"]
pub type JEOC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC1`"]
pub type EOC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `AWD1`"]
pub type AWD1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 21 - Overrun flag of ADC3"]
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Regular channel Start flag of ADC 3"]
    #[inline(always)]
    pub fn strt3(&self) -> STRT3_R {
        STRT3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Injected channel Start flag of ADC 3"]
    #[inline(always)]
    pub fn jstrt3(&self) -> JSTRT3_R {
        JSTRT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Injected channel end of conversion of ADC 3"]
    #[inline(always)]
    pub fn jeoc3(&self) -> JEOC3_R {
        JEOC3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - End of conversion of ADC 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R {
        EOC3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Analog watchdog flag of ADC 3"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Overrun flag of ADC 2"]
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Regular channel Start flag of ADC 2"]
    #[inline(always)]
    pub fn strt2(&self) -> STRT2_R {
        STRT2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Injected channel Start flag of ADC 2"]
    #[inline(always)]
    pub fn jstrt2(&self) -> JSTRT2_R {
        JSTRT2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Injected channel end of conversion of ADC 2"]
    #[inline(always)]
    pub fn jeoc2(&self) -> JEOC2_R {
        JEOC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - End of conversion of ADC 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R {
        EOC2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog flag of ADC 2"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun flag of ADC 1"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Regular channel Start flag of ADC 1"]
    #[inline(always)]
    pub fn strt1(&self) -> STRT1_R {
        STRT1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Injected channel Start flag of ADC 1"]
    #[inline(always)]
    pub fn jstrt1(&self) -> JSTRT1_R {
        JSTRT1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion of ADC 1"]
    #[inline(always)]
    pub fn jeoc1(&self) -> JEOC1_R {
        JEOC1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of conversion of ADC 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Analog watchdog flag of ADC 1"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new((self.bits & 0x01) != 0)
    }
}
