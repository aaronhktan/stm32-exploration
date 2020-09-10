#[doc = "Reader of register SDMMC_RESP2R"]
pub type R = crate::R<u32, super::SDMMC_RESP2R>;
#[doc = "Reader of field `CARDSTATUS2`"]
pub type CARDSTATUS2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - see Table404."]
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
