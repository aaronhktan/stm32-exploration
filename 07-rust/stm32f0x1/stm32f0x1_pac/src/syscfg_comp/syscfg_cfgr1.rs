#[doc = "Reader of register SYSCFG_CFGR1"]
pub type R = crate::R<u32, super::SYSCFG_CFGR1>;
#[doc = "Writer for register SYSCFG_CFGR1"]
pub type W = crate::W<u32, super::SYSCFG_CFGR1>;
#[doc = "Register SYSCFG_CFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG_CFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_MODE`"]
pub type MEM_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_MODE`"]
pub struct MEM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ADC_DMA_RMP`"]
pub type ADC_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_DMA_RMP`"]
pub struct ADC_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DMA_RMP_W<'a> {
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
#[doc = "Reader of field `USART1_TX_DMA_RMP`"]
pub type USART1_TX_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1_TX_DMA_RMP`"]
pub struct USART1_TX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1_TX_DMA_RMP_W<'a> {
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
#[doc = "Reader of field `USART1_RX_DMA_RMP`"]
pub type USART1_RX_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1_RX_DMA_RMP`"]
pub struct USART1_RX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1_RX_DMA_RMP_W<'a> {
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
#[doc = "Reader of field `TIM16_DMA_RMP`"]
pub type TIM16_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM16_DMA_RMP`"]
pub struct TIM16_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16_DMA_RMP_W<'a> {
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
#[doc = "Reader of field `TIM17_DMA_RMP`"]
pub type TIM17_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM17_DMA_RMP`"]
pub struct TIM17_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17_DMA_RMP_W<'a> {
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
#[doc = "Reader of field `I2C_PB6_FM`"]
pub type I2C_PB6_FM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_PB6_FM`"]
pub struct I2C_PB6_FM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB6_FM_W<'a> {
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
#[doc = "Reader of field `I2C_PB7_FM`"]
pub type I2C_PB7_FM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_PB7_FM`"]
pub struct I2C_PB7_FM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB7_FM_W<'a> {
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
#[doc = "Reader of field `I2C_PB8_FM`"]
pub type I2C_PB8_FM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_PB8_FM`"]
pub struct I2C_PB8_FM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB8_FM_W<'a> {
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
#[doc = "Reader of field `I2C_PB9_FM`"]
pub type I2C_PB9_FM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_PB9_FM`"]
pub struct I2C_PB9_FM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB9_FM_W<'a> {
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
#[doc = "Reader of field `I2C1_FM_plus`"]
pub type I2C1_FM_PLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1_FM_plus`"]
pub struct I2C1_FM_PLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_FM_PLUS_W<'a> {
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
#[doc = "Reader of field `I2C2_FM_plus`"]
pub type I2C2_FM_PLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2_FM_plus`"]
pub struct I2C2_FM_PLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_FM_PLUS_W<'a> {
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
#[doc = "Reader of field `SPI2_DMA_RMP`"]
pub type SPI2_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2_DMA_RMP`"]
pub struct SPI2_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_DMA_RMP_W<'a> {
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
#[doc = "Reader of field `USART2_DMA_RMP`"]
pub type USART2_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART2_DMA_RMP`"]
pub struct USART2_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2_DMA_RMP_W<'a> {
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
#[doc = "Reader of field `USART3_DMA_RMP`"]
pub type USART3_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART3_DMA_RMP`"]
pub struct USART3_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3_DMA_RMP_W<'a> {
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
#[doc = "Reader of field `I2C1_DMA_RMP`"]
pub type I2C1_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1_DMA_RMP`"]
pub struct I2C1_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_DMA_RMP_W<'a> {
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
#[doc = "Reader of field `TIM1_DMA_RMP`"]
pub type TIM1_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM1_DMA_RMP`"]
pub struct TIM1_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_DMA_RMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `TIM2_DMA_RMP`"]
pub type TIM2_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM2_DMA_RMP`"]
pub struct TIM2_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2_DMA_RMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `TIM3_DMA_RMP`"]
pub type TIM3_DMA_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM3_DMA_RMP`"]
pub struct TIM3_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3_DMA_RMP_W<'a> {
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
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - ADC DMA remapping bit"]
    #[inline(always)]
    pub fn adc_dma_rmp(&self) -> ADC_DMA_RMP_R {
        ADC_DMA_RMP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USART1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn usart1_tx_dma_rmp(&self) -> USART1_TX_DMA_RMP_R {
        USART1_TX_DMA_RMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USART1_RX DMA request remapping bit"]
    #[inline(always)]
    pub fn usart1_rx_dma_rmp(&self) -> USART1_RX_DMA_RMP_R {
        USART1_RX_DMA_RMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&self) -> TIM16_DMA_RMP_R {
        TIM16_DMA_RMP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&self) -> TIM17_DMA_RMP_R {
        TIM17_DMA_RMP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM plus) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fm(&self) -> I2C_PB6_FM_R {
        I2C_PB6_FM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fm(&self) -> I2C_PB7_FM_R {
        I2C_PB7_FM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fm(&self) -> I2C_PB8_FM_R {
        I2C_PB8_FM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fm(&self) -> I2C_PB9_FM_R {
        I2C_PB9_FM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - FM+ driving capability activation for I2C1"]
    #[inline(always)]
    pub fn i2c1_fm_plus(&self) -> I2C1_FM_PLUS_R {
        I2C1_FM_PLUS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - FM+ driving capability activation for I2C2"]
    #[inline(always)]
    pub fn i2c2_fm_plus(&self) -> I2C2_FM_PLUS_R {
        I2C2_FM_PLUS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SPI2 DMA request remapping bit"]
    #[inline(always)]
    pub fn spi2_dma_rmp(&self) -> SPI2_DMA_RMP_R {
        SPI2_DMA_RMP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USART2 DMA request remapping bit"]
    #[inline(always)]
    pub fn usart2_dma_rmp(&self) -> USART2_DMA_RMP_R {
        USART2_DMA_RMP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USART3 DMA request remapping bit"]
    #[inline(always)]
    pub fn usart3_dma_rmp(&self) -> USART3_DMA_RMP_R {
        USART3_DMA_RMP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - I2C1 DMA request remapping bit"]
    #[inline(always)]
    pub fn i2c1_dma_rmp(&self) -> I2C1_DMA_RMP_R {
        I2C1_DMA_RMP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TIM1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim1_dma_rmp(&self) -> TIM1_DMA_RMP_R {
        TIM1_DMA_RMP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TIM2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim2_dma_rmp(&self) -> TIM2_DMA_RMP_R {
        TIM2_DMA_RMP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TIM3 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim3_dma_rmp(&self) -> TIM3_DMA_RMP_R {
        TIM3_DMA_RMP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W {
        MEM_MODE_W { w: self }
    }
    #[doc = "Bit 8 - ADC DMA remapping bit"]
    #[inline(always)]
    pub fn adc_dma_rmp(&mut self) -> ADC_DMA_RMP_W {
        ADC_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 9 - USART1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn usart1_tx_dma_rmp(&mut self) -> USART1_TX_DMA_RMP_W {
        USART1_TX_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 10 - USART1_RX DMA request remapping bit"]
    #[inline(always)]
    pub fn usart1_rx_dma_rmp(&mut self) -> USART1_RX_DMA_RMP_W {
        USART1_RX_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&mut self) -> TIM16_DMA_RMP_W {
        TIM16_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&mut self) -> TIM17_DMA_RMP_W {
        TIM17_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM plus) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fm(&mut self) -> I2C_PB6_FM_W {
        I2C_PB6_FM_W { w: self }
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fm(&mut self) -> I2C_PB7_FM_W {
        I2C_PB7_FM_W { w: self }
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fm(&mut self) -> I2C_PB8_FM_W {
        I2C_PB8_FM_W { w: self }
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fm(&mut self) -> I2C_PB9_FM_W {
        I2C_PB9_FM_W { w: self }
    }
    #[doc = "Bit 20 - FM+ driving capability activation for I2C1"]
    #[inline(always)]
    pub fn i2c1_fm_plus(&mut self) -> I2C1_FM_PLUS_W {
        I2C1_FM_PLUS_W { w: self }
    }
    #[doc = "Bit 21 - FM+ driving capability activation for I2C2"]
    #[inline(always)]
    pub fn i2c2_fm_plus(&mut self) -> I2C2_FM_PLUS_W {
        I2C2_FM_PLUS_W { w: self }
    }
    #[doc = "Bit 24 - SPI2 DMA request remapping bit"]
    #[inline(always)]
    pub fn spi2_dma_rmp(&mut self) -> SPI2_DMA_RMP_W {
        SPI2_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 25 - USART2 DMA request remapping bit"]
    #[inline(always)]
    pub fn usart2_dma_rmp(&mut self) -> USART2_DMA_RMP_W {
        USART2_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 26 - USART3 DMA request remapping bit"]
    #[inline(always)]
    pub fn usart3_dma_rmp(&mut self) -> USART3_DMA_RMP_W {
        USART3_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 27 - I2C1 DMA request remapping bit"]
    #[inline(always)]
    pub fn i2c1_dma_rmp(&mut self) -> I2C1_DMA_RMP_W {
        I2C1_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 28 - TIM1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim1_dma_rmp(&mut self) -> TIM1_DMA_RMP_W {
        TIM1_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 29 - TIM2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim2_dma_rmp(&mut self) -> TIM2_DMA_RMP_W {
        TIM2_DMA_RMP_W { w: self }
    }
    #[doc = "Bit 30 - TIM3 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim3_dma_rmp(&mut self) -> TIM3_DMA_RMP_W {
        TIM3_DMA_RMP_W { w: self }
    }
}
