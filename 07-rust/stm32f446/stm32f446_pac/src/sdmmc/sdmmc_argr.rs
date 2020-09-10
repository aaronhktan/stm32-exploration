#[doc = "Reader of register SDMMC_ARGR"]
pub type R = crate::R<u32, super::SDMMC_ARGR>;
#[doc = "Writer for register SDMMC_ARGR"]
pub type W = crate::W<u32, super::SDMMC_ARGR>;
#[doc = "Register SDMMC_ARGR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMC_ARGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMDARG`"]
pub type CMDARG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CMDARG`"]
pub struct CMDARG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
    #[inline(always)]
    pub fn cmdarg(&self) -> CMDARG_R {
        CMDARG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
    #[inline(always)]
    pub fn cmdarg(&mut self) -> CMDARG_W {
        CMDARG_W { w: self }
    }
}
