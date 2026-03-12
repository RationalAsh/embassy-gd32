#[doc = "Register `MAC_PHY_CTL` reader"]
pub type R = crate::R<MacPhyCtlSpec>;
#[doc = "Register `MAC_PHY_CTL` writer"]
pub type W = crate::W<MacPhyCtlSpec>;
#[doc = "Field `PB` reader - PHY busy"]
pub type PbR = crate::BitReader;
#[doc = "Field `PB` writer - PHY busy"]
pub type PbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PW` reader - PHY write"]
pub type PwR = crate::BitReader;
#[doc = "Field `PW` writer - PHY write"]
pub type PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR` reader - Clock range"]
pub type ClrR = crate::FieldReader;
#[doc = "Field `CLR` writer - Clock range"]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PR` reader - PHY register"]
pub type PrR = crate::FieldReader;
#[doc = "Field `PR` writer - PHY register"]
pub type PrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - PHY address"]
pub type PaR = crate::FieldReader;
#[doc = "Field `PA` writer - PHY address"]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - PHY busy"]
    #[inline(always)]
    pub fn pb(&self) -> PbR {
        PbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PHY write"]
    #[inline(always)]
    pub fn pw(&self) -> PwR {
        PwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 6:10 - PHY register"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PHY busy"]
    #[inline(always)]
    pub fn pb(&mut self) -> PbW<'_, MacPhyCtlSpec> {
        PbW::new(self, 0)
    }
    #[doc = "Bit 1 - PHY write"]
    #[inline(always)]
    pub fn pw(&mut self) -> PwW<'_, MacPhyCtlSpec> {
        PwW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<'_, MacPhyCtlSpec> {
        ClrW::new(self, 2)
    }
    #[doc = "Bits 6:10 - PHY register"]
    #[inline(always)]
    pub fn pr(&mut self) -> PrW<'_, MacPhyCtlSpec> {
        PrW::new(self, 6)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PaW<'_, MacPhyCtlSpec> {
        PaW::new(self, 11)
    }
}
#[doc = "Ethernet MAC PHY control register (MAC_PHY_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_phy_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_phy_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacPhyCtlSpec;
impl crate::RegisterSpec for MacPhyCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_phy_ctl::R`](R) reader structure"]
impl crate::Readable for MacPhyCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_phy_ctl::W`](W) writer structure"]
impl crate::Writable for MacPhyCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_PHY_CTL to value 0"]
impl crate::Resettable for MacPhyCtlSpec {}
