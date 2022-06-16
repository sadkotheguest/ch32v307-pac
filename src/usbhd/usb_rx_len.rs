#[doc = "Register `USB_RX_LEN` reader"]
pub struct R(crate::R<USB_RX_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_RX_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_RX_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_RX_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `R16_USB_RX_LEN` reader - length of received bytes"]
pub type R16_USB_RX_LEN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - length of received bytes"]
    #[inline(always)]
    pub fn r16_usb_rx_len(&self) -> R16_USB_RX_LEN_R {
        R16_USB_RX_LEN_R::new(self.bits)
    }
}
#[doc = "USB receiving length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_rx_len](index.html) module"]
pub struct USB_RX_LEN_SPEC;
impl crate::RegisterSpec for USB_RX_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [usb_rx_len::R](R) reader structure"]
impl crate::Readable for USB_RX_LEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USB_RX_LEN to value 0"]
impl crate::Resettable for USB_RX_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}