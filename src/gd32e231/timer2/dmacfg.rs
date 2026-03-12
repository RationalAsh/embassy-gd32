#[doc = "Register `DMACFG` reader"]
pub type R = crate::R<DmacfgSpec>;
#[doc = "Register `DMACFG` writer"]
pub type W = crate::W<DmacfgSpec>;
#[doc = "Field `DMATA` reader - DMA base address"]
pub type DmataR = crate::FieldReader;
#[doc = "Field `DMATA` writer - DMA base address"]
pub type DmataW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `DMATC` reader - DMA burst length"]
pub type DmatcR = crate::FieldReader;
#[doc = "Field `DMATC` writer - DMA burst length"]
pub type DmatcW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline(always)]
    pub fn dmata(&self) -> DmataR {
        DmataR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline(always)]
    pub fn dmatc(&self) -> DmatcR {
        DmatcR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline(always)]
    pub fn dmata(&mut self) -> DmataW<'_, DmacfgSpec> {
        DmataW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline(always)]
    pub fn dmatc(&mut self) -> DmatcW<'_, DmacfgSpec> {
        DmatcW::new(self, 8)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacfgSpec;
impl crate::RegisterSpec for DmacfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacfg::R`](R) reader structure"]
impl crate::Readable for DmacfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacfg::W`](W) writer structure"]
impl crate::Writable for DmacfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACFG to value 0"]
impl crate::Resettable for DmacfgSpec {}
