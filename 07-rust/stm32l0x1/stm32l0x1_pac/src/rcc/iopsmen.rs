#[doc = "Reader of register IOPSMEN"]
pub type R = crate::R<u32, super::IOPSMEN>;
#[doc = "Writer for register IOPSMEN"]
pub type W = crate::W<u32, super::IOPSMEN>;
#[doc = "Register IOPSMEN `reset()`'s with value 0x8f"]
impl crate::ResetValue for super::IOPSMEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8f
    }
}
#[doc = "Reader of field `IOPHSMEN`"]
pub type IOPHSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOPHSMEN`"]
pub struct IOPHSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPHSMEN_W<'a> {
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
#[doc = "Reader of field `IOPDSMEN`"]
pub type IOPDSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOPDSMEN`"]
pub struct IOPDSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPDSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `IOPCSMEN`"]
pub type IOPCSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOPCSMEN`"]
pub struct IOPCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPCSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `IOPBSMEN`"]
pub type IOPBSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOPBSMEN`"]
pub struct IOPBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPBSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `IOPASMEN`"]
pub type IOPASMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOPASMEN`"]
pub struct IOPASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPASMEN_W<'a> {
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
#[doc = "Reader of field `IOPESMEN`"]
pub type IOPESMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOPESMEN`"]
pub struct IOPESMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOPESMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Port H clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iophsmen(&self) -> IOPHSMEN_R {
        IOPHSMEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port D clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopdsmen(&self) -> IOPDSMEN_R {
        IOPDSMEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port C clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopcsmen(&self) -> IOPCSMEN_R {
        IOPCSMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port B clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopbsmen(&self) -> IOPBSMEN_R {
        IOPBSMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port A clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopasmen(&self) -> IOPASMEN_R {
        IOPASMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port E clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopesmen(&self) -> IOPESMEN_R {
        IOPESMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Port H clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iophsmen(&mut self) -> IOPHSMEN_W {
        IOPHSMEN_W { w: self }
    }
    #[doc = "Bit 3 - Port D clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopdsmen(&mut self) -> IOPDSMEN_W {
        IOPDSMEN_W { w: self }
    }
    #[doc = "Bit 2 - Port C clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopcsmen(&mut self) -> IOPCSMEN_W {
        IOPCSMEN_W { w: self }
    }
    #[doc = "Bit 1 - Port B clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopbsmen(&mut self) -> IOPBSMEN_W {
        IOPBSMEN_W { w: self }
    }
    #[doc = "Bit 0 - Port A clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopasmen(&mut self) -> IOPASMEN_W {
        IOPASMEN_W { w: self }
    }
    #[doc = "Bit 4 - Port E clock enable during Sleep mode bit"]
    #[inline(always)]
    pub fn iopesmen(&mut self) -> IOPESMEN_W {
        IOPESMEN_W { w: self }
    }
}
