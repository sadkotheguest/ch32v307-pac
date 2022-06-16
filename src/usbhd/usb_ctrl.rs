#[doc = "Register `USB_CTRL` reader"]
pub struct R(crate::R<USB_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_CTRL` writer"]
pub struct W(crate::W<USB_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_CTRL_SPEC>;
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
impl From<crate::W<USB_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_UC_DMA_EN` reader - DMA enable and DMA interrupt enable for USB"]
pub type RB_UC_DMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UC_DMA_EN` writer - DMA enable and DMA interrupt enable for USB"]
pub type RB_UC_DMA_EN_W<'a> = crate::BitWriter<'a, u8, USB_CTRL_SPEC, bool, 0>;
#[doc = "Field `RB_UC_CLR_ALL` reader - force clear FIFO and count of USB"]
pub type RB_UC_CLR_ALL_R = crate::BitReader<bool>;
#[doc = "Field `RB_UC_CLR_ALL` writer - force clear FIFO and count of USB"]
pub type RB_UC_CLR_ALL_W<'a> = crate::BitWriter<'a, u8, USB_CTRL_SPEC, bool, 1>;
#[doc = "Field `RB_UC_RESET_SIE` reader - force reset USB SIE, need software clear"]
pub type RB_UC_RESET_SIE_R = crate::BitReader<bool>;
#[doc = "Field `RB_UC_RESET_SIE` writer - force reset USB SIE, need software clear"]
pub type RB_UC_RESET_SIE_W<'a> = crate::BitWriter<'a, u8, USB_CTRL_SPEC, bool, 2>;
#[doc = "Field `RB_UC_INT_BUSY` reader - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
pub type RB_UC_INT_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `RB_UC_INT_BUSY` writer - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
pub type RB_UC_INT_BUSY_W<'a> = crate::BitWriter<'a, u8, USB_CTRL_SPEC, bool, 3>;
#[doc = "Field `RB_UC_DEV_PU_EN` reader - USB device enable and internal pullup resistance enable"]
pub type RB_UC_DEV_PU_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_UC_DEV_PU_EN` writer - USB device enable and internal pullup resistance enable"]
pub type RB_UC_DEV_PU_EN_W<'a> = crate::BitWriter<'a, u8, USB_CTRL_SPEC, bool, 4>;
#[doc = "Field `RB_UC_SPEED_TYPE` reader - enable USB low speed: 00=full speed, 01=high speed, 10 =low speed"]
pub type RB_UC_SPEED_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_UC_SPEED_TYPE` writer - enable USB low speed: 00=full speed, 01=high speed, 10 =low speed"]
pub type RB_UC_SPEED_TYPE_W<'a> = crate::FieldWriter<'a, u8, USB_CTRL_SPEC, u8, u8, 2, 5>;
#[doc = "Field `RB_UC_HOST_MODE` reader - enable USB host mode: 0=device mode, 1=host mode"]
pub type RB_UC_HOST_MODE_R = crate::BitReader<bool>;
#[doc = "Field `RB_UC_HOST_MODE` writer - enable USB host mode: 0=device mode, 1=host mode"]
pub type RB_UC_HOST_MODE_W<'a> = crate::BitWriter<'a, u8, USB_CTRL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - DMA enable and DMA interrupt enable for USB"]
    #[inline(always)]
    pub fn rb_uc_dma_en(&self) -> RB_UC_DMA_EN_R {
        RB_UC_DMA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force clear FIFO and count of USB"]
    #[inline(always)]
    pub fn rb_uc_clr_all(&self) -> RB_UC_CLR_ALL_R {
        RB_UC_CLR_ALL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - force reset USB SIE, need software clear"]
    #[inline(always)]
    pub fn rb_uc_reset_sie(&self) -> RB_UC_RESET_SIE_R {
        RB_UC_RESET_SIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
    #[inline(always)]
    pub fn rb_uc_int_busy(&self) -> RB_UC_INT_BUSY_R {
        RB_UC_INT_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB device enable and internal pullup resistance enable"]
    #[inline(always)]
    pub fn rb_uc_dev_pu_en(&self) -> RB_UC_DEV_PU_EN_R {
        RB_UC_DEV_PU_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - enable USB low speed: 00=full speed, 01=high speed, 10 =low speed"]
    #[inline(always)]
    pub fn rb_uc_speed_type(&self) -> RB_UC_SPEED_TYPE_R {
        RB_UC_SPEED_TYPE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - enable USB host mode: 0=device mode, 1=host mode"]
    #[inline(always)]
    pub fn rb_uc_host_mode(&self) -> RB_UC_HOST_MODE_R {
        RB_UC_HOST_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA enable and DMA interrupt enable for USB"]
    #[inline(always)]
    pub fn rb_uc_dma_en(&mut self) -> RB_UC_DMA_EN_W {
        RB_UC_DMA_EN_W::new(self)
    }
    #[doc = "Bit 1 - force clear FIFO and count of USB"]
    #[inline(always)]
    pub fn rb_uc_clr_all(&mut self) -> RB_UC_CLR_ALL_W {
        RB_UC_CLR_ALL_W::new(self)
    }
    #[doc = "Bit 2 - force reset USB SIE, need software clear"]
    #[inline(always)]
    pub fn rb_uc_reset_sie(&mut self) -> RB_UC_RESET_SIE_W {
        RB_UC_RESET_SIE_W::new(self)
    }
    #[doc = "Bit 3 - enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid"]
    #[inline(always)]
    pub fn rb_uc_int_busy(&mut self) -> RB_UC_INT_BUSY_W {
        RB_UC_INT_BUSY_W::new(self)
    }
    #[doc = "Bit 4 - USB device enable and internal pullup resistance enable"]
    #[inline(always)]
    pub fn rb_uc_dev_pu_en(&mut self) -> RB_UC_DEV_PU_EN_W {
        RB_UC_DEV_PU_EN_W::new(self)
    }
    #[doc = "Bits 5:6 - enable USB low speed: 00=full speed, 01=high speed, 10 =low speed"]
    #[inline(always)]
    pub fn rb_uc_speed_type(&mut self) -> RB_UC_SPEED_TYPE_W {
        RB_UC_SPEED_TYPE_W::new(self)
    }
    #[doc = "Bit 7 - enable USB host mode: 0=device mode, 1=host mode"]
    #[inline(always)]
    pub fn rb_uc_host_mode(&mut self) -> RB_UC_HOST_MODE_W {
        RB_UC_HOST_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB base control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_ctrl](index.html) module"]
pub struct USB_CTRL_SPEC;
impl crate::RegisterSpec for USB_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb_ctrl::R](R) reader structure"]
impl crate::Readable for USB_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_ctrl::W](W) writer structure"]
impl crate::Writable for USB_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_CTRL to value 0x06"]
impl crate::Resettable for USB_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}