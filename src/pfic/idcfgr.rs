#[doc = "Register `IDCFGR` reader"]
pub struct R(crate::R<IDCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIID0` reader - FIID0"]
pub type FIID0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIID1` reader - FIID1"]
pub type FIID1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIID2` reader - FIID2"]
pub type FIID2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIID3` reader - FIID3"]
pub type FIID3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - FIID0"]
    #[inline(always)]
    pub fn fiid0(&self) -> FIID0_R {
        FIID0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - FIID1"]
    #[inline(always)]
    pub fn fiid1(&self) -> FIID1_R {
        FIID1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - FIID2"]
    #[inline(always)]
    pub fn fiid2(&self) -> FIID2_R {
        FIID2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - FIID3"]
    #[inline(always)]
    pub fn fiid3(&self) -> FIID3_R {
        FIID3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "ID Config Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idcfgr](index.html) module"]
pub struct IDCFGR_SPEC;
impl crate::RegisterSpec for IDCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idcfgr::R](R) reader structure"]
impl crate::Readable for IDCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDCFGR to value 0"]
impl crate::Resettable for IDCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}