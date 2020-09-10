#[doc = "Reader of register DTXFSTS1"]
pub type R = crate::R<u32, super::DTXFSTS1>;
#[doc = "Reader of field `INEPTFSAV`"]
pub type INEPTFSAV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
