    #[doc = "Register `USB_FRAME_NO` reader"]
    pub struct R(crate::R<USB_FRAME_NO_SPEC>);
    impl core::ops::Deref for R {
        type Target = crate::R<USB_FRAME_NO_SPEC>;
        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl From<crate::R<USB_FRAME_NO_SPEC>> for R {
        #[inline(always)]
        fn from(reader: crate::R<USB_FRAME_NO_SPEC>) -> Self {
            R(reader)
        }
    }
    #[doc = "Register `USB_FRAME_NO` writer"]
    pub struct W(crate::W<USB_FRAME_NO_SPEC>);
    impl core::ops::Deref for W {
        type Target = crate::W<USB_FRAME_NO_SPEC>;
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
    impl From<crate::W<USB_FRAME_NO_SPEC>> for W {
        #[inline(always)]
        fn from(writer: crate::W<USB_FRAME_NO_SPEC>) -> Self {
            W(writer)
        }
    }
    #[doc = "Field `USB_FRAME_NO` reader - USB_FRAME_NO"]
    pub type USB_FRAME_NO_R = crate::FieldReader<u16, u16>;
    #[doc = "Field `USB_FRAME_NO` writer - USB_FRAME_NO"]
    pub type USB_FRAME_NO_W<'a> =
        crate::FieldWriter<'a, u16, USB_FRAME_NO_SPEC, u16, u16, 16, 0>;
    impl R {
        #[doc = "Bits 0:15 - USB_FRAME_NO"]
        #[inline(always)]
        pub fn usb_frame_no(&self) -> USB_FRAME_NO_R {
            USB_FRAME_NO_R::new((self.bits & 0xffff) as u16)
        }
    }
    impl W {
        #[doc = "Bits 0:15 - USB_FRAME_NO"]
        #[inline(always)]
        pub fn usb_frame_no(&mut self) -> USB_FRAME_NO_W {
            USB_FRAME_NO_W::new(self)
        }
        #[doc = "Writes raw bits to the register."]
        #[inline(always)]
        pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
            self.0.bits(bits);
            self
        }
    }
    #[doc = "USB_FRAME_NO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_frame_no](index.html) module"]
    pub struct USB_FRAME_NO_SPEC;
    impl crate::RegisterSpec for USB_FRAME_NO_SPEC {
        type Ux = u16;
    }
    #[doc = "`read()` method returns [usb_frame_no::R](R) reader structure"]
    impl crate::Readable for USB_FRAME_NO_SPEC {
        type Reader = R;
    }
    #[doc = "`write(|w| ..)` method takes [usb_frame_no::W](W) writer structure"]
    impl crate::Writable for USB_FRAME_NO_SPEC {
        type Writer = W;
    }
    #[doc = "`reset()` method sets USB_FRAME_NO to value 0"]
    impl crate::Resettable for USB_FRAME_NO_SPEC {
        #[inline(always)]
        fn reset_value() -> Self::Ux {
            0
        }
    }