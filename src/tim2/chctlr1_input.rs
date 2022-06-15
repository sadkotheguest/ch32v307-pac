#[doc = "Register `CHCTLR1_Input` reader"]
pub struct R(crate::R<CHCTLR1_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTLR1_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTLR1_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTLR1_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTLR1_Input` writer"]
pub struct W(crate::W<CHCTLR1_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTLR1_INPUT_SPEC>;
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
impl From<crate::W<CHCTLR1_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTLR1_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC2F` reader - Input capture 2 filter"]
pub type IC2F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC2F` writer - Input capture 2 filter"]
pub type IC2F_W<'a> = crate::FieldWriter<'a, u32, CHCTLR1_INPUT_SPEC, u8, u8, 4, 12>;
#[doc = "Field `IC2PSC` reader - Input capture 2 prescaler"]
pub type IC2PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC2PSC` writer - Input capture 2 prescaler"]
pub type IC2PSC_W<'a> = crate::FieldWriter<'a, u32, CHCTLR1_INPUT_SPEC, u8, u8, 2, 10>;
#[doc = "Field `CC2S` reader - Capture/compare 2 selection"]
pub type CC2S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC2S` writer - Capture/compare 2 selection"]
pub type CC2S_W<'a> = crate::FieldWriter<'a, u32, CHCTLR1_INPUT_SPEC, u8, u8, 2, 8>;
#[doc = "Field `IC1F` reader - Input capture 1 filter"]
pub type IC1F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC1F` writer - Input capture 1 filter"]
pub type IC1F_W<'a> = crate::FieldWriter<'a, u32, CHCTLR1_INPUT_SPEC, u8, u8, 4, 4>;
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler"]
pub type IC1PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler"]
pub type IC1PSC_W<'a> = crate::FieldWriter<'a, u32, CHCTLR1_INPUT_SPEC, u8, u8, 2, 2>;
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection"]
pub type CC1S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection"]
pub type CC1S_W<'a> = crate::FieldWriter<'a, u32, CHCTLR1_INPUT_SPEC, u8, u8, 2, 0>;
impl R {
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Capture/compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    pub fn ic2f(&mut self) -> IC2F_W {
        IC2F_W::new(self)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn ic2psc(&mut self) -> IC2PSC_W {
        IC2PSC_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W {
        CC2S_W::new(self)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W {
        IC1F_W::new(self)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W {
        IC1PSC_W::new(self)
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 1 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctlr1_input](index.html) module"]
pub struct CHCTLR1_INPUT_SPEC;
impl crate::RegisterSpec for CHCTLR1_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctlr1_input::R](R) reader structure"]
impl crate::Readable for CHCTLR1_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctlr1_input::W](W) writer structure"]
impl crate::Writable for CHCTLR1_INPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCTLR1_Input to value 0"]
impl crate::Resettable for CHCTLR1_INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}