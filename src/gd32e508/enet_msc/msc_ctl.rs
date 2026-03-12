#[doc = "Register `MSC_CTL` reader"]
pub type R = crate::R<MscCtlSpec>;
#[doc = "Register `MSC_CTL` writer"]
pub type W = crate::W<MscCtlSpec>;
#[doc = "Field `CTR` reader - Counter reset"]
pub type CtrR = crate::BitReader;
#[doc = "Field `CTR` writer - Counter reset"]
pub type CtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSR` reader - Counter stop rollover"]
pub type CtsrR = crate::BitReader;
#[doc = "Field `CTSR` writer - Counter stop rollover"]
pub type CtsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOR` reader - Reset on read"]
pub type RtorR = crate::BitReader;
#[doc = "Field `RTOR` writer - Reset on read"]
pub type RtorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCFZ` reader - MSC counter freeze"]
pub type McfzR = crate::BitReader;
#[doc = "Field `MCFZ` writer - MSC counter freeze"]
pub type McfzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMC` writer - Preset MSC counter"]
pub type PmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFHPM` reader - Almost full or half preset mode"]
pub type AfhpmR = crate::BitReader;
#[doc = "Field `AFHPM` writer - Almost full or half preset mode"]
pub type AfhpmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn ctsr(&self) -> CtsrR {
        CtsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn rtor(&self) -> RtorR {
        RtorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MSC counter freeze"]
    #[inline(always)]
    pub fn mcfz(&self) -> McfzR {
        McfzR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Almost full or half preset mode"]
    #[inline(always)]
    pub fn afhpm(&self) -> AfhpmR {
        AfhpmR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CtrW<'_, MscCtlSpec> {
        CtrW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn ctsr(&mut self) -> CtsrW<'_, MscCtlSpec> {
        CtsrW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn rtor(&mut self) -> RtorW<'_, MscCtlSpec> {
        RtorW::new(self, 2)
    }
    #[doc = "Bit 3 - MSC counter freeze"]
    #[inline(always)]
    pub fn mcfz(&mut self) -> McfzW<'_, MscCtlSpec> {
        McfzW::new(self, 3)
    }
    #[doc = "Bit 4 - Preset MSC counter"]
    #[inline(always)]
    pub fn pmc(&mut self) -> PmcW<'_, MscCtlSpec> {
        PmcW::new(self, 4)
    }
    #[doc = "Bit 5 - Almost full or half preset mode"]
    #[inline(always)]
    pub fn afhpm(&mut self) -> AfhpmW<'_, MscCtlSpec> {
        AfhpmW::new(self, 5)
    }
}
#[doc = "Ethernet MSC control register (MSC_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`msc_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msc_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MscCtlSpec;
impl crate::RegisterSpec for MscCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msc_ctl::R`](R) reader structure"]
impl crate::Readable for MscCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`msc_ctl::W`](W) writer structure"]
impl crate::Writable for MscCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSC_CTL to value 0"]
impl crate::Resettable for MscCtlSpec {}
