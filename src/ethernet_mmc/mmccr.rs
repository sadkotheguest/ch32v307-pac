#[doc = "Register `MMCCR` reader"]
pub struct R(crate::R<MMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCCR` writer"]
pub struct W(crate::W<MMCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCCR_SPEC>;
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
impl From<crate::W<MMCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR` reader - Counter reset"]
pub type CR_R = crate::BitReader<bool>;
#[doc = "Field `CR` writer - Counter reset"]
pub type CR_W<'a> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, 0>;
#[doc = "Field `CSR` reader - Counter stop rollover"]
pub type CSR_R = crate::BitReader<bool>;
#[doc = "Field `CSR` writer - Counter stop rollover"]
pub type CSR_W<'a> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, 1>;
#[doc = "Field `ROR` reader - Reset on read"]
pub type ROR_R = crate::BitReader<bool>;
#[doc = "Field `ROR` writer - Reset on read"]
pub type ROR_W<'a> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, 2>;
#[doc = "Field `MCF` reader - MMC counter freeze"]
pub type MCF_R = crate::BitReader<bool>;
#[doc = "Field `MCF` writer - MMC counter freeze"]
pub type MCF_W<'a> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - MMC counter freeze"]
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W::new(self)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W {
        CSR_W::new(self)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn ror(&mut self) -> ROR_W {
        ROR_W::new(self)
    }
    #[doc = "Bit 31 - MMC counter freeze"]
    #[inline(always)]
    pub fn mcf(&mut self) -> MCF_W {
        MCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MMC control register (ETH_MMCCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmccr](index.html) module"]
pub struct MMCCR_SPEC;
impl crate::RegisterSpec for MMCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmccr::R](R) reader structure"]
impl crate::Readable for MMCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmccr::W](W) writer structure"]
impl crate::Writable for MMCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCCR to value 0"]
impl crate::Resettable for MMCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}