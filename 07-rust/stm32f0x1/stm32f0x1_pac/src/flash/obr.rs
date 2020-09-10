#[doc = "Reader of register OBR"]
pub type R = crate::R<u32, super::OBR>;
#[doc = "Reader of field `OPTERR`"]
pub type OPTERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDPRT`"]
pub type RDPRT_R = crate::R<u8, u8>;
#[doc = "Reader of field `WDG_SW`"]
pub type WDG_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `nRST_STOP`"]
pub type NRST_STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `nRST_STDBY`"]
pub type NRST_STDBY_R = crate::R<bool, bool>;
#[doc = "Reader of field `nBOOT0`"]
pub type NBOOT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `nBOOT1`"]
pub type NBOOT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDDA_MONITOR`"]
pub type VDDA_MONITOR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAM_PARITY_CHECK`"]
pub type RAM_PARITY_CHECK_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOOT_SEL`"]
pub type BOOT_SEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `Data0`"]
pub type DATA0_R = crate::R<u8, u8>;
#[doc = "Reader of field `Data1`"]
pub type DATA1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Read protection level status"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 8 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - nBOOT0"]
    #[inline(always)]
    pub fn n_boot0(&self) -> NBOOT0_R {
        NBOOT0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BOOT1"]
    #[inline(always)]
    pub fn n_boot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VDDA_MONITOR"]
    #[inline(always)]
    pub fn vdda_monitor(&self) -> VDDA_MONITOR_R {
        VDDA_MONITOR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RAM_PARITY_CHECK"]
    #[inline(always)]
    pub fn ram_parity_check(&self) -> RAM_PARITY_CHECK_R {
        RAM_PARITY_CHECK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - BOOT_SEL"]
    #[inline(always)]
    pub fn boot_sel(&self) -> BOOT_SEL_R {
        BOOT_SEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Data0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
