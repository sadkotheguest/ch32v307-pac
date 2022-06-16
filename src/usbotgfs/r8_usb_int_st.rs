#[doc = "Register `R8_USB_INT_ST` reader"]
pub struct R(crate::R<R8_USB_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_USB_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_USB_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_USB_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MASK_UIS_H_RES__MASK_UIS_ENDP` reader - RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode"]
pub type MASK_UIS_H_RES__MASK_UIS_ENDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_UIS_TOKEN` reader - RO, bit mask of current token PID code received for USB device mode"]
pub type MASK_UIS_TOKEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_UIS_TOG_OK` reader - RO, indicate current USB transfer toggle is OK"]
pub type RB_UIS_TOG_OK_R = crate::BitReader<bool>;
#[doc = "Field `RB_UIS_IS_NAK` reader - RO, indicate current USB transfer is NAK received for USB device mode"]
pub type RB_UIS_IS_NAK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode"]
    #[inline(always)]
    pub fn mask_uis_h_res__mask_uis_endp(&self) -> MASK_UIS_H_RES__MASK_UIS_ENDP_R {
        MASK_UIS_H_RES__MASK_UIS_ENDP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - RO, bit mask of current token PID code received for USB device mode"]
    #[inline(always)]
    pub fn mask_uis_token(&self) -> MASK_UIS_TOKEN_R {
        MASK_UIS_TOKEN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - RO, indicate current USB transfer toggle is OK"]
    #[inline(always)]
    pub fn rb_uis_tog_ok(&self) -> RB_UIS_TOG_OK_R {
        RB_UIS_TOG_OK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RO, indicate current USB transfer is NAK received for USB device mode"]
    #[inline(always)]
    pub fn rb_uis_is_nak(&self) -> RB_UIS_IS_NAK_R {
        RB_UIS_IS_NAK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "USB interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_usb_int_st](index.html) module"]
pub struct R8_USB_INT_ST_SPEC;
impl crate::RegisterSpec for R8_USB_INT_ST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_usb_int_st::R](R) reader structure"]
impl crate::Readable for R8_USB_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R8_USB_INT_ST to value 0"]
impl crate::Resettable for R8_USB_INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}