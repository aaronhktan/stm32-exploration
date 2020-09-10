#[doc = "Reader of register SMPR1"]
pub type R = crate::R<u32, super::SMPR1>;
#[doc = "Writer for register SMPR1"]
pub type W = crate::W<u32, super::SMPR1>;
#[doc = "Register SMPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SMPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMPx_x`"]
pub type SMPX_X_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SMPx_x`"]
pub struct SMPX_X_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPX_X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Sample time bits"]
    #[inline(always)]
    pub fn smpx_x(&self) -> SMPX_X_R {
        SMPX_X_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Sample time bits"]
    #[inline(always)]
    pub fn smpx_x(&mut self) -> SMPX_X_W {
        SMPX_X_W { w: self }
    }
}
