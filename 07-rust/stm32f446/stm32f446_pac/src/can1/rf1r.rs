#[doc = "Reader of register RF1R"]
pub type R = crate::R<u32, super::RF1R>;
#[doc = "Writer for register RF1R"]
pub type W = crate::W<u32, super::RF1R>;
#[doc = "Register RF1R `reset()`'s with value 0"]
impl crate::ResetValue for super::RF1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFOM1`"]
pub type RFOM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFOM1`"]
pub struct RFOM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RFOM1_W<'a> {
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
#[doc = "Reader of field `FOVR1`"]
pub type FOVR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FOVR1`"]
pub struct FOVR1_W<'a> {
    w: &'a mut W,
}
impl<'a> FOVR1_W<'a> {
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
#[doc = "Reader of field `FULL1`"]
pub type FULL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FULL1`"]
pub struct FULL1_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL1_W<'a> {
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
#[doc = "Reader of field `FMP1`"]
pub type FMP1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 5 - RFOM1"]
    #[inline(always)]
    pub fn rfom1(&self) -> RFOM1_R {
        RFOM1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FOVR1"]
    #[inline(always)]
    pub fn fovr1(&self) -> FOVR1_R {
        FOVR1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FULL1"]
    #[inline(always)]
    pub fn full1(&self) -> FULL1_R {
        FULL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - FMP1"]
    #[inline(always)]
    pub fn fmp1(&self) -> FMP1_R {
        FMP1_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - RFOM1"]
    #[inline(always)]
    pub fn rfom1(&mut self) -> RFOM1_W {
        RFOM1_W { w: self }
    }
    #[doc = "Bit 4 - FOVR1"]
    #[inline(always)]
    pub fn fovr1(&mut self) -> FOVR1_W {
        FOVR1_W { w: self }
    }
    #[doc = "Bit 3 - FULL1"]
    #[inline(always)]
    pub fn full1(&mut self) -> FULL1_W {
        FULL1_W { w: self }
    }
}
