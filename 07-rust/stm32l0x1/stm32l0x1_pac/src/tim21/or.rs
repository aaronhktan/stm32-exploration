#[doc = "Reader of register OR"]
pub type R = crate::R<u32, super::OR>;
#[doc = "Writer for register OR"]
pub type W = crate::W<u32, super::OR>;
#[doc = "Register OR `reset()`'s with value 0"]
impl crate::ResetValue for super::OR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ETR_RMP`"]
pub type ETR_RMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETR_RMP`"]
pub struct ETR_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TI1_RMP`"]
pub type TI1_RMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TI1_RMP`"]
pub struct TI1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `TI2_RMP`"]
pub type TI2_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TI2_RMP`"]
pub struct TI2_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI2_RMP_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Timer21 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Timer21 TI1"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Timer21 TI2"]
    #[inline(always)]
    pub fn ti2_rmp(&self) -> TI2_RMP_R {
        TI2_RMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer21 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W {
        ETR_RMP_W { w: self }
    }
    #[doc = "Bits 2:4 - Timer21 TI1"]
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W {
        TI1_RMP_W { w: self }
    }
    #[doc = "Bit 5 - Timer21 TI2"]
    #[inline(always)]
    pub fn ti2_rmp(&mut self) -> TI2_RMP_W {
        TI2_RMP_W { w: self }
    }
}
