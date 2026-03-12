#[doc = "Register `MAC_HLL` reader"]
pub type R = crate::R<MacHllSpec>;
#[doc = "Register `MAC_HLL` writer"]
pub type W = crate::W<MacHllSpec>;
#[doc = "Field `HLL` reader - Hash list low"]
pub type HllR = crate::FieldReader<u32>;
#[doc = "Field `HLL` writer - Hash list low"]
pub type HllW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash list low"]
    #[inline(always)]
    pub fn hll(&self) -> HllR {
        HllR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash list low"]
    #[inline(always)]
    pub fn hll(&mut self) -> HllW<'_, MacHllSpec> {
        HllW::new(self, 0)
    }
}
#[doc = "Ethernet MAC hash list low register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_hll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_hll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacHllSpec;
impl crate::RegisterSpec for MacHllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_hll::R`](R) reader structure"]
impl crate::Readable for MacHllSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_hll::W`](W) writer structure"]
impl crate::Writable for MacHllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_HLL to value 0"]
impl crate::Resettable for MacHllSpec {}
