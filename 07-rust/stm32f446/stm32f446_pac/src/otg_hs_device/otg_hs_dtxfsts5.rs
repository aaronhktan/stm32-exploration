#[doc = "Reader of register OTG_HS_DTXFSTS5"]
pub type R = crate::R<u32, super::OTG_HS_DTXFSTS5>;
#[doc = "Reader of field `INEPTFSAV`"]
pub type INEPTFSAV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space avail"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
