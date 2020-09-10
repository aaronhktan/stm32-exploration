#[doc = "Reader of register DCKCFGR2"]
pub type R = crate::R<u32, super::DCKCFGR2>;
#[doc = "Writer for register DCKCFGR2"]
pub type W = crate::W<u32, super::DCKCFGR2>;
#[doc = "Register DCKCFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCKCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FMPI2C1SEL`"]
pub type FMPI2C1SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FMPI2C1SEL`"]
pub struct FMPI2C1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FMPI2C1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `CECSEL`"]
pub type CECSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CECSEL`"]
pub struct CECSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CK48MSEL`"]
pub type CK48MSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CK48MSEL`"]
pub struct CK48MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CK48MSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SDIOSEL`"]
pub type SDIOSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIOSEL`"]
pub struct SDIOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SPDIFSEL`"]
pub type SPDIFSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPDIFSEL`"]
pub struct SPDIFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:23 - I2C4 kernel clock source selection"]
    #[inline(always)]
    pub fn fmpi2c1sel(&self) -> FMPI2C1SEL_R {
        FMPI2C1SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 26 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SDIO/USBFS/HS clock selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SDIO clock selection"]
    #[inline(always)]
    pub fn sdiosel(&self) -> SDIOSEL_R {
        SDIOSEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SPDIF clock selection"]
    #[inline(always)]
    pub fn spdifsel(&self) -> SPDIFSEL_R {
        SPDIFSEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 22:23 - I2C4 kernel clock source selection"]
    #[inline(always)]
    pub fn fmpi2c1sel(&mut self) -> FMPI2C1SEL_W {
        FMPI2C1SEL_W { w: self }
    }
    #[doc = "Bit 26 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W {
        CECSEL_W { w: self }
    }
    #[doc = "Bit 27 - SDIO/USBFS/HS clock selection"]
    #[inline(always)]
    pub fn ck48msel(&mut self) -> CK48MSEL_W {
        CK48MSEL_W { w: self }
    }
    #[doc = "Bit 28 - SDIO clock selection"]
    #[inline(always)]
    pub fn sdiosel(&mut self) -> SDIOSEL_W {
        SDIOSEL_W { w: self }
    }
    #[doc = "Bit 29 - SPDIF clock selection"]
    #[inline(always)]
    pub fn spdifsel(&mut self) -> SPDIFSEL_W {
        SPDIFSEL_W { w: self }
    }
}
