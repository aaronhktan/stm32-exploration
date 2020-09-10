#[doc = "Reader of register AHBSMENR"]
pub type R = crate::R<u32, super::AHBSMENR>;
#[doc = "Writer for register AHBSMENR"]
pub type W = crate::W<u32, super::AHBSMENR>;
#[doc = "Register AHBSMENR `reset()`'s with value 0x0111_1301"]
impl crate::ResetValue for super::AHBSMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0111_1301
    }
}
#[doc = "Reader of field `CRYPTSMEN`"]
pub type CRYPTSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYPTSMEN`"]
pub struct CRYPTSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTSMEN_W<'a> {
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
#[doc = "Reader of field `CRCSMEN`"]
pub type CRCSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCSMEN`"]
pub struct CRCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SRAMSMEN`"]
pub type SRAMSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAMSMEN`"]
pub struct SRAMSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMSMEN_W<'a> {
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
#[doc = "Reader of field `MIFSMEN`"]
pub type MIFSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIFSMEN`"]
pub struct MIFSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIFSMEN_W<'a> {
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
#[doc = "Reader of field `DMASMEN`"]
pub type DMASMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASMEN`"]
pub struct DMASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASMEN_W<'a> {
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
    #[doc = "Bit 24 - Crypto clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn cryptsmen(&self) -> CRYPTSMEN_R {
        CRYPTSMEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRAM interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - NVM interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn mifsmen(&self) -> MIFSMEN_R {
        MIFSMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn dmasmen(&self) -> DMASMEN_R {
        DMASMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Crypto clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn cryptsmen(&mut self) -> CRYPTSMEN_W {
        CRYPTSMEN_W { w: self }
    }
    #[doc = "Bit 12 - CRC clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W {
        CRCSMEN_W { w: self }
    }
    #[doc = "Bit 9 - SRAM interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W {
        SRAMSMEN_W { w: self }
    }
    #[doc = "Bit 8 - NVM interface clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn mifsmen(&mut self) -> MIFSMEN_W {
        MIFSMEN_W { w: self }
    }
    #[doc = "Bit 0 - DMA clock enable during sleep mode bit"]
    #[inline(always)]
    pub fn dmasmen(&mut self) -> DMASMEN_W {
        DMASMEN_W { w: self }
    }
}
