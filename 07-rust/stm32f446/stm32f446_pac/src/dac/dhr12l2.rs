#[doc = "Reader of register DHR12L2"]
pub type R = crate::R<u32, super::DHR12L2>;
#[doc = "Writer for register DHR12L2"]
pub type W = crate::W<u32, super::DHR12L2>;
#[doc = "Register DHR12L2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DHR12L2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DACC2DHR`"]
pub type DACC2DHR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DACC2DHR`"]
pub struct DACC2DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC2DHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:15 - DAC channel2 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC channel2 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W {
        DACC2DHR_W { w: self }
    }
}
