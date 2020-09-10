#[doc = "Reader of register HIFCR"]
pub type R = crate::R<u32, super::HIFCR>;
#[doc = "Writer for register HIFCR"]
pub type W = crate::W<u32, super::HIFCR>;
#[doc = "Register HIFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::HIFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTCIF7`"]
pub type CTCIF7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTCIF7`"]
pub struct CTCIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `CHTIF7`"]
pub type CHTIF7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHTIF7`"]
pub struct CHTIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CTEIF7`"]
pub type CTEIF7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTEIF7`"]
pub struct CTEIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `CDMEIF7`"]
pub type CDMEIF7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDMEIF7`"]
pub struct CDMEIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `CFEIF7`"]
pub type CFEIF7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFEIF7`"]
pub struct CFEIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CTCIF6`"]
pub type CTCIF6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTCIF6`"]
pub struct CTCIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `CHTIF6`"]
pub type CHTIF6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHTIF6`"]
pub struct CHTIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CTEIF6`"]
pub type CTEIF6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTEIF6`"]
pub struct CTEIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `CDMEIF6`"]
pub type CDMEIF6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDMEIF6`"]
pub struct CDMEIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CFEIF6`"]
pub type CFEIF6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFEIF6`"]
pub struct CFEIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTCIF5`"]
pub type CTCIF5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTCIF5`"]
pub struct CTCIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CHTIF5`"]
pub type CHTIF5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHTIF5`"]
pub struct CHTIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CTEIF5`"]
pub type CTEIF5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTEIF5`"]
pub struct CTEIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CDMEIF5`"]
pub type CDMEIF5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDMEIF5`"]
pub struct CDMEIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF5_W<'a> {
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
#[doc = "Reader of field `CFEIF5`"]
pub type CFEIF5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFEIF5`"]
pub struct CFEIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CTCIF4`"]
pub type CTCIF4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTCIF4`"]
pub struct CTCIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CHTIF4`"]
pub type CHTIF4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHTIF4`"]
pub struct CHTIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF4_W<'a> {
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
#[doc = "Reader of field `CTEIF4`"]
pub type CTEIF4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTEIF4`"]
pub struct CTEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF4_W<'a> {
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
#[doc = "Reader of field `CDMEIF4`"]
pub type CDMEIF4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDMEIF4`"]
pub struct CDMEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF4_W<'a> {
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
#[doc = "Reader of field `CFEIF4`"]
pub type CFEIF4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFEIF4`"]
pub struct CFEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF4_W<'a> {
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
impl R {
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif7(&self) -> CTCIF7_R {
        CTCIF7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif7(&self) -> CHTIF7_R {
        CHTIF7_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif7(&self) -> CTEIF7_R {
        CTEIF7_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif7(&self) -> CDMEIF7_R {
        CDMEIF7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif7(&self) -> CFEIF7_R {
        CFEIF7_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif6(&self) -> CTCIF6_R {
        CTCIF6_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif6(&self) -> CHTIF6_R {
        CHTIF6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif6(&self) -> CTEIF6_R {
        CTEIF6_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif6(&self) -> CDMEIF6_R {
        CDMEIF6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif6(&self) -> CFEIF6_R {
        CFEIF6_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif5(&self) -> CTCIF5_R {
        CTCIF5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif5(&self) -> CHTIF5_R {
        CHTIF5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif5(&self) -> CTEIF5_R {
        CTEIF5_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif5(&self) -> CDMEIF5_R {
        CDMEIF5_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif5(&self) -> CFEIF5_R {
        CFEIF5_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif4(&self) -> CTCIF4_R {
        CTCIF4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif4(&self) -> CHTIF4_R {
        CHTIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif4(&self) -> CTEIF4_R {
        CTEIF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif4(&self) -> CDMEIF4_R {
        CDMEIF4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif4(&self) -> CFEIF4_R {
        CFEIF4_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF7_W {
        CTCIF7_W { w: self }
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF7_W {
        CHTIF7_W { w: self }
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF7_W {
        CTEIF7_W { w: self }
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif7(&mut self) -> CDMEIF7_W {
        CDMEIF7_W { w: self }
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif7(&mut self) -> CFEIF7_W {
        CFEIF7_W { w: self }
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF6_W {
        CTCIF6_W { w: self }
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF6_W {
        CHTIF6_W { w: self }
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W {
        CTEIF6_W { w: self }
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif6(&mut self) -> CDMEIF6_W {
        CDMEIF6_W { w: self }
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif6(&mut self) -> CFEIF6_W {
        CFEIF6_W { w: self }
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W {
        CTCIF5_W { w: self }
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W {
        CHTIF5_W { w: self }
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W {
        CTEIF5_W { w: self }
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif5(&mut self) -> CDMEIF5_W {
        CDMEIF5_W { w: self }
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif5(&mut self) -> CFEIF5_W {
        CFEIF5_W { w: self }
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W {
        CTCIF4_W { w: self }
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W {
        CHTIF4_W { w: self }
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W {
        CTEIF4_W { w: self }
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif4(&mut self) -> CDMEIF4_W {
        CDMEIF4_W { w: self }
    }
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif4(&mut self) -> CFEIF4_W {
        CFEIF4_W { w: self }
    }
}
