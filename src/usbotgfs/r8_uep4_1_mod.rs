#[doc = "Register `R8_UEP4_1_MOD` reader"]
pub struct R(crate::R<R8_UEP4_1_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UEP4_1_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UEP4_1_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UEP4_1_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UEP4_1_MOD` writer"]
pub struct W(crate::W<R8_UEP4_1_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UEP4_1_MOD_SPEC>;
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
impl From<crate::W<R8_UEP4_1_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UEP4_1_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_UEP4_TX_EN` reader - enable USB endpoint 4 transmittal (IN)"]
pub type RB_UEP4_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP4_TX_EN` writer - enable USB endpoint 4 transmittal (IN)"]
pub type RB_UEP4_TX_EN_W<'a> = crate::BitWriter<'a, u8, R8_UEP4_1_MOD_SPEC, bool, 2>;
#[doc = "Field `RB_UEP4_RX_EN` reader - enable USB endpoint 4 receiving (OUT)"]
pub type RB_UEP4_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP4_RX_EN` writer - enable USB endpoint 4 receiving (OUT)"]
pub type RB_UEP4_RX_EN_W<'a> = crate::BitWriter<'a, u8, R8_UEP4_1_MOD_SPEC, bool, 3>;
#[doc = "Field `RB_UEP1_BUF_MOD` reader - buffer mode of USB endpoint 1"]
pub type RB_UEP1_BUF_MOD_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP1_BUF_MOD` writer - buffer mode of USB endpoint 1"]
pub type RB_UEP1_BUF_MOD_W<'a> = crate::BitWriter<'a, u8, R8_UEP4_1_MOD_SPEC, bool, 4>;
#[doc = "Field `RB_UEP1_TX_EN` reader - enable USB endpoint 1 transmittal (IN)"]
pub type RB_UEP1_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP1_TX_EN` writer - enable USB endpoint 1 transmittal (IN)"]
pub type RB_UEP1_TX_EN_W<'a> = crate::BitWriter<'a, u8, R8_UEP4_1_MOD_SPEC, bool, 6>;
#[doc = "Field `RB_UEP1_RX_EN` reader - enable USB endpoint 1 receiving (OUT)"]
pub type RB_UEP1_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP1_RX_EN` writer - enable USB endpoint 1 receiving (OUT)"]
pub type RB_UEP1_RX_EN_W<'a> = crate::BitWriter<'a, u8, R8_UEP4_1_MOD_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 2 - enable USB endpoint 4 transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep4_tx_en(&self) -> RB_UEP4_TX_EN_R {
        RB_UEP4_TX_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable USB endpoint 4 receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep4_rx_en(&self) -> RB_UEP4_RX_EN_R {
        RB_UEP4_RX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 1"]
    #[inline(always)]
    pub fn rb_uep1_buf_mod(&self) -> RB_UEP1_BUF_MOD_R {
        RB_UEP1_BUF_MOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - enable USB endpoint 1 transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep1_tx_en(&self) -> RB_UEP1_TX_EN_R {
        RB_UEP1_TX_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable USB endpoint 1 receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep1_rx_en(&self) -> RB_UEP1_RX_EN_R {
        RB_UEP1_RX_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - enable USB endpoint 4 transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep4_tx_en(&mut self) -> RB_UEP4_TX_EN_W {
        RB_UEP4_TX_EN_W::new(self)
    }
    #[doc = "Bit 3 - enable USB endpoint 4 receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep4_rx_en(&mut self) -> RB_UEP4_RX_EN_W {
        RB_UEP4_RX_EN_W::new(self)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 1"]
    #[inline(always)]
    pub fn rb_uep1_buf_mod(&mut self) -> RB_UEP1_BUF_MOD_W {
        RB_UEP1_BUF_MOD_W::new(self)
    }
    #[doc = "Bit 6 - enable USB endpoint 1 transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep1_tx_en(&mut self) -> RB_UEP1_TX_EN_W {
        RB_UEP1_TX_EN_W::new(self)
    }
    #[doc = "Bit 7 - enable USB endpoint 1 receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep1_rx_en(&mut self) -> RB_UEP1_RX_EN_W {
        RB_UEP1_RX_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 4/1 mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uep4_1_mod](index.html) module"]
pub struct R8_UEP4_1_MOD_SPEC;
impl crate::RegisterSpec for R8_UEP4_1_MOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uep4_1_mod::R](R) reader structure"]
impl crate::Readable for R8_UEP4_1_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uep4_1_mod::W](W) writer structure"]
impl crate::Writable for R8_UEP4_1_MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UEP4_1_MOD to value 0"]
impl crate::Resettable for R8_UEP4_1_MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}