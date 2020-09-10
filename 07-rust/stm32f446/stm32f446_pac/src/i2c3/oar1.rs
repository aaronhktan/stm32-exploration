#[doc = "Reader of register OAR1"]
pub type R = crate::R<u32, super::OAR1>;
#[doc = "Writer for register OAR1"]
pub type W = crate::W<u32, super::OAR1>;
#[doc = "Register OAR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::OAR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDMODE`"]
pub type ADDMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDMODE`"]
pub struct ADDMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `ADD10`"]
pub type ADD10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADD10`"]
pub struct ADD10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADD7`"]
pub type ADD7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADD7`"]
pub struct ADD7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADD0`"]
pub type ADD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD0`"]
pub struct ADD0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD0_W<'a> {
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
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&self) -> ADDMODE_R {
        ADDMODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add7(&self) -> ADD7_R {
        ADD7_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn add0(&self) -> ADD0_R {
        ADD0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&mut self) -> ADDMODE_W {
        ADDMODE_W { w: self }
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W {
        ADD10_W { w: self }
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add7(&mut self) -> ADD7_W {
        ADD7_W { w: self }
    }
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn add0(&mut self) -> ADD0_W {
        ADD0_W { w: self }
    }
}
