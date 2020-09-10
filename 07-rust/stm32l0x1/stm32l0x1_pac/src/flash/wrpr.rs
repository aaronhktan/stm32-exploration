#[doc = "Reader of register WRPR"]
pub type R = crate::R<u32, super::WRPR>;
#[doc = "Writer for register WRPR"]
pub type W = crate::W<u32, super::WRPR>;
#[doc = "Register WRPR `reset()`'s with value 0"]
impl crate::ResetValue for super::WRPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRP`"]
pub type WRP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WRP`"]
pub struct WRP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Write protection"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write protection"]
    #[inline(always)]
    pub fn wrp(&mut self) -> WRP_W {
        WRP_W { w: self }
    }
}
