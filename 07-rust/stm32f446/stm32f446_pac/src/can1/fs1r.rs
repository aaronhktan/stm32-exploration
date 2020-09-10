#[doc = "Reader of register FS1R"]
pub type R = crate::R<u32, super::FS1R>;
#[doc = "Writer for register FS1R"]
pub type W = crate::W<u32, super::FS1R>;
#[doc = "Register FS1R `reset()`'s with value 0"]
impl crate::ResetValue for super::FS1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSC0`"]
pub type FSC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC0`"]
pub struct FSC0_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC0_W<'a> {
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
#[doc = "Reader of field `FSC1`"]
pub type FSC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC1`"]
pub struct FSC1_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC1_W<'a> {
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
#[doc = "Reader of field `FSC2`"]
pub type FSC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC2`"]
pub struct FSC2_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC2_W<'a> {
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
#[doc = "Reader of field `FSC3`"]
pub type FSC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC3`"]
pub struct FSC3_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC3_W<'a> {
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
#[doc = "Reader of field `FSC4`"]
pub type FSC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC4`"]
pub struct FSC4_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC4_W<'a> {
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
#[doc = "Reader of field `FSC5`"]
pub type FSC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC5`"]
pub struct FSC5_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC5_W<'a> {
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
#[doc = "Reader of field `FSC6`"]
pub type FSC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC6`"]
pub struct FSC6_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC6_W<'a> {
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
#[doc = "Reader of field `FSC7`"]
pub type FSC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC7`"]
pub struct FSC7_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC7_W<'a> {
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
#[doc = "Reader of field `FSC8`"]
pub type FSC8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC8`"]
pub struct FSC8_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC8_W<'a> {
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
#[doc = "Reader of field `FSC9`"]
pub type FSC9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC9`"]
pub struct FSC9_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC9_W<'a> {
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
#[doc = "Reader of field `FSC10`"]
pub type FSC10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC10`"]
pub struct FSC10_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC10_W<'a> {
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
#[doc = "Reader of field `FSC11`"]
pub type FSC11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC11`"]
pub struct FSC11_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC11_W<'a> {
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
#[doc = "Reader of field `FSC12`"]
pub type FSC12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC12`"]
pub struct FSC12_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC12_W<'a> {
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
#[doc = "Reader of field `FSC13`"]
pub type FSC13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC13`"]
pub struct FSC13_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FSC14`"]
pub type FSC14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC14`"]
pub struct FSC14_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC14_W<'a> {
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
#[doc = "Reader of field `FSC15`"]
pub type FSC15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC15`"]
pub struct FSC15_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC15_W<'a> {
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
#[doc = "Reader of field `FSC16`"]
pub type FSC16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC16`"]
pub struct FSC16_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC16_W<'a> {
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
#[doc = "Reader of field `FSC17`"]
pub type FSC17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC17`"]
pub struct FSC17_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `FSC18`"]
pub type FSC18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC18`"]
pub struct FSC18_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC18_W<'a> {
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
#[doc = "Reader of field `FSC19`"]
pub type FSC19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC19`"]
pub struct FSC19_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC19_W<'a> {
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
#[doc = "Reader of field `FSC20`"]
pub type FSC20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC20`"]
pub struct FSC20_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC20_W<'a> {
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
#[doc = "Reader of field `FSC21`"]
pub type FSC21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC21`"]
pub struct FSC21_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC21_W<'a> {
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
#[doc = "Reader of field `FSC22`"]
pub type FSC22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC22`"]
pub struct FSC22_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC22_W<'a> {
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
#[doc = "Reader of field `FSC23`"]
pub type FSC23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC23`"]
pub struct FSC23_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `FSC24`"]
pub type FSC24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC24`"]
pub struct FSC24_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC24_W<'a> {
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
#[doc = "Reader of field `FSC25`"]
pub type FSC25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC25`"]
pub struct FSC25_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC25_W<'a> {
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
#[doc = "Reader of field `FSC26`"]
pub type FSC26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC26`"]
pub struct FSC26_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC26_W<'a> {
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
#[doc = "Reader of field `FSC27`"]
pub type FSC27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSC27`"]
pub struct FSC27_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC27_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc0(&self) -> FSC0_R {
        FSC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc1(&self) -> FSC1_R {
        FSC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc2(&self) -> FSC2_R {
        FSC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc3(&self) -> FSC3_R {
        FSC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc4(&self) -> FSC4_R {
        FSC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc5(&self) -> FSC5_R {
        FSC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc6(&self) -> FSC6_R {
        FSC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc7(&self) -> FSC7_R {
        FSC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc8(&self) -> FSC8_R {
        FSC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc9(&self) -> FSC9_R {
        FSC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc10(&self) -> FSC10_R {
        FSC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc11(&self) -> FSC11_R {
        FSC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc12(&self) -> FSC12_R {
        FSC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc13(&self) -> FSC13_R {
        FSC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc14(&self) -> FSC14_R {
        FSC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc15(&self) -> FSC15_R {
        FSC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc16(&self) -> FSC16_R {
        FSC16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc17(&self) -> FSC17_R {
        FSC17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc18(&self) -> FSC18_R {
        FSC18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc19(&self) -> FSC19_R {
        FSC19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc20(&self) -> FSC20_R {
        FSC20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc21(&self) -> FSC21_R {
        FSC21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc22(&self) -> FSC22_R {
        FSC22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc23(&self) -> FSC23_R {
        FSC23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc24(&self) -> FSC24_R {
        FSC24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc25(&self) -> FSC25_R {
        FSC25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc26(&self) -> FSC26_R {
        FSC26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc27(&self) -> FSC27_R {
        FSC27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc0(&mut self) -> FSC0_W {
        FSC0_W { w: self }
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc1(&mut self) -> FSC1_W {
        FSC1_W { w: self }
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc2(&mut self) -> FSC2_W {
        FSC2_W { w: self }
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc3(&mut self) -> FSC3_W {
        FSC3_W { w: self }
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc4(&mut self) -> FSC4_W {
        FSC4_W { w: self }
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc5(&mut self) -> FSC5_W {
        FSC5_W { w: self }
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc6(&mut self) -> FSC6_W {
        FSC6_W { w: self }
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc7(&mut self) -> FSC7_W {
        FSC7_W { w: self }
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc8(&mut self) -> FSC8_W {
        FSC8_W { w: self }
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc9(&mut self) -> FSC9_W {
        FSC9_W { w: self }
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc10(&mut self) -> FSC10_W {
        FSC10_W { w: self }
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc11(&mut self) -> FSC11_W {
        FSC11_W { w: self }
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc12(&mut self) -> FSC12_W {
        FSC12_W { w: self }
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc13(&mut self) -> FSC13_W {
        FSC13_W { w: self }
    }
    #[doc = "Bit 14 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc14(&mut self) -> FSC14_W {
        FSC14_W { w: self }
    }
    #[doc = "Bit 15 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc15(&mut self) -> FSC15_W {
        FSC15_W { w: self }
    }
    #[doc = "Bit 16 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc16(&mut self) -> FSC16_W {
        FSC16_W { w: self }
    }
    #[doc = "Bit 17 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc17(&mut self) -> FSC17_W {
        FSC17_W { w: self }
    }
    #[doc = "Bit 18 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc18(&mut self) -> FSC18_W {
        FSC18_W { w: self }
    }
    #[doc = "Bit 19 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc19(&mut self) -> FSC19_W {
        FSC19_W { w: self }
    }
    #[doc = "Bit 20 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc20(&mut self) -> FSC20_W {
        FSC20_W { w: self }
    }
    #[doc = "Bit 21 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc21(&mut self) -> FSC21_W {
        FSC21_W { w: self }
    }
    #[doc = "Bit 22 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc22(&mut self) -> FSC22_W {
        FSC22_W { w: self }
    }
    #[doc = "Bit 23 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc23(&mut self) -> FSC23_W {
        FSC23_W { w: self }
    }
    #[doc = "Bit 24 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc24(&mut self) -> FSC24_W {
        FSC24_W { w: self }
    }
    #[doc = "Bit 25 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc25(&mut self) -> FSC25_W {
        FSC25_W { w: self }
    }
    #[doc = "Bit 26 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc26(&mut self) -> FSC26_W {
        FSC26_W { w: self }
    }
    #[doc = "Bit 27 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc27(&mut self) -> FSC27_W {
        FSC27_W { w: self }
    }
}
