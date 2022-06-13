#[doc = "Register `OBR` reader"]
pub struct R(crate::R<OBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OBERR` reader - Option byte error"]
pub type OBERR_R = crate::BitReader<bool>;
#[doc = "Field `RDPRT` reader - Read protection"]
pub type RDPRT_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_SW` reader - IWDG_SW"]
pub type IWDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `STOP_RST` reader - STOP_RST"]
pub type STOP_RST_R = crate::BitReader<bool>;
#[doc = "Field `STANDY_RST` reader - STANDY_RST"]
pub type STANDY_RST_R = crate::BitReader<bool>;
#[doc = "Field `USBD_MODE` reader - USBD_MODE"]
pub type USBD_MODE_R = crate::BitReader<bool>;
#[doc = "Field `USBD_PU` reader - USBD_PU"]
pub type USBD_PU_R = crate::BitReader<bool>;
#[doc = "Field `POR_CTR` reader - POR_CTR"]
pub type POR_CTR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn oberr(&self) -> OBERR_R {
        OBERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read protection"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IWDG_SW"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP_RST"]
    #[inline(always)]
    pub fn stop_rst(&self) -> STOP_RST_R {
        STOP_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STANDY_RST"]
    #[inline(always)]
    pub fn standy_rst(&self) -> STANDY_RST_R {
        STANDY_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USBD_MODE"]
    #[inline(always)]
    pub fn usbd_mode(&self) -> USBD_MODE_R {
        USBD_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USBD_PU"]
    #[inline(always)]
    pub fn usbd_pu(&self) -> USBD_PU_R {
        USBD_PU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - POR_CTR"]
    #[inline(always)]
    pub fn por_ctr(&self) -> POR_CTR_R {
        POR_CTR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Option byte register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obr](index.html) module"]
pub struct OBR_SPEC;
impl crate::RegisterSpec for OBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [obr::R](R) reader structure"]
impl crate::Readable for OBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OBR to value 0x03ff_fffc"]
impl crate::Resettable for OBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff_fffc
    }
}