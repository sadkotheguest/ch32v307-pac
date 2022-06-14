#[doc = "Register `OADDR1` reader"]
pub struct R(crate::R<OADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OADDR1` writer"]
pub struct W(crate::W<OADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OADDR1_SPEC>;
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
impl From<crate::W<OADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDMODE` reader - Addressing mode (slave mode)"]
pub type ADDMODE_R = crate::BitReader<bool>;
#[doc = "Field `ADDMODE` writer - Addressing mode (slave mode)"]
pub type ADDMODE_W<'a> = crate::BitWriter<'a, u32, OADDR1_SPEC, bool, 15>;
#[doc = "Field `MUST1` reader - Must be 1"]
pub type MUST1_R = crate::BitReader<bool>;
#[doc = "Field `MUST1` writer - Must be 1"]
pub type MUST1_W<'a> = crate::BitWriter<'a, u32, OADDR1_SPEC, bool, 14>;
#[doc = "Field `ADD9_8` reader - Interface address"]
pub type ADD9_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD9_8` writer - Interface address"]
pub type ADD9_8_W<'a> = crate::FieldWriter<'a, u32, OADDR1_SPEC, u8, u8, 2, 8>;
#[doc = "Field `ADD7_1` reader - Interface address"]
pub type ADD7_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD7_1` writer - Interface address"]
pub type ADD7_1_W<'a> = crate::FieldWriter<'a, u32, OADDR1_SPEC, u8, u8, 7, 1>;
#[doc = "Field `ADD0` reader - Interface address"]
pub type ADD0_R = crate::BitReader<bool>;
#[doc = "Field `ADD0` writer - Interface address"]
pub type ADD0_W<'a> = crate::BitWriter<'a, u32, OADDR1_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&self) -> ADDMODE_R {
        ADDMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Must be 1"]
    #[inline(always)]
    pub fn must1(&self) -> MUST1_R {
        MUST1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn add9_8(&self) -> ADD9_8_R {
        ADD9_8_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add7_1(&self) -> ADD7_1_R {
        ADD7_1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn add0(&self) -> ADD0_R {
        ADD0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&mut self) -> ADDMODE_W {
        ADDMODE_W::new(self)
    }
    #[doc = "Bit 14 - Must be 1"]
    #[inline(always)]
    pub fn must1(&mut self) -> MUST1_W {
        MUST1_W::new(self)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn add9_8(&mut self) -> ADD9_8_W {
        ADD9_8_W::new(self)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add7_1(&mut self) -> ADD7_1_W {
        ADD7_1_W::new(self)
    }
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn add0(&mut self) -> ADD0_W {
        ADD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Own address register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oaddr1](index.html) module"]
pub struct OADDR1_SPEC;
impl crate::RegisterSpec for OADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oaddr1::R](R) reader structure"]
impl crate::Readable for OADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oaddr1::W](W) writer structure"]
impl crate::Writable for OADDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OADDR1 to value 0"]
impl crate::Resettable for OADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}