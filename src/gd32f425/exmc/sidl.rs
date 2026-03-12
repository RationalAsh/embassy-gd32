#[doc = "Register `SIDL` reader"]
pub type R = crate::R<SidlSpec>;
#[doc = "Register `SIDL` writer"]
pub type W = crate::W<SidlSpec>;
#[doc = "Field `SIDL` reader - ID Low Data saved for SPI Read ID Command"]
pub type SidlR = crate::FieldReader<u32>;
#[doc = "Field `SIDL` writer - ID Low Data saved for SPI Read ID Command"]
pub type SidlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID Low Data saved for SPI Read ID Command"]
    #[inline(always)]
    pub fn sidl(&self) -> SidlR {
        SidlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ID Low Data saved for SPI Read ID Command"]
    #[inline(always)]
    pub fn sidl(&mut self) -> SidlW<'_, SidlSpec> {
        SidlW::new(self, 0)
    }
}
#[doc = "SPI ID low register\n\nYou can [`read`](crate::Reg::read) this register and get [`sidl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SidlSpec;
impl crate::RegisterSpec for SidlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidl::R`](R) reader structure"]
impl crate::Readable for SidlSpec {}
#[doc = "`write(|w| ..)` method takes [`sidl::W`](W) writer structure"]
impl crate::Writable for SidlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIDL to value 0"]
impl crate::Resettable for SidlSpec {}
