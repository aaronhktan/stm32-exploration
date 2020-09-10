#[doc = "Reader of register CKGATENR"]
pub type R = crate::R<u32, super::CKGATENR>;
#[doc = "Writer for register CKGATENR"]
pub type W = crate::W<u32, super::CKGATENR>;
#[doc = "Register CKGATENR `reset()`'s with value 0"]
impl crate::ResetValue for super::CKGATENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AHB2APB1_CKEN`"]
pub type AHB2APB1_CKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB2APB1_CKEN`"]
pub struct AHB2APB1_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB2APB1_CKEN_W<'a> {
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
#[doc = "Reader of field `AHB2APB2_CKEN`"]
pub type AHB2APB2_CKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB2APB2_CKEN`"]
pub struct AHB2APB2_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB2APB2_CKEN_W<'a> {
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
#[doc = "Reader of field `CM4DBG_CKEN`"]
pub type CM4DBG_CKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CM4DBG_CKEN`"]
pub struct CM4DBG_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CM4DBG_CKEN_W<'a> {
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
#[doc = "Reader of field `SPARE_CKEN`"]
pub type SPARE_CKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPARE_CKEN`"]
pub struct SPARE_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_CKEN_W<'a> {
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
#[doc = "Reader of field `SRAM_CKEN`"]
pub type SRAM_CKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM_CKEN`"]
pub struct SRAM_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CKEN_W<'a> {
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
#[doc = "Reader of field `FLITF_CKEN`"]
pub type FLITF_CKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLITF_CKEN`"]
pub struct FLITF_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLITF_CKEN_W<'a> {
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
#[doc = "Reader of field `RCC_CKEN`"]
pub type RCC_CKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCC_CKEN`"]
pub struct RCC_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCC_CKEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - AHB to APB1 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb1_cken(&self) -> AHB2APB1_CKEN_R {
        AHB2APB1_CKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AHB to APB2 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb2_cken(&self) -> AHB2APB2_CKEN_R {
        AHB2APB2_CKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cortex M4 ETM clock enable"]
    #[inline(always)]
    pub fn cm4dbg_cken(&self) -> CM4DBG_CKEN_R {
        CM4DBG_CKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Spare clock enable"]
    #[inline(always)]
    pub fn spare_cken(&self) -> SPARE_CKEN_R {
        SPARE_CKEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SRQAM controller clock enable"]
    #[inline(always)]
    pub fn sram_cken(&self) -> SRAM_CKEN_R {
        SRAM_CKEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Flash Interface clock enable"]
    #[inline(always)]
    pub fn flitf_cken(&self) -> FLITF_CKEN_R {
        FLITF_CKEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RCC clock enable"]
    #[inline(always)]
    pub fn rcc_cken(&self) -> RCC_CKEN_R {
        RCC_CKEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB to APB1 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb1_cken(&mut self) -> AHB2APB1_CKEN_W {
        AHB2APB1_CKEN_W { w: self }
    }
    #[doc = "Bit 1 - AHB to APB2 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb2_cken(&mut self) -> AHB2APB2_CKEN_W {
        AHB2APB2_CKEN_W { w: self }
    }
    #[doc = "Bit 2 - Cortex M4 ETM clock enable"]
    #[inline(always)]
    pub fn cm4dbg_cken(&mut self) -> CM4DBG_CKEN_W {
        CM4DBG_CKEN_W { w: self }
    }
    #[doc = "Bit 3 - Spare clock enable"]
    #[inline(always)]
    pub fn spare_cken(&mut self) -> SPARE_CKEN_W {
        SPARE_CKEN_W { w: self }
    }
    #[doc = "Bit 4 - SRQAM controller clock enable"]
    #[inline(always)]
    pub fn sram_cken(&mut self) -> SRAM_CKEN_W {
        SRAM_CKEN_W { w: self }
    }
    #[doc = "Bit 5 - Flash Interface clock enable"]
    #[inline(always)]
    pub fn flitf_cken(&mut self) -> FLITF_CKEN_W {
        FLITF_CKEN_W { w: self }
    }
    #[doc = "Bit 6 - RCC clock enable"]
    #[inline(always)]
    pub fn rcc_cken(&mut self) -> RCC_CKEN_W {
        RCC_CKEN_W { w: self }
    }
}
