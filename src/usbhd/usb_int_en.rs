#[doc = "Register `USB_INT_EN` reader"]
pub struct R(crate::R<USB_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_INT_EN` writer"]
pub struct W(crate::W<USB_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_INT_EN_SPEC>;
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
impl From<crate::W<USB_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_UIE_BUS_RST__RB_UIE_DETECT` reader - enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode"]
pub type RB_UIE_BUS_RST__RB_UIE_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIE_BUS_RST__RB_UIE_DETECT` writer - enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode"]
pub type RB_UIE_BUS_RST__RB_UIE_DETECT_W<'a> =
    crate::BitWriter<'a, u8, USB_INT_EN_SPEC, bool, 0>;
#[doc = "Field `RB_UIE_TRANSFER` reader - enable interrupt for USB transfer completion"]
pub type RB_UIE_TRANSFER_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIE_TRANSFER` writer - enable interrupt for USB transfer completion"]
pub type RB_UIE_TRANSFER_W<'a> = crate::BitWriter<'a, u8, USB_INT_EN_SPEC, bool, 1>;
#[doc = "Field `RB_UIE_SUSPEND` reader - enable interrupt for USB suspend or resume event"]
pub type RB_UIE_SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIE_SUSPEND` writer - enable interrupt for USB suspend or resume event"]
pub type RB_UIE_SUSPEND_W<'a> = crate::BitWriter<'a, u8, USB_INT_EN_SPEC, bool, 2>;
#[doc = "Field `RB_UIE_SOF_ACT` reader - indicate host SOF timer action status for USB host"]
pub type RB_UIE_SOF_ACT_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIE_SOF_ACT` writer - indicate host SOF timer action status for USB host"]
pub type RB_UIE_SOF_ACT_W<'a> = crate::BitWriter<'a, u8, USB_INT_EN_SPEC, bool, 3>;
#[doc = "Field `RB_UIE_FIFO_OV` reader - enable interrupt for FIFO overflow"]
pub type RB_UIE_FIFO_OV_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIE_FIFO_OV` writer - enable interrupt for FIFO overflow"]
pub type RB_UIE_FIFO_OV_W<'a> = crate::BitWriter<'a, u8, USB_INT_EN_SPEC, bool, 4>;
#[doc = "Field `RB_UIE_SETUP_ACT` reader - indicate host SETUP timer action status for USB host"]
pub type RB_UIE_SETUP_ACT_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIE_SETUP_ACT` writer - indicate host SETUP timer action status for USB host"]
pub type RB_UIE_SETUP_ACT_W<'a> = crate::BitWriter<'a, u8, USB_INT_EN_SPEC, bool, 5>;
#[doc = "Field `RB_UIE_ISO_ACT` reader - enable interrupt for NAK responded for USB device mode"]
pub type RB_UIE_ISO_ACT_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIE_ISO_ACT` writer - enable interrupt for NAK responded for USB device mode"]
pub type RB_UIE_ISO_ACT_W<'a> = crate::BitWriter<'a, u8, USB_INT_EN_SPEC, bool, 6>;
#[doc = "Field `RB_UIE_DEV_NAK` reader - enable interrupt for NAK responded for USB device mode"]
pub type RB_UIE_DEV_NAK_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIE_DEV_NAK` writer - enable interrupt for NAK responded for USB device mode"]
pub type RB_UIE_DEV_NAK_W<'a> = crate::BitWriter<'a, u8, USB_INT_EN_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode"]
    #[inline(always)]
    pub fn rb_uie_bus_rst__rb_uie_detect(&self) -> RB_UIE_BUS_RST__RB_UIE_DETECT_R {
        RB_UIE_BUS_RST__RB_UIE_DETECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    pub fn rb_uie_transfer(&self) -> RB_UIE_TRANSFER_R {
        RB_UIE_TRANSFER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    pub fn rb_uie_suspend(&self) -> RB_UIE_SUSPEND_R {
        RB_UIE_SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - indicate host SOF timer action status for USB host"]
    #[inline(always)]
    pub fn rb_uie_sof_act(&self) -> RB_UIE_SOF_ACT_R {
        RB_UIE_SOF_ACT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    pub fn rb_uie_fifo_ov(&self) -> RB_UIE_FIFO_OV_R {
        RB_UIE_FIFO_OV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - indicate host SETUP timer action status for USB host"]
    #[inline(always)]
    pub fn rb_uie_setup_act(&self) -> RB_UIE_SETUP_ACT_R {
        RB_UIE_SETUP_ACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_uie_iso_act(&self) -> RB_UIE_ISO_ACT_R {
        RB_UIE_ISO_ACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_uie_dev_nak(&self) -> RB_UIE_DEV_NAK_R {
        RB_UIE_DEV_NAK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode"]
    #[inline(always)]
    pub fn rb_uie_bus_rst__rb_uie_detect(&mut self) -> RB_UIE_BUS_RST__RB_UIE_DETECT_W {
        RB_UIE_BUS_RST__RB_UIE_DETECT_W::new(self)
    }
    #[doc = "Bit 1 - enable interrupt for USB transfer completion"]
    #[inline(always)]
    pub fn rb_uie_transfer(&mut self) -> RB_UIE_TRANSFER_W {
        RB_UIE_TRANSFER_W::new(self)
    }
    #[doc = "Bit 2 - enable interrupt for USB suspend or resume event"]
    #[inline(always)]
    pub fn rb_uie_suspend(&mut self) -> RB_UIE_SUSPEND_W {
        RB_UIE_SUSPEND_W::new(self)
    }
    #[doc = "Bit 3 - indicate host SOF timer action status for USB host"]
    #[inline(always)]
    pub fn rb_uie_sof_act(&mut self) -> RB_UIE_SOF_ACT_W {
        RB_UIE_SOF_ACT_W::new(self)
    }
    #[doc = "Bit 4 - enable interrupt for FIFO overflow"]
    #[inline(always)]
    pub fn rb_uie_fifo_ov(&mut self) -> RB_UIE_FIFO_OV_W {
        RB_UIE_FIFO_OV_W::new(self)
    }
    #[doc = "Bit 5 - indicate host SETUP timer action status for USB host"]
    #[inline(always)]
    pub fn rb_uie_setup_act(&mut self) -> RB_UIE_SETUP_ACT_W {
        RB_UIE_SETUP_ACT_W::new(self)
    }
    #[doc = "Bit 6 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_uie_iso_act(&mut self) -> RB_UIE_ISO_ACT_W {
        RB_UIE_ISO_ACT_W::new(self)
    }
    #[doc = "Bit 7 - enable interrupt for NAK responded for USB device mode"]
    #[inline(always)]
    pub fn rb_uie_dev_nak(&mut self) -> RB_UIE_DEV_NAK_W {
        RB_UIE_DEV_NAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_int_en](index.html) module"]
pub struct USB_INT_EN_SPEC;
impl crate::RegisterSpec for USB_INT_EN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb_int_en::R](R) reader structure"]
impl crate::Readable for USB_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_int_en::W](W) writer structure"]
impl crate::Writable for USB_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_INT_EN to value 0"]
impl crate::Resettable for USB_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}