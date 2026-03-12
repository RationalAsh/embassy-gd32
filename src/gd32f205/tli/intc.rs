#[doc = "Register `INTC` writer"]
pub type W = crate::W<IntcSpec>;
#[doc = "Field `LMC` writer - Line Mark Flag Clear"]
pub type LmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEC` writer - FIFO Error Flag Clear"]
pub type FecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEC` writer - Transaction Error Flag Clear"]
pub type TecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCRC` writer - Layer Configuration Reloaded Flag Clear"]
pub type LcrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Line Mark Flag Clear"]
    #[inline(always)]
    pub fn lmc(&mut self) -> LmcW<'_, IntcSpec> {
        LmcW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Error Flag Clear"]
    #[inline(always)]
    pub fn fec(&mut self) -> FecW<'_, IntcSpec> {
        FecW::new(self, 1)
    }
    #[doc = "Bit 2 - Transaction Error Flag Clear"]
    #[inline(always)]
    pub fn tec(&mut self) -> TecW<'_, IntcSpec> {
        TecW::new(self, 2)
    }
    #[doc = "Bit 3 - Layer Configuration Reloaded Flag Clear"]
    #[inline(always)]
    pub fn lcrc(&mut self) -> LcrcW<'_, IntcSpec> {
        LcrcW::new(self, 3)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcSpec;
impl crate::RegisterSpec for IntcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for IntcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for IntcSpec {}
