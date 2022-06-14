#[doc = "Register `EXTEN_INTFR` reader"]
pub struct R(crate::R<EXTEN_INTFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTEN_INTFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTEN_INTFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTEN_INTFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GIF8` reader - Channel 8 Global interrupt flag"]
pub type GIF8_R = crate::BitReader<bool>;
#[doc = "Field `TCIF8` reader - Channel 8 Transfer Complete flag"]
pub type TCIF8_R = crate::BitReader<bool>;
#[doc = "Field `HTIF8` reader - Channel 8 Half Transfer Complete flag"]
pub type HTIF8_R = crate::BitReader<bool>;
#[doc = "Field `TEIF8` reader - Channel 8 Transfer Error flag"]
pub type TEIF8_R = crate::BitReader<bool>;
#[doc = "Field `GIF9` reader - Channel 9 Global interrupt flag"]
pub type GIF9_R = crate::BitReader<bool>;
#[doc = "Field `TCIF9` reader - Channel 9 Transfer Complete flag"]
pub type TCIF9_R = crate::BitReader<bool>;
#[doc = "Field `HTIF9` reader - Channel 9 Half Transfer Complete flag"]
pub type HTIF9_R = crate::BitReader<bool>;
#[doc = "Field `TEIF9` reader - Channel 9 Transfer Error flag"]
pub type TEIF9_R = crate::BitReader<bool>;
#[doc = "Field `GIF10` reader - Channel 10 Global interrupt flag"]
pub type GIF10_R = crate::BitReader<bool>;
#[doc = "Field `TCIF10` reader - Channel 10 Transfer Complete flag"]
pub type TCIF10_R = crate::BitReader<bool>;
#[doc = "Field `HTIF10` reader - Channel 10 Half Transfer Complete flag"]
pub type HTIF10_R = crate::BitReader<bool>;
#[doc = "Field `TEIF10` reader - Channel 10 Transfer Error flag"]
pub type TEIF10_R = crate::BitReader<bool>;
#[doc = "Field `GIF11` reader - Channel 11 Global interrupt flag"]
pub type GIF11_R = crate::BitReader<bool>;
#[doc = "Field `TCIF11` reader - Channel 11 Transfer Complete flag"]
pub type TCIF11_R = crate::BitReader<bool>;
#[doc = "Field `HTIF11` reader - Channel 11 Half Transfer Complete flag"]
pub type HTIF11_R = crate::BitReader<bool>;
#[doc = "Field `TEIF11` reader - Channel 11 Transfer Error flag"]
pub type TEIF11_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 8 Global interrupt flag"]
    #[inline(always)]
    pub fn gif8(&self) -> GIF8_R {
        GIF8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 8 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif8(&self) -> TCIF8_R {
        TCIF8_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 8 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif8(&self) -> HTIF8_R {
        HTIF8_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 8 Transfer Error flag"]
    #[inline(always)]
    pub fn teif8(&self) -> TEIF8_R {
        TEIF8_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 9 Global interrupt flag"]
    #[inline(always)]
    pub fn gif9(&self) -> GIF9_R {
        GIF9_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 9 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif9(&self) -> TCIF9_R {
        TCIF9_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 9 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif9(&self) -> HTIF9_R {
        HTIF9_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 9 Transfer Error flag"]
    #[inline(always)]
    pub fn teif9(&self) -> TEIF9_R {
        TEIF9_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 10 Global interrupt flag"]
    #[inline(always)]
    pub fn gif10(&self) -> GIF10_R {
        GIF10_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 10 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif10(&self) -> TCIF10_R {
        TCIF10_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif10(&self) -> HTIF10_R {
        HTIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 10 Transfer Error flag"]
    #[inline(always)]
    pub fn teif10(&self) -> TEIF10_R {
        TEIF10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 11 Global interrupt flag"]
    #[inline(always)]
    pub fn gif11(&self) -> GIF11_R {
        GIF11_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 11 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif11(&self) -> TCIF11_R {
        TCIF11_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 11 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif11(&self) -> HTIF11_R {
        HTIF11_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 11 Transfer Error flag"]
    #[inline(always)]
    pub fn teif11(&self) -> TEIF11_R {
        TEIF11_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "DMA2 EXTEN interrupt status register (DMA_INTFR)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exten_intfr](index.html) module"]
pub struct EXTEN_INTFR_SPEC;
impl crate::RegisterSpec for EXTEN_INTFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exten_intfr::R](R) reader structure"]
impl crate::Readable for EXTEN_INTFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTEN_INTFR to value 0"]
impl crate::Resettable for EXTEN_INTFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}