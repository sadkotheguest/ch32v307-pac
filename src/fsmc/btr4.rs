#[doc = "Register `BTR4` reader"]
pub struct R(crate::R<BTR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTR4` writer"]
pub struct W(crate::W<BTR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTR4_SPEC>;
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
impl From<crate::W<BTR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACCMOD` reader - ACCMOD"]
pub type ACCMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACCMOD` writer - ACCMOD"]
pub type ACCMOD_W<'a> = crate::FieldWriter<'a, u32, BTR4_SPEC, u8, u8, 2, 28>;
#[doc = "Field `DATLAT` reader - DATLAT"]
pub type DATLAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATLAT` writer - DATLAT"]
pub type DATLAT_W<'a> = crate::FieldWriter<'a, u32, BTR4_SPEC, u8, u8, 4, 24>;
#[doc = "Field `CLKDIV` reader - CLKDIV"]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - CLKDIV"]
pub type CLKDIV_W<'a> = crate::FieldWriter<'a, u32, BTR4_SPEC, u8, u8, 4, 20>;
#[doc = "Field `BUSTURN` reader - BUSTURN"]
pub type BUSTURN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSTURN` writer - BUSTURN"]
pub type BUSTURN_W<'a> = crate::FieldWriter<'a, u32, BTR4_SPEC, u8, u8, 4, 16>;
#[doc = "Field `DATAST` reader - DATAST"]
pub type DATAST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAST` writer - DATAST"]
pub type DATAST_W<'a> = crate::FieldWriter<'a, u32, BTR4_SPEC, u8, u8, 8, 8>;
#[doc = "Field `ADDHLD` reader - ADDHLD"]
pub type ADDHLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDHLD` writer - ADDHLD"]
pub type ADDHLD_W<'a> = crate::FieldWriter<'a, u32, BTR4_SPEC, u8, u8, 4, 4>;
#[doc = "Field `ADDSET` reader - ADDSET"]
pub type ADDSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDSET` writer - ADDSET"]
pub type ADDSET_W<'a> = crate::FieldWriter<'a, u32, BTR4_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 28:29 - ACCMOD"]
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DATLAT"]
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - BUSTURN"]
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - DATAST"]
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - ADDHLD"]
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - ADDSET"]
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29 - ACCMOD"]
    #[inline(always)]
    pub fn accmod(&mut self) -> ACCMOD_W {
        ACCMOD_W::new(self)
    }
    #[doc = "Bits 24:27 - DATLAT"]
    #[inline(always)]
    pub fn datlat(&mut self) -> DATLAT_W {
        DATLAT_W::new(self)
    }
    #[doc = "Bits 20:23 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 16:19 - BUSTURN"]
    #[inline(always)]
    pub fn busturn(&mut self) -> BUSTURN_W {
        BUSTURN_W::new(self)
    }
    #[doc = "Bits 8:15 - DATAST"]
    #[inline(always)]
    pub fn datast(&mut self) -> DATAST_W {
        DATAST_W::new(self)
    }
    #[doc = "Bits 4:7 - ADDHLD"]
    #[inline(always)]
    pub fn addhld(&mut self) -> ADDHLD_W {
        ADDHLD_W::new(self)
    }
    #[doc = "Bits 0:3 - ADDSET"]
    #[inline(always)]
    pub fn addset(&mut self) -> ADDSET_W {
        ADDSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM/NOR-Flash chip-select timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr4](index.html) module"]
pub struct BTR4_SPEC;
impl crate::RegisterSpec for BTR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btr4::R](R) reader structure"]
impl crate::Readable for BTR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btr4::W](W) writer structure"]
impl crate::Writable for BTR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BTR4 to value 0xffff_ffff"]
impl crate::Resettable for BTR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}