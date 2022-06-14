#[doc = "Register `PCR4` reader"]
pub struct R(crate::R<PCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR4` writer"]
pub struct W(crate::W<PCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR4_SPEC>;
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
impl From<crate::W<PCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCPS` reader - ECCPS"]
pub type ECCPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECCPS` writer - ECCPS"]
pub type ECCPS_W<'a> = crate::FieldWriter<'a, u32, PCR4_SPEC, u8, u8, 3, 17>;
#[doc = "Field `TAR` reader - TAR"]
pub type TAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAR` writer - TAR"]
pub type TAR_W<'a> = crate::FieldWriter<'a, u32, PCR4_SPEC, u8, u8, 4, 13>;
#[doc = "Field `TCLR` reader - TCLR"]
pub type TCLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCLR` writer - TCLR"]
pub type TCLR_W<'a> = crate::FieldWriter<'a, u32, PCR4_SPEC, u8, u8, 4, 9>;
#[doc = "Field `ECCEN` reader - ECCEN"]
pub type ECCEN_R = crate::BitReader<bool>;
#[doc = "Field `ECCEN` writer - ECCEN"]
pub type ECCEN_W<'a> = crate::BitWriter<'a, u32, PCR4_SPEC, bool, 6>;
#[doc = "Field `PWID` reader - PWID"]
pub type PWID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWID` writer - PWID"]
pub type PWID_W<'a> = crate::FieldWriter<'a, u32, PCR4_SPEC, u8, u8, 2, 4>;
#[doc = "Field `PTYP` reader - PTYP"]
pub type PTYP_R = crate::BitReader<bool>;
#[doc = "Field `PTYP` writer - PTYP"]
pub type PTYP_W<'a> = crate::BitWriter<'a, u32, PCR4_SPEC, bool, 3>;
#[doc = "Field `PBKEN` reader - PBKEN"]
pub type PBKEN_R = crate::BitReader<bool>;
#[doc = "Field `PBKEN` writer - PBKEN"]
pub type PBKEN_W<'a> = crate::BitWriter<'a, u32, PCR4_SPEC, bool, 2>;
#[doc = "Field `PWAITEN` reader - PWAITEN"]
pub type PWAITEN_R = crate::BitReader<bool>;
#[doc = "Field `PWAITEN` writer - PWAITEN"]
pub type PWAITEN_W<'a> = crate::BitWriter<'a, u32, PCR4_SPEC, bool, 1>;
impl R {
    #[doc = "Bits 17:19 - ECCPS"]
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 3 - PTYP"]
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 17:19 - ECCPS"]
    #[inline(always)]
    pub fn eccps(&mut self) -> ECCPS_W {
        ECCPS_W::new(self)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W {
        TAR_W::new(self)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W {
        TCLR_W::new(self)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W {
        ECCEN_W::new(self)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&mut self) -> PWID_W {
        PWID_W::new(self)
    }
    #[doc = "Bit 3 - PTYP"]
    #[inline(always)]
    pub fn ptyp(&mut self) -> PTYP_W {
        PTYP_W::new(self)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&mut self) -> PBKEN_W {
        PBKEN_W::new(self)
    }
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PWAITEN_W {
        PWAITEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PC Card/NAND Flash control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr4](index.html) module"]
pub struct PCR4_SPEC;
impl crate::RegisterSpec for PCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr4::R](R) reader structure"]
impl crate::Readable for PCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr4::W](W) writer structure"]
impl crate::Writable for PCR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCR4 to value 0x18"]
impl crate::Resettable for PCR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x18
    }
}