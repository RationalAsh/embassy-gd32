#[doc = "Register `MAC_WUM` reader"]
pub type R = crate::R<MacWumSpec>;
#[doc = "Register `MAC_WUM` writer"]
pub type W = crate::W<MacWumSpec>;
#[doc = "Field `PWD` reader - Power down"]
pub type PwdR = crate::BitReader;
#[doc = "Field `PWD` writer - Power down"]
pub type PwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPEN` reader - Magic Packet enable"]
pub type MpenR = crate::BitReader;
#[doc = "Field `MPEN` writer - Magic Packet enable"]
pub type MpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFEN` reader - Wakeup frame enable"]
pub type WfenR = crate::BitReader;
#[doc = "Field `WFEN` writer - Wakeup frame enable"]
pub type WfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPKR` reader - Magic packet received"]
pub type MpkrR = crate::BitReader;
#[doc = "Field `MPKR` writer - Magic packet received"]
pub type MpkrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUFR` reader - Wakeup frame received"]
pub type WufrR = crate::BitReader;
#[doc = "Field `WUFR` writer - Wakeup frame received"]
pub type WufrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GU` reader - Global unicast"]
pub type GuR = crate::BitReader;
#[doc = "Field `GU` writer - Global unicast"]
pub type GuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUFFRPR` reader - Wakeup frame filter register pointer reset"]
pub type WuffrprR = crate::BitReader;
#[doc = "Field `WUFFRPR` writer - Wakeup frame filter register pointer reset"]
pub type WuffrprW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pwd(&self) -> PwdR {
        PwdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet enable"]
    #[inline(always)]
    pub fn mpen(&self) -> MpenR {
        MpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfen(&self) -> WfenR {
        WfenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpkr(&self) -> MpkrR {
        MpkrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wufr(&self) -> WufrR {
        WufrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&self) -> GuR {
        GuR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wuffrpr(&self) -> WuffrprR {
        WuffrprR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pwd(&mut self) -> PwdW<'_, MacWumSpec> {
        PwdW::new(self, 0)
    }
    #[doc = "Bit 1 - Magic Packet enable"]
    #[inline(always)]
    pub fn mpen(&mut self) -> MpenW<'_, MacWumSpec> {
        MpenW::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfen(&mut self) -> WfenW<'_, MacWumSpec> {
        WfenW::new(self, 2)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpkr(&mut self) -> MpkrW<'_, MacWumSpec> {
        MpkrW::new(self, 5)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wufr(&mut self) -> WufrW<'_, MacWumSpec> {
        WufrW::new(self, 6)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&mut self) -> GuW<'_, MacWumSpec> {
        GuW::new(self, 9)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wuffrpr(&mut self) -> WuffrprW<'_, MacWumSpec> {
        WuffrprW::new(self, 31)
    }
}
#[doc = "Ethernet MAC wakeup management register (MAC_WUM)\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_wum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_wum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacWumSpec;
impl crate::RegisterSpec for MacWumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_wum::R`](R) reader structure"]
impl crate::Readable for MacWumSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_wum::W`](W) writer structure"]
impl crate::Writable for MacWumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC_WUM to value 0"]
impl crate::Resettable for MacWumSpec {}
