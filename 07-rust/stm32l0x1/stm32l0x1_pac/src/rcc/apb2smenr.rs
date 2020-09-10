#[doc = "Reader of register APB2SMENR"]
pub type R = crate::R<u32, super::APB2SMENR>;
#[doc = "Writer for register APB2SMENR"]
pub type W = crate::W<u32, super::APB2SMENR>;
#[doc = "Register APB2SMENR `reset()`'s with value 0x0040_5225"]
impl crate::ResetValue for super::APB2SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0040_5225
    }
}
#[doc = "Reader of field `DBGSMEN`"]
pub type DBGSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGSMEN`"]
pub struct DBGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSMEN_W<'a> {
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
#[doc = "Reader of field `USART1SMEN`"]
pub type USART1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1SMEN`"]
pub struct USART1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SMEN_W<'a> {
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
#[doc = "Reader of field `SPI1SMEN`"]
pub type SPI1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1SMEN`"]
pub struct SPI1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1SMEN_W<'a> {
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
#[doc = "Reader of field `ADCSMEN`"]
pub type ADCSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCSMEN`"]
pub struct ADCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSMEN_W<'a> {
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
#[doc = "Reader of field `TIM22SMEN`"]
pub type TIM22SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM22SMEN`"]
pub struct TIM22SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM22SMEN_W<'a> {
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
#[doc = "Reader of field `TIM21SMEN`"]
pub type TIM21SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM21SMEN`"]
pub struct TIM21SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM21SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SYSCFGSMEN`"]
pub type SYSCFGSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCFGSMEN`"]
pub struct SYSCFGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGSMEN_W<'a> {
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
    #[doc = "Bit 22 - DBG clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM22 timer clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim22smen(&self) -> TIM22SMEN_R {
        TIM22SMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM21 timer clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim21smen(&self) -> TIM21SMEN_R {
        TIM21SMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - System configuration controller clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - DBG clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W {
        DBGSMEN_W { w: self }
    }
    #[doc = "Bit 14 - USART1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W {
        USART1SMEN_W { w: self }
    }
    #[doc = "Bit 12 - SPI1 clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W {
        SPI1SMEN_W { w: self }
    }
    #[doc = "Bit 9 - ADC clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn adcsmen(&mut self) -> ADCSMEN_W {
        ADCSMEN_W { w: self }
    }
    #[doc = "Bit 5 - TIM22 timer clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim22smen(&mut self) -> TIM22SMEN_W {
        TIM22SMEN_W { w: self }
    }
    #[doc = "Bit 2 - TIM21 timer clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn tim21smen(&mut self) -> TIM21SMEN_W {
        TIM21SMEN_W { w: self }
    }
    #[doc = "Bit 0 - System configuration controller clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W {
        SYSCFGSMEN_W { w: self }
    }
}
