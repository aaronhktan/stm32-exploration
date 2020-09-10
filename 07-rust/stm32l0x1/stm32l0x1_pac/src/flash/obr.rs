#[doc = "Reader of register OBR"]
pub type R = crate::R<u32, super::OBR>;
#[doc = "Reader of field `RDPRT`"]
pub type RDPRT_R = crate::R<u8, u8>;
#[doc = "Reader of field `BOR_LEV`"]
pub type BOR_LEV_R = crate::R<u8, u8>;
#[doc = "Reader of field `SPRMOD`"]
pub type SPRMOD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - Read protection"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - BOR_LEV"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Selection of protection mode of WPR bits"]
    #[inline(always)]
    pub fn sprmod(&self) -> SPRMOD_R {
        SPRMOD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
