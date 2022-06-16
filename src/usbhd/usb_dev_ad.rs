#[doc = "Register `USB_DEV_AD` reader"]
pub struct R(crate::R<USB_DEV_AD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_DEV_AD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_DEV_AD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_DEV_AD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_DEV_AD` writer"]
pub struct W(crate::W<USB_DEV_AD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_DEV_AD_SPEC>;
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
impl From<crate::W<USB_DEV_AD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_DEV_AD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK_USB_ADDR` reader - bit mask for USB device address"]
pub type MASK_USB_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_USB_ADDR` writer - bit mask for USB device address"]
pub type MASK_USB_ADDR_W<'a> = crate::FieldWriter<'a, u8, USB_DEV_AD_SPEC, u8, u8, 7, 0>;
#[doc = "Field `RB_UDA_GP_BIT` reader - general purpose bit"]
pub type RB_UDA_GP_BIT_R = crate::BitReader<bool>;
#[doc = "Field `RB_UDA_GP_BIT` writer - general purpose bit"]
pub type RB_UDA_GP_BIT_W<'a> = crate::BitWriter<'a, u8, USB_DEV_AD_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:6 - bit mask for USB device address"]
    #[inline(always)]
    pub fn mask_usb_addr(&self) -> MASK_USB_ADDR_R {
        MASK_USB_ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - general purpose bit"]
    #[inline(always)]
    pub fn rb_uda_gp_bit(&self) -> RB_UDA_GP_BIT_R {
        RB_UDA_GP_BIT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - bit mask for USB device address"]
    #[inline(always)]
    pub fn mask_usb_addr(&mut self) -> MASK_USB_ADDR_W {
        MASK_USB_ADDR_W::new(self)
    }
    #[doc = "Bit 7 - general purpose bit"]
    #[inline(always)]
    pub fn rb_uda_gp_bit(&mut self) -> RB_UDA_GP_BIT_W {
        RB_UDA_GP_BIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB device address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_dev_ad](index.html) module"]
pub struct USB_DEV_AD_SPEC;
impl crate::RegisterSpec for USB_DEV_AD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb_dev_ad::R](R) reader structure"]
impl crate::Readable for USB_DEV_AD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_dev_ad::W](W) writer structure"]
impl crate::Writable for USB_DEV_AD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_DEV_AD to value 0"]
impl crate::Resettable for USB_DEV_AD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}