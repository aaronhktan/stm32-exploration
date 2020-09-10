#[doc = "Reader of register SDMMC_DCNTR"]
pub type R = crate::R<u32, super::SDMMC_DCNTR>;
#[doc = "Reader of field `DATACOUNT`"]
pub type DATACOUNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:26 - Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect."]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new((self.bits & 0x07ff_ffff) as u32)
    }
}
