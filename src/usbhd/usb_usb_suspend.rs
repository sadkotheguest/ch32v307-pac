#[doc = "Register `USB_USB_SUSPEND` reader"]
pub struct R(crate::R<USB_USB_SUSPEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_USB_SUSPEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_USB_SUSPEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_USB_SUSPEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_USB_SUSPEND` writer"]
pub struct W(crate::W<USB_USB_SUSPEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_USB_SUSPEND_SPEC>;
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
impl From<crate::W<USB_USB_SUSPEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_USB_SUSPEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_SYS_MOD` reader - USB_SYS_MOD"]
pub type USB_SYS_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_SYS_MOD` writer - USB_SYS_MOD"]
pub type USB_SYS_MOD_W<'a> = crate::FieldWriter<'a, u8, USB_USB_SUSPEND_SPEC, u8, u8, 2, 0>;
#[doc = "Field `USB_WAKEUP` reader - remote resume"]
pub type USB_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `USB_WAKEUP` writer - remote resume"]
pub type USB_WAKEUP_W<'a> = crate::BitWriter<'a, u8, USB_USB_SUSPEND_SPEC, bool, 2>;
#[doc = "Field `USB_LINESTATE` reader - USB_LINESTATE"]
pub type USB_LINESTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_LINESTATE` writer - USB_LINESTATE"]
pub type USB_LINESTATE_W<'a> =
    crate::FieldWriter<'a, u8, USB_USB_SUSPEND_SPEC, u8, u8, 2, 4>;
impl R {
    #[doc = "Bits 0:1 - USB_SYS_MOD"]
    #[inline(always)]
    pub fn usb_sys_mod(&self) -> USB_SYS_MOD_R {
        USB_SYS_MOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - remote resume"]
    #[inline(always)]
    pub fn usb_wakeup(&self) -> USB_WAKEUP_R {
        USB_WAKEUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USB_LINESTATE"]
    #[inline(always)]
    pub fn usb_linestate(&self) -> USB_LINESTATE_R {
        USB_LINESTATE_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB_SYS_MOD"]
    #[inline(always)]
    pub fn usb_sys_mod(&mut self) -> USB_SYS_MOD_W {
        USB_SYS_MOD_W::new(self)
    }
    #[doc = "Bit 2 - remote resume"]
    #[inline(always)]
    pub fn usb_wakeup(&mut self) -> USB_WAKEUP_W {
        USB_WAKEUP_W::new(self)
    }
    #[doc = "Bits 4:5 - USB_LINESTATE"]
    #[inline(always)]
    pub fn usb_linestate(&mut self) -> USB_LINESTATE_W {
        USB_LINESTATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "indicate USB suspend status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_usb_suspend](index.html) module"]
pub struct USB_USB_SUSPEND_SPEC;
impl crate::RegisterSpec for USB_USB_SUSPEND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usb_usb_suspend::R](R) reader structure"]
impl crate::Readable for USB_USB_SUSPEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_usb_suspend::W](W) writer structure"]
impl crate::Writable for USB_USB_SUSPEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_USB_SUSPEND to value 0"]
impl crate::Resettable for USB_USB_SUSPEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}