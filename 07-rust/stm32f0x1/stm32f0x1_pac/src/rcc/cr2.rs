#[doc = "Reader of register CR2"]
pub type R = crate::R<u32, super::CR2>;
#[doc = "Writer for register CR2"]
pub type W = crate::W<u32, super::CR2>;
#[doc = "Register CR2 `reset()`'s with value 0x80"]
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `HSI14ON`"]
pub type HSI14ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSI14ON`"]
pub struct HSI14ON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI14ON_W<'a> {
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
#[doc = "Reader of field `HSI14RDY`"]
pub type HSI14RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSI14DIS`"]
pub type HSI14DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSI14DIS`"]
pub struct HSI14DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI14DIS_W<'a> {
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
#[doc = "Reader of field `HSI14TRIM`"]
pub type HSI14TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSI14TRIM`"]
pub struct HSI14TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI14TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `HSI14CAL`"]
pub type HSI14CAL_R = crate::R<u8, u8>;
#[doc = "Reader of field `HSI48ON`"]
pub type HSI48ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSI48ON`"]
pub struct HSI48ON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48ON_W<'a> {
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
#[doc = "Reader of field `HSI48RDY`"]
pub type HSI48RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSI48CAL`"]
pub type HSI48CAL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - HSI14 clock enable"]
    #[inline(always)]
    pub fn hsi14on(&self) -> HSI14ON_R {
        HSI14ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HR14 clock ready flag"]
    #[inline(always)]
    pub fn hsi14rdy(&self) -> HSI14RDY_R {
        HSI14RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI14 clock request from ADC disable"]
    #[inline(always)]
    pub fn hsi14dis(&self) -> HSI14DIS_R {
        HSI14DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - HSI14 clock trimming"]
    #[inline(always)]
    pub fn hsi14trim(&self) -> HSI14TRIM_R {
        HSI14TRIM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - HSI14 clock calibration"]
    #[inline(always)]
    pub fn hsi14cal(&self) -> HSI14CAL_R {
        HSI14CAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - HSI48 clock enable"]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HSI48 clock ready flag"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - HSI48 factory clock calibration"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSI14 clock enable"]
    #[inline(always)]
    pub fn hsi14on(&mut self) -> HSI14ON_W {
        HSI14ON_W { w: self }
    }
    #[doc = "Bit 2 - HSI14 clock request from ADC disable"]
    #[inline(always)]
    pub fn hsi14dis(&mut self) -> HSI14DIS_W {
        HSI14DIS_W { w: self }
    }
    #[doc = "Bits 3:7 - HSI14 clock trimming"]
    #[inline(always)]
    pub fn hsi14trim(&mut self) -> HSI14TRIM_W {
        HSI14TRIM_W { w: self }
    }
    #[doc = "Bit 16 - HSI48 clock enable"]
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W {
        HSI48ON_W { w: self }
    }
}
