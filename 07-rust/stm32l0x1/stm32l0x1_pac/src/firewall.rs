#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Code segment start address"]
    pub firewall_cssa: FIREWALL_CSSA,
    #[doc = "0x04 - Code segment length"]
    pub firewall_csl: FIREWALL_CSL,
    #[doc = "0x08 - Non-volatile data segment start address"]
    pub firewall_nvdssa: FIREWALL_NVDSSA,
    #[doc = "0x0c - Non-volatile data segment length"]
    pub firewall_nvdsl: FIREWALL_NVDSL,
    #[doc = "0x10 - Volatile data segment start address"]
    pub firewall_vdssa: FIREWALL_VDSSA,
    #[doc = "0x14 - Volatile data segment length"]
    pub firewall_vdsl: FIREWALL_VDSL,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Configuration register"]
    pub firewall_cr: FIREWALL_CR,
}
#[doc = "Code segment start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firewall_cssa](firewall_cssa) module"]
pub type FIREWALL_CSSA = crate::Reg<u32, _FIREWALL_CSSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIREWALL_CSSA;
#[doc = "`read()` method returns [firewall_cssa::R](firewall_cssa::R) reader structure"]
impl crate::Readable for FIREWALL_CSSA {}
#[doc = "`write(|w| ..)` method takes [firewall_cssa::W](firewall_cssa::W) writer structure"]
impl crate::Writable for FIREWALL_CSSA {}
#[doc = "Code segment start address"]
pub mod firewall_cssa;
#[doc = "Code segment length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firewall_csl](firewall_csl) module"]
pub type FIREWALL_CSL = crate::Reg<u32, _FIREWALL_CSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIREWALL_CSL;
#[doc = "`read()` method returns [firewall_csl::R](firewall_csl::R) reader structure"]
impl crate::Readable for FIREWALL_CSL {}
#[doc = "`write(|w| ..)` method takes [firewall_csl::W](firewall_csl::W) writer structure"]
impl crate::Writable for FIREWALL_CSL {}
#[doc = "Code segment length"]
pub mod firewall_csl;
#[doc = "Non-volatile data segment start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firewall_nvdssa](firewall_nvdssa) module"]
pub type FIREWALL_NVDSSA = crate::Reg<u32, _FIREWALL_NVDSSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIREWALL_NVDSSA;
#[doc = "`read()` method returns [firewall_nvdssa::R](firewall_nvdssa::R) reader structure"]
impl crate::Readable for FIREWALL_NVDSSA {}
#[doc = "`write(|w| ..)` method takes [firewall_nvdssa::W](firewall_nvdssa::W) writer structure"]
impl crate::Writable for FIREWALL_NVDSSA {}
#[doc = "Non-volatile data segment start address"]
pub mod firewall_nvdssa;
#[doc = "Non-volatile data segment length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firewall_nvdsl](firewall_nvdsl) module"]
pub type FIREWALL_NVDSL = crate::Reg<u32, _FIREWALL_NVDSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIREWALL_NVDSL;
#[doc = "`read()` method returns [firewall_nvdsl::R](firewall_nvdsl::R) reader structure"]
impl crate::Readable for FIREWALL_NVDSL {}
#[doc = "`write(|w| ..)` method takes [firewall_nvdsl::W](firewall_nvdsl::W) writer structure"]
impl crate::Writable for FIREWALL_NVDSL {}
#[doc = "Non-volatile data segment length"]
pub mod firewall_nvdsl;
#[doc = "Volatile data segment start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firewall_vdssa](firewall_vdssa) module"]
pub type FIREWALL_VDSSA = crate::Reg<u32, _FIREWALL_VDSSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIREWALL_VDSSA;
#[doc = "`read()` method returns [firewall_vdssa::R](firewall_vdssa::R) reader structure"]
impl crate::Readable for FIREWALL_VDSSA {}
#[doc = "`write(|w| ..)` method takes [firewall_vdssa::W](firewall_vdssa::W) writer structure"]
impl crate::Writable for FIREWALL_VDSSA {}
#[doc = "Volatile data segment start address"]
pub mod firewall_vdssa;
#[doc = "Volatile data segment length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firewall_vdsl](firewall_vdsl) module"]
pub type FIREWALL_VDSL = crate::Reg<u32, _FIREWALL_VDSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIREWALL_VDSL;
#[doc = "`read()` method returns [firewall_vdsl::R](firewall_vdsl::R) reader structure"]
impl crate::Readable for FIREWALL_VDSL {}
#[doc = "`write(|w| ..)` method takes [firewall_vdsl::W](firewall_vdsl::W) writer structure"]
impl crate::Writable for FIREWALL_VDSL {}
#[doc = "Volatile data segment length"]
pub mod firewall_vdsl;
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firewall_cr](firewall_cr) module"]
pub type FIREWALL_CR = crate::Reg<u32, _FIREWALL_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIREWALL_CR;
#[doc = "`read()` method returns [firewall_cr::R](firewall_cr::R) reader structure"]
impl crate::Readable for FIREWALL_CR {}
#[doc = "`write(|w| ..)` method takes [firewall_cr::W](firewall_cr::W) writer structure"]
impl crate::Writable for FIREWALL_CR {}
#[doc = "Configuration register"]
pub mod firewall_cr;
