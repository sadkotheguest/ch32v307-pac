#[doc = "Register `R8_UEP5_6_MOD` reader"]
pub struct R(crate::R<R8_UEP5_6_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UEP5_6_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UEP5_6_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UEP5_6_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UEP5_6_MOD` writer"]
pub struct W(crate::W<R8_UEP5_6_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UEP5_6_MOD_SPEC>;
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
impl From<crate::W<R8_UEP5_6_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UEP5_6_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_UEP5_BUF_MOD` reader - buffer mode of USB endpoint 5"]
pub type RB_UEP5_BUF_MOD_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP5_BUF_MOD` writer - buffer mode of USB endpoint 5"]
pub type RB_UEP5_BUF_MOD_W<'a> = crate::BitWriter<'a, u8, R8_UEP5_6_MOD_SPEC, bool, 0>;
#[doc = "Field `RB_UEP5_TX_EN` reader - enable USB endpoint 5 transmittal (IN)"]
pub type RB_UEP5_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP5_TX_EN` writer - enable USB endpoint 5 transmittal (IN)"]
pub type RB_UEP5_TX_EN_W<'a> = crate::BitWriter<'a, u8, R8_UEP5_6_MOD_SPEC, bool, 2>;
#[doc = "Field `RB_UEP5_RX_EN` reader - enable USB endpoint 5 receiving (OUT)"]
pub type RB_UEP5_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP5_RX_EN` writer - enable USB endpoint 5 receiving (OUT)"]
pub type RB_UEP5_RX_EN_W<'a> = crate::BitWriter<'a, u8, R8_UEP5_6_MOD_SPEC, bool, 3>;
#[doc = "Field `RB_UEP6_BUF_MOD` reader - buffer mode of USB endpoint 6"]
pub type RB_UEP6_BUF_MOD_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP6_BUF_MOD` writer - buffer mode of USB endpoint 6"]
pub type RB_UEP6_BUF_MOD_W<'a> = crate::BitWriter<'a, u8, R8_UEP5_6_MOD_SPEC, bool, 4>;
#[doc = "Field `RB_UEP6_TX_EN` reader - enable USB endpoint 6 transmittal (IN)"]
pub type RB_UEP6_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP6_TX_EN` writer - enable USB endpoint 6 transmittal (IN)"]
pub type RB_UEP6_TX_EN_W<'a> = crate::BitWriter<'a, u8, R8_UEP5_6_MOD_SPEC, bool, 6>;
#[doc = "Field `RB_UEP3_RX_EN` reader - enable USB endpoint 6 receiving (OUT)"]
pub type RB_UEP3_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UEP3_RX_EN` writer - enable USB endpoint 6 receiving (OUT)"]
pub type RB_UEP3_RX_EN_W<'a> = crate::BitWriter<'a, u8, R8_UEP5_6_MOD_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - buffer mode of USB endpoint 5"]
    #[inline(always)]
    pub fn rb_uep5_buf_mod(&self) -> RB_UEP5_BUF_MOD_R {
        RB_UEP5_BUF_MOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - enable USB endpoint 5 transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep5_tx_en(&self) -> RB_UEP5_TX_EN_R {
        RB_UEP5_TX_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable USB endpoint 5 receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep5_rx_en(&self) -> RB_UEP5_RX_EN_R {
        RB_UEP5_RX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 6"]
    #[inline(always)]
    pub fn rb_uep6_buf_mod(&self) -> RB_UEP6_BUF_MOD_R {
        RB_UEP6_BUF_MOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - enable USB endpoint 6 transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep6_tx_en(&self) -> RB_UEP6_TX_EN_R {
        RB_UEP6_TX_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable USB endpoint 6 receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep3_rx_en(&self) -> RB_UEP3_RX_EN_R {
        RB_UEP3_RX_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - buffer mode of USB endpoint 5"]
    #[inline(always)]
    pub fn rb_uep5_buf_mod(&mut self) -> RB_UEP5_BUF_MOD_W {
        RB_UEP5_BUF_MOD_W::new(self)
    }
    #[doc = "Bit 2 - enable USB endpoint 5 transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep5_tx_en(&mut self) -> RB_UEP5_TX_EN_W {
        RB_UEP5_TX_EN_W::new(self)
    }
    #[doc = "Bit 3 - enable USB endpoint 5 receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep5_rx_en(&mut self) -> RB_UEP5_RX_EN_W {
        RB_UEP5_RX_EN_W::new(self)
    }
    #[doc = "Bit 4 - buffer mode of USB endpoint 6"]
    #[inline(always)]
    pub fn rb_uep6_buf_mod(&mut self) -> RB_UEP6_BUF_MOD_W {
        RB_UEP6_BUF_MOD_W::new(self)
    }
    #[doc = "Bit 6 - enable USB endpoint 6 transmittal (IN)"]
    #[inline(always)]
    pub fn rb_uep6_tx_en(&mut self) -> RB_UEP6_TX_EN_W {
        RB_UEP6_TX_EN_W::new(self)
    }
    #[doc = "Bit 7 - enable USB endpoint 6 receiving (OUT)"]
    #[inline(always)]
    pub fn rb_uep3_rx_en(&mut self) -> RB_UEP3_RX_EN_W {
        RB_UEP3_RX_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 5/6 mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uep5_6_mod](index.html) module"]
pub struct R8_UEP5_6_MOD_SPEC;
impl crate::RegisterSpec for R8_UEP5_6_MOD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uep5_6_mod::R](R) reader structure"]
impl crate::Readable for R8_UEP5_6_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uep5_6_mod::W](W) writer structure"]
impl crate::Writable for R8_UEP5_6_MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UEP5_6_MOD to value 0"]
impl crate::Resettable for R8_UEP5_6_MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}