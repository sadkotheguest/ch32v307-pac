#[doc = "Register `CTLR1` reader"]
pub struct R(crate::R<CTLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLR1` writer"]
pub struct W(crate::W<CTLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLR1_SPEC>;
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
impl From<crate::W<CTLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_PGA` reader - ADC_PGA"]
pub type ADC_PGA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_PGA` writer - ADC_PGA"]
pub type ADC_PGA_W<'a> = crate::FieldWriter<'a, u32, CTLR1_SPEC, u8, u8, 2, 27>;
#[doc = "Field `ADC_BUF_EN` reader - TKEY_BUF_Enable"]
pub type ADC_BUF_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC_BUF_EN` writer - TKEY_BUF_Enable"]
pub type ADC_BUF_EN_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 26>;
#[doc = "Field `TKEY_ITUNE` reader - TKEY_I enable"]
pub type TKEY_ITUNE_R = crate::BitReader<bool>;
#[doc = "Field `TKEY_ITUNE` writer - TKEY_I enable"]
pub type TKEY_ITUNE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 25>;
#[doc = "Field `TKEY_EN` reader - TKEY enable, including TKEY_F and TKEY_V"]
pub type TKEY_EN_R = crate::BitReader<bool>;
#[doc = "Field `TKEY_EN` writer - TKEY enable, including TKEY_F and TKEY_V"]
pub type TKEY_EN_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 24>;
#[doc = "Field `RAWDEN` reader - Analog watchdog enable on regular channels"]
pub type RAWDEN_R = crate::BitReader<bool>;
#[doc = "Field `RAWDEN` writer - Analog watchdog enable on regular channels"]
pub type RAWDEN_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 23>;
#[doc = "Field `IAWDEN` reader - Analog watchdog enable on injected channels"]
pub type IAWDEN_R = crate::BitReader<bool>;
#[doc = "Field `IAWDEN` writer - Analog watchdog enable on injected channels"]
pub type IAWDEN_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 22>;
#[doc = "Field `DUALMOD` reader - Dual mode selection"]
pub type DUALMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUALMOD` writer - Dual mode selection"]
pub type DUALMOD_W<'a> = crate::FieldWriter<'a, u32, CTLR1_SPEC, u8, u8, 4, 16>;
// #[doc = "Field `IAWDEN` reader - Analog watchdog enable on injected channels"]
// pub type IAWDEN_R = crate::BitReader<bool>;
// #[doc = "Field `IAWDEN` writer - Analog watchdog enable on injected channels"]
// pub type IAWDEN_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 22>;
#[doc = "Field `DISCNUM` reader - Discontinuous mode channel count"]
pub type DISCNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISCNUM` writer - Discontinuous mode channel count"]
pub type DISCNUM_W<'a> = crate::FieldWriter<'a, u32, CTLR1_SPEC, u8, u8, 3, 13>;
#[doc = "Field `IDISCEN` reader - Discontinuous mode on injected channels"]
pub type IDISCEN_R = crate::BitReader<bool>;
#[doc = "Field `IDISCEN` writer - Discontinuous mode on injected channels"]
pub type IDISCEN_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 12>;
#[doc = "Field `RDISCEN` reader - Discontinuous mode on regular channels"]
pub type RDISCEN_R = crate::BitReader<bool>;
#[doc = "Field `RDISCEN` writer - Discontinuous mode on regular channels"]
pub type RDISCEN_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 11>;
#[doc = "Field `IAUTO` reader - Automatic injected group conversion"]
pub type IAUTO_R = crate::BitReader<bool>;
#[doc = "Field `IAUTO` writer - Automatic injected group conversion"]
pub type IAUTO_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 10>;
#[doc = "Field `AWDSGL` reader - Enable the watchdog on a single channel in scan mode"]
pub type AWDSGL_R = crate::BitReader<bool>;
#[doc = "Field `AWDSGL` writer - Enable the watchdog on a single channel in scan mode"]
pub type AWDSGL_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 9>;
#[doc = "Field `SCAN` reader - Scan mode enable"]
pub type SCAN_R = crate::BitReader<bool>;
#[doc = "Field `SCAN` writer - Scan mode enable"]
pub type SCAN_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 8>;
#[doc = "Field `IEOCIE` reader - Interrupt enable for injected channels"]
pub type IEOCIE_R = crate::BitReader<bool>;
#[doc = "Field `IEOCIE` writer - Interrupt enable for injected channels"]
pub type IEOCIE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 7>;
#[doc = "Field `AWDIE` reader - Analog watchdog interrupt enable"]
pub type AWDIE_R = crate::BitReader<bool>;
#[doc = "Field `AWDIE` writer - Analog watchdog interrupt enable"]
pub type AWDIE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 6>;
#[doc = "Field `EOCIE` reader - Interrupt enable for EOC"]
pub type EOCIE_R = crate::BitReader<bool>;
#[doc = "Field `EOCIE` writer - Interrupt enable for EOC"]
pub type EOCIE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 5>;
#[doc = "Field `AWDCH` reader - Analog watchdog channel select bits"]
pub type AWDCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWDCH` writer - Analog watchdog channel select bits"]
pub type AWDCH_W<'a> = crate::FieldWriter<'a, u32, CTLR1_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 27:28 - ADC_PGA"]
    #[inline(always)]
    pub fn adc_pga(&self) -> ADC_PGA_R {
        ADC_PGA_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 26 - TKEY_BUF_Enable"]
    #[inline(always)]
    pub fn adc_buf_en(&self) -> ADC_BUF_EN_R {
        ADC_BUF_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - TKEY_I enable"]
    #[inline(always)]
    pub fn tkey_itune(&self) -> TKEY_ITUNE_R {
        TKEY_ITUNE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - TKEY enable, including TKEY_F and TKEY_V"]
    #[inline(always)]
    pub fn tkey_en(&self) -> TKEY_EN_R {
        TKEY_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog enable on regular channels"]
    #[inline(always)]
    pub fn rawden(&self) -> RAWDEN_R {
        RAWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog watchdog enable on injected channels"]
    #[inline(always)]
    pub fn iawden(&self) -> IAWDEN_R {
        IAWDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Dual mode selection"]
    #[inline(always)]
    pub fn dualmod(&self) -> DUALMOD_R {
        DUALMOD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Discontinuous mode channel count"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline(always)]
    pub fn idiscen(&self) -> IDISCEN_R {
        IDISCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn rdiscen(&self) -> RDISCEN_R {
        RDISCEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Automatic injected group conversion"]
    #[inline(always)]
    pub fn iauto(&self) -> IAUTO_R {
        IAUTO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable the watchdog on a single channel in scan mode"]
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan mode enable"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for injected channels"]
    #[inline(always)]
    pub fn ieocie(&self) -> IEOCIE_R {
        IEOCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 0:4 - Analog watchdog channel select bits"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:28 - ADC_PGA"]
    #[inline(always)]
    pub fn adc_pga(&mut self) -> ADC_PGA_W {
        ADC_PGA_W::new(self)
    }
    #[doc = "Bit 26 - TKEY_BUF_Enable"]
    #[inline(always)]
    pub fn adc_buf_en(&mut self) -> ADC_BUF_EN_W {
        ADC_BUF_EN_W::new(self)
    }
    #[doc = "Bit 25 - TKEY_I enable"]
    #[inline(always)]
    pub fn tkey_itune(&mut self) -> TKEY_ITUNE_W {
        TKEY_ITUNE_W::new(self)
    }
    #[doc = "Bit 24 - TKEY enable, including TKEY_F and TKEY_V"]
    #[inline(always)]
    pub fn tkey_en(&mut self) -> TKEY_EN_W {
        TKEY_EN_W::new(self)
    }
    #[doc = "Bit 23 - Analog watchdog enable on regular channels"]
    #[inline(always)]
    pub fn rawden(&mut self) -> RAWDEN_W {
        RAWDEN_W::new(self)
    }
    #[doc = "Bit 22 - Analog watchdog enable on injected channels"]
    #[inline(always)]
    pub fn iawden(&mut self) -> IAWDEN_W {
        IAWDEN_W::new(self)
    }
    #[doc = "Bits 16:19 - Dual mode selection"]
    #[inline(always)]
    pub fn dualmod(&mut self) -> DUALMOD_W {
        DUALMOD_W::new(self)
    }
    #[doc = "Bits 13:15 - Discontinuous mode channel count"]
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W {
        DISCNUM_W::new(self)
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline(always)]
    pub fn idiscen(&mut self) -> IDISCEN_W {
        IDISCEN_W::new(self)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn rdiscen(&mut self) -> RDISCEN_W {
        RDISCEN_W::new(self)
    }
    #[doc = "Bit 10 - Automatic injected group conversion"]
    #[inline(always)]
    pub fn iauto(&mut self) -> IAUTO_W {
        IAUTO_W::new(self)
    }
    #[doc = "Bit 9 - Enable the watchdog on a single channel in scan mode"]
    #[inline(always)]
    pub fn awdsgl(&mut self) -> AWDSGL_W {
        AWDSGL_W::new(self)
    }
    #[doc = "Bit 8 - Scan mode enable"]
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W {
        SCAN_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt enable for injected channels"]
    #[inline(always)]
    pub fn ieocie(&mut self) -> IEOCIE_W {
        IEOCIE_W::new(self)
    }
    #[doc = "Bit 6 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W {
        AWDIE_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W {
        EOCIE_W::new(self)
    }
    #[doc = "Bits 0:4 - Analog watchdog channel select bits"]
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W {
        AWDCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1/TKEY_V_CTLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr1](index.html) module"]
pub struct CTLR1_SPEC;
impl crate::RegisterSpec for CTLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlr1::R](R) reader structure"]
impl crate::Readable for CTLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlr1::W](W) writer structure"]
impl crate::Writable for CTLR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLR1 to value 0"]
impl crate::Resettable for CTLR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}