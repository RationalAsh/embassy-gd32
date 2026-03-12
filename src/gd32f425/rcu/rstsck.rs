#[doc = "Register `RSTSCK` reader"]
pub type R = crate::R<RstsckSpec>;
#[doc = "Register `RSTSCK` writer"]
pub type W = crate::W<RstsckSpec>;
#[doc = "IRC32K enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc32ken {
    #[doc = "0: IRC32K oscillator disabled"]
    Off = 0,
    #[doc = "1: IRC32K oscillator enabled"]
    On = 1,
}
impl From<Irc32ken> for bool {
    #[inline(always)]
    fn from(variant: Irc32ken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC32KEN` reader - IRC32K enable"]
pub type Irc32kenR = crate::BitReader<Irc32ken>;
impl Irc32kenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc32ken {
        match self.bits {
            false => Irc32ken::Off,
            true => Irc32ken::On,
        }
    }
    #[doc = "IRC32K oscillator disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Irc32ken::Off
    }
    #[doc = "IRC32K oscillator enabled"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Irc32ken::On
    }
}
#[doc = "Field `IRC32KEN` writer - IRC32K enable"]
pub type Irc32kenW<'a, REG> = crate::BitWriter<'a, REG, Irc32ken>;
impl<'a, REG> Irc32kenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IRC32K oscillator disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Irc32ken::Off)
    }
    #[doc = "IRC32K oscillator enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Irc32ken::On)
    }
}
#[doc = "IRC32K stabilization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc32kstbr {
    #[doc = "0: IRC32K oscillator is not stable"]
    NotReady = 0,
    #[doc = "1: IRC32K oscillator is stable"]
    Ready = 1,
}
impl From<Irc32kstbr> for bool {
    #[inline(always)]
    fn from(variant: Irc32kstbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC32KSTB` reader - IRC32K stabilization"]
pub type Irc32kstbR = crate::BitReader<Irc32kstbr>;
impl Irc32kstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc32kstbr {
        match self.bits {
            false => Irc32kstbr::NotReady,
            true => Irc32kstbr::Ready,
        }
    }
    #[doc = "IRC32K oscillator is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Irc32kstbr::NotReady
    }
    #[doc = "IRC32K oscillator is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Irc32kstbr::Ready
    }
}
#[doc = "Reset flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstfcw {
    #[doc = "1: Clears reset flags"]
    Clear = 1,
}
impl From<Rstfcw> for bool {
    #[inline(always)]
    fn from(variant: Rstfcw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTFC` reader - Reset flag clear"]
pub type RstfcR = crate::BitReader<Rstfcw>;
impl RstfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rstfcw> {
        match self.bits {
            true => Some(Rstfcw::Clear),
            _ => None,
        }
    }
    #[doc = "Clears reset flags"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Rstfcw::Clear
    }
}
#[doc = "Field `RSTFC` writer - Reset flag clear"]
pub type RstfcW<'a, REG> = crate::BitWriter<'a, REG, Rstfcw>;
impl<'a, REG> RstfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears reset flags"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfcw::Clear)
    }
}
#[doc = "BOR reset flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Borrstfr {
    #[doc = "0: No reset has occured"]
    NoReset = 0,
    #[doc = "1: A reset has occured"]
    Reset = 1,
}
impl From<Borrstfr> for bool {
    #[inline(always)]
    fn from(variant: Borrstfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORRSTF` reader - BOR reset flag"]
pub type BorrstfR = crate::BitReader<Borrstfr>;
impl BorrstfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Borrstfr {
        match self.bits {
            false => Borrstfr::NoReset,
            true => Borrstfr::Reset,
        }
    }
    #[doc = "No reset has occured"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == Borrstfr::NoReset
    }
    #[doc = "A reset has occured"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Borrstfr::Reset
    }
}
#[doc = "Field `EPRSTF` reader - External PIN reset flag"]
pub use BorrstfR as EprstfR;
#[doc = "Field `PORRSTF` reader - Power reset flag"]
pub use BorrstfR as PorrstfR;
#[doc = "Field `SWRSTF` reader - Software reset flag"]
pub use BorrstfR as SwrstfR;
#[doc = "Field `FWDGTRSTF` reader - Free Watchdog timer reset flag"]
pub use BorrstfR as FwdgtrstfR;
#[doc = "Field `WWDGTRSTF` reader - Window watchdog timer reset flag"]
pub use BorrstfR as WwdgtrstfR;
#[doc = "Field `LPRSTF` reader - Low-power reset flag"]
pub use BorrstfR as LprstfR;
impl R {
    #[doc = "Bit 0 - IRC32K enable"]
    #[inline(always)]
    pub fn irc32ken(&self) -> Irc32kenR {
        Irc32kenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC32K stabilization"]
    #[inline(always)]
    pub fn irc32kstb(&self) -> Irc32kstbR {
        Irc32kstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    pub fn rstfc(&self) -> RstfcR {
        RstfcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - BOR reset flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BorrstfR {
        BorrstfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - External PIN reset flag"]
    #[inline(always)]
    pub fn eprstf(&self) -> EprstfR {
        EprstfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Power reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PorrstfR {
        PorrstfR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&self) -> SwrstfR {
        SwrstfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Free Watchdog timer reset flag"]
    #[inline(always)]
    pub fn fwdgtrstf(&self) -> FwdgtrstfR {
        FwdgtrstfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    pub fn wwdgtrstf(&self) -> WwdgtrstfR {
        WwdgtrstfR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lprstf(&self) -> LprstfR {
        LprstfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRC32K enable"]
    #[inline(always)]
    pub fn irc32ken(&mut self) -> Irc32kenW<'_, RstsckSpec> {
        Irc32kenW::new(self, 0)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    pub fn rstfc(&mut self) -> RstfcW<'_, RstsckSpec> {
        RstfcW::new(self, 24)
    }
}
#[doc = "Reset source /clock register (RCU_RSTSCK)\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstsckSpec;
impl crate::RegisterSpec for RstsckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstsck::R`](R) reader structure"]
impl crate::Readable for RstsckSpec {}
#[doc = "`write(|w| ..)` method takes [`rstsck::W`](W) writer structure"]
impl crate::Writable for RstsckSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSTSCK to value 0x0e00_0000"]
impl crate::Resettable for RstsckSpec {
    const RESET_VALUE: u32 = 0x0e00_0000;
}
