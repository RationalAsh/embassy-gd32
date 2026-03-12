#[doc = "Register `INT` reader"]
pub type R = crate::R<IntSpec>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<IntSpec>;
#[doc = "IRC32K stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc32kstbifr {
    #[doc = "0: No interrupt generated"]
    NotInterrupted = 0,
    #[doc = "1: IRC32K stabilisation interrupt generated"]
    Interrupted = 1,
}
impl From<Irc32kstbifr> for bool {
    #[inline(always)]
    fn from(variant: Irc32kstbifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC32KSTBIF` reader - IRC32K stabilization interrupt flag"]
pub type Irc32kstbifR = crate::BitReader<Irc32kstbifr>;
impl Irc32kstbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc32kstbifr {
        match self.bits {
            false => Irc32kstbifr::NotInterrupted,
            true => Irc32kstbifr::Interrupted,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Irc32kstbifr::NotInterrupted
    }
    #[doc = "IRC32K stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Irc32kstbifr::Interrupted
    }
}
#[doc = "LXTAL stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lxtalstbifr {
    #[doc = "0: No interrupt generated"]
    NotInterrupted = 0,
    #[doc = "1: LXTAL stabilisation interrupt generated"]
    Interrupted = 1,
}
impl From<Lxtalstbifr> for bool {
    #[inline(always)]
    fn from(variant: Lxtalstbifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALSTBIF` reader - LXTAL stabilization interrupt flag"]
pub type LxtalstbifR = crate::BitReader<Lxtalstbifr>;
impl LxtalstbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lxtalstbifr {
        match self.bits {
            false => Lxtalstbifr::NotInterrupted,
            true => Lxtalstbifr::Interrupted,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Lxtalstbifr::NotInterrupted
    }
    #[doc = "LXTAL stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Lxtalstbifr::Interrupted
    }
}
#[doc = "IRC16M stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc16mstbifr {
    #[doc = "0: No interrupt generated"]
    NotInterrupted = 0,
    #[doc = "1: IRC16M stabilisation interrupt generated"]
    Interrupted = 1,
}
impl From<Irc16mstbifr> for bool {
    #[inline(always)]
    fn from(variant: Irc16mstbifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC16MSTBIF` reader - IRC16M stabilization interrupt flag"]
pub type Irc16mstbifR = crate::BitReader<Irc16mstbifr>;
impl Irc16mstbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc16mstbifr {
        match self.bits {
            false => Irc16mstbifr::NotInterrupted,
            true => Irc16mstbifr::Interrupted,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Irc16mstbifr::NotInterrupted
    }
    #[doc = "IRC16M stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Irc16mstbifr::Interrupted
    }
}
#[doc = "HXTAL stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hxtalstbifr {
    #[doc = "0: No interrupt generated"]
    NotInterrupted = 0,
    #[doc = "1: HXTAL stabilisation interrupt generated"]
    Interrupted = 1,
}
impl From<Hxtalstbifr> for bool {
    #[inline(always)]
    fn from(variant: Hxtalstbifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALSTBIF` reader - HXTAL stabilization interrupt flag"]
pub type HxtalstbifR = crate::BitReader<Hxtalstbifr>;
impl HxtalstbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hxtalstbifr {
        match self.bits {
            false => Hxtalstbifr::NotInterrupted,
            true => Hxtalstbifr::Interrupted,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Hxtalstbifr::NotInterrupted
    }
    #[doc = "HXTAL stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Hxtalstbifr::Interrupted
    }
}
#[doc = "PLL stabilization interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllstbifr {
    #[doc = "0: No interrupt generated"]
    NotInterrupted = 0,
    #[doc = "1: PLL stabilisation interrupt generated"]
    Interrupted = 1,
}
impl From<Pllstbifr> for bool {
    #[inline(always)]
    fn from(variant: Pllstbifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTBIF` reader - PLL stabilization interrupt flag"]
pub type PllstbifR = crate::BitReader<Pllstbifr>;
impl PllstbifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllstbifr {
        match self.bits {
            false => Pllstbifr::NotInterrupted,
            true => Pllstbifr::Interrupted,
        }
    }
    #[doc = "No interrupt generated"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Pllstbifr::NotInterrupted
    }
    #[doc = "PLL stabilisation interrupt generated"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Pllstbifr::Interrupted
    }
}
#[doc = "Field `PLLI2SSTBIF` reader - PLLI2S stabilization interrupt flag"]
pub use PllstbifR as Plli2sstbifR;
#[doc = "Field `PLLSAISTBIF` reader - PLLSAI stabilization interrupt flag"]
pub use PllstbifR as PllsaistbifR;
#[doc = "HXTAL Clock Stuck Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckmifr {
    #[doc = "0: Clock operating normally"]
    NotInterrupted = 0,
    #[doc = "1: HXTAL clock stuck"]
    Interrupted = 1,
}
impl From<Ckmifr> for bool {
    #[inline(always)]
    fn from(variant: Ckmifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKMIF` reader - HXTAL Clock Stuck Interrupt Flag"]
pub type CkmifR = crate::BitReader<Ckmifr>;
impl CkmifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckmifr {
        match self.bits {
            false => Ckmifr::NotInterrupted,
            true => Ckmifr::Interrupted,
        }
    }
    #[doc = "Clock operating normally"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == Ckmifr::NotInterrupted
    }
    #[doc = "HXTAL clock stuck"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == Ckmifr::Interrupted
    }
}
#[doc = "IRC32K Stabilization interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc32kstbie {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<Irc32kstbie> for bool {
    #[inline(always)]
    fn from(variant: Irc32kstbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC32KSTBIE` reader - IRC32K Stabilization interrupt enable"]
pub type Irc32kstbieR = crate::BitReader<Irc32kstbie>;
impl Irc32kstbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc32kstbie {
        match self.bits {
            false => Irc32kstbie::Disabled,
            true => Irc32kstbie::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Irc32kstbie::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Irc32kstbie::Enabled
    }
}
#[doc = "Field `IRC32KSTBIE` writer - IRC32K Stabilization interrupt enable"]
pub type Irc32kstbieW<'a, REG> = crate::BitWriter<'a, REG, Irc32kstbie>;
impl<'a, REG> Irc32kstbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Irc32kstbie::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Irc32kstbie::Enabled)
    }
}
#[doc = "LXTAL Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lxtalstbie {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<Lxtalstbie> for bool {
    #[inline(always)]
    fn from(variant: Lxtalstbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALSTBIE` reader - LXTAL Stabilization Interrupt Enable"]
pub type LxtalstbieR = crate::BitReader<Lxtalstbie>;
impl LxtalstbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lxtalstbie {
        match self.bits {
            false => Lxtalstbie::Disabled,
            true => Lxtalstbie::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lxtalstbie::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lxtalstbie::Enabled
    }
}
#[doc = "Field `LXTALSTBIE` writer - LXTAL Stabilization Interrupt Enable"]
pub type LxtalstbieW<'a, REG> = crate::BitWriter<'a, REG, Lxtalstbie>;
impl<'a, REG> LxtalstbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtalstbie::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtalstbie::Enabled)
    }
}
#[doc = "IRC16M Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc16mstbie {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<Irc16mstbie> for bool {
    #[inline(always)]
    fn from(variant: Irc16mstbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC16MSTBIE` reader - IRC16M Stabilization Interrupt Enable"]
pub type Irc16mstbieR = crate::BitReader<Irc16mstbie>;
impl Irc16mstbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc16mstbie {
        match self.bits {
            false => Irc16mstbie::Disabled,
            true => Irc16mstbie::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Irc16mstbie::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Irc16mstbie::Enabled
    }
}
#[doc = "Field `IRC16MSTBIE` writer - IRC16M Stabilization Interrupt Enable"]
pub type Irc16mstbieW<'a, REG> = crate::BitWriter<'a, REG, Irc16mstbie>;
impl<'a, REG> Irc16mstbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Irc16mstbie::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Irc16mstbie::Enabled)
    }
}
#[doc = "HXTAL Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hxtalstbie {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<Hxtalstbie> for bool {
    #[inline(always)]
    fn from(variant: Hxtalstbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALSTBIE` reader - HXTAL Stabilization Interrupt Enable"]
pub type HxtalstbieR = crate::BitReader<Hxtalstbie>;
impl HxtalstbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hxtalstbie {
        match self.bits {
            false => Hxtalstbie::Disabled,
            true => Hxtalstbie::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hxtalstbie::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hxtalstbie::Enabled
    }
}
#[doc = "Field `HXTALSTBIE` writer - HXTAL Stabilization Interrupt Enable"]
pub type HxtalstbieW<'a, REG> = crate::BitWriter<'a, REG, Hxtalstbie>;
impl<'a, REG> HxtalstbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalstbie::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalstbie::Enabled)
    }
}
#[doc = "PLL Stabilization Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllstbie {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<Pllstbie> for bool {
    #[inline(always)]
    fn from(variant: Pllstbie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTBIE` reader - PLL Stabilization Interrupt Enable"]
pub type PllstbieR = crate::BitReader<Pllstbie>;
impl PllstbieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllstbie {
        match self.bits {
            false => Pllstbie::Disabled,
            true => Pllstbie::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pllstbie::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pllstbie::Enabled
    }
}
#[doc = "Field `PLLSTBIE` writer - PLL Stabilization Interrupt Enable"]
pub type PllstbieW<'a, REG> = crate::BitWriter<'a, REG, Pllstbie>;
impl<'a, REG> PllstbieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pllstbie::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pllstbie::Enabled)
    }
}
#[doc = "Field `PLLI2SSTBIE` reader - PLLI2S Stabilization Interrupt Enable"]
pub use PllstbieR as Plli2sstbieR;
#[doc = "Field `PLLSAISTBIE` reader - PLLSAI Stabilization Interrupt Enable"]
pub use PllstbieR as PllsaistbieR;
#[doc = "Field `PLLI2SSTBIE` writer - PLLI2S Stabilization Interrupt Enable"]
pub use PllstbieW as Plli2sstbieW;
#[doc = "Field `PLLSAISTBIE` writer - PLLSAI Stabilization Interrupt Enable"]
pub use PllstbieW as PllsaistbieW;
#[doc = "IRC32K Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc32kstbicw {
    #[doc = "1: Clear IRC32KSTBIF flag"]
    Clear = 1,
}
impl From<Irc32kstbicw> for bool {
    #[inline(always)]
    fn from(variant: Irc32kstbicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC32KSTBIC` writer - IRC32K Stabilization Interrupt Clear"]
pub type Irc32kstbicW<'a, REG> = crate::BitWriter<'a, REG, Irc32kstbicw>;
impl<'a, REG> Irc32kstbicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IRC32KSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Irc32kstbicw::Clear)
    }
}
#[doc = "LXTAL Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lxtalstbicw {
    #[doc = "1: Clear LXTALSTBIF flag"]
    Clear = 1,
}
impl From<Lxtalstbicw> for bool {
    #[inline(always)]
    fn from(variant: Lxtalstbicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LXTALSTBIC` writer - LXTAL Stabilization Interrupt Clear"]
pub type LxtalstbicW<'a, REG> = crate::BitWriter<'a, REG, Lxtalstbicw>;
impl<'a, REG> LxtalstbicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear LXTALSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Lxtalstbicw::Clear)
    }
}
#[doc = "IRC16M Stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc16mstbicw {
    #[doc = "1: Clear IRC16MSTBIF flag"]
    Clear = 1,
}
impl From<Irc16mstbicw> for bool {
    #[inline(always)]
    fn from(variant: Irc16mstbicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC16MSTBIC` writer - IRC16M Stabilization Interrupt Clear"]
pub type Irc16mstbicW<'a, REG> = crate::BitWriter<'a, REG, Irc16mstbicw>;
impl<'a, REG> Irc16mstbicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IRC16MSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Irc16mstbicw::Clear)
    }
}
#[doc = "Field `HXTALSTBIC` writer - HXTAL Stabilization Interrupt Clear"]
pub type HxtalstbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PLL stabilization Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllstbicw {
    #[doc = "1: Clear PLLSTBIF flag"]
    Clear = 1,
}
impl From<Pllstbicw> for bool {
    #[inline(always)]
    fn from(variant: Pllstbicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTBIC` writer - PLL stabilization Interrupt Clear"]
pub type PllstbicW<'a, REG> = crate::BitWriter<'a, REG, Pllstbicw>;
impl<'a, REG> PllstbicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear PLLSTBIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Pllstbicw::Clear)
    }
}
#[doc = "Field `PLLI2SSTBIC` writer - PLLI2S stabilization Interrupt Clear"]
pub use PllstbicW as Plli2sstbicW;
#[doc = "Field `PLLSAISTBIC` writer - PLLSAI stabilization Interrupt Clear"]
pub use PllstbicW as PllsaistbicW;
#[doc = "HXTAL Clock Stuck Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckmicw {
    #[doc = "1: Clear CKMIF flag"]
    Clear = 1,
}
impl From<Ckmicw> for bool {
    #[inline(always)]
    fn from(variant: Ckmicw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKMIC` writer - HXTAL Clock Stuck Interrupt Clear"]
pub type CkmicW<'a, REG> = crate::BitWriter<'a, REG, Ckmicw>;
impl<'a, REG> CkmicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear CKMIF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ckmicw::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - IRC32K stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc32kstbif(&self) -> Irc32kstbifR {
        Irc32kstbifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn lxtalstbif(&self) -> LxtalstbifR {
        LxtalstbifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRC16M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc16mstbif(&self) -> Irc16mstbifR {
        Irc16mstbifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn hxtalstbif(&self) -> HxtalstbifR {
        HxtalstbifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL stabilization interrupt flag"]
    #[inline(always)]
    pub fn pllstbif(&self) -> PllstbifR {
        PllstbifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLLI2S stabilization interrupt flag"]
    #[inline(always)]
    pub fn plli2sstbif(&self) -> Plli2sstbifR {
        Plli2sstbifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLLSAI stabilization interrupt flag"]
    #[inline(always)]
    pub fn pllsaistbif(&self) -> PllsaistbifR {
        PllsaistbifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HXTAL Clock Stuck Interrupt Flag"]
    #[inline(always)]
    pub fn ckmif(&self) -> CkmifR {
        CkmifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IRC32K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc32kstbie(&self) -> Irc32kstbieR {
        Irc32kstbieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&self) -> LxtalstbieR {
        LxtalstbieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRC16M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc16mstbie(&self) -> Irc16mstbieR {
        Irc16mstbieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&self) -> HxtalstbieR {
        HxtalstbieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&self) -> PllstbieR {
        PllstbieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLLI2S Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn plli2sstbie(&self) -> Plli2sstbieR {
        Plli2sstbieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PLLSAI Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllsaistbie(&self) -> PllsaistbieR {
        PllsaistbieR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - IRC32K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc32kstbie(&mut self) -> Irc32kstbieW<'_, IntSpec> {
        Irc32kstbieW::new(self, 8)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&mut self) -> LxtalstbieW<'_, IntSpec> {
        LxtalstbieW::new(self, 9)
    }
    #[doc = "Bit 10 - IRC16M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc16mstbie(&mut self) -> Irc16mstbieW<'_, IntSpec> {
        Irc16mstbieW::new(self, 10)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&mut self) -> HxtalstbieW<'_, IntSpec> {
        HxtalstbieW::new(self, 11)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&mut self) -> PllstbieW<'_, IntSpec> {
        PllstbieW::new(self, 12)
    }
    #[doc = "Bit 13 - PLLI2S Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn plli2sstbie(&mut self) -> Plli2sstbieW<'_, IntSpec> {
        Plli2sstbieW::new(self, 13)
    }
    #[doc = "Bit 14 - PLLSAI Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllsaistbie(&mut self) -> PllsaistbieW<'_, IntSpec> {
        PllsaistbieW::new(self, 14)
    }
    #[doc = "Bit 16 - IRC32K Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc32kstbic(&mut self) -> Irc32kstbicW<'_, IntSpec> {
        Irc32kstbicW::new(self, 16)
    }
    #[doc = "Bit 17 - LXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn lxtalstbic(&mut self) -> LxtalstbicW<'_, IntSpec> {
        LxtalstbicW::new(self, 17)
    }
    #[doc = "Bit 18 - IRC16M Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc16mstbic(&mut self) -> Irc16mstbicW<'_, IntSpec> {
        Irc16mstbicW::new(self, 18)
    }
    #[doc = "Bit 19 - HXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn hxtalstbic(&mut self) -> HxtalstbicW<'_, IntSpec> {
        HxtalstbicW::new(self, 19)
    }
    #[doc = "Bit 20 - PLL stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn pllstbic(&mut self) -> PllstbicW<'_, IntSpec> {
        PllstbicW::new(self, 20)
    }
    #[doc = "Bit 21 - PLLI2S stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn plli2sstbic(&mut self) -> Plli2sstbicW<'_, IntSpec> {
        Plli2sstbicW::new(self, 21)
    }
    #[doc = "Bit 22 - PLLSAI stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn pllsaistbic(&mut self) -> PllsaistbicW<'_, IntSpec> {
        PllsaistbicW::new(self, 22)
    }
    #[doc = "Bit 23 - HXTAL Clock Stuck Interrupt Clear"]
    #[inline(always)]
    pub fn ckmic(&mut self) -> CkmicW<'_, IntSpec> {
        CkmicW::new(self, 23)
    }
}
#[doc = "Clock interrupt register (RCU_INT)\n\nYou can [`read`](crate::Reg::read) this register and get [`int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSpec;
impl crate::RegisterSpec for IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for IntSpec {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for IntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for IntSpec {}
