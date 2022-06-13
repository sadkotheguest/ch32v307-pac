#[doc = "Register `TXMIR1` reader"]
pub struct R(crate::R<TXMIR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXMIR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXMIR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXMIR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXMIR1` writer"]
pub struct W(crate::W<TXMIR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXMIR1_SPEC>;
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
impl From<crate::W<TXMIR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXMIR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STID` reader - STID"]
pub type STID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STID` writer - STID"]
pub type STID_W<'a> = crate::FieldWriter<'a, u32, TXMIR1_SPEC, u16, u16, 11, 21>;
#[doc = "Field `EXID` reader - EXID"]
pub type EXID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EXID` writer - EXID"]
pub type EXID_W<'a> = crate::FieldWriter<'a, u32, TXMIR1_SPEC, u32, u32, 18, 3>;
#[doc = "Field `IDE` reader - IDE"]
pub type IDE_R = crate::BitReader<bool>;
#[doc = "Field `IDE` writer - IDE"]
pub type IDE_W<'a> = crate::BitWriter<'a, u32, TXMIR1_SPEC, bool, 2>;
#[doc = "Field `RTR` reader - RTR"]
pub type RTR_R = crate::BitReader<bool>;
#[doc = "Field `RTR` writer - RTR"]
pub type RTR_W<'a> = crate::BitWriter<'a, u32, TXMIR1_SPEC, bool, 1>;
#[doc = "Field `TXRQ` reader - TXRQ"]
pub type TXRQ_R = crate::BitReader<bool>;
#[doc = "Field `TXRQ` writer - TXRQ"]
pub type TXRQ_W<'a> = crate::BitWriter<'a, u32, TXMIR1_SPEC, bool, 0>;
impl R {
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&mut self) -> STID_W {
        STID_W::new(self)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&mut self) -> EXID_W {
        EXID_W::new(self)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W {
        IDE_W::new(self)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&mut self) -> RTR_W {
        RTR_W::new(self)
    }
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&mut self) -> TXRQ_W {
        TXRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXMIR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmir1](index.html) module"]
pub struct TXMIR1_SPEC;
impl crate::RegisterSpec for TXMIR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txmir1::R](R) reader structure"]
impl crate::Readable for TXMIR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txmir1::W](W) writer structure"]
impl crate::Writable for TXMIR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXMIR1 to value 0"]
impl crate::Resettable for TXMIR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}