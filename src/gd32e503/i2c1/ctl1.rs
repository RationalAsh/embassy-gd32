#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `I2CCLK` reader - I2C Peripheral clock frequency"]
pub type I2cclkR = crate::FieldReader;
#[doc = "Field `I2CCLK` writer - I2C Peripheral clock frequency"]
pub type I2cclkW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVIE` reader - Event interrupt enable"]
pub type EvieR = crate::BitReader;
#[doc = "Field `EVIE` writer - Event interrupt enable"]
pub type EvieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFIE` reader - Buffer interrupt enable"]
pub type BufieR = crate::BitReader;
#[doc = "Field `BUFIE` writer - Buffer interrupt enable"]
pub type BufieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAON` reader - DMA mode switch"]
pub type DmaonR = crate::BitReader;
#[doc = "Field `DMAON` writer - DMA mode switch"]
pub type DmaonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMALST` reader - Flag indicating DMA last transfer"]
pub type DmalstR = crate::BitReader;
#[doc = "Field `DMALST` writer - Flag indicating DMA last transfer"]
pub type DmalstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - I2C Peripheral clock frequency"]
    #[inline(always)]
    pub fn i2cclk(&self) -> I2cclkR {
        I2cclkR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn evie(&self) -> EvieR {
        EvieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn bufie(&self) -> BufieR {
        BufieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA mode switch"]
    #[inline(always)]
    pub fn dmaon(&self) -> DmaonR {
        DmaonR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Flag indicating DMA last transfer"]
    #[inline(always)]
    pub fn dmalst(&self) -> DmalstR {
        DmalstR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - I2C Peripheral clock frequency"]
    #[inline(always)]
    pub fn i2cclk(&mut self) -> I2cclkW<'_, Ctl1Spec> {
        I2cclkW::new(self, 0)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, Ctl1Spec> {
        ErrieW::new(self, 8)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn evie(&mut self) -> EvieW<'_, Ctl1Spec> {
        EvieW::new(self, 9)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn bufie(&mut self) -> BufieW<'_, Ctl1Spec> {
        BufieW::new(self, 10)
    }
    #[doc = "Bit 11 - DMA mode switch"]
    #[inline(always)]
    pub fn dmaon(&mut self) -> DmaonW<'_, Ctl1Spec> {
        DmaonW::new(self, 11)
    }
    #[doc = "Bit 12 - Flag indicating DMA last transfer"]
    #[inline(always)]
    pub fn dmalst(&mut self) -> DmalstW<'_, Ctl1Spec> {
        DmalstW::new(self, 12)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
