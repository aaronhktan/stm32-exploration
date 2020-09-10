#[doc = "Reader of register CFR"]
pub type R = crate::R<u32, super::CFR>;
#[doc = "Writer for register CFR"]
pub type W = crate::W<u32, super::CFR>;
#[doc = "Register CFR `reset()`'s with value 0x7f"]
impl crate::ResetValue for super::CFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7f
    }
}
#[doc = "Reader of field `EWI`"]
pub type EWI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWI`"]
pub struct EWI_W<'a> {
    w: &'a mut W,
}
impl<'a> EWI_W<'a> {
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
#[doc = "Reader of field `WDGTB1`"]
pub type WDGTB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDGTB1`"]
pub struct WDGTB1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGTB1_W<'a> {
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
#[doc = "Reader of field `WDGTB0`"]
pub type WDGTB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDGTB0`"]
pub struct WDGTB0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGTB0_W<'a> {
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
#[doc = "Reader of field `W`"]
pub type W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `W`"]
pub struct W_W<'a> {
    w: &'a mut W,
}
impl<'a> W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Timer base"]
    #[inline(always)]
    pub fn wdgtb1(&self) -> WDGTB1_R {
        WDGTB1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer base"]
    #[inline(always)]
    pub fn wdgtb0(&self) -> WDGTB0_R {
        WDGTB0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W {
        EWI_W { w: self }
    }
    #[doc = "Bit 8 - Timer base"]
    #[inline(always)]
    pub fn wdgtb1(&mut self) -> WDGTB1_W {
        WDGTB1_W { w: self }
    }
    #[doc = "Bit 7 - Timer base"]
    #[inline(always)]
    pub fn wdgtb0(&mut self) -> WDGTB0_W {
        WDGTB0_W { w: self }
    }
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn w(&mut self) -> W_W {
        W_W { w: self }
    }
}
