#[doc = "Register `TPCSR` reader"]
pub struct R(crate::R<TPCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPCSR` writer"]
pub struct W(crate::W<TPCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPCSR_SPEC>;
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
impl From<crate::W<TPCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTE` writer - Clear Tamper event"]
pub type CTE_W<'a> = crate::BitWriter<'a, u32, TPCSR_SPEC, bool, 0>;
#[doc = "Field `CTI` writer - Clear Tamper Interrupt"]
pub type CTI_W<'a> = crate::BitWriter<'a, u32, TPCSR_SPEC, bool, 1>;
#[doc = "Field `TPIE` reader - Tamper Pin interrupt enable"]
pub type TPIE_R = crate::BitReader<bool>;
#[doc = "Field `TPIE` writer - Tamper Pin interrupt enable"]
pub type TPIE_W<'a> = crate::BitWriter<'a, u32, TPCSR_SPEC, bool, 2>;
#[doc = "Field `TEF` reader - Tamper Event Flag"]
pub type TEF_R = crate::BitReader<bool>;
#[doc = "Field `TIF` reader - Tamper Interrupt Flag"]
pub type TIF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 2 - Tamper Pin interrupt enable"]
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper Event Flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper Interrupt Flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Tamper event"]
    #[inline(always)]
    pub fn cte(&mut self) -> CTE_W {
        CTE_W::new(self)
    }
    #[doc = "Bit 1 - Clear Tamper Interrupt"]
    #[inline(always)]
    pub fn cti(&mut self) -> CTI_W {
        CTI_W::new(self)
    }
    #[doc = "Bit 2 - Tamper Pin interrupt enable"]
    #[inline(always)]
    pub fn tpie(&mut self) -> TPIE_W {
        TPIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BKP_TPCSR control/status register (BKP_CSR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpcsr](index.html) module"]
pub struct TPCSR_SPEC;
impl crate::RegisterSpec for TPCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpcsr::R](R) reader structure"]
impl crate::Readable for TPCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpcsr::W](W) writer structure"]
impl crate::Writable for TPCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPCSR to value 0"]
impl crate::Resettable for TPCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}