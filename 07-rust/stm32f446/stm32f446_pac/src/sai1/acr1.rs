#[doc = "Reader of register ACR1"]
pub type R = crate::R<u32, super::ACR1>;
#[doc = "Writer for register ACR1"]
pub type W = crate::W<u32, super::ACR1>;
#[doc = "Register ACR1 `reset()`'s with value 0x40"]
impl crate::ResetValue for super::ACR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `MCJDIV`"]
pub type MCJDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCJDIV`"]
pub struct MCJDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCJDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `NODIV`"]
pub type NODIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NODIV`"]
pub struct NODIV_W<'a> {
    w: &'a mut W,
}
impl<'a> NODIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SAIAEN`"]
pub type SAIAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAIAEN`"]
pub struct SAIAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAIAEN_W<'a> {
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
#[doc = "Reader of field `OutDri`"]
pub type OUTDRI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OutDri`"]
pub struct OUTDRI_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDRI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `MONO`"]
pub type MONO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONO`"]
pub struct MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_W<'a> {
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
#[doc = "Reader of field `SYNCEN`"]
pub type SYNCEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNCEN`"]
pub struct SYNCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `CKSTR`"]
pub type CKSTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKSTR`"]
pub struct CKSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSTR_W<'a> {
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
#[doc = "Reader of field `LSBFIRST`"]
pub type LSBFIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSBFIRST`"]
pub struct LSBFIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBFIRST_W<'a> {
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
#[doc = "Reader of field `DS`"]
pub type DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DS`"]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `PRTCFG`"]
pub type PRTCFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRTCFG`"]
pub struct PRTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:23 - Master clock divider"]
    #[inline(always)]
    pub fn mcjdiv(&self) -> MCJDIV_R {
        MCJDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    pub fn nodiv(&self) -> NODIV_R {
        NODIV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Audio block A enable"]
    #[inline(always)]
    pub fn saiaen(&self) -> SAIAEN_R {
        SAIAEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Output drive"]
    #[inline(always)]
    pub fn out_dri(&self) -> OUTDRI_R {
        OUTDRI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Mono mode"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Synchronization enable"]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Clock strobing edge"]
    #[inline(always)]
    pub fn ckstr(&self) -> CKSTR_R {
        CKSTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Least significant bit first"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Data size"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 2:3 - Protocol configuration"]
    #[inline(always)]
    pub fn prtcfg(&self) -> PRTCFG_R {
        PRTCFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Audio block mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - Master clock divider"]
    #[inline(always)]
    pub fn mcjdiv(&mut self) -> MCJDIV_W {
        MCJDIV_W { w: self }
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    pub fn nodiv(&mut self) -> NODIV_W {
        NODIV_W { w: self }
    }
    #[doc = "Bit 17 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 16 - Audio block A enable"]
    #[inline(always)]
    pub fn saiaen(&mut self) -> SAIAEN_W {
        SAIAEN_W { w: self }
    }
    #[doc = "Bit 13 - Output drive"]
    #[inline(always)]
    pub fn out_dri(&mut self) -> OUTDRI_W {
        OUTDRI_W { w: self }
    }
    #[doc = "Bit 12 - Mono mode"]
    #[inline(always)]
    pub fn mono(&mut self) -> MONO_W {
        MONO_W { w: self }
    }
    #[doc = "Bits 10:11 - Synchronization enable"]
    #[inline(always)]
    pub fn syncen(&mut self) -> SYNCEN_W {
        SYNCEN_W { w: self }
    }
    #[doc = "Bit 9 - Clock strobing edge"]
    #[inline(always)]
    pub fn ckstr(&mut self) -> CKSTR_W {
        CKSTR_W { w: self }
    }
    #[doc = "Bit 8 - Least significant bit first"]
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W {
        LSBFIRST_W { w: self }
    }
    #[doc = "Bits 5:7 - Data size"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    #[doc = "Bits 2:3 - Protocol configuration"]
    #[inline(always)]
    pub fn prtcfg(&mut self) -> PRTCFG_W {
        PRTCFG_W { w: self }
    }
    #[doc = "Bits 0:1 - Audio block mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
