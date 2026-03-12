#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `ENET_PHY_SEL` reader - Ethernet PHY selection"]
pub type EnetPhySelR = crate::BitReader;
#[doc = "Field `ENET_PHY_SEL` writer - Ethernet PHY selection"]
pub type EnetPhySelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 23 - Ethernet PHY selection"]
    #[inline(always)]
    pub fn enet_phy_sel(&self) -> EnetPhySelR {
        EnetPhySelR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Ethernet PHY selection"]
    #[inline(always)]
    pub fn enet_phy_sel(&mut self) -> EnetPhySelW<'_, Cfg1Spec> {
        EnetPhySelW::new(self, 23)
    }
}
#[doc = "Configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {}
