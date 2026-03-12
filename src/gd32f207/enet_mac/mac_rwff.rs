#[doc = "Register `MAC_RWFF` reader"]
pub type R = crate::R<MacRwffSpec>;
#[doc = "Register `MAC_RWFF` writer"]
pub type W = crate::W<MacRwffSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Ethernet MAC remote wakeup frame filter register (MAC_RWFF)\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_rwff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_rwff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacRwffSpec;
impl crate::RegisterSpec for MacRwffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_rwff::R`](R) reader structure"]
impl crate::Readable for MacRwffSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_rwff::W`](W) writer structure"]
impl crate::Writable for MacRwffSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_RWFF to value 0"]
impl crate::Resettable for MacRwffSpec {}
