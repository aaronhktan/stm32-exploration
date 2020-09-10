#[doc = "Reader of register DCKCFGR"]
pub type R = crate::R<u32, super::DCKCFGR>;
#[doc = "Writer for register DCKCFGR"]
pub type W = crate::W<u32, super::DCKCFGR>;
#[doc = "Register DCKCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCKCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLLI2SDIVQ`"]
pub type PLLI2SDIVQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLI2SDIVQ`"]
pub struct PLLI2SDIVQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SDIVQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `PLLSAIDIVQ`"]
pub type PLLSAIDIVQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAIDIVQ`"]
pub struct PLLSAIDIVQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIDIVQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SAI1SRC`"]
pub type SAI1SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAI1SRC`"]
pub struct SAI1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `SAI2SRC`"]
pub type SAI2SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAI2SRC`"]
pub struct SAI2SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `TIMPRE`"]
pub type TIMPRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMPRE`"]
pub struct TIMPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `I2S1SRC`"]
pub type I2S1SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S1SRC`"]
pub struct I2S1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `I2S2SRC`"]
pub type I2S2SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S2SRC`"]
pub struct I2S2SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S2SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PLLI2S division factor for SAIs clock"]
    #[inline(always)]
    pub fn plli2sdivq(&self) -> PLLI2SDIVQ_R {
        PLLI2SDIVQ_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAIs clock"]
    #[inline(always)]
    pub fn pllsaidivq(&self) -> PLLSAIDIVQ_R {
        PLLSAIDIVQ_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1src(&self) -> SAI1SRC_R {
        SAI1SRC_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2src(&self) -> SAI2SRC_R {
        SAI2SRC_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - I2S APB1 clock source selection"]
    #[inline(always)]
    pub fn i2s1src(&self) -> I2S1SRC_R {
        I2S1SRC_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 27:28 - I2S APB2 clock source selection"]
    #[inline(always)]
    pub fn i2s2src(&self) -> I2S2SRC_R {
        I2S2SRC_R::new(((self.bits >> 27) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLLI2S division factor for SAIs clock"]
    #[inline(always)]
    pub fn plli2sdivq(&mut self) -> PLLI2SDIVQ_W {
        PLLI2SDIVQ_W { w: self }
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAIs clock"]
    #[inline(always)]
    pub fn pllsaidivq(&mut self) -> PLLSAIDIVQ_W {
        PLLSAIDIVQ_W { w: self }
    }
    #[doc = "Bits 20:21 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1src(&mut self) -> SAI1SRC_W {
        SAI1SRC_W { w: self }
    }
    #[doc = "Bits 22:23 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2src(&mut self) -> SAI2SRC_W {
        SAI2SRC_W { w: self }
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W {
        TIMPRE_W { w: self }
    }
    #[doc = "Bits 25:26 - I2S APB1 clock source selection"]
    #[inline(always)]
    pub fn i2s1src(&mut self) -> I2S1SRC_W {
        I2S1SRC_W { w: self }
    }
    #[doc = "Bits 27:28 - I2S APB2 clock source selection"]
    #[inline(always)]
    pub fn i2s2src(&mut self) -> I2S2SRC_W {
        I2S2SRC_W { w: self }
    }
}
