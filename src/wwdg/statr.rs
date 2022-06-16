#[doc = "Register `STATR` reader"]
pub struct R(crate::R<STATR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATR` writer"]
pub struct W(crate::W<STATR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATR_SPEC>;
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
impl From<crate::W<STATR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WEIF` reader - Early Wakeup Interrupt Flag"]
pub type WEIF_R = crate::BitReader<bool>;
#[doc = "Field `WEIF` writer - Early Wakeup Interrupt Flag"]
pub type WEIF_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Early Wakeup Interrupt Flag"]
    #[inline(always)]
    pub fn weif(&self) -> WEIF_R {
        WEIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Early Wakeup Interrupt Flag"]
    #[inline(always)]
    pub fn weif(&mut self) -> WEIF_W {
        WEIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register (WWDG_SR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statr](index.html) module"]
pub struct STATR_SPEC;
impl crate::RegisterSpec for STATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statr::R](R) reader structure"]
impl crate::Readable for STATR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [statr::W](W) writer structure"]
impl crate::Writable for STATR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATR to value 0"]
impl crate::Resettable for STATR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}