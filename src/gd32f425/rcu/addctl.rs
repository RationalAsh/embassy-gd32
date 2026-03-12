#[doc = "Register `ADDCTL` reader"]
pub type R = crate::R<AddctlSpec>;
#[doc = "Register `ADDCTL` writer"]
pub type W = crate::W<AddctlSpec>;
#[doc = "48MHz clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ck48msel {
    #[doc = "0: Don’t select IRC48M clock(use CK_PLLQ clock or CK_PLLSAIP clock select by PLL48MSEL)"]
    None = 0,
    #[doc = "1: Select the IRC48M clock."]
    Irc48m = 1,
}
impl From<Ck48msel> for bool {
    #[inline(always)]
    fn from(variant: Ck48msel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CK48MSEL` reader - 48MHz clock selection"]
pub type Ck48mselR = crate::BitReader<Ck48msel>;
impl Ck48mselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ck48msel {
        match self.bits {
            false => Ck48msel::None,
            true => Ck48msel::Irc48m,
        }
    }
    #[doc = "Don’t select IRC48M clock(use CK_PLLQ clock or CK_PLLSAIP clock select by PLL48MSEL)"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ck48msel::None
    }
    #[doc = "Select the IRC48M clock."]
    #[inline(always)]
    pub fn is_irc48m(&self) -> bool {
        *self == Ck48msel::Irc48m
    }
}
#[doc = "Field `CK48MSEL` writer - 48MHz clock selection"]
pub type Ck48mselW<'a, REG> = crate::BitWriter<'a, REG, Ck48msel>;
impl<'a, REG> Ck48mselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don’t select IRC48M clock(use CK_PLLQ clock or CK_PLLSAIP clock select by PLL48MSEL)"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ck48msel::None)
    }
    #[doc = "Select the IRC48M clock."]
    #[inline(always)]
    pub fn irc48m(self) -> &'a mut crate::W<REG> {
        self.variant(Ck48msel::Irc48m)
    }
}
#[doc = "PLL48M clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pll48msel {
    #[doc = "0: Select the PLLQ clock."]
    Pllq = 0,
    #[doc = "1: Select the PLLSAIP clock."]
    Pllsaip = 1,
}
impl From<Pll48msel> for bool {
    #[inline(always)]
    fn from(variant: Pll48msel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL48MSEL` reader - PLL48M clock selection"]
pub type Pll48mselR = crate::BitReader<Pll48msel>;
impl Pll48mselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pll48msel {
        match self.bits {
            false => Pll48msel::Pllq,
            true => Pll48msel::Pllsaip,
        }
    }
    #[doc = "Select the PLLQ clock."]
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == Pll48msel::Pllq
    }
    #[doc = "Select the PLLSAIP clock."]
    #[inline(always)]
    pub fn is_pllsaip(&self) -> bool {
        *self == Pll48msel::Pllsaip
    }
}
#[doc = "Field `PLL48MSEL` writer - PLL48M clock selection"]
pub type Pll48mselW<'a, REG> = crate::BitWriter<'a, REG, Pll48msel>;
impl<'a, REG> Pll48mselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select the PLLQ clock."]
    #[inline(always)]
    pub fn pllq(self) -> &'a mut crate::W<REG> {
        self.variant(Pll48msel::Pllq)
    }
    #[doc = "Select the PLLSAIP clock."]
    #[inline(always)]
    pub fn pllsaip(self) -> &'a mut crate::W<REG> {
        self.variant(Pll48msel::Pllsaip)
    }
}
#[doc = "Internal 48MHz RC oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc48men {
    #[doc = "0: IRC48M Clock off."]
    Off = 0,
    #[doc = "1: IRC48M Clock on."]
    On = 1,
}
impl From<Irc48men> for bool {
    #[inline(always)]
    fn from(variant: Irc48men) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC48MEN` reader - Internal 48MHz RC oscillator enable"]
pub type Irc48menR = crate::BitReader<Irc48men>;
impl Irc48menR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc48men {
        match self.bits {
            false => Irc48men::Off,
            true => Irc48men::On,
        }
    }
    #[doc = "IRC48M Clock off."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Irc48men::Off
    }
    #[doc = "IRC48M Clock on."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Irc48men::On
    }
}
#[doc = "Field `IRC48MEN` writer - Internal 48MHz RC oscillator enable"]
pub type Irc48menW<'a, REG> = crate::BitWriter<'a, REG, Irc48men>;
impl<'a, REG> Irc48menW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IRC48M Clock off."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Irc48men::Off)
    }
    #[doc = "IRC48M Clock on."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Irc48men::On)
    }
}
#[doc = "Internal 48MHz RC oscillator clock stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc48mstbr {
    #[doc = "0: IRC48M oscillator is not stable."]
    NotReady = 0,
    #[doc = "1: IRC48M oscillator is stable."]
    Ready = 1,
}
impl From<Irc48mstbr> for bool {
    #[inline(always)]
    fn from(variant: Irc48mstbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC48MSTB` reader - Internal 48MHz RC oscillator clock stabilization Flag"]
pub type Irc48mstbR = crate::BitReader<Irc48mstbr>;
impl Irc48mstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc48mstbr {
        match self.bits {
            false => Irc48mstbr::NotReady,
            true => Irc48mstbr::Ready,
        }
    }
    #[doc = "IRC48M oscillator is not stable."]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Irc48mstbr::NotReady
    }
    #[doc = "IRC48M oscillator is stable."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Irc48mstbr::Ready
    }
}
#[doc = "Field `IRC48MCAL` reader - Internal 48MHz RC oscillator calibration value register"]
pub type Irc48mcalR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 48MHz clock selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> Ck48mselR {
        Ck48mselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL48M clock selection"]
    #[inline(always)]
    pub fn pll48msel(&self) -> Pll48mselR {
        Pll48mselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Internal 48MHz RC oscillator enable"]
    #[inline(always)]
    pub fn irc48men(&self) -> Irc48menR {
        Irc48menR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Internal 48MHz RC oscillator clock stabilization Flag"]
    #[inline(always)]
    pub fn irc48mstb(&self) -> Irc48mstbR {
        Irc48mstbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Internal 48MHz RC oscillator calibration value register"]
    #[inline(always)]
    pub fn irc48mcal(&self) -> Irc48mcalR {
        Irc48mcalR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 48MHz clock selection"]
    #[inline(always)]
    pub fn ck48msel(&mut self) -> Ck48mselW<'_, AddctlSpec> {
        Ck48mselW::new(self, 0)
    }
    #[doc = "Bit 1 - PLL48M clock selection"]
    #[inline(always)]
    pub fn pll48msel(&mut self) -> Pll48mselW<'_, AddctlSpec> {
        Pll48mselW::new(self, 1)
    }
    #[doc = "Bit 16 - Internal 48MHz RC oscillator enable"]
    #[inline(always)]
    pub fn irc48men(&mut self) -> Irc48menW<'_, AddctlSpec> {
        Irc48menW::new(self, 16)
    }
}
#[doc = "Additional clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`addctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddctlSpec;
impl crate::RegisterSpec for AddctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addctl::R`](R) reader structure"]
impl crate::Readable for AddctlSpec {}
#[doc = "`write(|w| ..)` method takes [`addctl::W`](W) writer structure"]
impl crate::Writable for AddctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDCTL to value 0"]
impl crate::Resettable for AddctlSpec {}
