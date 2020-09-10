#[doc = "Reader of register LIFCR"]
pub type R = crate::R<u32, super::LIFCR>;
#[doc = "Writer for register LIFCR"]
pub type W = crate::W<u32, super::LIFCR>;
#[doc = "Register LIFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LIFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTCIF3`"]
pub type CTCIF3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTCIF3`"]
pub struct CTCIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF3_W<'a> {
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
#[doc = "Reader of field `CHTIF3`"]
pub type CHTIF3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHTIF3`"]
pub struct CHTIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF3_W<'a> {
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
#[doc = "Reader of field `CTEIF3`"]
pub type CTEIF3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTEIF3`"]
pub struct CTEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF3_W<'a> {
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
#[doc = "Reader of field `CDMEIF3`"]
pub type CDMEIF3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDMEIF3`"]
pub struct CDMEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF3_W<'a> {
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
#[doc = "Reader of field `CFEIF3`"]
pub type CFEIF3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFEIF3`"]
pub struct CFEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF3_W<'a> {
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
#[doc = "Reader of field `CTCIF2`"]
pub type CTCIF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTCIF2`"]
pub struct CTCIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF2_W<'a> {
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
#[doc = "Reader of field `CHTIF2`"]
pub type CHTIF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHTIF2`"]
pub struct CHTIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF2_W<'a> {
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
#[doc = "Reader of field `CTEIF2`"]
pub type CTEIF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTEIF2`"]
pub struct CTEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF2_W<'a> {
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
#[doc = "Reader of field `CDMEIF2`"]
pub type CDMEIF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDMEIF2`"]
pub struct CDMEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF2_W<'a> {
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
#[doc = "Reader of field `CFEIF2`"]
pub type CFEIF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFEIF2`"]
pub struct CFEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF2_W<'a> {
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
#[doc = "Reader of field `CTCIF1`"]
pub type CTCIF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTCIF1`"]
pub struct CTCIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF1_W<'a> {
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
#[doc = "Reader of field `CHTIF1`"]
pub type CHTIF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHTIF1`"]
pub struct CHTIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF1_W<'a> {
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
#[doc = "Reader of field `CTEIF1`"]
pub type CTEIF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTEIF1`"]
pub struct CTEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF1_W<'a> {
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
#[doc = "Reader of field `CDMEIF1`"]
pub type CDMEIF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDMEIF1`"]
pub struct CDMEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF1_W<'a> {
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
#[doc = "Reader of field `CFEIF1`"]
pub type CFEIF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFEIF1`"]
pub struct CFEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF1_W<'a> {
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
#[doc = "Reader of field `CTCIF0`"]
pub type CTCIF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTCIF0`"]
pub struct CTCIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF0_W<'a> {
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
#[doc = "Reader of field `CHTIF0`"]
pub type CHTIF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHTIF0`"]
pub struct CHTIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF0_W<'a> {
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
#[doc = "Reader of field `CTEIF0`"]
pub type CTEIF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTEIF0`"]
pub struct CTEIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF0_W<'a> {
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
#[doc = "Reader of field `CDMEIF0`"]
pub type CDMEIF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDMEIF0`"]
pub struct CDMEIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF0_W<'a> {
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
#[doc = "Reader of field `CFEIF0`"]
pub type CFEIF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFEIF0`"]
pub struct CFEIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF0_W<'a> {
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
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif3(&self) -> CTCIF3_R {
        CTCIF3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif3(&self) -> CHTIF3_R {
        CHTIF3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif3(&self) -> CTEIF3_R {
        CTEIF3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif3(&self) -> CDMEIF3_R {
        CDMEIF3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif3(&self) -> CFEIF3_R {
        CFEIF3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif2(&self) -> CTCIF2_R {
        CTCIF2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif2(&self) -> CHTIF2_R {
        CHTIF2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif2(&self) -> CTEIF2_R {
        CTEIF2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif2(&self) -> CDMEIF2_R {
        CDMEIF2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif2(&self) -> CFEIF2_R {
        CFEIF2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif1(&self) -> CTCIF1_R {
        CTCIF1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif1(&self) -> CHTIF1_R {
        CHTIF1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif1(&self) -> CTEIF1_R {
        CTEIF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif1(&self) -> CDMEIF1_R {
        CDMEIF1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif1(&self) -> CFEIF1_R {
        CFEIF1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif0(&self) -> CTCIF0_R {
        CTCIF0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif0(&self) -> CHTIF0_R {
        CHTIF0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif0(&self) -> CTEIF0_R {
        CTEIF0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif0(&self) -> CDMEIF0_R {
        CDMEIF0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif0(&self) -> CFEIF0_R {
        CFEIF0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W {
        CTCIF3_W { w: self }
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W {
        CHTIF3_W { w: self }
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W {
        CTEIF3_W { w: self }
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif3(&mut self) -> CDMEIF3_W {
        CDMEIF3_W { w: self }
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif3(&mut self) -> CFEIF3_W {
        CFEIF3_W { w: self }
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W {
        CTCIF2_W { w: self }
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W {
        CHTIF2_W { w: self }
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W {
        CTEIF2_W { w: self }
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif2(&mut self) -> CDMEIF2_W {
        CDMEIF2_W { w: self }
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif2(&mut self) -> CFEIF2_W {
        CFEIF2_W { w: self }
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W {
        CTCIF1_W { w: self }
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W {
        CHTIF1_W { w: self }
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W {
        CTEIF1_W { w: self }
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif1(&mut self) -> CDMEIF1_W {
        CDMEIF1_W { w: self }
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif1(&mut self) -> CFEIF1_W {
        CFEIF1_W { w: self }
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif0(&mut self) -> CTCIF0_W {
        CTCIF0_W { w: self }
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif0(&mut self) -> CHTIF0_W {
        CHTIF0_W { w: self }
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif0(&mut self) -> CTEIF0_W {
        CTEIF0_W { w: self }
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif0(&mut self) -> CDMEIF0_W {
        CDMEIF0_W { w: self }
    }
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif0(&mut self) -> CFEIF0_W {
        CFEIF0_W { w: self }
    }
}
