#[doc = "Register `TXMDTR1` reader"]
pub struct R(crate::R<TXMDTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXMDTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXMDTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXMDTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXMDTR1` writer"]
pub struct W(crate::W<TXMDTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXMDTR1_SPEC>;
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
impl From<crate::W<TXMDTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXMDTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME` reader - TIME"]
pub type TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIME` writer - TIME"]
pub type TIME_W<'a> = crate::FieldWriter<'a, u32, TXMDTR1_SPEC, u16, u16, 16, 16>;
#[doc = "Field `TGT` reader - TGT"]
pub type TGT_R = crate::BitReader<bool>;
#[doc = "Field `TGT` writer - TGT"]
pub type TGT_W<'a> = crate::BitWriter<'a, u32, TXMDTR1_SPEC, bool, 8>;
#[doc = "Field `DLC` reader - DLC"]
pub type DLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLC` writer - DLC"]
pub type DLC_W<'a> = crate::FieldWriter<'a, u32, TXMDTR1_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 8 - TGT"]
    #[inline(always)]
    pub fn tgt(&self) -> TGT_R {
        TGT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W {
        TIME_W::new(self)
    }
    #[doc = "Bit 8 - TGT"]
    #[inline(always)]
    pub fn tgt(&mut self) -> TGT_W {
        TGT_W::new(self)
    }
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXMDTR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmdtr1](index.html) module"]
pub struct TXMDTR1_SPEC;
impl crate::RegisterSpec for TXMDTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txmdtr1::R](R) reader structure"]
impl crate::Readable for TXMDTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txmdtr1::W](W) writer structure"]
impl crate::Writable for TXMDTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXMDTR1 to value 0"]
impl crate::Resettable for TXMDTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}