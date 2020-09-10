#[doc = "Reader of register COMP1_CTRL"]
pub type R = crate::R<u32, super::COMP1_CTRL>;
#[doc = "Writer for register COMP1_CTRL"]
pub type W = crate::W<u32, super::COMP1_CTRL>;
#[doc = "Register COMP1_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP1_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP1EN`"]
pub type COMP1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP1EN`"]
pub struct COMP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1EN_W<'a> {
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
#[doc = "Reader of field `COMP1INNSEL`"]
pub type COMP1INNSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP1INNSEL`"]
pub struct COMP1INNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1INNSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `COMP1WM`"]
pub type COMP1WM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP1WM`"]
pub struct COMP1WM_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1WM_W<'a> {
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
#[doc = "Reader of field `COMP1LPTIMIN1`"]
pub type COMP1LPTIMIN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP1LPTIMIN1`"]
pub struct COMP1LPTIMIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1LPTIMIN1_W<'a> {
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
#[doc = "Reader of field `COMP1POLARITY`"]
pub type COMP1POLARITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP1POLARITY`"]
pub struct COMP1POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1POLARITY_W<'a> {
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
#[doc = "Reader of field `COMP1VALUE`"]
pub type COMP1VALUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP1VALUE`"]
pub struct COMP1VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1VALUE_W<'a> {
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
#[doc = "Reader of field `COMP1LOCK`"]
pub type COMP1LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP1LOCK`"]
pub struct COMP1LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1LOCK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1innsel(&self) -> COMP1INNSEL_R {
        COMP1INNSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    pub fn comp1wm(&self) -> COMP1WM_R {
        COMP1WM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comparator 1 LPTIM input propagation bit"]
    #[inline(always)]
    pub fn comp1lptimin1(&self) -> COMP1LPTIMIN1_R {
        COMP1LPTIMIN1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1polarity(&self) -> COMP1POLARITY_R {
        COMP1POLARITY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn comp1value(&self) -> COMP1VALUE_R {
        COMP1VALUE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1en(&mut self) -> COMP1EN_W {
        COMP1EN_W { w: self }
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1innsel(&mut self) -> COMP1INNSEL_W {
        COMP1INNSEL_W { w: self }
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    pub fn comp1wm(&mut self) -> COMP1WM_W {
        COMP1WM_W { w: self }
    }
    #[doc = "Bit 12 - Comparator 1 LPTIM input propagation bit"]
    #[inline(always)]
    pub fn comp1lptimin1(&mut self) -> COMP1LPTIMIN1_W {
        COMP1LPTIMIN1_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1polarity(&mut self) -> COMP1POLARITY_W {
        COMP1POLARITY_W { w: self }
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn comp1value(&mut self) -> COMP1VALUE_W {
        COMP1VALUE_W { w: self }
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    pub fn comp1lock(&mut self) -> COMP1LOCK_W {
        COMP1LOCK_W { w: self }
    }
}
