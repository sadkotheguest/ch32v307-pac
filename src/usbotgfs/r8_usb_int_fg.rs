#[doc = "Register `R8_USB_INT_FG` reader"]
pub struct R(crate::R<R8_USB_INT_FG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_USB_INT_FG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_USB_INT_FG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_USB_INT_FG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_USB_INT_FG` writer"]
pub struct W(crate::W<R8_USB_INT_FG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_USB_INT_FG_SPEC>;
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
impl From<crate::W<R8_USB_INT_FG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_USB_INT_FG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_UIF_BUS_RST__RB_UIF_DETECT` reader - bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
pub type RB_UIF_BUS_RST__RB_UIF_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIF_BUS_RST__RB_UIF_DETECT` writer - bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
pub type RB_UIF_BUS_RST__RB_UIF_DETECT_W<'a> =
    crate::BitWriter<'a, u8, R8_USB_INT_FG_SPEC, bool, 0>;
#[doc = "Field `RB_UIF_TRANSFER` reader - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
pub type RB_UIF_TRANSFER_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIF_TRANSFER` writer - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
pub type RB_UIF_TRANSFER_W<'a> = crate::BitWriter<'a, u8, R8_USB_INT_FG_SPEC, bool, 1>;
#[doc = "Field `RB_UIF_SUSPEND` reader - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
pub type RB_UIF_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIF_SUSPEND` writer - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
pub type RB_UIF_SUSPEND_W<'a> = crate::BitWriter<'a, u8, R8_USB_INT_FG_SPEC, bool, 2>;
#[doc = "Field `RB_UIF_HST_SOF` reader - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
pub type RB_UIF_HST_SOF_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIF_HST_SOF` writer - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
pub type RB_UIF_HST_SOF_W<'a> = crate::BitWriter<'a, u8, R8_USB_INT_FG_SPEC, bool, 3>;
#[doc = "Field `RB_UIF_FIFO_OV` reader - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
pub type RB_UIF_FIFO_OV_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIF_FIFO_OV` writer - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
pub type RB_UIF_FIFO_OV_W<'a> = crate::BitWriter<'a, u8, R8_USB_INT_FG_SPEC, bool, 4>;
#[doc = "Field `RB_U_SIE_FREE` reader - RO, indicate USB SIE free status"]
pub type RB_U_SIE_FREE_R = crate::BitReader<bool>;
#[doc = "Field `RB_U_TOG_OK` reader - RO, indicate current USB transfer toggle is OK"]
pub type RB_U_TOG_OK_R = crate::BitReader<bool>;
#[doc = "Field `RB_U_IS_NAK` reader - RO, indicate current USB transfer is NAK received"]
pub type RB_U_IS_NAK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_bus_rst__rb_uif_detect(&self) -> RB_UIF_BUS_RST__RB_UIF_DETECT_R {
        RB_UIF_BUS_RST__RB_UIF_DETECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_transfer(&self) -> RB_UIF_TRANSFER_R {
        RB_UIF_TRANSFER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_suspend(&self) -> RB_UIF_SUSPEND_R {
        RB_UIF_SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_hst_sof(&self) -> RB_UIF_HST_SOF_R {
        RB_UIF_HST_SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_fifo_ov(&self) -> RB_UIF_FIFO_OV_R {
        RB_UIF_FIFO_OV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RO, indicate USB SIE free status"]
    #[inline(always)]
    pub fn rb_u_sie_free(&self) -> RB_U_SIE_FREE_R {
        RB_U_SIE_FREE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RO, indicate current USB transfer toggle is OK"]
    #[inline(always)]
    pub fn rb_u_tog_ok(&self) -> RB_U_TOG_OK_R {
        RB_U_TOG_OK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, indicate current USB transfer is NAK received"]
    #[inline(always)]
    pub fn rb_u_is_nak(&self) -> RB_U_IS_NAK_R {
        RB_U_IS_NAK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_bus_rst__rb_uif_detect(&mut self) -> RB_UIF_BUS_RST__RB_UIF_DETECT_W {
        RB_UIF_BUS_RST__RB_UIF_DETECT_W::new(self)
    }
    #[doc = "Bit 1 - USB transfer completion interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_transfer(&mut self) -> RB_UIF_TRANSFER_W {
        RB_UIF_TRANSFER_W::new(self)
    }
    #[doc = "Bit 2 - USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_suspend(&mut self) -> RB_UIF_SUSPEND_W {
        RB_UIF_SUSPEND_W::new(self)
    }
    #[doc = "Bit 3 - host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_hst_sof(&mut self) -> RB_UIF_HST_SOF_W {
        RB_UIF_HST_SOF_W::new(self)
    }
    #[doc = "Bit 4 - FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear"]
    #[inline(always)]
    pub fn rb_uif_fifo_ov(&mut self) -> RB_UIF_FIFO_OV_W {
        RB_UIF_FIFO_OV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_usb_int_fg](index.html) module"]
pub struct R8_USB_INT_FG_SPEC;
impl crate::RegisterSpec for R8_USB_INT_FG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_usb_int_fg::R](R) reader structure"]
impl crate::Readable for R8_USB_INT_FG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_usb_int_fg::W](W) writer structure"]
impl crate::Writable for R8_USB_INT_FG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_USB_INT_FG to value 0"]
impl crate::Resettable for R8_USB_INT_FG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}