#[doc = "Register `BRR` reader"]
pub struct R(crate::R<BRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRR` writer"]
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_M` reader - mantissa of USARTDIV"]
pub type DIV_M_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIV_M` writer - mantissa of USARTDIV"]
pub type DIV_M_W<'a> = crate::FieldWriter<'a, u32, BRR_SPEC, u16, u16, 12, 4>;
#[doc = "Field `DIV_F` reader - fraction of USARTDIV"]
pub type DIV_F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV_F` writer - fraction of USARTDIV"]
pub type DIV_F_W<'a> = crate::FieldWriter<'a, u32, BRR_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 4:15 - mantissa of USARTDIV"]
    #[inline(always)]
    pub fn div_m(&self) -> DIV_M_R {
        DIV_M_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - fraction of USARTDIV"]
    #[inline(always)]
    pub fn div_f(&self) -> DIV_F_R {
        DIV_F_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:15 - mantissa of USARTDIV"]
    #[inline(always)]
    pub fn div_m(&mut self) -> DIV_M_W {
        DIV_M_W::new(self)
    }
    #[doc = "Bits 0:3 - fraction of USARTDIV"]
    #[inline(always)]
    pub fn div_f(&mut self) -> DIV_F_W {
        DIV_F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud rate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](index.html) module"]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brr::R](R) reader structure"]
impl crate::Readable for BRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brr::W](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}