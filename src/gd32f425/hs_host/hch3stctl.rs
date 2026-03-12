#[doc = "Register `HCH3STCTL` reader"]
pub type R = crate::R<Hch3stctlSpec>;
#[doc = "Register `HCH3STCTL` writer"]
pub type W = crate::W<Hch3stctlSpec>;
#[doc = "Field `PADDR` reader - Port address"]
pub type PaddrR = crate::FieldReader;
#[doc = "Field `PADDR` writer - Port address"]
pub type PaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HADDR` reader - HUB address"]
pub type HaddrR = crate::FieldReader;
#[doc = "Field `HADDR` writer - HUB address"]
pub type HaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ISOPCE` reader - Isochronous OUT payload continuation encoding"]
pub type IsopceR = crate::FieldReader;
#[doc = "Field `ISOPCE` writer - Isochronous OUT payload continuation encoding"]
pub type IsopceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSPLT` reader - Complete split enable"]
pub type CspltR = crate::BitReader;
#[doc = "Field `CSPLT` writer - Complete split enable"]
pub type CspltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLEN` reader - Enable high speed split transaction"]
pub type SplenR = crate::BitReader;
#[doc = "Field `SPLEN` writer - Enable high speed split transaction"]
pub type SplenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn paddr(&self) -> PaddrR {
        PaddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - HUB address"]
    #[inline(always)]
    pub fn haddr(&self) -> HaddrR {
        HaddrR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - Isochronous OUT payload continuation encoding"]
    #[inline(always)]
    pub fn isopce(&self) -> IsopceR {
        IsopceR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Complete split enable"]
    #[inline(always)]
    pub fn csplt(&self) -> CspltR {
        CspltR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable high speed split transaction"]
    #[inline(always)]
    pub fn splen(&self) -> SplenR {
        SplenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Port address"]
    #[inline(always)]
    pub fn paddr(&mut self) -> PaddrW<'_, Hch3stctlSpec> {
        PaddrW::new(self, 0)
    }
    #[doc = "Bits 7:13 - HUB address"]
    #[inline(always)]
    pub fn haddr(&mut self) -> HaddrW<'_, Hch3stctlSpec> {
        HaddrW::new(self, 7)
    }
    #[doc = "Bits 14:15 - Isochronous OUT payload continuation encoding"]
    #[inline(always)]
    pub fn isopce(&mut self) -> IsopceW<'_, Hch3stctlSpec> {
        IsopceW::new(self, 14)
    }
    #[doc = "Bit 16 - Complete split enable"]
    #[inline(always)]
    pub fn csplt(&mut self) -> CspltW<'_, Hch3stctlSpec> {
        CspltW::new(self, 16)
    }
    #[doc = "Bit 31 - Enable high speed split transaction"]
    #[inline(always)]
    pub fn splen(&mut self) -> SplenW<'_, Hch3stctlSpec> {
        SplenW::new(self, 31)
    }
}
#[doc = "host channel-3 split transaction control register (HCH3STCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hch3stctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hch3stctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hch3stctlSpec;
impl crate::RegisterSpec for Hch3stctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch3stctl::R`](R) reader structure"]
impl crate::Readable for Hch3stctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hch3stctl::W`](W) writer structure"]
impl crate::Writable for Hch3stctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCH3STCTL to value 0"]
impl crate::Resettable for Hch3stctlSpec {}
