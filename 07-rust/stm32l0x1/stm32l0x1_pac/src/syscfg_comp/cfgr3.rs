#[doc = "Reader of register CFGR3"]
pub type R = crate::R<u32, super::CFGR3>;
#[doc = "Writer for register CFGR3"]
pub type W = crate::W<u32, super::CFGR3>;
#[doc = "Register CFGR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `REF_LOCK`"]
pub struct REF_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `VREFINT_RDYF`"]
pub type VREFINT_RDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `VREFINT_COMP_RDYF`"]
pub type VREFINT_COMP_RDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `VREFINT_ADC_RDYF`"]
pub type VREFINT_ADC_RDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SENSOR_ADC_RDYF`"]
pub type SENSOR_ADC_RDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `REF_RC48MHz_RDYF`"]
pub type REF_RC48MHZ_RDYF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENREF_RC48MHz`"]
pub type ENREF_RC48MHZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENREF_RC48MHz`"]
pub struct ENREF_RC48MHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ENREF_RC48MHZ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ENBUF_VREFINT_COMP`"]
pub type ENBUF_VREFINT_COMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBUF_VREFINT_COMP`"]
pub struct ENBUF_VREFINT_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBUF_VREFINT_COMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ENBUF_SENSOR_ADC`"]
pub type ENBUF_SENSOR_ADC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBUF_SENSOR_ADC`"]
pub struct ENBUF_SENSOR_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBUF_SENSOR_ADC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ENBUF_BGAP_ADC`"]
pub type ENBUF_BGAP_ADC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBUF_BGAP_ADC`"]
pub struct ENBUF_BGAP_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBUF_BGAP_ADC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SEL_VREF_OUT`"]
pub type SEL_VREF_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL_VREF_OUT`"]
pub struct SEL_VREF_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_VREF_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `EN_BGAP`"]
pub type EN_BGAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_BGAP`"]
pub struct EN_BGAP_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_BGAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - VREFINT ready flag"]
    #[inline(always)]
    pub fn vrefint_rdyf(&self) -> VREFINT_RDYF_R {
        VREFINT_RDYF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - VREFINT for comparator ready flag"]
    #[inline(always)]
    pub fn vrefint_comp_rdyf(&self) -> VREFINT_COMP_RDYF_R {
        VREFINT_COMP_RDYF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - VREFINT for ADC ready flag"]
    #[inline(always)]
    pub fn vrefint_adc_rdyf(&self) -> VREFINT_ADC_RDYF_R {
        VREFINT_ADC_RDYF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Sensor for ADC ready flag"]
    #[inline(always)]
    pub fn sensor_adc_rdyf(&self) -> SENSOR_ADC_RDYF_R {
        SENSOR_ADC_RDYF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - VREFINT for 48 MHz RC oscillator ready flag"]
    #[inline(always)]
    pub fn ref_rc48mhz_rdyf(&self) -> REF_RC48MHZ_RDYF_R {
        REF_RC48MHZ_RDYF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VREFINT reference for 48 MHz RC oscillator enable bit"]
    #[inline(always)]
    pub fn enref_rc48mhz(&self) -> ENREF_RC48MHZ_R {
        ENREF_RC48MHZ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - VREFINT reference for comparator 2 enable bit"]
    #[inline(always)]
    pub fn enbuf_vrefint_comp(&self) -> ENBUF_VREFINT_COMP_R {
        ENBUF_VREFINT_COMP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Sensor reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_sensor_adc(&self) -> ENBUF_SENSOR_ADC_R {
        ENBUF_SENSOR_ADC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VREFINT reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_bgap_adc(&self) -> ENBUF_BGAP_ADC_R {
        ENBUF_BGAP_ADC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - BGAP_ADC connection bit"]
    #[inline(always)]
    pub fn sel_vref_out(&self) -> SEL_VREF_OUT_R {
        SEL_VREF_OUT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Vref Enable bit"]
    #[inline(always)]
    pub fn en_bgap(&self) -> EN_BGAP_R {
        EN_BGAP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - REF_CTRL lock bit"]
    #[inline(always)]
    pub fn ref_lock(&mut self) -> REF_LOCK_W {
        REF_LOCK_W { w: self }
    }
    #[doc = "Bit 13 - VREFINT reference for 48 MHz RC oscillator enable bit"]
    #[inline(always)]
    pub fn enref_rc48mhz(&mut self) -> ENREF_RC48MHZ_W {
        ENREF_RC48MHZ_W { w: self }
    }
    #[doc = "Bit 12 - VREFINT reference for comparator 2 enable bit"]
    #[inline(always)]
    pub fn enbuf_vrefint_comp(&mut self) -> ENBUF_VREFINT_COMP_W {
        ENBUF_VREFINT_COMP_W { w: self }
    }
    #[doc = "Bit 9 - Sensor reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_sensor_adc(&mut self) -> ENBUF_SENSOR_ADC_W {
        ENBUF_SENSOR_ADC_W { w: self }
    }
    #[doc = "Bit 8 - VREFINT reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_bgap_adc(&mut self) -> ENBUF_BGAP_ADC_W {
        ENBUF_BGAP_ADC_W { w: self }
    }
    #[doc = "Bits 4:5 - BGAP_ADC connection bit"]
    #[inline(always)]
    pub fn sel_vref_out(&mut self) -> SEL_VREF_OUT_W {
        SEL_VREF_OUT_W { w: self }
    }
    #[doc = "Bit 0 - Vref Enable bit"]
    #[inline(always)]
    pub fn en_bgap(&mut self) -> EN_BGAP_W {
        EN_BGAP_W { w: self }
    }
}
