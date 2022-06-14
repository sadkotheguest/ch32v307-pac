#[doc = "Register `OADDR2` reader"]
pub struct R(crate::R<OADDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OADDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OADDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OADDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OADDR2` writer"]
pub struct W(crate::W<OADDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OADDR2_SPEC>;
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
impl From<crate::W<OADDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OADDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD2` reader - Interface address"]
pub type ADD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD2` writer - Interface address"]
pub type ADD2_W<'a> = crate::FieldWriter<'a, u32, OADDR2_SPEC, u8, u8, 7, 1>;
#[doc = "Field `ENDUAL` reader - Dual addressing mode enable"]
pub type ENDUAL_R = crate::BitReader<bool>;
#[doc = "Field `ENDUAL` writer - Dual addressing mode enable"]
pub type ENDUAL_W<'a> = crate::BitWriter<'a, u32, OADDR2_SPEC, bool, 0>;
impl R {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add2(&self) -> ADD2_R {
        ADD2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    pub fn endual(&self) -> ENDUAL_R {
        ENDUAL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add2(&mut self) -> ADD2_W {
        ADD2_W::new(self)
    }
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    pub fn endual(&mut self) -> ENDUAL_W {
        ENDUAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Own address register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oaddr2](index.html) module"]
pub struct OADDR2_SPEC;
impl crate::RegisterSpec for OADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oaddr2::R](R) reader structure"]
impl crate::Readable for OADDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oaddr2::W](W) writer structure"]
impl crate::Writable for OADDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OADDR2 to value 0"]
impl crate::Resettable for OADDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}