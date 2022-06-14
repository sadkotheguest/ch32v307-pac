#[doc = "Register `CKCFGR` reader"]
pub struct R(crate::R<CKCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCFGR` writer"]
pub struct W(crate::W<CKCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCFGR_SPEC>;
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
impl From<crate::W<CKCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F_S` reader - I2C master mode selection"]
pub type F_S_R = crate::BitReader<bool>;
#[doc = "Field `F_S` writer - I2C master mode selection"]
pub type F_S_W<'a> = crate::BitWriter<'a, u32, CKCFGR_SPEC, bool, 15>;
#[doc = "Field `DUTY` reader - Fast mode duty cycle"]
pub type DUTY_R = crate::BitReader<bool>;
#[doc = "Field `DUTY` writer - Fast mode duty cycle"]
pub type DUTY_W<'a> = crate::BitWriter<'a, u32, CKCFGR_SPEC, bool, 14>;
#[doc = "Field `CCR` reader - Clock control register in Fast/Standard mode (Master mode)"]
pub type CCR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR` writer - Clock control register in Fast/Standard mode (Master mode)"]
pub type CCR_W<'a> = crate::FieldWriter<'a, u32, CKCFGR_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    pub fn f_s(&self) -> F_S_R {
        F_S_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    pub fn f_s(&mut self) -> F_S_W {
        F_S_W::new(self)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W {
        DUTY_W::new(self)
    }
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn ccr(&mut self) -> CCR_W {
        CCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcfgr](index.html) module"]
pub struct CKCFGR_SPEC;
impl crate::RegisterSpec for CKCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcfgr::R](R) reader structure"]
impl crate::Readable for CKCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcfgr::W](W) writer structure"]
impl crate::Writable for CKCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CKCFGR to value 0"]
impl crate::Resettable for CKCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}