#[doc = "Register `UEP3_T_LEN___UH_TX_LEN_H` reader"]
pub struct R(crate::R<UEP3_T_LEN___UH_TX_LEN_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP3_T_LEN___UH_TX_LEN_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP3_T_LEN___UH_TX_LEN_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP3_T_LEN___UH_TX_LEN_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP3_T_LEN___UH_TX_LEN_H` writer"]
pub struct W(crate::W<UEP3_T_LEN___UH_TX_LEN_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP3_T_LEN___UH_TX_LEN_H_SPEC>;
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
impl From<crate::W<UEP3_T_LEN___UH_TX_LEN_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP3_T_LEN___UH_TX_LEN_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEP3_T_LEN___UH_TX_LEN_H` reader - endpoint 3 send the length"]
pub type UEP3_T_LEN___UH_TX_LEN_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UEP3_T_LEN___UH_TX_LEN_H` writer - endpoint 3 send the length"]
pub type UEP3_T_LEN___UH_TX_LEN_H_W<'a> =
    crate::FieldWriter<'a, u16, UEP3_T_LEN___UH_TX_LEN_H_SPEC, u16, u16, 11, 0>;
impl R {
    #[doc = "Bits 0:10 - endpoint 3 send the length"]
    #[inline(always)]
    pub fn uep3_t_len___uh_tx_len_h(&self) -> UEP3_T_LEN___UH_TX_LEN_H_R {
        UEP3_T_LEN___UH_TX_LEN_H_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - endpoint 3 send the length"]
    #[inline(always)]
    pub fn uep3_t_len___uh_tx_len_h(&mut self) -> UEP3_T_LEN___UH_TX_LEN_H_W {
        UEP3_T_LEN___UH_TX_LEN_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 3 send the length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep3_t_len___uh_tx_len_h](index.html) module"]
pub struct UEP3_T_LEN___UH_TX_LEN_H_SPEC;
impl crate::RegisterSpec for UEP3_T_LEN___UH_TX_LEN_H_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uep3_t_len___uh_tx_len_h::R](R) reader structure"]
impl crate::Readable for UEP3_T_LEN___UH_TX_LEN_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep3_t_len___uh_tx_len_h::W](W) writer structure"]
impl crate::Writable for UEP3_T_LEN___UH_TX_LEN_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP3_T_LEN___UH_TX_LEN_H to value 0"]
impl crate::Resettable for UEP3_T_LEN___UH_TX_LEN_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}