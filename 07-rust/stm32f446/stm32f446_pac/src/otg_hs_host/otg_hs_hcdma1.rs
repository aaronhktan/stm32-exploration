#[doc = "Reader of register OTG_HS_HCDMA1"]
pub type R = crate::R<u32, super::OTG_HS_HCDMA1>;
#[doc = "Writer for register OTG_HS_HCDMA1"]
pub type W = crate::W<u32, super::OTG_HS_HCDMA1>;
#[doc = "Register OTG_HS_HCDMA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::OTG_HS_HCDMA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMAADDR`"]
pub type DMAADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMAADDR`"]
pub struct DMAADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W {
        DMAADDR_W { w: self }
    }
}
