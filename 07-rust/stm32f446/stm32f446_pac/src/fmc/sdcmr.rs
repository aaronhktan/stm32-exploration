#[doc = "Reader of register SDCMR"]
pub type R = crate::R<u32, super::SDCMR>;
#[doc = "Writer for register SDCMR"]
pub type W = crate::W<u32, super::SDCMR>;
#[doc = "Register SDCMR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Write proxy for field `CTB2`"]
pub struct CTB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTB2_W<'a> {
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
#[doc = "Write proxy for field `CTB1`"]
pub struct CTB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTB1_W<'a> {
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
#[doc = "Reader of field `NRFS`"]
pub type NRFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NRFS`"]
pub struct NRFS_W<'a> {
    w: &'a mut W,
}
impl<'a> NRFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Reader of field `MRD`"]
pub type MRD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MRD`"]
pub struct MRD_W<'a> {
    w: &'a mut W,
}
impl<'a> MRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 9)) | (((value as u32) & 0x1fff) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:8 - Number of Auto-refresh"]
    #[inline(always)]
    pub fn nrfs(&self) -> NRFS_R {
        NRFS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:21 - Mode Register definition"]
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 9) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Command mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - Command target bank 2"]
    #[inline(always)]
    pub fn ctb2(&mut self) -> CTB2_W {
        CTB2_W { w: self }
    }
    #[doc = "Bit 4 - Command target bank 1"]
    #[inline(always)]
    pub fn ctb1(&mut self) -> CTB1_W {
        CTB1_W { w: self }
    }
    #[doc = "Bits 5:8 - Number of Auto-refresh"]
    #[inline(always)]
    pub fn nrfs(&mut self) -> NRFS_W {
        NRFS_W { w: self }
    }
    #[doc = "Bits 9:21 - Mode Register definition"]
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W {
        MRD_W { w: self }
    }
}
