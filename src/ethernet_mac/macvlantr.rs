#[doc = "Register `MACVLANTR` reader"]
pub struct R(crate::R<MACVLANTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACVLANTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACVLANTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACVLANTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACVLANTR` writer"]
pub struct W(crate::W<MACVLANTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACVLANTR_SPEC>;
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
impl From<crate::W<MACVLANTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACVLANTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLANTI` reader - VLAN tag identifier (for receive frames)"]
pub type VLANTI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VLANTI` writer - VLAN tag identifier (for receive frames)"]
pub type VLANTI_W<'a> = crate::FieldWriter<'a, u32, MACVLANTR_SPEC, u16, u16, 16, 0>;
#[doc = "Field `VLANTC` reader - 12-bit VLAN tag comparison"]
pub type VLANTC_R = crate::BitReader<bool>;
#[doc = "Field `VLANTC` writer - 12-bit VLAN tag comparison"]
pub type VLANTC_W<'a> = crate::BitWriter<'a, u32, MACVLANTR_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlanti(&self) -> VLANTI_R {
        VLANTI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vlantc(&self) -> VLANTC_R {
        VLANTC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlanti(&mut self) -> VLANTI_W {
        VLANTI_W::new(self)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vlantc(&mut self) -> VLANTC_W {
        VLANTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC VLAN tag register (ETH_MACVLANTR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macvlantr](index.html) module"]
pub struct MACVLANTR_SPEC;
impl crate::RegisterSpec for MACVLANTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macvlantr::R](R) reader structure"]
impl crate::Readable for MACVLANTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macvlantr::W](W) writer structure"]
impl crate::Writable for MACVLANTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACVLANTR to value 0"]
impl crate::Resettable for MACVLANTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}