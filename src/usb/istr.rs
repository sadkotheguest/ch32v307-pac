#[doc = "Register `ISTR` reader"]
pub struct R(crate::R<ISTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISTR` writer"]
pub struct W(crate::W<ISTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISTR_SPEC>;
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
impl From<crate::W<ISTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP_ID` reader - Endpoint Identifier"]
pub type EP_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP_ID` writer - Endpoint Identifier"]
pub type EP_ID_W<'a> = crate::FieldWriter<'a, u32, ISTR_SPEC, u8, u8, 4, 0>;
#[doc = "Field `DIR` reader - Direction of transaction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Direction of transaction"]
pub type DIR_W<'a> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, 4>;
#[doc = "Field `ESOF` reader - Expected start frame"]
pub type ESOF_R = crate::BitReader<bool>;
#[doc = "Field `ESOF` writer - Expected start frame"]
pub type ESOF_W<'a> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, 8>;
#[doc = "Field `SOF` reader - start of frame"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` writer - start of frame"]
pub type SOF_W<'a> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, 9>;
#[doc = "Field `RESET` reader - reset request"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - reset request"]
pub type RESET_W<'a> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, 10>;
#[doc = "Field `SUSP` reader - Suspend mode request"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` writer - Suspend mode request"]
pub type SUSP_W<'a> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, 11>;
#[doc = "Field `WKUP` reader - Wakeup"]
pub type WKUP_R = crate::BitReader<bool>;
#[doc = "Field `WKUP` writer - Wakeup"]
pub type WKUP_W<'a> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, 12>;
#[doc = "Field `ERR` reader - Error"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - Error"]
pub type ERR_W<'a> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, 13>;
#[doc = "Field `PMAOVR` reader - Packet memory area over / underrun"]
pub type PMAOVR_R = crate::BitReader<bool>;
#[doc = "Field `PMAOVR` writer - Packet memory area over / underrun"]
pub type PMAOVR_W<'a> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, 14>;
#[doc = "Field `CTR` reader - Correct transfer"]
pub type CTR_R = crate::BitReader<bool>;
#[doc = "Field `CTR` writer - Correct transfer"]
pub type CTR_W<'a> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline(always)]
    pub fn ep_id(&self) -> EP_ID_R {
        EP_ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start frame"]
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reset request"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode request"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun"]
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline(always)]
    pub fn ep_id(&mut self) -> EP_ID_W {
        EP_ID_W::new(self)
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W::new(self)
    }
    #[doc = "Bit 8 - Expected start frame"]
    #[inline(always)]
    pub fn esof(&mut self) -> ESOF_W {
        ESOF_W::new(self)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W::new(self)
    }
    #[doc = "Bit 10 - reset request"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W::new(self)
    }
    #[doc = "Bit 11 - Suspend mode request"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wkup(&mut self) -> WKUP_W {
        WKUP_W::new(self)
    }
    #[doc = "Bit 13 - Error"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W::new(self)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun"]
    #[inline(always)]
    pub fn pmaovr(&mut self) -> PMAOVR_W {
        PMAOVR_W::new(self)
    }
    #[doc = "Bit 15 - Correct transfer"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CTR_W {
        CTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istr](index.html) module"]
pub struct ISTR_SPEC;
impl crate::RegisterSpec for ISTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [istr::R](R) reader structure"]
impl crate::Readable for ISTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [istr::W](W) writer structure"]
impl crate::Writable for ISTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISTR to value 0"]
impl crate::Resettable for ISTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}