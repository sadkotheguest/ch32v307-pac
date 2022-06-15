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
#[doc = "Field `BIDIMODE` reader - Bidirectional data mode enable"]
pub type BIDIMODE_R = crate::BitReader<bool>;
#[doc = "Field `BIDIMODE` writer - Bidirectional data mode enable"]
pub type BIDIMODE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 15>;
#[doc = "Field `BIDIOE` reader - Output enable in bidirectional mode"]
pub type BIDIOE_R = crate::BitReader<bool>;
#[doc = "Field `BIDIOE` writer - Output enable in bidirectional mode"]
pub type BIDIOE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 14>;
#[doc = "Field `CRCEN` reader - Hardware CRC calculation enable"]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - Hardware CRC calculation enable"]
pub type CRCEN_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 13>;
#[doc = "Field `CRCNEXT` reader - CRC transfer next"]
pub type CRCNEXT_R = crate::BitReader<bool>;
#[doc = "Field `CRCNEXT` writer - CRC transfer next"]
pub type CRCNEXT_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 12>;
#[doc = "Field `DFF` reader - Data frame format"]
pub type DFF_R = crate::BitReader<bool>;
#[doc = "Field `DFF` writer - Data frame format"]
pub type DFF_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 11>;
#[doc = "Field `RXONLY` reader - Receive only"]
pub type RXONLY_R = crate::BitReader<bool>;
#[doc = "Field `RXONLY` writer - Receive only"]
pub type RXONLY_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 10>;
#[doc = "Field `SSM` reader - Software slave management"]
pub type SSM_R = crate::BitReader<bool>;
#[doc = "Field `SSM` writer - Software slave management"]
pub type SSM_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 9>;
#[doc = "Field `SSI` reader - Internal slave select"]
pub type SSI_R = crate::BitReader<bool>;
#[doc = "Field `SSI` writer - Internal slave select"]
pub type SSI_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 8>;
#[doc = "Field `LSBFIRST` reader - Frame format"]
pub type LSBFIRST_R = crate::BitReader<bool>;
#[doc = "Field `LSBFIRST` writer - Frame format"]
pub type LSBFIRST_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 7>;
#[doc = "Field `SPE` reader - SPI enable"]
pub type SPE_R = crate::BitReader<bool>;
#[doc = "Field `SPE` writer - SPI enable"]
pub type SPE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 6>;
#[doc = "Field `BR` reader - Baud rate control"]
pub type BR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BR` writer - Baud rate control"]
pub type BR_W<'a> = crate::FieldWriter<'a, u32, CTLR1_SPEC, u8, u8, 3, 3>;
#[doc = "Field `MSTR` reader - Master selection"]
pub type MSTR_R = crate::BitReader<bool>;
#[doc = "Field `MSTR` writer - Master selection"]
pub type MSTR_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 2>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CPOL_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 1>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CPHA_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    pub fn dff(&self) -> DFF_R {
        DFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline(always)]
    pub fn bidimode(&mut self) -> BIDIMODE_W {
        BIDIMODE_W::new(self)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline(always)]
    pub fn bidioe(&mut self) -> BIDIOE_W {
        BIDIOE_W::new(self)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnext(&mut self) -> CRCNEXT_W {
        CRCNEXT_W::new(self)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    pub fn dff(&mut self) -> DFF_W {
        DFF_W::new(self)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn rxonly(&mut self) -> RXONLY_W {
        RXONLY_W::new(self)
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W {
        SSM_W::new(self)
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W {
        SSI_W::new(self)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W {
        LSBFIRST_W::new(self)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W {
        SPE_W::new(self)
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline(always)]
    pub fn br(&mut self) -> BR_W {
        BR_W::new(self)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W {
        MSTR_W::new(self)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W::new(self)
    }
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr1](index.html) module"]
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