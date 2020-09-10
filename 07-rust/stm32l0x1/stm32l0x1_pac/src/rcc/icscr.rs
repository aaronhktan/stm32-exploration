#[doc = "Reader of register ICSCR"]
pub type R = crate::R<u32, super::ICSCR>;
#[doc = "Writer for register ICSCR"]
pub type W = crate::W<u32, super::ICSCR>;
#[doc = "Register ICSCR `reset()`'s with value 0xb000"]
impl crate::ResetValue for super::ICSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xb000
    }
}
#[doc = "Reader of field `MSITRIM`"]
pub type MSITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSITRIM`"]
pub struct MSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `MSICAL`"]
pub type MSICAL_R = crate::R<u8, u8>;
#[doc = "Reader of field `MSIRANGE`"]
pub type MSIRANGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSIRANGE`"]
pub struct MSIRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `HSI16TRIM`"]
pub type HSI16TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSI16TRIM`"]
pub struct HSI16TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI16TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HSI16CAL`"]
pub type HSI16CAL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - MSI clock trimming"]
    #[inline(always)]
    pub fn msitrim(&self) -> MSITRIM_R {
        MSITRIM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MSI clock calibration"]
    #[inline(always)]
    pub fn msical(&self) -> MSICAL_R {
        MSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 13:15 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hsi16trim(&self) -> HSI16TRIM_R {
        HSI16TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:7 - nternal high speed clock calibration"]
    #[inline(always)]
    pub fn hsi16cal(&self) -> HSI16CAL_R {
        HSI16CAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - MSI clock trimming"]
    #[inline(always)]
    pub fn msitrim(&mut self) -> MSITRIM_W {
        MSITRIM_W { w: self }
    }
    #[doc = "Bits 13:15 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&mut self) -> MSIRANGE_W {
        MSIRANGE_W { w: self }
    }
    #[doc = "Bits 8:12 - High speed internal clock trimming"]
    #[inline(always)]
    pub fn hsi16trim(&mut self) -> HSI16TRIM_W {
        HSI16TRIM_W { w: self }
    }
}
