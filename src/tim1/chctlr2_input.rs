#[doc = "Register `CHCTLR2_Input` reader"]
pub struct R(crate::R<CHCTLR2_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTLR2_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTLR2_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTLR2_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTLR2_Input` writer"]
pub struct W(crate::W<CHCTLR2_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTLR2_INPUT_SPEC>;
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
impl From<crate::W<CHCTLR2_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTLR2_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC4F` reader - Input capture 4 filter"]
pub type IC4F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC4F` writer - Input capture 4 filter"]
pub type IC4F_W<'a> = crate::FieldWriter<'a, u32, CHCTLR2_INPUT_SPEC, u8, u8, 4, 12>;
#[doc = "Field `IC4PSC` reader - Input capture 4 prescaler"]
pub type IC4PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC4PSC` writer - Input capture 4 prescaler"]
pub type IC4PSC_W<'a> = crate::FieldWriter<'a, u32, CHCTLR2_INPUT_SPEC, u8, u8, 2, 10>;
#[doc = "Field `CC4S` reader - Capture/Compare 4 selection"]
pub type CC4S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC4S` writer - Capture/Compare 4 selection"]
pub type CC4S_W<'a> = crate::FieldWriter<'a, u32, CHCTLR2_INPUT_SPEC, u8, u8, 2, 8>;
#[doc = "Field `IC3F` reader - Input capture 3 filter"]
pub type IC3F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC3F` writer - Input capture 3 filter"]
pub type IC3F_W<'a> = crate::FieldWriter<'a, u32, CHCTLR2_INPUT_SPEC, u8, u8, 4, 4>;
#[doc = "Field `IC3PSC` reader - Input capture 3 prescaler"]
pub type IC3PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IC3PSC` writer - Input capture 3 prescaler"]
pub type IC3PSC_W<'a> = crate::FieldWriter<'a, u32, CHCTLR2_INPUT_SPEC, u8, u8, 2, 2>;
#[doc = "Field `CC3S` reader - Capture/compare 3 selection"]
pub type CC3S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC3S` writer - Capture/compare 3 selection"]
pub type CC3S_W<'a> = crate::FieldWriter<'a, u32, CHCTLR2_INPUT_SPEC, u8, u8, 2, 0>;
impl R {
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Capture/compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W {
        IC4F_W::new(self)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W {
        IC4PSC_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W {
        CC4S_W::new(self)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W {
        IC3F_W::new(self)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W {
        IC3PSC_W::new(self)
    }
    #[doc = "Bits 0:1 - Capture/compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W {
        CC3S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 2 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctlr2_input](index.html) module"]
pub struct CHCTLR2_INPUT_SPEC;
impl crate::RegisterSpec for CHCTLR2_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctlr2_input::R](R) reader structure"]
impl crate::Readable for CHCTLR2_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctlr2_input::W](W) writer structure"]
impl crate::Writable for CHCTLR2_INPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCTLR2_Input to value 0"]
impl crate::Resettable for CHCTLR2_INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}