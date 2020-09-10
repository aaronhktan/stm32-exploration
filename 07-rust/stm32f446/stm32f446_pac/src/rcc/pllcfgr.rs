#[doc = "Reader of register PLLCFGR"]
pub type R = crate::R<u32, super::PLLCFGR>;
#[doc = "Writer for register PLLCFGR"]
pub type W = crate::W<u32, super::PLLCFGR>;
#[doc = "Register PLLCFGR `reset()`'s with value 0x2400_3010"]
impl crate::ResetValue for super::PLLCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2400_3010
    }
}
#[doc = "Reader of field `PLLQ3`"]
pub type PLLQ3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLQ3`"]
pub struct PLLQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ3_W<'a> {
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
#[doc = "Reader of field `PLLQ2`"]
pub type PLLQ2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLQ2`"]
pub struct PLLQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ2_W<'a> {
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
#[doc = "Reader of field `PLLQ1`"]
pub type PLLQ1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLQ1`"]
pub struct PLLQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ1_W<'a> {
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
#[doc = "Reader of field `PLLQ0`"]
pub type PLLQ0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLQ0`"]
pub struct PLLQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ0_W<'a> {
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
#[doc = "Reader of field `PLLSRC`"]
pub type PLLSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSRC`"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
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
#[doc = "Reader of field `PLLP1`"]
pub type PLLP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLP1`"]
pub struct PLLP1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLP1_W<'a> {
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
#[doc = "Reader of field `PLLP0`"]
pub type PLLP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLP0`"]
pub struct PLLP0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLP0_W<'a> {
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
#[doc = "Reader of field `PLLN8`"]
pub type PLLN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLN8`"]
pub struct PLLN8_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN8_W<'a> {
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
#[doc = "Reader of field `PLLN7`"]
pub type PLLN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLN7`"]
pub struct PLLN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN7_W<'a> {
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
#[doc = "Reader of field `PLLN6`"]
pub type PLLN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLN6`"]
pub struct PLLN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN6_W<'a> {
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
#[doc = "Reader of field `PLLN5`"]
pub type PLLN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLN5`"]
pub struct PLLN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN5_W<'a> {
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
#[doc = "Reader of field `PLLN4`"]
pub type PLLN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLN4`"]
pub struct PLLN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN4_W<'a> {
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
#[doc = "Reader of field `PLLN3`"]
pub type PLLN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLN3`"]
pub struct PLLN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN3_W<'a> {
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
#[doc = "Reader of field `PLLN2`"]
pub type PLLN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLN2`"]
pub struct PLLN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN2_W<'a> {
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
#[doc = "Reader of field `PLLN1`"]
pub type PLLN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLN1`"]
pub struct PLLN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN1_W<'a> {
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
#[doc = "Reader of field `PLLN0`"]
pub type PLLN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLN0`"]
pub struct PLLN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN0_W<'a> {
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
#[doc = "Reader of field `PLLM5`"]
pub type PLLM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLM5`"]
pub struct PLLM5_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM5_W<'a> {
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
#[doc = "Reader of field `PLLM4`"]
pub type PLLM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLM4`"]
pub struct PLLM4_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM4_W<'a> {
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
#[doc = "Reader of field `PLLM3`"]
pub type PLLM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLM3`"]
pub struct PLLM3_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM3_W<'a> {
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
#[doc = "Reader of field `PLLM2`"]
pub type PLLM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLM2`"]
pub struct PLLM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM2_W<'a> {
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
#[doc = "Reader of field `PLLM1`"]
pub type PLLM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLM1`"]
pub struct PLLM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM1_W<'a> {
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
#[doc = "Reader of field `PLLM0`"]
pub type PLLM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLM0`"]
pub struct PLLM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM0_W<'a> {
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
    #[doc = "Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq3(&self) -> PLLQ3_R {
        PLLQ3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq2(&self) -> PLLQ2_R {
        PLLQ2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq1(&self) -> PLLQ1_R {
        PLLQ1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq0(&self) -> PLLQ0_R {
        PLLQ0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp1(&self) -> PLLP1_R {
        PLLP1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp0(&self) -> PLLP0_R {
        PLLP0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln8(&self) -> PLLN8_R {
        PLLN8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln7(&self) -> PLLN7_R {
        PLLN7_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln6(&self) -> PLLN6_R {
        PLLN6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln5(&self) -> PLLN5_R {
        PLLN5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln4(&self) -> PLLN4_R {
        PLLN4_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln3(&self) -> PLLN3_R {
        PLLN3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln2(&self) -> PLLN2_R {
        PLLN2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln1(&self) -> PLLN1_R {
        PLLN1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln0(&self) -> PLLN0_R {
        PLLN0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm5(&self) -> PLLM5_R {
        PLLM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm4(&self) -> PLLM4_R {
        PLLM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm3(&self) -> PLLM3_R {
        PLLM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm2(&self) -> PLLM2_R {
        PLLM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm1(&self) -> PLLM1_R {
        PLLM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm0(&self) -> PLLM0_R {
        PLLM0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq3(&mut self) -> PLLQ3_W {
        PLLQ3_W { w: self }
    }
    #[doc = "Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq2(&mut self) -> PLLQ2_W {
        PLLQ2_W { w: self }
    }
    #[doc = "Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq1(&mut self) -> PLLQ1_W {
        PLLQ1_W { w: self }
    }
    #[doc = "Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq0(&mut self) -> PLLQ0_W {
        PLLQ0_W { w: self }
    }
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Bit 17 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp1(&mut self) -> PLLP1_W {
        PLLP1_W { w: self }
    }
    #[doc = "Bit 16 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp0(&mut self) -> PLLP0_W {
        PLLP0_W { w: self }
    }
    #[doc = "Bit 14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln8(&mut self) -> PLLN8_W {
        PLLN8_W { w: self }
    }
    #[doc = "Bit 13 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln7(&mut self) -> PLLN7_W {
        PLLN7_W { w: self }
    }
    #[doc = "Bit 12 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln6(&mut self) -> PLLN6_W {
        PLLN6_W { w: self }
    }
    #[doc = "Bit 11 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln5(&mut self) -> PLLN5_W {
        PLLN5_W { w: self }
    }
    #[doc = "Bit 10 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln4(&mut self) -> PLLN4_W {
        PLLN4_W { w: self }
    }
    #[doc = "Bit 9 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln3(&mut self) -> PLLN3_W {
        PLLN3_W { w: self }
    }
    #[doc = "Bit 8 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln2(&mut self) -> PLLN2_W {
        PLLN2_W { w: self }
    }
    #[doc = "Bit 7 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln1(&mut self) -> PLLN1_W {
        PLLN1_W { w: self }
    }
    #[doc = "Bit 6 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln0(&mut self) -> PLLN0_W {
        PLLN0_W { w: self }
    }
    #[doc = "Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm5(&mut self) -> PLLM5_W {
        PLLM5_W { w: self }
    }
    #[doc = "Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm4(&mut self) -> PLLM4_W {
        PLLM4_W { w: self }
    }
    #[doc = "Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm3(&mut self) -> PLLM3_W {
        PLLM3_W { w: self }
    }
    #[doc = "Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm2(&mut self) -> PLLM2_W {
        PLLM2_W { w: self }
    }
    #[doc = "Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm1(&mut self) -> PLLM1_W {
        PLLM1_W { w: self }
    }
    #[doc = "Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm0(&mut self) -> PLLM0_W {
        PLLM0_W { w: self }
    }
}
