#[doc = "Register `PATT2` reader"]
pub struct R(crate::R<PATT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PATT2` writer"]
pub struct W(crate::W<PATT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATT2_SPEC>;
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
impl From<crate::W<PATT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTHIZx` reader - Attribute memory x databus HiZ time"]
pub type ATTHIZX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTHIZx` writer - Attribute memory x databus HiZ time"]
pub type ATTHIZX_W<'a> = crate::FieldWriter<'a, u32, PATT2_SPEC, u8, u8, 8, 24>;
#[doc = "Field `ATTHOLDx` reader - Attribute memory x hold time"]
pub type ATTHOLDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTHOLDx` writer - Attribute memory x hold time"]
pub type ATTHOLDX_W<'a> = crate::FieldWriter<'a, u32, PATT2_SPEC, u8, u8, 8, 16>;
#[doc = "Field `ATTWAITx` reader - Attribute memory x wait time"]
pub type ATTWAITX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTWAITx` writer - Attribute memory x wait time"]
pub type ATTWAITX_W<'a> = crate::FieldWriter<'a, u32, PATT2_SPEC, u8, u8, 8, 8>;
#[doc = "Field `ATTSETx` reader - Attribute memory x setup time"]
pub type ATTSETX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTSETx` writer - Attribute memory x setup time"]
pub type ATTSETX_W<'a> = crate::FieldWriter<'a, u32, PATT2_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 24:31 - Attribute memory x databus HiZ time"]
    #[inline(always)]
    pub fn atthizx(&self) -> ATTHIZX_R {
        ATTHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Attribute memory x hold time"]
    #[inline(always)]
    pub fn attholdx(&self) -> ATTHOLDX_R {
        ATTHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Attribute memory x wait time"]
    #[inline(always)]
    pub fn attwaitx(&self) -> ATTWAITX_R {
        ATTWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Attribute memory x setup time"]
    #[inline(always)]
    pub fn attsetx(&self) -> ATTSETX_R {
        ATTSETX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Attribute memory x databus HiZ time"]
    #[inline(always)]
    pub fn atthizx(&mut self) -> ATTHIZX_W {
        ATTHIZX_W::new(self)
    }
    #[doc = "Bits 16:23 - Attribute memory x hold time"]
    #[inline(always)]
    pub fn attholdx(&mut self) -> ATTHOLDX_W {
        ATTHOLDX_W::new(self)
    }
    #[doc = "Bits 8:15 - Attribute memory x wait time"]
    #[inline(always)]
    pub fn attwaitx(&mut self) -> ATTWAITX_W {
        ATTWAITX_W::new(self)
    }
    #[doc = "Bits 0:7 - Attribute memory x setup time"]
    #[inline(always)]
    pub fn attsetx(&mut self) -> ATTSETX_W {
        ATTSETX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Attribute memory space timing register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt2](index.html) module"]
pub struct PATT2_SPEC;
impl crate::RegisterSpec for PATT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [patt2::R](R) reader structure"]
impl crate::Readable for PATT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [patt2::W](W) writer structure"]
impl crate::Writable for PATT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PATT2 to value 0xfcfc_fcfc"]
impl crate::Resettable for PATT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}