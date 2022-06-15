#[doc = "Register `SAMPTR1` reader"]
pub struct R(crate::R<SAMPTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPTR1` writer"]
pub struct W(crate::W<SAMPTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPTR1_SPEC>;
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
impl From<crate::W<SAMPTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP10` reader - Channel 10 sample time selection"]
pub type SMP10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP10` writer - Channel 10 sample time selection"]
pub type SMP10_W<'a> = crate::FieldWriter<'a, u32, SAMPTR1_SPEC, u8, u8, 3, 0>;
#[doc = "Field `SMP11` reader - Channel 11 sample time selection"]
pub type SMP11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP11` writer - Channel 11 sample time selection"]
pub type SMP11_W<'a> = crate::FieldWriter<'a, u32, SAMPTR1_SPEC, u8, u8, 3, 3>;
#[doc = "Field `SMP12` reader - Channel 12 sample time selection"]
pub type SMP12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP12` writer - Channel 12 sample time selection"]
pub type SMP12_W<'a> = crate::FieldWriter<'a, u32, SAMPTR1_SPEC, u8, u8, 3, 6>;
#[doc = "Field `SMP13` reader - Channel 13 sample time selection"]
pub type SMP13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP13` writer - Channel 13 sample time selection"]
pub type SMP13_W<'a> = crate::FieldWriter<'a, u32, SAMPTR1_SPEC, u8, u8, 3, 9>;
#[doc = "Field `SMP14` reader - Channel 14 sample time selection"]
pub type SMP14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP14` writer - Channel 14 sample time selection"]
pub type SMP14_W<'a> = crate::FieldWriter<'a, u32, SAMPTR1_SPEC, u8, u8, 3, 12>;
#[doc = "Field `SMP15` reader - Channel 15 sample time selection"]
pub type SMP15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP15` writer - Channel 15 sample time selection"]
pub type SMP15_W<'a> = crate::FieldWriter<'a, u32, SAMPTR1_SPEC, u8, u8, 3, 15>;
#[doc = "Field `SMP16` reader - Channel 16 sample time selection"]
pub type SMP16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP16` writer - Channel 16 sample time selection"]
pub type SMP16_W<'a> = crate::FieldWriter<'a, u32, SAMPTR1_SPEC, u8, u8, 3, 18>;
#[doc = "Field `SMP17` reader - Channel 17 sample time selection"]
pub type SMP17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP17` writer - Channel 17 sample time selection"]
pub type SMP17_W<'a> = crate::FieldWriter<'a, u32, SAMPTR1_SPEC, u8, u8, 3, 21>;
impl R {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    pub fn smp10(&mut self) -> SMP10_W {
        SMP10_W::new(self)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    pub fn smp11(&mut self) -> SMP11_W {
        SMP11_W::new(self)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    pub fn smp12(&mut self) -> SMP12_W {
        SMP12_W::new(self)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    pub fn smp13(&mut self) -> SMP13_W {
        SMP13_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    pub fn smp14(&mut self) -> SMP14_W {
        SMP14_W::new(self)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    pub fn smp15(&mut self) -> SMP15_W {
        SMP15_W::new(self)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn smp16(&mut self) -> SMP16_W {
        SMP16_W::new(self)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn smp17(&mut self) -> SMP17_W {
        SMP17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sample time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [samptr1](index.html) module"]
pub struct SAMPTR1_SPEC;
impl crate::RegisterSpec for SAMPTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [samptr1::R](R) reader structure"]
impl crate::Readable for SAMPTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [samptr1::W](W) writer structure"]
impl crate::Writable for SAMPTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMPTR1 to value 0"]
impl crate::Resettable for SAMPTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}