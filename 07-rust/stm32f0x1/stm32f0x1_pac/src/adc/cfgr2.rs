#[doc = "Reader of register CFGR2"]
pub type R = crate::R<u32, super::CFGR2>;
#[doc = "Writer for register CFGR2"]
pub type W = crate::W<u32, super::CFGR2>;
#[doc = "Register CFGR2 `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::CFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Reader of field `JITOFF_D4`"]
pub type JITOFF_D4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JITOFF_D4`"]
pub struct JITOFF_D4_W<'a> {
    w: &'a mut W,
}
impl<'a> JITOFF_D4_W<'a> {
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
#[doc = "Reader of field `JITOFF_D2`"]
pub type JITOFF_D2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JITOFF_D2`"]
pub struct JITOFF_D2_W<'a> {
    w: &'a mut W,
}
impl<'a> JITOFF_D2_W<'a> {
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
impl R {
    #[doc = "Bit 31 - JITOFF_D4"]
    #[inline(always)]
    pub fn jitoff_d4(&self) -> JITOFF_D4_R {
        JITOFF_D4_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - JITOFF_D2"]
    #[inline(always)]
    pub fn jitoff_d2(&self) -> JITOFF_D2_R {
        JITOFF_D2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - JITOFF_D4"]
    #[inline(always)]
    pub fn jitoff_d4(&mut self) -> JITOFF_D4_W {
        JITOFF_D4_W { w: self }
    }
    #[doc = "Bit 30 - JITOFF_D2"]
    #[inline(always)]
    pub fn jitoff_d2(&mut self) -> JITOFF_D2_W {
        JITOFF_D2_W { w: self }
    }
}
