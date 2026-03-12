#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `DDUDR0` reader - DAC0 DMA underrun flag"]
pub type Ddudr0R = crate::BitReader;
#[doc = "Field `DDUDR0` writer - DAC0 DMA underrun flag"]
pub type Ddudr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDUDR1` reader - DAC1 DMA underrun flag"]
pub type Ddudr1R = crate::BitReader;
#[doc = "Field `DDUDR1` writer - DAC1 DMA underrun flag"]
pub type Ddudr1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13 - DAC0 DMA underrun flag"]
    #[inline(always)]
    pub fn ddudr0(&self) -> Ddudr0R {
        Ddudr0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 DMA underrun flag"]
    #[inline(always)]
    pub fn ddudr1(&self) -> Ddudr1R {
        Ddudr1R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC0 DMA underrun flag"]
    #[inline(always)]
    pub fn ddudr0(&mut self) -> Ddudr0W<'_, StatSpec> {
        Ddudr0W::new(self, 13)
    }
    #[doc = "Bit 29 - DAC1 DMA underrun flag"]
    #[inline(always)]
    pub fn ddudr1(&mut self) -> Ddudr1W<'_, StatSpec> {
        Ddudr1W::new(self, 29)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
