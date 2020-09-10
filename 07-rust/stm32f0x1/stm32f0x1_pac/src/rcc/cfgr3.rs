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
#[doc = "Reader of field `USART1SW`"]
pub type USART1SW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USART1SW`"]
pub struct USART1SW_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `I2C1SW`"]
pub type I2C1SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1SW`"]
pub struct I2C1SW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SW_W<'a> {
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
#[doc = "Reader of field `CECSW`"]
pub type CECSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CECSW`"]
pub struct CECSW_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `USBSW`"]
pub type USBSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBSW`"]
pub struct USBSW_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ADCSW`"]
pub type ADCSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCSW`"]
pub struct ADCSW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSW_W<'a> {
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
#[doc = "Reader of field `USART2SW`"]
pub type USART2SW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USART2SW`"]
pub struct USART2SW_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sw(&self) -> USART1SW_R {
        USART1SW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sw(&self) -> I2C1SW_R {
        I2C1SW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsw(&self) -> CECSW_R {
        CECSW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB clock source selection"]
    #[inline(always)]
    pub fn usbsw(&self) -> USBSW_R {
        USBSW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC clock source selection"]
    #[inline(always)]
    pub fn adcsw(&self) -> ADCSW_R {
        ADCSW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sw(&self) -> USART2SW_R {
        USART2SW_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sw(&mut self) -> USART1SW_W {
        USART1SW_W { w: self }
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sw(&mut self) -> I2C1SW_W {
        I2C1SW_W { w: self }
    }
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsw(&mut self) -> CECSW_W {
        CECSW_W { w: self }
    }
    #[doc = "Bit 7 - USB clock source selection"]
    #[inline(always)]
    pub fn usbsw(&mut self) -> USBSW_W {
        USBSW_W { w: self }
    }
    #[doc = "Bit 8 - ADC clock source selection"]
    #[inline(always)]
    pub fn adcsw(&mut self) -> ADCSW_W {
        ADCSW_W { w: self }
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sw(&mut self) -> USART2SW_W {
        USART2SW_W { w: self }
    }
}
