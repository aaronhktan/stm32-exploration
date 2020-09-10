#[doc = "Reader of register SMPR"]
pub type R = crate::R<u32, super::SMPR>;
#[doc = "Writer for register SMPR"]
pub type W = crate::W<u32, super::SMPR>;
#[doc = "Register SMPR `reset()`'s with value 0"]
impl crate::ResetValue for super::SMPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMPR`"]
pub type SMPR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMPR`"]
pub struct SMPR_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smpr(&self) -> SMPR_R {
        SMPR_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smpr(&mut self) -> SMPR_W {
        SMPR_W { w: self }
    }
}
