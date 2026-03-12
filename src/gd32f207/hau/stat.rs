#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `DINT` reader - Data input interrupt status flag"]
pub type DintR = crate::BitReader;
#[doc = "Field `DINT` writer - Data input interrupt status flag"]
pub type DintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CINT` reader - Digest calculation completion interrupt flag"]
pub type CintR = crate::BitReader;
#[doc = "Field `CINT` writer - Digest calculation completion interrupt flag"]
pub type CintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAS` reader - DMA status flag"]
pub type DmasR = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy flag bit"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data input interrupt status flag"]
    #[inline(always)]
    pub fn dint(&self) -> DintR {
        DintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt flag"]
    #[inline(always)]
    pub fn cint(&self) -> CintR {
        CintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA status flag"]
    #[inline(always)]
    pub fn dmas(&self) -> DmasR {
        DmasR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy flag bit"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data input interrupt status flag"]
    #[inline(always)]
    pub fn dint(&mut self) -> DintW<'_, StatSpec> {
        DintW::new(self, 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt flag"]
    #[inline(always)]
    pub fn cint(&mut self) -> CintW<'_, StatSpec> {
        CintW::new(self, 1)
    }
}
#[doc = "HAU status and interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STAT to value 0x01"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x01;
}
