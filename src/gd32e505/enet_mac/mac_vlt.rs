#[doc = "Register `MAC_VLT` reader"]
pub type R = crate::R<MacVltSpec>;
#[doc = "Register `MAC_VLT` writer"]
pub type W = crate::W<MacVltSpec>;
#[doc = "Field `VLTI` reader - VLAN tag identifier (for receive frames)"]
pub type VltiR = crate::FieldReader<u16>;
#[doc = "Field `VLTI` writer - VLAN tag identifier (for receive frames)"]
pub type VltiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VLTC` reader - 12-bit VLAN tag comparison"]
pub type VltcR = crate::BitReader;
#[doc = "Field `VLTC` writer - 12-bit VLAN tag comparison"]
pub type VltcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlti(&self) -> VltiR {
        VltiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vltc(&self) -> VltcR {
        VltcR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlti(&mut self) -> VltiW<'_, MacVltSpec> {
        VltiW::new(self, 0)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vltc(&mut self) -> VltcW<'_, MacVltSpec> {
        VltcW::new(self, 16)
    }
}
#[doc = "Ethernet MAC VLAN tag register (MAC_VLT)\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_vlt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_vlt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacVltSpec;
impl crate::RegisterSpec for MacVltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_vlt::R`](R) reader structure"]
impl crate::Readable for MacVltSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_vlt::W`](W) writer structure"]
impl crate::Writable for MacVltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_VLT to value 0"]
impl crate::Resettable for MacVltSpec {}
