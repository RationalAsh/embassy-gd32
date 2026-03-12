#[doc = "Register `SIDH` reader"]
pub type R = crate::R<SidhSpec>;
#[doc = "Register `SIDH` writer"]
pub type W = crate::W<SidhSpec>;
#[doc = "Field `SIDH` reader - ID High Data saved for SPI Read ID Command"]
pub type SidhR = crate::FieldReader<u32>;
#[doc = "Field `SIDH` writer - ID High Data saved for SPI Read ID Command"]
pub type SidhW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID High Data saved for SPI Read ID Command"]
    #[inline(always)]
    pub fn sidh(&self) -> SidhR {
        SidhR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ID High Data saved for SPI Read ID Command"]
    #[inline(always)]
    pub fn sidh(&mut self) -> SidhW<'_, SidhSpec> {
        SidhW::new(self, 0)
    }
}
#[doc = "SPI ID high register\n\nYou can [`read`](crate::Reg::read) this register and get [`sidh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SidhSpec;
impl crate::RegisterSpec for SidhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidh::R`](R) reader structure"]
impl crate::Readable for SidhSpec {}
#[doc = "`write(|w| ..)` method takes [`sidh::W`](W) writer structure"]
impl crate::Writable for SidhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIDH to value 0"]
impl crate::Resettable for SidhSpec {}
