#[doc = "Register `R16_USB_RX_LEN` reader"]
pub struct R(crate::R<R16_USB_RX_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R16_USB_RX_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R16_USB_RX_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R16_USB_RX_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "USB receiving length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16_usb_rx_len](index.html) module"]
pub struct R16_USB_RX_LEN_SPEC;
impl crate::RegisterSpec for R16_USB_RX_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [r16_usb_rx_len::R](R) reader structure"]
impl crate::Readable for R16_USB_RX_LEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R16_USB_RX_LEN to value 0"]
impl crate::Resettable for R16_USB_RX_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}