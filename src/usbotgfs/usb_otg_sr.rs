#[doc = "Register `USB_OTG_SR` reader"]
pub struct R(crate::R<USB_OTG_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_OTG_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_OTG_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_OTG_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_OTG_SR` writer"]
pub struct W(crate::W<USB_OTG_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_OTG_SR_SPEC>;
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
impl From<crate::W<USB_OTG_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_OTG_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_OTG_SR_VBUS_VLD` reader - usb otg status"]
pub type USB_OTG_SR_VBUS_VLD_R = crate::BitReader<bool>;
#[doc = "Field `USB_OTG_SR_VBUS_VLD` writer - usb otg status"]
pub type USB_OTG_SR_VBUS_VLD_W<'a> = crate::BitWriter<'a, u32, USB_OTG_SR_SPEC, bool, 0>;
#[doc = "Field `USB_OTG_SR_SESS_VLD` reader - usb otg status"]
pub type USB_OTG_SR_SESS_VLD_R = crate::BitReader<bool>;
#[doc = "Field `USB_OTG_SR_SESS_VLD` writer - usb otg status"]
pub type USB_OTG_SR_SESS_VLD_W<'a> = crate::BitWriter<'a, u32, USB_OTG_SR_SPEC, bool, 1>;
#[doc = "Field `USB_OTG_SR_SESS_END` reader - usb otg status"]
pub type USB_OTG_SR_SESS_END_R = crate::BitReader<bool>;
#[doc = "Field `USB_OTG_SR_SESS_END` writer - usb otg status"]
pub type USB_OTG_SR_SESS_END_W<'a> = crate::BitWriter<'a, u32, USB_OTG_SR_SPEC, bool, 2>;
#[doc = "Field `USB_OTG_SR_ID_DIG` reader - usb otg status"]
pub type USB_OTG_SR_ID_DIG_R = crate::BitReader<bool>;
#[doc = "Field `USB_OTG_SR_ID_DIG` writer - usb otg status"]
pub type USB_OTG_SR_ID_DIG_W<'a> = crate::BitWriter<'a, u32, USB_OTG_SR_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - usb otg status"]
    #[inline(always)]
    pub fn usb_otg_sr_vbus_vld(&self) -> USB_OTG_SR_VBUS_VLD_R {
        USB_OTG_SR_VBUS_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - usb otg status"]
    #[inline(always)]
    pub fn usb_otg_sr_sess_vld(&self) -> USB_OTG_SR_SESS_VLD_R {
        USB_OTG_SR_SESS_VLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - usb otg status"]
    #[inline(always)]
    pub fn usb_otg_sr_sess_end(&self) -> USB_OTG_SR_SESS_END_R {
        USB_OTG_SR_SESS_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - usb otg status"]
    #[inline(always)]
    pub fn usb_otg_sr_id_dig(&self) -> USB_OTG_SR_ID_DIG_R {
        USB_OTG_SR_ID_DIG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - usb otg status"]
    #[inline(always)]
    pub fn usb_otg_sr_vbus_vld(&mut self) -> USB_OTG_SR_VBUS_VLD_W {
        USB_OTG_SR_VBUS_VLD_W::new(self)
    }
    #[doc = "Bit 1 - usb otg status"]
    #[inline(always)]
    pub fn usb_otg_sr_sess_vld(&mut self) -> USB_OTG_SR_SESS_VLD_W {
        USB_OTG_SR_SESS_VLD_W::new(self)
    }
    #[doc = "Bit 2 - usb otg status"]
    #[inline(always)]
    pub fn usb_otg_sr_sess_end(&mut self) -> USB_OTG_SR_SESS_END_W {
        USB_OTG_SR_SESS_END_W::new(self)
    }
    #[doc = "Bit 3 - usb otg status"]
    #[inline(always)]
    pub fn usb_otg_sr_id_dig(&mut self) -> USB_OTG_SR_ID_DIG_W {
        USB_OTG_SR_ID_DIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb otg status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_otg_sr](index.html) module"]
pub struct USB_OTG_SR_SPEC;
impl crate::RegisterSpec for USB_OTG_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_otg_sr::R](R) reader structure"]
impl crate::Readable for USB_OTG_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_otg_sr::W](W) writer structure"]
impl crate::Writable for USB_OTG_SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_OTG_SR to value 0"]
impl crate::Resettable for USB_OTG_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}