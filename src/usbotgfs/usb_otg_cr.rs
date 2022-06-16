#[doc = "Register `USB_OTG_CR` reader"]
pub struct R(crate::R<USB_OTG_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_OTG_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_OTG_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_OTG_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_OTG_CR` writer"]
pub struct W(crate::W<USB_OTG_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_OTG_CR_SPEC>;
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
impl From<crate::W<USB_OTG_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_OTG_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_OTG_CR_DISCHARGEVBUS` reader - usb otg control"]
pub type USB_OTG_CR_DISCHARGEVBUS_R = crate::BitReader<bool>;
#[doc = "Field `USB_OTG_CR_DISCHARGEVBUS` writer - usb otg control"]
pub type USB_OTG_CR_DISCHARGEVBUS_W<'a> =
    crate::BitWriter<'a, u32, USB_OTG_CR_SPEC, bool, 0>;
#[doc = "Field `USB_OTG_CR_CHARGEVBUS` reader - usb otg control"]
pub type USB_OTG_CR_CHARGEVBUS_R = crate::BitReader<bool>;
#[doc = "Field `USB_OTG_CR_CHARGEVBUS` writer - usb otg control"]
pub type USB_OTG_CR_CHARGEVBUS_W<'a> = crate::BitWriter<'a, u32, USB_OTG_CR_SPEC, bool, 1>;
#[doc = "Field `USB_OTG_CR_IDPU` reader - usb otg control"]
pub type USB_OTG_CR_IDPU_R = crate::BitReader<bool>;
#[doc = "Field `USB_OTG_CR_IDPU` writer - usb otg control"]
pub type USB_OTG_CR_IDPU_W<'a> = crate::BitWriter<'a, u32, USB_OTG_CR_SPEC, bool, 2>;
#[doc = "Field `USB_OTG_CR_OTG_EN` reader - usb otg control"]
pub type USB_OTG_CR_OTG_EN_R = crate::BitReader<bool>;
#[doc = "Field `USB_OTG_CR_OTG_EN` writer - usb otg control"]
pub type USB_OTG_CR_OTG_EN_W<'a> = crate::BitWriter<'a, u32, USB_OTG_CR_SPEC, bool, 3>;
#[doc = "Field `USB_OTG_CR_VBUS` reader - usb otg control"]
pub type USB_OTG_CR_VBUS_R = crate::BitReader<bool>;
#[doc = "Field `USB_OTG_CR_VBUS` writer - usb otg control"]
pub type USB_OTG_CR_VBUS_W<'a> = crate::BitWriter<'a, u32, USB_OTG_CR_SPEC, bool, 4>;
#[doc = "Field `USB_OTG_CR_SESS` reader - usb otg control"]
pub type USB_OTG_CR_SESS_R = crate::BitReader<bool>;
#[doc = "Field `USB_OTG_CR_SESS` writer - usb otg control"]
pub type USB_OTG_CR_SESS_W<'a> = crate::BitWriter<'a, u32, USB_OTG_CR_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_dischargevbus(&self) -> USB_OTG_CR_DISCHARGEVBUS_R {
        USB_OTG_CR_DISCHARGEVBUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_chargevbus(&self) -> USB_OTG_CR_CHARGEVBUS_R {
        USB_OTG_CR_CHARGEVBUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_idpu(&self) -> USB_OTG_CR_IDPU_R {
        USB_OTG_CR_IDPU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_otg_en(&self) -> USB_OTG_CR_OTG_EN_R {
        USB_OTG_CR_OTG_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_vbus(&self) -> USB_OTG_CR_VBUS_R {
        USB_OTG_CR_VBUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_sess(&self) -> USB_OTG_CR_SESS_R {
        USB_OTG_CR_SESS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_dischargevbus(&mut self) -> USB_OTG_CR_DISCHARGEVBUS_W {
        USB_OTG_CR_DISCHARGEVBUS_W::new(self)
    }
    #[doc = "Bit 1 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_chargevbus(&mut self) -> USB_OTG_CR_CHARGEVBUS_W {
        USB_OTG_CR_CHARGEVBUS_W::new(self)
    }
    #[doc = "Bit 2 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_idpu(&mut self) -> USB_OTG_CR_IDPU_W {
        USB_OTG_CR_IDPU_W::new(self)
    }
    #[doc = "Bit 3 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_otg_en(&mut self) -> USB_OTG_CR_OTG_EN_W {
        USB_OTG_CR_OTG_EN_W::new(self)
    }
    #[doc = "Bit 4 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_vbus(&mut self) -> USB_OTG_CR_VBUS_W {
        USB_OTG_CR_VBUS_W::new(self)
    }
    #[doc = "Bit 5 - usb otg control"]
    #[inline(always)]
    pub fn usb_otg_cr_sess(&mut self) -> USB_OTG_CR_SESS_W {
        USB_OTG_CR_SESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb otg control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_otg_cr](index.html) module"]
pub struct USB_OTG_CR_SPEC;
impl crate::RegisterSpec for USB_OTG_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_otg_cr::R](R) reader structure"]
impl crate::Readable for USB_OTG_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_otg_cr::W](W) writer structure"]
impl crate::Writable for USB_OTG_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_OTG_CR to value 0"]
impl crate::Resettable for USB_OTG_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}