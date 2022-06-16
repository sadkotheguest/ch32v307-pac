#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPA1_EN` reader - OPA1 Enable"]
pub type OPA1_EN_R = crate::BitReader<bool>;
#[doc = "Field `OPA1_EN` writer - OPA1 Enable"]
pub type OPA1_EN_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 0>;
#[doc = "Field `OPA1_MODE` reader - OPA1_MODE"]
pub type OPA1_MODE_R = crate::BitReader<bool>;
#[doc = "Field `OPA1_MODE` writer - OPA1_MODE"]
pub type OPA1_MODE_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 1>;
#[doc = "Field `OPA1_NSEL` reader - OPA1_NSEL"]
pub type OPA1_NSEL_R = crate::BitReader<bool>;
#[doc = "Field `OPA1_NSEL` writer - OPA1_NSEL"]
pub type OPA1_NSEL_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 2>;
#[doc = "Field `OPA1_PSEL` reader - OPA1_PSEL"]
pub type OPA1_PSEL_R = crate::BitReader<bool>;
#[doc = "Field `OPA1_PSEL` writer - OPA1_PSEL"]
pub type OPA1_PSEL_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 3>;
#[doc = "Field `OPA2_EN` reader - OPA2 Enable"]
pub type OPA2_EN_R = crate::BitReader<bool>;
#[doc = "Field `OPA2_EN` writer - OPA2 Enable"]
pub type OPA2_EN_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 4>;
#[doc = "Field `OPA2_MODE` reader - OPA2 MODE"]
pub type OPA2_MODE_R = crate::BitReader<bool>;
#[doc = "Field `OPA2_MODE` writer - OPA2 MODE"]
pub type OPA2_MODE_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 5>;
#[doc = "Field `OPA2_NSEL` reader - OPA2_NSEL"]
pub type OPA2_NSEL_R = crate::BitReader<bool>;
#[doc = "Field `OPA2_NSEL` writer - OPA2_NSEL"]
pub type OPA2_NSEL_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 6>;
#[doc = "Field `OPA2_PSEL` reader - OPA2_PSEL"]
pub type OPA2_PSEL_R = crate::BitReader<bool>;
#[doc = "Field `OPA2_PSEL` writer - OPA2_PSEL"]
pub type OPA2_PSEL_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 7>;
#[doc = "Field `OPA3_EN` reader - OPA3 Eable"]
pub type OPA3_EN_R = crate::BitReader<bool>;
#[doc = "Field `OPA3_EN` writer - OPA3 Eable"]
pub type OPA3_EN_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 8>;
#[doc = "Field `OPA3_MODE` reader - OPA3 MODE"]
pub type OPA3_MODE_R = crate::BitReader<bool>;
#[doc = "Field `OPA3_MODE` writer - OPA3 MODE"]
pub type OPA3_MODE_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 9>;
#[doc = "Field `OPA3_NSEL` reader - OPA3 NSEL"]
pub type OPA3_NSEL_R = crate::BitReader<bool>;
#[doc = "Field `OPA3_NSEL` writer - OPA3 NSEL"]
pub type OPA3_NSEL_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 10>;
#[doc = "Field `OPA3_PSEL` reader - OPA3 PSEL"]
pub type OPA3_PSEL_R = crate::BitReader<bool>;
#[doc = "Field `OPA3_PSEL` writer - OPA3 PSEL"]
pub type OPA3_PSEL_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 11>;
#[doc = "Field `OPA4_EN` reader - OPA4 Enable"]
pub type OPA4_EN_R = crate::BitReader<bool>;
#[doc = "Field `OPA4_EN` writer - OPA4 Enable"]
pub type OPA4_EN_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 12>;
#[doc = "Field `OPA4_MODE` reader - OPA4_MODE"]
pub type OPA4_MODE_R = crate::BitReader<bool>;
#[doc = "Field `OPA4_MODE` writer - OPA4_MODE"]
pub type OPA4_MODE_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 13>;
#[doc = "Field `OPA4_NSEL` reader - OPA4_NSEL"]
pub type OPA4_NSEL_R = crate::BitReader<bool>;
#[doc = "Field `OPA4_NSEL` writer - OPA4_NSEL"]
pub type OPA4_NSEL_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 14>;
#[doc = "Field `OPA4_PSEL` reader - OPA4_PSEL"]
pub type OPA4_PSEL_R = crate::BitReader<bool>;
#[doc = "Field `OPA4_PSEL` writer - OPA4_PSEL"]
pub type OPA4_PSEL_W<'a> = crate::BitWriter<'a, u32, CR_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - OPA1 Enable"]
    #[inline(always)]
    pub fn opa1_en(&self) -> OPA1_EN_R {
        OPA1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPA1_MODE"]
    #[inline(always)]
    pub fn opa1_mode(&self) -> OPA1_MODE_R {
        OPA1_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OPA1_NSEL"]
    #[inline(always)]
    pub fn opa1_nsel(&self) -> OPA1_NSEL_R {
        OPA1_NSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OPA1_PSEL"]
    #[inline(always)]
    pub fn opa1_psel(&self) -> OPA1_PSEL_R {
        OPA1_PSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OPA2 Enable"]
    #[inline(always)]
    pub fn opa2_en(&self) -> OPA2_EN_R {
        OPA2_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OPA2 MODE"]
    #[inline(always)]
    pub fn opa2_mode(&self) -> OPA2_MODE_R {
        OPA2_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OPA2_NSEL"]
    #[inline(always)]
    pub fn opa2_nsel(&self) -> OPA2_NSEL_R {
        OPA2_NSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OPA2_PSEL"]
    #[inline(always)]
    pub fn opa2_psel(&self) -> OPA2_PSEL_R {
        OPA2_PSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPA3 Eable"]
    #[inline(always)]
    pub fn opa3_en(&self) -> OPA3_EN_R {
        OPA3_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OPA3 MODE"]
    #[inline(always)]
    pub fn opa3_mode(&self) -> OPA3_MODE_R {
        OPA3_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OPA3 NSEL"]
    #[inline(always)]
    pub fn opa3_nsel(&self) -> OPA3_NSEL_R {
        OPA3_NSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OPA3 PSEL"]
    #[inline(always)]
    pub fn opa3_psel(&self) -> OPA3_PSEL_R {
        OPA3_PSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OPA4 Enable"]
    #[inline(always)]
    pub fn opa4_en(&self) -> OPA4_EN_R {
        OPA4_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OPA4_MODE"]
    #[inline(always)]
    pub fn opa4_mode(&self) -> OPA4_MODE_R {
        OPA4_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OPA4_NSEL"]
    #[inline(always)]
    pub fn opa4_nsel(&self) -> OPA4_NSEL_R {
        OPA4_NSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OPA4_PSEL"]
    #[inline(always)]
    pub fn opa4_psel(&self) -> OPA4_PSEL_R {
        OPA4_PSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPA1 Enable"]
    #[inline(always)]
    pub fn opa1_en(&mut self) -> OPA1_EN_W {
        OPA1_EN_W::new(self)
    }
    #[doc = "Bit 1 - OPA1_MODE"]
    #[inline(always)]
    pub fn opa1_mode(&mut self) -> OPA1_MODE_W {
        OPA1_MODE_W::new(self)
    }
    #[doc = "Bit 2 - OPA1_NSEL"]
    #[inline(always)]
    pub fn opa1_nsel(&mut self) -> OPA1_NSEL_W {
        OPA1_NSEL_W::new(self)
    }
    #[doc = "Bit 3 - OPA1_PSEL"]
    #[inline(always)]
    pub fn opa1_psel(&mut self) -> OPA1_PSEL_W {
        OPA1_PSEL_W::new(self)
    }
    #[doc = "Bit 4 - OPA2 Enable"]
    #[inline(always)]
    pub fn opa2_en(&mut self) -> OPA2_EN_W {
        OPA2_EN_W::new(self)
    }
    #[doc = "Bit 5 - OPA2 MODE"]
    #[inline(always)]
    pub fn opa2_mode(&mut self) -> OPA2_MODE_W {
        OPA2_MODE_W::new(self)
    }
    #[doc = "Bit 6 - OPA2_NSEL"]
    #[inline(always)]
    pub fn opa2_nsel(&mut self) -> OPA2_NSEL_W {
        OPA2_NSEL_W::new(self)
    }
    #[doc = "Bit 7 - OPA2_PSEL"]
    #[inline(always)]
    pub fn opa2_psel(&mut self) -> OPA2_PSEL_W {
        OPA2_PSEL_W::new(self)
    }
    #[doc = "Bit 8 - OPA3 Eable"]
    #[inline(always)]
    pub fn opa3_en(&mut self) -> OPA3_EN_W {
        OPA3_EN_W::new(self)
    }
    #[doc = "Bit 9 - OPA3 MODE"]
    #[inline(always)]
    pub fn opa3_mode(&mut self) -> OPA3_MODE_W {
        OPA3_MODE_W::new(self)
    }
    #[doc = "Bit 10 - OPA3 NSEL"]
    #[inline(always)]
    pub fn opa3_nsel(&mut self) -> OPA3_NSEL_W {
        OPA3_NSEL_W::new(self)
    }
    #[doc = "Bit 11 - OPA3 PSEL"]
    #[inline(always)]
    pub fn opa3_psel(&mut self) -> OPA3_PSEL_W {
        OPA3_PSEL_W::new(self)
    }
    #[doc = "Bit 12 - OPA4 Enable"]
    #[inline(always)]
    pub fn opa4_en(&mut self) -> OPA4_EN_W {
        OPA4_EN_W::new(self)
    }
    #[doc = "Bit 13 - OPA4_MODE"]
    #[inline(always)]
    pub fn opa4_mode(&mut self) -> OPA4_MODE_W {
        OPA4_MODE_W::new(self)
    }
    #[doc = "Bit 14 - OPA4_NSEL"]
    #[inline(always)]
    pub fn opa4_nsel(&mut self) -> OPA4_NSEL_W {
        OPA4_NSEL_W::new(self)
    }
    #[doc = "Bit 15 - OPA4_PSEL"]
    #[inline(always)]
    pub fn opa4_psel(&mut self) -> OPA4_PSEL_W {
        OPA4_PSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x20"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}