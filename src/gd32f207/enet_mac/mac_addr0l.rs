#[doc = "Register `MAC_ADDR0L` reader"]
pub type R = crate::R<MacAddr0lSpec>;
#[doc = "Register `MAC_ADDR0L` writer"]
pub type W = crate::W<MacAddr0lSpec>;
#[doc = "Field `ADDR0L` reader - MAC address0 low"]
pub type Addr0lR = crate::FieldReader<u32>;
#[doc = "Field `ADDR0L` writer - MAC address0 low"]
pub type Addr0lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address0 low"]
    #[inline(always)]
    pub fn addr0l(&self) -> Addr0lR {
        Addr0lR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address0 low"]
    #[inline(always)]
    pub fn addr0l(&mut self) -> Addr0lW<'_, MacAddr0lSpec> {
        Addr0lW::new(self, 0)
    }
}
#[doc = "Ethernet MAC address 0 low register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_addr0l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_addr0l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacAddr0lSpec;
impl crate::RegisterSpec for MacAddr0lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_addr0l::R`](R) reader structure"]
impl crate::Readable for MacAddr0lSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_addr0l::W`](W) writer structure"]
impl crate::Writable for MacAddr0lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_ADDR0L to value 0xffff_ffff"]
impl crate::Resettable for MacAddr0lSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
