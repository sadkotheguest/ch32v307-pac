#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCRCFAILC` reader - CCRCFAILC"]
pub type CCRCFAILC_R = crate::BitReader<bool>;
#[doc = "Field `CCRCFAILC` writer - CCRCFAILC"]
pub type CCRCFAILC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 0>;
#[doc = "Field `DCRCFAILC` reader - DCRCFAILC"]
pub type DCRCFAILC_R = crate::BitReader<bool>;
#[doc = "Field `DCRCFAILC` writer - DCRCFAILC"]
pub type DCRCFAILC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 1>;
#[doc = "Field `CTIMEOUTC` reader - CTIMEOUTC"]
pub type CTIMEOUTC_R = crate::BitReader<bool>;
#[doc = "Field `CTIMEOUTC` writer - CTIMEOUTC"]
pub type CTIMEOUTC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 2>;
#[doc = "Field `DTIMEOUTC` reader - DTIMEOUTC"]
pub type DTIMEOUTC_R = crate::BitReader<bool>;
#[doc = "Field `DTIMEOUTC` writer - DTIMEOUTC"]
pub type DTIMEOUTC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 3>;
#[doc = "Field `TXUNDERRC` reader - TXUNDERRC"]
pub type TXUNDERRC_R = crate::BitReader<bool>;
#[doc = "Field `TXUNDERRC` writer - TXUNDERRC"]
pub type TXUNDERRC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 4>;
#[doc = "Field `RXOVERRC` reader - RXOVERRC"]
pub type RXOVERRC_R = crate::BitReader<bool>;
#[doc = "Field `RXOVERRC` writer - RXOVERRC"]
pub type RXOVERRC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 5>;
#[doc = "Field `CMDRENDC` reader - CMDRENDC"]
pub type CMDRENDC_R = crate::BitReader<bool>;
#[doc = "Field `CMDRENDC` writer - CMDRENDC"]
pub type CMDRENDC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 6>;
#[doc = "Field `CMDSENTC` reader - CMDSENTC"]
pub type CMDSENTC_R = crate::BitReader<bool>;
#[doc = "Field `CMDSENTC` writer - CMDSENTC"]
pub type CMDSENTC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 7>;
#[doc = "Field `DATAENDC` reader - DATAENDC"]
pub type DATAENDC_R = crate::BitReader<bool>;
#[doc = "Field `DATAENDC` writer - DATAENDC"]
pub type DATAENDC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 8>;
#[doc = "Field `STBITERRC` reader - STBITERRC"]
pub type STBITERRC_R = crate::BitReader<bool>;
#[doc = "Field `STBITERRC` writer - STBITERRC"]
pub type STBITERRC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 9>;
#[doc = "Field `DBCKENDC` reader - DBCKENDC"]
pub type DBCKENDC_R = crate::BitReader<bool>;
#[doc = "Field `DBCKENDC` writer - DBCKENDC"]
pub type DBCKENDC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 10>;
#[doc = "Field `SDIOITC` reader - SDIOITC"]
pub type SDIOITC_R = crate::BitReader<bool>;
#[doc = "Field `SDIOITC` writer - SDIOITC"]
pub type SDIOITC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 22>;
#[doc = "Field `CEATAENDC` reader - CEATAENDC"]
pub type CEATAENDC_R = crate::BitReader<bool>;
#[doc = "Field `CEATAENDC` writer - CEATAENDC"]
pub type CEATAENDC_W<'a> = crate::BitWriter<'a, u32, ICR_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 0 - CCRCFAILC"]
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CCRCFAILC_R {
        CCRCFAILC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAILC"]
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DCRCFAILC_R {
        DCRCFAILC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUTC"]
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CTIMEOUTC_R {
        CTIMEOUTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUTC"]
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DTIMEOUTC_R {
        DTIMEOUTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERRC"]
    #[inline(always)]
    pub fn txunderrc(&self) -> TXUNDERRC_R {
        TXUNDERRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERRC"]
    #[inline(always)]
    pub fn rxoverrc(&self) -> RXOVERRC_R {
        RXOVERRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDRENDC"]
    #[inline(always)]
    pub fn cmdrendc(&self) -> CMDRENDC_R {
        CMDRENDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENTC"]
    #[inline(always)]
    pub fn cmdsentc(&self) -> CMDSENTC_R {
        CMDSENTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAENDC"]
    #[inline(always)]
    pub fn dataendc(&self) -> DATAENDC_R {
        DATAENDC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STBITERRC"]
    #[inline(always)]
    pub fn stbiterrc(&self) -> STBITERRC_R {
        STBITERRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKENDC"]
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOITC"]
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CEATAENDC"]
    #[inline(always)]
    pub fn ceataendc(&self) -> CEATAENDC_R {
        CEATAENDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAILC"]
    #[inline(always)]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W {
        CCRCFAILC_W::new(self)
    }
    #[doc = "Bit 1 - DCRCFAILC"]
    #[inline(always)]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W {
        DCRCFAILC_W::new(self)
    }
    #[doc = "Bit 2 - CTIMEOUTC"]
    #[inline(always)]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W {
        CTIMEOUTC_W::new(self)
    }
    #[doc = "Bit 3 - DTIMEOUTC"]
    #[inline(always)]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W {
        DTIMEOUTC_W::new(self)
    }
    #[doc = "Bit 4 - TXUNDERRC"]
    #[inline(always)]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W {
        TXUNDERRC_W::new(self)
    }
    #[doc = "Bit 5 - RXOVERRC"]
    #[inline(always)]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W {
        RXOVERRC_W::new(self)
    }
    #[doc = "Bit 6 - CMDRENDC"]
    #[inline(always)]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W {
        CMDRENDC_W::new(self)
    }
    #[doc = "Bit 7 - CMDSENTC"]
    #[inline(always)]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W {
        CMDSENTC_W::new(self)
    }
    #[doc = "Bit 8 - DATAENDC"]
    #[inline(always)]
    pub fn dataendc(&mut self) -> DATAENDC_W {
        DATAENDC_W::new(self)
    }
    #[doc = "Bit 9 - STBITERRC"]
    #[inline(always)]
    pub fn stbiterrc(&mut self) -> STBITERRC_W {
        STBITERRC_W::new(self)
    }
    #[doc = "Bit 10 - DBCKENDC"]
    #[inline(always)]
    pub fn dbckendc(&mut self) -> DBCKENDC_W {
        DBCKENDC_W::new(self)
    }
    #[doc = "Bit 22 - SDIOITC"]
    #[inline(always)]
    pub fn sdioitc(&mut self) -> SDIOITC_W {
        SDIOITC_W::new(self)
    }
    #[doc = "Bit 23 - CEATAENDC"]
    #[inline(always)]
    pub fn ceataendc(&mut self) -> CEATAENDC_W {
        CEATAENDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO interrupt clear register (SDIO_ICR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}