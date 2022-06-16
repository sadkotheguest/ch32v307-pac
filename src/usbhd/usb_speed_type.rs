#[doc = "Register `USB_SPEED_TYPE` reader"]
pub struct R(crate::R<USB_SPEED_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_SPEED_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_SPEED_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_SPEED_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_SPEED_TYPE` writer"]
pub struct W(crate::W<USB_SPEED_TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_SPEED_TYPE_SPEC>;
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
impl From<crate::W<USB_SPEED_TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_SPEED_TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_SPEED_TYPE` reader - USB_SPEED_TYPE"]
pub type USB_SPEED_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_SPEED_TYPE` writer - USB_SPEED_TYPE"]
pub type USB_SPEED_TYPE_W<'a> =
    crate::FieldWriter<'a, u8, USB_SPEED_TYPE_SPEC, u8, u8, 2, 0>;
impl R {
    #[doc = "Bits 0:1 - USB_SPEED_TYPE"]
    #[inline(always)]
    pub fn usb_speed_type(&self) -> USB_SPEED_TYPE_R {
        USB_SPEED_TYPE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB_SPEED_TYPE"]
    #[inline(always)]
    pub fn usb_speed_type(&mut self) -> USB_SPEED_TYPE_W {
        USB_SPEED_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_SPEED_TYPE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_speed_type](index.html) module"]
pub struct USB_SPEED_TYPE_SPEC;
impl crate::RegisterSpec for USB_SPEED_TYPE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb_speed_type::R](R) reader structure"]
impl crate::Readable for USB_SPEED_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_speed_type::W](W) writer structure"]
impl crate::Writable for USB_SPEED_TYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_SPEED_TYPE to value 0"]
impl crate::Resettable for USB_SPEED_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}