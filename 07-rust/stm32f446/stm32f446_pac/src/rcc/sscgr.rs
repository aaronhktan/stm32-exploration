#[doc = "Reader of register SSCGR"]
pub type R = crate::R<u32, super::SSCGR>;
#[doc = "Writer for register SSCGR"]
pub type W = crate::W<u32, super::SSCGR>;
#[doc = "Register SSCGR `reset()`'s with value 0"]
impl crate::ResetValue for super::SSCGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSCGEN`"]
pub type SSCGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSCGEN`"]
pub struct SSCGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSCGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SPREADSEL`"]
pub type SPREADSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPREADSEL`"]
pub struct SPREADSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPREADSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `INCSTEP`"]
pub type INCSTEP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INCSTEP`"]
pub struct INCSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INCSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 13)) | (((value as u32) & 0x7fff) << 13);
        self.w
    }
}
#[doc = "Reader of field `MODPER`"]
pub type MODPER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MODPER`"]
pub struct MODPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MODPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgen(&self) -> SSCGEN_R {
        SSCGEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    pub fn spreadsel(&self) -> SPREADSEL_R {
        SPREADSEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    pub fn incstep(&self) -> INCSTEP_R {
        INCSTEP_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    pub fn modper(&self) -> MODPER_R {
        MODPER_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgen(&mut self) -> SSCGEN_W {
        SSCGEN_W { w: self }
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    pub fn spreadsel(&mut self) -> SPREADSEL_W {
        SPREADSEL_W { w: self }
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    pub fn incstep(&mut self) -> INCSTEP_W {
        INCSTEP_W { w: self }
    }
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    pub fn modper(&mut self) -> MODPER_W {
        MODPER_W { w: self }
    }
}
