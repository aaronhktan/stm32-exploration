#[doc = "Reader of register AHB2RSTR"]
pub type R = crate::R<u32, super::AHB2RSTR>;
#[doc = "Writer for register AHB2RSTR"]
pub type W = crate::W<u32, super::AHB2RSTR>;
#[doc = "Register AHB2RSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB2RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OTGFSRST`"]
pub type OTGFSRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGFSRST`"]
pub struct OTGFSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGFSRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DCMIRST`"]
pub type DCMIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCMIRST`"]
pub struct DCMIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMIRST_W<'a> {
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
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Camera interface reset"]
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - USB OTG FS module reset"]
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W {
        OTGFSRST_W { w: self }
    }
    #[doc = "Bit 0 - Camera interface reset"]
    #[inline(always)]
    pub fn dcmirst(&mut self) -> DCMIRST_W {
        DCMIRST_W { w: self }
    }
}
