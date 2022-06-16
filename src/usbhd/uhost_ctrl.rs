#[doc = "Register `UHOST_CTRL` reader"]
pub struct R(crate::R<UHOST_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHOST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHOST_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHOST_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UHOST_CTRL` writer"]
pub struct W(crate::W<UHOST_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHOST_CTRL_SPEC>;
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
impl From<crate::W<UHOST_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHOST_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bUH_TX_BUS_RESET` reader - USB host bus reset status"]
pub type BUH_TX_BUS_RESET_R = crate::BitReader<bool>;
#[doc = "Field `bUH_TX_BUS_RESET` writer - USB host bus reset status"]
pub type BUH_TX_BUS_RESET_W<'a> = crate::BitWriter<'a, u8, UHOST_CTRL_SPEC, bool, 0>;
#[doc = "Field `bUH_TX_BUS_SUSPEND` reader - the host sends hang sigal"]
pub type BUH_TX_BUS_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `bUH_TX_BUS_SUSPEND` writer - the host sends hang sigal"]
pub type BUH_TX_BUS_SUSPEND_W<'a> = crate::BitWriter<'a, u8, UHOST_CTRL_SPEC, bool, 1>;
#[doc = "Field `bUH_TX_BUS_RESUME` reader - host wake up device"]
pub type BUH_TX_BUS_RESUME_R = crate::BitReader<bool>;
#[doc = "Field `bUH_TX_BUS_RESUME` writer - host wake up device"]
pub type BUH_TX_BUS_RESUME_W<'a> = crate::BitWriter<'a, u8, UHOST_CTRL_SPEC, bool, 2>;
#[doc = "Field `bUH_REMOTE_WKUP` reader - the remoke wake-up"]
pub type BUH_REMOTE_WKUP_R = crate::BitReader<bool>;
#[doc = "Field `bUH_REMOTE_WKUP` writer - the remoke wake-up"]
pub type BUH_REMOTE_WKUP_W<'a> = crate::BitWriter<'a, u8, UHOST_CTRL_SPEC, bool, 3>;
#[doc = "Field `bUH_PHY_SUSPENDM` reader - USB-PHY thesuspended state the internal USB-PLL is turned off"]
pub type BUH_PHY_SUSPENDM_R = crate::BitReader<bool>;
#[doc = "Field `bUH_PHY_SUSPENDM` writer - USB-PHY thesuspended state the internal USB-PLL is turned off"]
pub type BUH_PHY_SUSPENDM_W<'a> = crate::BitWriter<'a, u8, UHOST_CTRL_SPEC, bool, 4>;
#[doc = "Field `bUH_SOF_FREE` reader - the bus is idle"]
pub type BUH_SOF_FREE_R = crate::BitReader<bool>;
#[doc = "Field `bUH_SOF_EN` reader - automatically generate the SOF packet enabling control bit"]
pub type BUH_SOF_EN_R = crate::BitReader<bool>;
#[doc = "Field `bUH_SOF_EN` writer - automatically generate the SOF packet enabling control bit"]
pub type BUH_SOF_EN_W<'a> = crate::BitWriter<'a, u8, UHOST_CTRL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - USB host bus reset status"]
    #[inline(always)]
    pub fn b_uh_tx_bus_reset(&self) -> BUH_TX_BUS_RESET_R {
        BUH_TX_BUS_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the host sends hang sigal"]
    #[inline(always)]
    pub fn b_uh_tx_bus_suspend(&self) -> BUH_TX_BUS_SUSPEND_R {
        BUH_TX_BUS_SUSPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - host wake up device"]
    #[inline(always)]
    pub fn b_uh_tx_bus_resume(&self) -> BUH_TX_BUS_RESUME_R {
        BUH_TX_BUS_RESUME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the remoke wake-up"]
    #[inline(always)]
    pub fn b_uh_remote_wkup(&self) -> BUH_REMOTE_WKUP_R {
        BUH_REMOTE_WKUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB-PHY thesuspended state the internal USB-PLL is turned off"]
    #[inline(always)]
    pub fn b_uh_phy_suspendm(&self) -> BUH_PHY_SUSPENDM_R {
        BUH_PHY_SUSPENDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - the bus is idle"]
    #[inline(always)]
    pub fn b_uh_sof_free(&self) -> BUH_SOF_FREE_R {
        BUH_SOF_FREE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - automatically generate the SOF packet enabling control bit"]
    #[inline(always)]
    pub fn b_uh_sof_en(&self) -> BUH_SOF_EN_R {
        BUH_SOF_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB host bus reset status"]
    #[inline(always)]
    pub fn b_uh_tx_bus_reset(&mut self) -> BUH_TX_BUS_RESET_W {
        BUH_TX_BUS_RESET_W::new(self)
    }
    #[doc = "Bit 1 - the host sends hang sigal"]
    #[inline(always)]
    pub fn b_uh_tx_bus_suspend(&mut self) -> BUH_TX_BUS_SUSPEND_W {
        BUH_TX_BUS_SUSPEND_W::new(self)
    }
    #[doc = "Bit 2 - host wake up device"]
    #[inline(always)]
    pub fn b_uh_tx_bus_resume(&mut self) -> BUH_TX_BUS_RESUME_W {
        BUH_TX_BUS_RESUME_W::new(self)
    }
    #[doc = "Bit 3 - the remoke wake-up"]
    #[inline(always)]
    pub fn b_uh_remote_wkup(&mut self) -> BUH_REMOTE_WKUP_W {
        BUH_REMOTE_WKUP_W::new(self)
    }
    #[doc = "Bit 4 - USB-PHY thesuspended state the internal USB-PLL is turned off"]
    #[inline(always)]
    pub fn b_uh_phy_suspendm(&mut self) -> BUH_PHY_SUSPENDM_W {
        BUH_PHY_SUSPENDM_W::new(self)
    }
    #[doc = "Bit 7 - automatically generate the SOF packet enabling control bit"]
    #[inline(always)]
    pub fn b_uh_sof_en(&mut self) -> BUH_SOF_EN_W {
        BUH_SOF_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB HOST control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhost_ctrl](index.html) module"]
pub struct UHOST_CTRL_SPEC;
impl crate::RegisterSpec for UHOST_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uhost_ctrl::R](R) reader structure"]
impl crate::Readable for UHOST_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhost_ctrl::W](W) writer structure"]
impl crate::Writable for UHOST_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UHOST_CTRL to value 0"]
impl crate::Resettable for UHOST_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}