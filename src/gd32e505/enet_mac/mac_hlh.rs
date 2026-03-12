#[doc = "Register `MAC_HLH` reader"]
pub type R = crate::R<MacHlhSpec>;
#[doc = "Register `MAC_HLH` writer"]
pub type W = crate::W<MacHlhSpec>;
#[doc = "Field `HLH` reader - Hash list high"]
pub type HlhR = crate::FieldReader<u32>;
#[doc = "Field `HLH` writer - Hash list high"]
pub type HlhW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash list high"]
    #[inline(always)]
    pub fn hlh(&self) -> HlhR {
        HlhR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash list high"]
    #[inline(always)]
    pub fn hlh(&mut self) -> HlhW<'_, MacHlhSpec> {
        HlhW::new(self, 0)
    }
}
#[doc = "Ethernet MAC hash list high register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_hlh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_hlh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacHlhSpec;
impl crate::RegisterSpec for MacHlhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_hlh::R`](R) reader structure"]
impl crate::Readable for MacHlhSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_hlh::W`](W) writer structure"]
impl crate::Writable for MacHlhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_HLH to value 0"]
impl crate::Resettable for MacHlhSpec {}
