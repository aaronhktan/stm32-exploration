#[doc = "Reader of register CCIPR"]
pub type R = crate::R<u32, super::CCIPR>;
#[doc = "Writer for register CCIPR"]
pub type W = crate::W<u32, super::CCIPR>;
#[doc = "Register CCIPR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCIPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPTIM1SEL1`"]
pub type LPTIM1SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM1SEL1`"]
pub struct LPTIM1SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `LPTIM1SEL0`"]
pub type LPTIM1SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM1SEL0`"]
pub struct LPTIM1SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL0_W<'a> {
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
#[doc = "Reader of field `I2C1SEL1`"]
pub type I2C1SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1SEL1`"]
pub struct I2C1SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SEL1_W<'a> {
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
#[doc = "Reader of field `I2C1SEL0`"]
pub type I2C1SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1SEL0`"]
pub struct I2C1SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SEL0_W<'a> {
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
#[doc = "Reader of field `LPUART1SEL1`"]
pub type LPUART1SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPUART1SEL1`"]
pub struct LPUART1SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SEL1_W<'a> {
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
#[doc = "Reader of field `LPUART1SEL0`"]
pub type LPUART1SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPUART1SEL0`"]
pub struct LPUART1SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SEL0_W<'a> {
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
#[doc = "Reader of field `USART2SEL1`"]
pub type USART2SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART2SEL1`"]
pub struct USART2SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `USART2SEL0`"]
pub type USART2SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART2SEL0`"]
pub struct USART2SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEL0_W<'a> {
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
#[doc = "Reader of field `USART1SEL1`"]
pub type USART1SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1SEL1`"]
pub struct USART1SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEL1_W<'a> {
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
#[doc = "Reader of field `USART1SEL0`"]
pub type USART1SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1SEL0`"]
pub struct USART1SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEL0_W<'a> {
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
#[doc = "Reader of field `I2C3SEL0`"]
pub type I2C3SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C3SEL0`"]
pub struct I2C3SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SEL0_W<'a> {
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
#[doc = "Reader of field `I2C3SEL1`"]
pub type I2C3SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C3SEL1`"]
pub struct I2C3SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SEL1_W<'a> {
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
impl R {
    #[doc = "Bit 19 - Low Power Timer clock source selection bits"]
    #[inline(always)]
    pub fn lptim1sel1(&self) -> LPTIM1SEL1_R {
        LPTIM1SEL1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - LPTIM1SEL0"]
    #[inline(always)]
    pub fn lptim1sel0(&self) -> LPTIM1SEL0_R {
        LPTIM1SEL0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 13 - I2C1 clock source selection bits"]
    #[inline(always)]
    pub fn i2c1sel1(&self) -> I2C1SEL1_R {
        I2C1SEL1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - I2C1SEL0"]
    #[inline(always)]
    pub fn i2c1sel0(&self) -> I2C1SEL0_R {
        I2C1SEL0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LPUART1 clock source selection bits"]
    #[inline(always)]
    pub fn lpuart1sel1(&self) -> LPUART1SEL1_R {
        LPUART1SEL1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPUART1SEL0"]
    #[inline(always)]
    pub fn lpuart1sel0(&self) -> LPUART1SEL0_R {
        LPUART1SEL0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USART2 clock source selection bits"]
    #[inline(always)]
    pub fn usart2sel1(&self) -> USART2SEL1_R {
        USART2SEL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USART2SEL0"]
    #[inline(always)]
    pub fn usart2sel0(&self) -> USART2SEL0_R {
        USART2SEL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - USART1 clock source selection bits"]
    #[inline(always)]
    pub fn usart1sel1(&self) -> USART1SEL1_R {
        USART1SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - USART1SEL0"]
    #[inline(always)]
    pub fn usart1sel0(&self) -> USART1SEL0_R {
        USART1SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C3 clock source selection bits"]
    #[inline(always)]
    pub fn i2c3sel0(&self) -> I2C3SEL0_R {
        I2C3SEL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - I2C3 clock source selection bits"]
    #[inline(always)]
    pub fn i2c3sel1(&self) -> I2C3SEL1_R {
        I2C3SEL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - Low Power Timer clock source selection bits"]
    #[inline(always)]
    pub fn lptim1sel1(&mut self) -> LPTIM1SEL1_W {
        LPTIM1SEL1_W { w: self }
    }
    #[doc = "Bit 18 - LPTIM1SEL0"]
    #[inline(always)]
    pub fn lptim1sel0(&mut self) -> LPTIM1SEL0_W {
        LPTIM1SEL0_W { w: self }
    }
    #[doc = "Bit 13 - I2C1 clock source selection bits"]
    #[inline(always)]
    pub fn i2c1sel1(&mut self) -> I2C1SEL1_W {
        I2C1SEL1_W { w: self }
    }
    #[doc = "Bit 12 - I2C1SEL0"]
    #[inline(always)]
    pub fn i2c1sel0(&mut self) -> I2C1SEL0_W {
        I2C1SEL0_W { w: self }
    }
    #[doc = "Bit 11 - LPUART1 clock source selection bits"]
    #[inline(always)]
    pub fn lpuart1sel1(&mut self) -> LPUART1SEL1_W {
        LPUART1SEL1_W { w: self }
    }
    #[doc = "Bit 10 - LPUART1SEL0"]
    #[inline(always)]
    pub fn lpuart1sel0(&mut self) -> LPUART1SEL0_W {
        LPUART1SEL0_W { w: self }
    }
    #[doc = "Bit 3 - USART2 clock source selection bits"]
    #[inline(always)]
    pub fn usart2sel1(&mut self) -> USART2SEL1_W {
        USART2SEL1_W { w: self }
    }
    #[doc = "Bit 2 - USART2SEL0"]
    #[inline(always)]
    pub fn usart2sel0(&mut self) -> USART2SEL0_W {
        USART2SEL0_W { w: self }
    }
    #[doc = "Bit 1 - USART1 clock source selection bits"]
    #[inline(always)]
    pub fn usart1sel1(&mut self) -> USART1SEL1_W {
        USART1SEL1_W { w: self }
    }
    #[doc = "Bit 0 - USART1SEL0"]
    #[inline(always)]
    pub fn usart1sel0(&mut self) -> USART1SEL0_W {
        USART1SEL0_W { w: self }
    }
    #[doc = "Bit 16 - I2C3 clock source selection bits"]
    #[inline(always)]
    pub fn i2c3sel0(&mut self) -> I2C3SEL0_W {
        I2C3SEL0_W { w: self }
    }
    #[doc = "Bit 17 - I2C3 clock source selection bits"]
    #[inline(always)]
    pub fn i2c3sel1(&mut self) -> I2C3SEL1_W {
        I2C3SEL1_W { w: self }
    }
}
