#[doc = "Reader of register OAR2"]
pub type R = crate::R<u32, super::OAR2>;
#[doc = "Writer for register OAR2"]
pub type W = crate::W<u32, super::OAR2>;
#[doc = "Register OAR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::OAR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD2`"]
pub type ADD2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADD2`"]
pub struct ADD2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `ENDUAL`"]
pub type ENDUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENDUAL`"]
pub struct ENDUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDUAL_W<'a> {
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
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add2(&self) -> ADD2_R {
        ADD2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    pub fn endual(&self) -> ENDUAL_R {
        ENDUAL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add2(&mut self) -> ADD2_W {
        ADD2_W { w: self }
    }
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    pub fn endual(&mut self) -> ENDUAL_W {
        ENDUAL_W { w: self }
    }
}
