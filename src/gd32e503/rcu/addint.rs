#[doc = "Register `ADDINT` reader"]
pub type R = crate::R<AddintSpec>;
#[doc = "Register `ADDINT` writer"]
pub type W = crate::W<AddintSpec>;
#[doc = "Field `IRC48MSTBIF` reader - IRC48M stabilization interrupt flag"]
pub type Irc48mstbifR = crate::BitReader;
#[doc = "Field `PLLUSBSTBIF` reader - PLLUSB stabilization interrupt flag"]
pub type PllusbstbifR = crate::BitReader;
#[doc = "Field `IRC48MSTBIE` reader - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
pub type Irc48mstbieR = crate::BitReader;
#[doc = "Field `IRC48MSTBIE` writer - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
pub type Irc48mstbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC48MSTBIC` writer - Internal 48 MHz RC oscillator Stabilization Interrupt Clear"]
pub type Irc48mstbicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - IRC48M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc48mstbif(&self) -> Irc48mstbifR {
        Irc48mstbifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLLUSB stabilization interrupt flag"]
    #[inline(always)]
    pub fn pllusbstbif(&self) -> PllusbstbifR {
        PllusbstbifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 14 - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc48mstbie(&self) -> Irc48mstbieR {
        Irc48mstbieR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc48mstbie(&mut self) -> Irc48mstbieW<'_, AddintSpec> {
        Irc48mstbieW::new(self, 14)
    }
    #[doc = "Bit 22 - Internal 48 MHz RC oscillator Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc48mstbic(&mut self) -> Irc48mstbicW<'_, AddintSpec> {
        Irc48mstbicW::new(self, 22)
    }
}
#[doc = "Additional clock interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`addint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddintSpec;
impl crate::RegisterSpec for AddintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addint::R`](R) reader structure"]
impl crate::Readable for AddintSpec {}
#[doc = "`write(|w| ..)` method takes [`addint::W`](W) writer structure"]
impl crate::Writable for AddintSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDINT to value 0"]
impl crate::Resettable for AddintSpec {}
