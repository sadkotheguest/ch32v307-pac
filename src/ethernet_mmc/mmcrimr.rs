#[doc = "Register `MMCRIMR` reader"]
pub struct R(crate::R<MMCRIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCRIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCRIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCRIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCRIMR` writer"]
pub struct W(crate::W<MMCRIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCRIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MMCRIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCRIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFCEM` reader - Received frame CRC error mask"]
pub type RFCEM_R = crate::BitReader<bool>;
#[doc = "Field `RFCEM` writer - Received frame CRC error mask"]
pub type RFCEM_W<'a> = crate::BitWriter<'a, u32, MMCRIMR_SPEC, bool, 5>;
#[doc = "Field `RFAEM` reader - Received frames alignment error mask"]
pub type RFAEM_R = crate::BitReader<bool>;
#[doc = "Field `RFAEM` writer - Received frames alignment error mask"]
pub type RFAEM_W<'a> = crate::BitWriter<'a, u32, MMCRIMR_SPEC, bool, 6>;
#[doc = "Field `RGUFM` reader - Received good unicast frames mask"]
pub type RGUFM_R = crate::BitReader<bool>;
#[doc = "Field `RGUFM` writer - Received good unicast frames mask"]
pub type RGUFM_W<'a> = crate::BitWriter<'a, u32, MMCRIMR_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    pub fn rfcem(&self) -> RFCEM_R {
        RFCEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    pub fn rfaem(&self) -> RFAEM_R {
        RFAEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received good unicast frames mask"]
    #[inline(always)]
    pub fn rgufm(&self) -> RGUFM_R {
        RGUFM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Received frame CRC error mask"]
    #[inline(always)]
    pub fn rfcem(&mut self) -> RFCEM_W {
        RFCEM_W::new(self)
    }
    #[doc = "Bit 6 - Received frames alignment error mask"]
    #[inline(always)]
    pub fn rfaem(&mut self) -> RFAEM_W {
        RFAEM_W::new(self)
    }
    #[doc = "Bit 17 - Received good unicast frames mask"]
    #[inline(always)]
    pub fn rgufm(&mut self) -> RGUFM_W {
        RGUFM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MMC receive interrupt mask register (ETH_MMCRIMR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrimr](index.html) module"]
pub struct MMCRIMR_SPEC;
impl crate::RegisterSpec for MMCRIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmcrimr::R](R) reader structure"]
impl crate::Readable for MMCRIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmcrimr::W](W) writer structure"]
impl crate::Writable for MMCRIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCRIMR to value 0"]
impl crate::Resettable for MMCRIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}