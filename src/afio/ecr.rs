#[doc = "Register `ECR` reader"]
pub struct R(crate::R<ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECR` writer"]
pub struct W(crate::W<ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECR_SPEC>;
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
impl From<crate::W<ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN` reader - Pin selection"]
pub type PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN` writer - Pin selection"]
pub type PIN_W<'a> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 4, 0>;
#[doc = "Field `PORT` reader - Port selection"]
pub type PORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT` writer - Port selection"]
pub type PORT_W<'a> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 3, 4>;
#[doc = "Field `EVOE` reader - Event Output Enable"]
pub type EVOE_R = crate::BitReader<bool>;
#[doc = "Field `EVOE` writer - Event Output Enable"]
pub type EVOE_W<'a> = crate::BitWriter<'a, u32, ECR_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:3 - Pin selection"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Port selection"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EVOE_R {
        EVOE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin selection"]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W {
        PIN_W::new(self)
    }
    #[doc = "Bits 4:6 - Port selection"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W::new(self)
    }
    #[doc = "Bit 7 - Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&mut self) -> EVOE_W {
        EVOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control Register (AFIO_ECR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](index.html) module"]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecr::R](R) reader structure"]
impl crate::Readable for ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecr::W](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}