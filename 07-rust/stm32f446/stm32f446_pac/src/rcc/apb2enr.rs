#[doc = "Reader of register APB2ENR"]
pub type R = crate::R<u32, super::APB2ENR>;
#[doc = "Writer for register APB2ENR"]
pub type W = crate::W<u32, super::APB2ENR>;
#[doc = "Register APB2ENR `reset()`'s with value 0"]
impl crate::ResetValue for super::APB2ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIM1EN`"]
pub type TIM1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM1EN`"]
pub struct TIM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1EN_W<'a> {
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
#[doc = "Reader of field `TIM8EN`"]
pub type TIM8EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM8EN`"]
pub struct TIM8EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `USART1EN`"]
pub type USART1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1EN`"]
pub struct USART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `USART6EN`"]
pub type USART6EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART6EN`"]
pub struct USART6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ADC1EN`"]
pub type ADC1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC1EN`"]
pub struct ADC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1EN_W<'a> {
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
#[doc = "Reader of field `ADC2EN`"]
pub type ADC2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC2EN`"]
pub struct ADC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2EN_W<'a> {
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
#[doc = "Reader of field `ADC3EN`"]
pub type ADC3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC3EN`"]
pub struct ADC3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC3EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SDIOEN`"]
pub type SDIOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIOEN`"]
pub struct SDIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SPI1EN`"]
pub type SPI1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1EN`"]
pub struct SPI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1EN_W<'a> {
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
#[doc = "Reader of field `SPI4ENR`"]
pub type SPI4ENR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI4ENR`"]
pub struct SPI4ENR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4ENR_W<'a> {
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
#[doc = "Reader of field `SYSCFGEN`"]
pub type SYSCFGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCFGEN`"]
pub struct SYSCFGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TIM9EN`"]
pub type TIM9EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM9EN`"]
pub struct TIM9EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM9EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TIM10EN`"]
pub type TIM10EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM10EN`"]
pub struct TIM10EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM10EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TIM11EN`"]
pub type TIM11EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM11EN`"]
pub struct TIM11EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM11EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SAI1EN`"]
pub type SAI1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAI1EN`"]
pub struct SAI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `SAI2EN`"]
pub type SAI2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAI2EN`"]
pub struct SAI2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM8 clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4enr(&self) -> SPI4ENR_R {
        SPI4ENR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIM10 clock enable"]
    #[inline(always)]
    pub fn tim10en(&self) -> TIM10EN_R {
        TIM10EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline(always)]
    pub fn tim11en(&self) -> TIM11EN_R {
        TIM11EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SAI2 clock enable"]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W {
        TIM1EN_W { w: self }
    }
    #[doc = "Bit 1 - TIM8 clock enable"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W {
        TIM8EN_W { w: self }
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W { w: self }
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6en(&mut self) -> USART6EN_W {
        USART6EN_W { w: self }
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&mut self) -> ADC1EN_W {
        ADC1EN_W { w: self }
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&mut self) -> ADC2EN_W {
        ADC2EN_W { w: self }
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3en(&mut self) -> ADC3EN_W {
        ADC3EN_W { w: self }
    }
    #[doc = "Bit 11 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W {
        SDIOEN_W { w: self }
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W { w: self }
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4enr(&mut self) -> SPI4ENR_W {
        SPI4ENR_W { w: self }
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W {
        SYSCFGEN_W { w: self }
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline(always)]
    pub fn tim9en(&mut self) -> TIM9EN_W {
        TIM9EN_W { w: self }
    }
    #[doc = "Bit 17 - TIM10 clock enable"]
    #[inline(always)]
    pub fn tim10en(&mut self) -> TIM10EN_W {
        TIM10EN_W { w: self }
    }
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline(always)]
    pub fn tim11en(&mut self) -> TIM11EN_W {
        TIM11EN_W { w: self }
    }
    #[doc = "Bit 22 - SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&mut self) -> SAI1EN_W {
        SAI1EN_W { w: self }
    }
    #[doc = "Bit 23 - SAI2 clock enable"]
    #[inline(always)]
    pub fn sai2en(&mut self) -> SAI2EN_W {
        SAI2EN_W { w: self }
    }
}
