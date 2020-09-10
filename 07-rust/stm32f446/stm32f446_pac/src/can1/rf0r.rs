#[doc = "Reader of register RF0R"]
pub type R = crate::R<u32, super::RF0R>;
#[doc = "Writer for register RF0R"]
pub type W = crate::W<u32, super::RF0R>;
#[doc = "Register RF0R `reset()`'s with value 0"]
impl crate::ResetValue for super::RF0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFOM0`"]
pub type RFOM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFOM0`"]
pub struct RFOM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFOM0_W<'a> {
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
#[doc = "Reader of field `FOVR0`"]
pub type FOVR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FOVR0`"]
pub struct FOVR0_W<'a> {
    w: &'a mut W,
}
impl<'a> FOVR0_W<'a> {
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
#[doc = "Reader of field `FULL0`"]
pub type FULL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FULL0`"]
pub struct FULL0_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL0_W<'a> {
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
#[doc = "Reader of field `FMP0`"]
pub type FMP0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom0(&self) -> RFOM0_R {
        RFOM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr0(&self) -> FOVR0_R {
        FOVR0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full0(&self) -> FULL0_R {
        FULL0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - FMP0"]
    #[inline(always)]
    pub fn fmp0(&self) -> FMP0_R {
        FMP0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom0(&mut self) -> RFOM0_W {
        RFOM0_W { w: self }
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr0(&mut self) -> FOVR0_W {
        FOVR0_W { w: self }
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full0(&mut self) -> FULL0_W {
        FULL0_W { w: self }
    }
}
