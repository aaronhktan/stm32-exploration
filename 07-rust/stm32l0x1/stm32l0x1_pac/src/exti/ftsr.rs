#[doc = "Reader of register FTSR"]
pub type R = crate::R<u32, super::FTSR>;
#[doc = "Writer for register FTSR"]
pub type W = crate::W<u32, super::FTSR>;
#[doc = "Register FTSR `reset()`'s with value 0"]
impl crate::ResetValue for super::FTSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FT0`"]
pub type FT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT0`"]
pub struct FT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FT0_W<'a> {
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
#[doc = "Reader of field `FT1`"]
pub type FT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT1`"]
pub struct FT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FT1_W<'a> {
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
#[doc = "Reader of field `FT2`"]
pub type FT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT2`"]
pub struct FT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FT2_W<'a> {
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
#[doc = "Reader of field `FT3`"]
pub type FT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT3`"]
pub struct FT3_W<'a> {
    w: &'a mut W,
}
impl<'a> FT3_W<'a> {
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
#[doc = "Reader of field `FT4`"]
pub type FT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT4`"]
pub struct FT4_W<'a> {
    w: &'a mut W,
}
impl<'a> FT4_W<'a> {
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
#[doc = "Reader of field `FT5`"]
pub type FT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT5`"]
pub struct FT5_W<'a> {
    w: &'a mut W,
}
impl<'a> FT5_W<'a> {
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
#[doc = "Reader of field `FT6`"]
pub type FT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT6`"]
pub struct FT6_W<'a> {
    w: &'a mut W,
}
impl<'a> FT6_W<'a> {
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
#[doc = "Reader of field `FT7`"]
pub type FT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT7`"]
pub struct FT7_W<'a> {
    w: &'a mut W,
}
impl<'a> FT7_W<'a> {
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
#[doc = "Reader of field `FT8`"]
pub type FT8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT8`"]
pub struct FT8_W<'a> {
    w: &'a mut W,
}
impl<'a> FT8_W<'a> {
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
#[doc = "Reader of field `FT9`"]
pub type FT9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT9`"]
pub struct FT9_W<'a> {
    w: &'a mut W,
}
impl<'a> FT9_W<'a> {
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
#[doc = "Reader of field `FT10`"]
pub type FT10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT10`"]
pub struct FT10_W<'a> {
    w: &'a mut W,
}
impl<'a> FT10_W<'a> {
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
#[doc = "Reader of field `FT11`"]
pub type FT11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT11`"]
pub struct FT11_W<'a> {
    w: &'a mut W,
}
impl<'a> FT11_W<'a> {
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
#[doc = "Reader of field `FT12`"]
pub type FT12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT12`"]
pub struct FT12_W<'a> {
    w: &'a mut W,
}
impl<'a> FT12_W<'a> {
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
#[doc = "Reader of field `FT13`"]
pub type FT13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT13`"]
pub struct FT13_W<'a> {
    w: &'a mut W,
}
impl<'a> FT13_W<'a> {
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
#[doc = "Reader of field `FT14`"]
pub type FT14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT14`"]
pub struct FT14_W<'a> {
    w: &'a mut W,
}
impl<'a> FT14_W<'a> {
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
#[doc = "Reader of field `FT15`"]
pub type FT15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT15`"]
pub struct FT15_W<'a> {
    w: &'a mut W,
}
impl<'a> FT15_W<'a> {
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
#[doc = "Reader of field `FT16`"]
pub type FT16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT16`"]
pub struct FT16_W<'a> {
    w: &'a mut W,
}
impl<'a> FT16_W<'a> {
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
#[doc = "Reader of field `FT17`"]
pub type FT17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT17`"]
pub struct FT17_W<'a> {
    w: &'a mut W,
}
impl<'a> FT17_W<'a> {
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
#[doc = "Reader of field `FT19`"]
pub type FT19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT19`"]
pub struct FT19_W<'a> {
    w: &'a mut W,
}
impl<'a> FT19_W<'a> {
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
#[doc = "Reader of field `FT20`"]
pub type FT20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT20`"]
pub struct FT20_W<'a> {
    w: &'a mut W,
}
impl<'a> FT20_W<'a> {
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
#[doc = "Reader of field `FT21`"]
pub type FT21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT21`"]
pub struct FT21_W<'a> {
    w: &'a mut W,
}
impl<'a> FT21_W<'a> {
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
#[doc = "Reader of field `FT22`"]
pub type FT22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT22`"]
pub struct FT22_W<'a> {
    w: &'a mut W,
}
impl<'a> FT22_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration of line 0"]
    #[inline(always)]
    pub fn ft0(&self) -> FT0_R {
        FT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 1"]
    #[inline(always)]
    pub fn ft1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Falling trigger event configuration of line 2"]
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Falling trigger event configuration of line 3"]
    #[inline(always)]
    pub fn ft3(&self) -> FT3_R {
        FT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration of line 4"]
    #[inline(always)]
    pub fn ft4(&self) -> FT4_R {
        FT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration of line 5"]
    #[inline(always)]
    pub fn ft5(&self) -> FT5_R {
        FT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration of line 6"]
    #[inline(always)]
    pub fn ft6(&self) -> FT6_R {
        FT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Falling trigger event configuration of line 7"]
    #[inline(always)]
    pub fn ft7(&self) -> FT7_R {
        FT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 8"]
    #[inline(always)]
    pub fn ft8(&self) -> FT8_R {
        FT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 9"]
    #[inline(always)]
    pub fn ft9(&self) -> FT9_R {
        FT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Falling trigger event configuration of line 10"]
    #[inline(always)]
    pub fn ft10(&self) -> FT10_R {
        FT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Falling trigger event configuration of line 11"]
    #[inline(always)]
    pub fn ft11(&self) -> FT11_R {
        FT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Falling trigger event configuration of line 12"]
    #[inline(always)]
    pub fn ft12(&self) -> FT12_R {
        FT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Falling trigger event configuration of line 13"]
    #[inline(always)]
    pub fn ft13(&self) -> FT13_R {
        FT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Falling trigger event configuration of line 14"]
    #[inline(always)]
    pub fn ft14(&self) -> FT14_R {
        FT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Falling trigger event configuration of line 15"]
    #[inline(always)]
    pub fn ft15(&self) -> FT15_R {
        FT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Falling trigger event configuration of line 16"]
    #[inline(always)]
    pub fn ft16(&self) -> FT16_R {
        FT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Falling trigger event configuration of line 17"]
    #[inline(always)]
    pub fn ft17(&self) -> FT17_R {
        FT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Falling trigger event configuration of line 19"]
    #[inline(always)]
    pub fn ft19(&self) -> FT19_R {
        FT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Falling trigger event configuration of line 20"]
    #[inline(always)]
    pub fn ft20(&self) -> FT20_R {
        FT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Falling trigger event configuration of line 21"]
    #[inline(always)]
    pub fn ft21(&self) -> FT21_R {
        FT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Falling trigger event configuration of line 22"]
    #[inline(always)]
    pub fn ft22(&self) -> FT22_R {
        FT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration of line 0"]
    #[inline(always)]
    pub fn ft0(&mut self) -> FT0_W {
        FT0_W { w: self }
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 1"]
    #[inline(always)]
    pub fn ft1(&mut self) -> FT1_W {
        FT1_W { w: self }
    }
    #[doc = "Bit 2 - Falling trigger event configuration of line 2"]
    #[inline(always)]
    pub fn ft2(&mut self) -> FT2_W {
        FT2_W { w: self }
    }
    #[doc = "Bit 3 - Falling trigger event configuration of line 3"]
    #[inline(always)]
    pub fn ft3(&mut self) -> FT3_W {
        FT3_W { w: self }
    }
    #[doc = "Bit 4 - Falling trigger event configuration of line 4"]
    #[inline(always)]
    pub fn ft4(&mut self) -> FT4_W {
        FT4_W { w: self }
    }
    #[doc = "Bit 5 - Falling trigger event configuration of line 5"]
    #[inline(always)]
    pub fn ft5(&mut self) -> FT5_W {
        FT5_W { w: self }
    }
    #[doc = "Bit 6 - Falling trigger event configuration of line 6"]
    #[inline(always)]
    pub fn ft6(&mut self) -> FT6_W {
        FT6_W { w: self }
    }
    #[doc = "Bit 7 - Falling trigger event configuration of line 7"]
    #[inline(always)]
    pub fn ft7(&mut self) -> FT7_W {
        FT7_W { w: self }
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 8"]
    #[inline(always)]
    pub fn ft8(&mut self) -> FT8_W {
        FT8_W { w: self }
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 9"]
    #[inline(always)]
    pub fn ft9(&mut self) -> FT9_W {
        FT9_W { w: self }
    }
    #[doc = "Bit 10 - Falling trigger event configuration of line 10"]
    #[inline(always)]
    pub fn ft10(&mut self) -> FT10_W {
        FT10_W { w: self }
    }
    #[doc = "Bit 11 - Falling trigger event configuration of line 11"]
    #[inline(always)]
    pub fn ft11(&mut self) -> FT11_W {
        FT11_W { w: self }
    }
    #[doc = "Bit 12 - Falling trigger event configuration of line 12"]
    #[inline(always)]
    pub fn ft12(&mut self) -> FT12_W {
        FT12_W { w: self }
    }
    #[doc = "Bit 13 - Falling trigger event configuration of line 13"]
    #[inline(always)]
    pub fn ft13(&mut self) -> FT13_W {
        FT13_W { w: self }
    }
    #[doc = "Bit 14 - Falling trigger event configuration of line 14"]
    #[inline(always)]
    pub fn ft14(&mut self) -> FT14_W {
        FT14_W { w: self }
    }
    #[doc = "Bit 15 - Falling trigger event configuration of line 15"]
    #[inline(always)]
    pub fn ft15(&mut self) -> FT15_W {
        FT15_W { w: self }
    }
    #[doc = "Bit 16 - Falling trigger event configuration of line 16"]
    #[inline(always)]
    pub fn ft16(&mut self) -> FT16_W {
        FT16_W { w: self }
    }
    #[doc = "Bit 17 - Falling trigger event configuration of line 17"]
    #[inline(always)]
    pub fn ft17(&mut self) -> FT17_W {
        FT17_W { w: self }
    }
    #[doc = "Bit 19 - Falling trigger event configuration of line 19"]
    #[inline(always)]
    pub fn ft19(&mut self) -> FT19_W {
        FT19_W { w: self }
    }
    #[doc = "Bit 20 - Falling trigger event configuration of line 20"]
    #[inline(always)]
    pub fn ft20(&mut self) -> FT20_W {
        FT20_W { w: self }
    }
    #[doc = "Bit 21 - Falling trigger event configuration of line 21"]
    #[inline(always)]
    pub fn ft21(&mut self) -> FT21_W {
        FT21_W { w: self }
    }
    #[doc = "Bit 22 - Falling trigger event configuration of line 22"]
    #[inline(always)]
    pub fn ft22(&mut self) -> FT22_W {
        FT22_W { w: self }
    }
}
