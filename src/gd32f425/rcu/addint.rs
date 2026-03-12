#[doc = "Register `ADDINT` reader"]
pub type R = crate::R<AddintSpec>;
#[doc = "Register `ADDINT` writer"]
pub type W = crate::W<AddintSpec>;
#[doc = "IRC48M stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc48mstbifr {
    #[doc = "0: No interrupt generated"]
    NotInterrupted = 0,
    #[doc = "1: PLL stabilisation interrupt generated"]
    Interrupted = 1,
}
impl From<Irc48mstbifr> for bool {
    #[inline(always)]
    fn from(variant: Irc48mstbifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC48MSTBIF` reader - IRC48M stabilization interrupt flag"]
pub type Irc48mstbifR = crate::BitReader<Irc48mstbifr>;
impl Irc48mstbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc48mstbifr {
        match self.bits {
            false => Irc48mstbifr::NotInterrupted,
            true => Irc48mstbifr::Interrupted,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Irc48mstbifr::NotInterrupted
    }
    #[doc = "PLL stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Irc48mstbifr::Interrupted
    }
}
#[doc = "Internal 48 MHz RC oscillator Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc48mstbie {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<Irc48mstbie> for bool {
    #[inline(always)]
    fn from(variant: Irc48mstbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC48MSTBIE` reader - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
pub type Irc48mstbieR = crate::BitReader<Irc48mstbie>;
impl Irc48mstbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc48mstbie {
        match self.bits {
            false => Irc48mstbie::Disabled,
            true => Irc48mstbie::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Irc48mstbie::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Irc48mstbie::Enabled
    }
}
#[doc = "Field `IRC48MSTBIE` writer - Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
pub type Irc48mstbieW<'a, REG> = crate::BitWriter<'a, REG, Irc48mstbie>;
impl<'a, REG> Irc48mstbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Irc48mstbie::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Irc48mstbie::Enabled)
    }
}
#[doc = "Internal 48 MHz RC oscillator Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc48mstbicw {
    #[doc = "1: Clear PLLSTBIF flag"]
    Clear = 1,
}
impl From<Irc48mstbicw> for bool {
    #[inline(always)]
    fn from(variant: Irc48mstbicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC48MSTBIC` writer - Internal 48 MHz RC oscillator Stabilization Interrupt Clear"]
pub type Irc48mstbicW<'a, REG> = crate::BitWriter<'a, REG, Irc48mstbicw>;
impl<'a, REG> Irc48mstbicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear PLLSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Irc48mstbicw::Clear)
    }
}
impl R {
    #[doc = "Bit 6 - IRC48M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc48mstbif(&self) -> Irc48mstbifR {
        Irc48mstbifR::new(((self.bits >> 6) & 1) != 0)
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
