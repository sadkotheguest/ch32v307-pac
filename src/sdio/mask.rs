#[doc = "Register `MASK` reader"]
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK` writer"]
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
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
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCRCFAILIE` reader - CCRCFAILIE"]
pub type CCRCFAILIE_R = crate::BitReader<bool>;
#[doc = "Field `CCRCFAILIE` writer - CCRCFAILIE"]
pub type CCRCFAILIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 0>;
#[doc = "Field `DCRCFAILIE` reader - DCRCFAILIE"]
pub type DCRCFAILIE_R = crate::BitReader<bool>;
#[doc = "Field `DCRCFAILIE` writer - DCRCFAILIE"]
pub type DCRCFAILIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 1>;
#[doc = "Field `CTIMEOUTIE` reader - CTIMEOUTIE"]
pub type CTIMEOUTIE_R = crate::BitReader<bool>;
#[doc = "Field `CTIMEOUTIE` writer - CTIMEOUTIE"]
pub type CTIMEOUTIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 2>;
#[doc = "Field `DTIMEOUTIE` reader - DTIMEOUTIE"]
pub type DTIMEOUTIE_R = crate::BitReader<bool>;
#[doc = "Field `DTIMEOUTIE` writer - DTIMEOUTIE"]
pub type DTIMEOUTIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 3>;
#[doc = "Field `TXUNDERRIE` reader - TXUNDERRIE"]
pub type TXUNDERRIE_R = crate::BitReader<bool>;
#[doc = "Field `TXUNDERRIE` writer - TXUNDERRIE"]
pub type TXUNDERRIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 4>;
#[doc = "Field `RXOVERRIE` reader - RXOVERRIE"]
pub type RXOVERRIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOVERRIE` writer - RXOVERRIE"]
pub type RXOVERRIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 5>;
#[doc = "Field `CMDRENDIE` reader - CMDRENDIE"]
pub type CMDRENDIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDRENDIE` writer - CMDRENDIE"]
pub type CMDRENDIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 6>;
#[doc = "Field `CMDSENTIE` reader - CMDSENTIE"]
pub type CMDSENTIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDSENTIE` writer - CMDSENTIE"]
pub type CMDSENTIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 7>;
#[doc = "Field `DATAENDIE` reader - DATAENDIE"]
pub type DATAENDIE_R = crate::BitReader<bool>;
#[doc = "Field `DATAENDIE` writer - DATAENDIE"]
pub type DATAENDIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 8>;
#[doc = "Field `STBITERRIE` reader - STBITERRIE"]
pub type STBITERRIE_R = crate::BitReader<bool>;
#[doc = "Field `STBITERRIE` writer - STBITERRIE"]
pub type STBITERRIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 9>;
#[doc = "Field `DBACKENDIE` reader - DBACKENDIE"]
pub type DBACKENDIE_R = crate::BitReader<bool>;
#[doc = "Field `DBACKENDIE` writer - DBACKENDIE"]
pub type DBACKENDIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 10>;
#[doc = "Field `CMDACTIE` reader - CMDACTIE"]
pub type CMDACTIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDACTIE` writer - CMDACTIE"]
pub type CMDACTIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 11>;
#[doc = "Field `TXACTIE` reader - TXACTIE"]
pub type TXACTIE_R = crate::BitReader<bool>;
#[doc = "Field `TXACTIE` writer - TXACTIE"]
pub type TXACTIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 12>;
#[doc = "Field `RXACTIE` reader - RXACTIE"]
pub type RXACTIE_R = crate::BitReader<bool>;
#[doc = "Field `RXACTIE` writer - RXACTIE"]
pub type RXACTIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 13>;
#[doc = "Field `TXFIFOHEIE` reader - TXFIFOHEIE"]
pub type TXFIFOHEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOHEIE` writer - TXFIFOHEIE"]
pub type TXFIFOHEIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 14>;
#[doc = "Field `RXFIFOHFIE` reader - RXFIFOHFIE"]
pub type RXFIFOHFIE_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOHFIE` writer - RXFIFOHFIE"]
pub type RXFIFOHFIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 15>;
#[doc = "Field `TXFIFOFIE` reader - TXFIFOFIE"]
pub type TXFIFOFIE_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOFIE` writer - TXFIFOFIE"]
pub type TXFIFOFIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 16>;
#[doc = "Field `RXFIFOFIE` reader - RXFIFOFIE"]
pub type RXFIFOFIE_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOFIE` writer - RXFIFOFIE"]
pub type RXFIFOFIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 17>;
#[doc = "Field `TXFIFOEIE` reader - TXFIFOEIE"]
pub type TXFIFOEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOEIE` writer - TXFIFOEIE"]
pub type TXFIFOEIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 18>;
#[doc = "Field `RXFIFOEIE` reader - RXFIFOEIE"]
pub type RXFIFOEIE_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOEIE` writer - RXFIFOEIE"]
pub type RXFIFOEIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 19>;
#[doc = "Field `TXDAVLIE` reader - TXDAVLIE"]
pub type TXDAVLIE_R = crate::BitReader<bool>;
#[doc = "Field `TXDAVLIE` writer - TXDAVLIE"]
pub type TXDAVLIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 20>;
#[doc = "Field `RXDAVLIE` reader - RXDAVLIE"]
pub type RXDAVLIE_R = crate::BitReader<bool>;
#[doc = "Field `RXDAVLIE` writer - RXDAVLIE"]
pub type RXDAVLIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 21>;
#[doc = "Field `SDIOITIE` reader - SDIOITIE"]
pub type SDIOITIE_R = crate::BitReader<bool>;
#[doc = "Field `SDIOITIE` writer - SDIOITIE"]
pub type SDIOITIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 22>;
#[doc = "Field `CEATENDIE` reader - CEATENDIE"]
pub type CEATENDIE_R = crate::BitReader<bool>;
#[doc = "Field `CEATENDIE` writer - CEATENDIE"]
pub type CEATENDIE_W<'a> = crate::BitWriter<'a, u32, MASK_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STBITERRIE"]
    #[inline(always)]
    pub fn stbiterrie(&self) -> STBITERRIE_R {
        STBITERRIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBACKENDIE"]
    #[inline(always)]
    pub fn dbackendie(&self) -> DBACKENDIE_R {
        DBACKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CMDACTIE"]
    #[inline(always)]
    pub fn cmdactie(&self) -> CMDACTIE_R {
        CMDACTIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXACTIE"]
    #[inline(always)]
    pub fn txactie(&self) -> TXACTIE_R {
        TXACTIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RXACTIE"]
    #[inline(always)]
    pub fn rxactie(&self) -> RXACTIE_R {
        RXACTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TXFIFOFIE"]
    #[inline(always)]
    pub fn txfifofie(&self) -> TXFIFOFIE_R {
        TXFIFOFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RXFIFOEIE"]
    #[inline(always)]
    pub fn rxfifoeie(&self) -> RXFIFOEIE_R {
        RXFIFOEIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TXDAVLIE"]
    #[inline(always)]
    pub fn txdavlie(&self) -> TXDAVLIE_R {
        TXDAVLIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RXDAVLIE"]
    #[inline(always)]
    pub fn rxdavlie(&self) -> RXDAVLIE_R {
        RXDAVLIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CEATENDIE"]
    #[inline(always)]
    pub fn ceatendie(&self) -> CEATENDIE_R {
        CEATENDIE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W {
        CCRCFAILIE_W::new(self)
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W {
        DCRCFAILIE_W::new(self)
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W {
        CTIMEOUTIE_W::new(self)
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W {
        DTIMEOUTIE_W::new(self)
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W {
        TXUNDERRIE_W::new(self)
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W {
        RXOVERRIE_W::new(self)
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W {
        CMDRENDIE_W::new(self)
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W {
        CMDSENTIE_W::new(self)
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    pub fn dataendie(&mut self) -> DATAENDIE_W {
        DATAENDIE_W::new(self)
    }
    #[doc = "Bit 9 - STBITERRIE"]
    #[inline(always)]
    pub fn stbiterrie(&mut self) -> STBITERRIE_W {
        STBITERRIE_W::new(self)
    }
    #[doc = "Bit 10 - DBACKENDIE"]
    #[inline(always)]
    pub fn dbackendie(&mut self) -> DBACKENDIE_W {
        DBACKENDIE_W::new(self)
    }
    #[doc = "Bit 11 - CMDACTIE"]
    #[inline(always)]
    pub fn cmdactie(&mut self) -> CMDACTIE_W {
        CMDACTIE_W::new(self)
    }
    #[doc = "Bit 12 - TXACTIE"]
    #[inline(always)]
    pub fn txactie(&mut self) -> TXACTIE_W {
        TXACTIE_W::new(self)
    }
    #[doc = "Bit 13 - RXACTIE"]
    #[inline(always)]
    pub fn rxactie(&mut self) -> RXACTIE_W {
        RXACTIE_W::new(self)
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W {
        TXFIFOHEIE_W::new(self)
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W {
        RXFIFOHFIE_W::new(self)
    }
    #[doc = "Bit 16 - TXFIFOFIE"]
    #[inline(always)]
    pub fn txfifofie(&mut self) -> TXFIFOFIE_W {
        TXFIFOFIE_W::new(self)
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W {
        RXFIFOFIE_W::new(self)
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W {
        TXFIFOEIE_W::new(self)
    }
    #[doc = "Bit 19 - RXFIFOEIE"]
    #[inline(always)]
    pub fn rxfifoeie(&mut self) -> RXFIFOEIE_W {
        RXFIFOEIE_W::new(self)
    }
    #[doc = "Bit 20 - TXDAVLIE"]
    #[inline(always)]
    pub fn txdavlie(&mut self) -> TXDAVLIE_W {
        TXDAVLIE_W::new(self)
    }
    #[doc = "Bit 21 - RXDAVLIE"]
    #[inline(always)]
    pub fn rxdavlie(&mut self) -> RXDAVLIE_W {
        RXDAVLIE_W::new(self)
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    pub fn sdioitie(&mut self) -> SDIOITIE_W {
        SDIOITIE_W::new(self)
    }
    #[doc = "Bit 23 - CEATENDIE"]
    #[inline(always)]
    pub fn ceatendie(&mut self) -> CEATENDIE_W {
        CEATENDIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO mask register (SDIO_MASK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](index.html) module"]
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask::R](R) reader structure"]
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask::W](W) writer structure"]
impl crate::Writable for MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}