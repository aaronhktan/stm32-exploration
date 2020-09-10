#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `F_S`"]
pub type F_S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `F_S`"]
pub struct F_S_W<'a> {
    w: &'a mut W,
}
impl<'a> F_S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DUTY`"]
pub type DUTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY`"]
pub struct DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CCR`"]
pub type CCR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCR`"]
pub struct CCR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    pub fn f_s(&self) -> F_S_R {
        F_S_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    pub fn f_s(&mut self) -> F_S_W {
        F_S_W { w: self }
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W {
        DUTY_W { w: self }
    }
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn ccr(&mut self) -> CCR_W {
        CCR_W { w: self }
    }
}
