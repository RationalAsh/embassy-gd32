#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Internal 16MHz RC oscillator Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc16men {
    #[doc = "0: Clock Off"]
    Off = 0,
    #[doc = "1: Clock On"]
    On = 1,
}
impl From<Irc16men> for bool {
    #[inline(always)]
    fn from(variant: Irc16men) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC16MEN` reader - Internal 16MHz RC oscillator Enable"]
pub type Irc16menR = crate::BitReader<Irc16men>;
impl Irc16menR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc16men {
        match self.bits {
            false => Irc16men::Off,
            true => Irc16men::On,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Irc16men::Off
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Irc16men::On
    }
}
#[doc = "Field `IRC16MEN` writer - Internal 16MHz RC oscillator Enable"]
pub type Irc16menW<'a, REG> = crate::BitWriter<'a, REG, Irc16men>;
impl<'a, REG> Irc16menW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Irc16men::Off)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Irc16men::On)
    }
}
#[doc = "IRC16M Internal 16MHz RC Oscillator stabilization Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irc16mstbr {
    #[doc = "0: IRC16M is not stable"]
    NotReady = 0,
    #[doc = "1: IRC16M is stable"]
    Ready = 1,
}
impl From<Irc16mstbr> for bool {
    #[inline(always)]
    fn from(variant: Irc16mstbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC16MSTB` reader - IRC16M Internal 16MHz RC Oscillator stabilization Flag"]
pub type Irc16mstbR = crate::BitReader<Irc16mstbr>;
impl Irc16mstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irc16mstbr {
        match self.bits {
            false => Irc16mstbr::NotReady,
            true => Irc16mstbr::Ready,
        }
    }
    #[doc = "IRC16M is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Irc16mstbr::NotReady
    }
    #[doc = "IRC16M is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Irc16mstbr::Ready
    }
}
#[doc = "Field `IRC16MADJ` reader - Internal 16MHz RC Oscillator clock trim adjust value"]
pub type Irc16madjR = crate::FieldReader;
#[doc = "Field `IRC16MADJ` writer - Internal 16MHz RC Oscillator clock trim adjust value"]
pub type Irc16madjW<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `IRC16MCALIB` reader - Internal 16MHz RC Oscillator calibration value register"]
pub type Irc16mcalibR = crate::FieldReader;
#[doc = "External High Speed oscillator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hxtalen {
    #[doc = "0: Clock Off"]
    Off = 0,
    #[doc = "1: Clock On"]
    On = 1,
}
impl From<Hxtalen> for bool {
    #[inline(always)]
    fn from(variant: Hxtalen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALEN` reader - External High Speed oscillator Enable"]
pub type HxtalenR = crate::BitReader<Hxtalen>;
impl HxtalenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hxtalen {
        match self.bits {
            false => Hxtalen::Off,
            true => Hxtalen::On,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Hxtalen::Off
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Hxtalen::On
    }
}
#[doc = "Field `HXTALEN` writer - External High Speed oscillator Enable"]
pub type HxtalenW<'a, REG> = crate::BitWriter<'a, REG, Hxtalen>;
impl<'a, REG> HxtalenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalen::Off)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalen::On)
    }
}
#[doc = "External crystal oscillator (HXTAL) clock stabilization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hxtalstbr {
    #[doc = "0: HXTAL is not stable"]
    NotReady = 0,
    #[doc = "1: HXTAL is stable"]
    Ready = 1,
}
impl From<Hxtalstbr> for bool {
    #[inline(always)]
    fn from(variant: Hxtalstbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALSTB` reader - External crystal oscillator (HXTAL) clock stabilization flag"]
pub type HxtalstbR = crate::BitReader<Hxtalstbr>;
impl HxtalstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hxtalstbr {
        match self.bits {
            false => Hxtalstbr::NotReady,
            true => Hxtalstbr::Ready,
        }
    }
    #[doc = "HXTAL is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Hxtalstbr::NotReady
    }
    #[doc = "HXTAL is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Hxtalstbr::Ready
    }
}
#[doc = "External crystal oscillator (HXTAL) clock bypass mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hxtalbps {
    #[doc = "0: HXTAL crystal oscillator not bypassed"]
    NotBypassed = 0,
    #[doc = "1: HXTAL crystal oscillator bypassed with external clock"]
    Bypassed = 1,
}
impl From<Hxtalbps> for bool {
    #[inline(always)]
    fn from(variant: Hxtalbps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HXTALBPS` reader - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HxtalbpsR = crate::BitReader<Hxtalbps>;
impl HxtalbpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hxtalbps {
        match self.bits {
            false => Hxtalbps::NotBypassed,
            true => Hxtalbps::Bypassed,
        }
    }
    #[doc = "HXTAL crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == Hxtalbps::NotBypassed
    }
    #[doc = "HXTAL crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == Hxtalbps::Bypassed
    }
}
#[doc = "Field `HXTALBPS` writer - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HxtalbpsW<'a, REG> = crate::BitWriter<'a, REG, Hxtalbps>;
impl<'a, REG> HxtalbpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HXTAL crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalbps::NotBypassed)
    }
    #[doc = "HXTAL crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(Hxtalbps::Bypassed)
    }
}
#[doc = "HXTAL Clock Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ckmen {
    #[doc = "0: Clock monitor disabled"]
    Off = 0,
    #[doc = "1: Clock monitor enabled"]
    On = 1,
}
impl From<Ckmen> for bool {
    #[inline(always)]
    fn from(variant: Ckmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKMEN` reader - HXTAL Clock Monitor Enable"]
pub type CkmenR = crate::BitReader<Ckmen>;
impl CkmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckmen {
        match self.bits {
            false => Ckmen::Off,
            true => Ckmen::On,
        }
    }
    #[doc = "Clock monitor disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ckmen::Off
    }
    #[doc = "Clock monitor enabled"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ckmen::On
    }
}
#[doc = "Field `CKMEN` writer - HXTAL Clock Monitor Enable"]
pub type CkmenW<'a, REG> = crate::BitWriter<'a, REG, Ckmen>;
impl<'a, REG> CkmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock monitor disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Ckmen::Off)
    }
    #[doc = "Clock monitor enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Ckmen::On)
    }
}
#[doc = "PLL enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllen {
    #[doc = "0: Clock Off"]
    Off = 0,
    #[doc = "1: Clock On"]
    On = 1,
}
impl From<Pllen> for bool {
    #[inline(always)]
    fn from(variant: Pllen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLEN` reader - PLL enable"]
pub type PllenR = crate::BitReader<Pllen>;
impl PllenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllen {
        match self.bits {
            false => Pllen::Off,
            true => Pllen::On,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Pllen::Off
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Pllen::On
    }
}
#[doc = "Field `PLLEN` writer - PLL enable"]
pub type PllenW<'a, REG> = crate::BitWriter<'a, REG, Pllen>;
impl<'a, REG> PllenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Pllen::Off)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Pllen::On)
    }
}
#[doc = "PLL Clock Stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllstbr {
    #[doc = "0: PLL is not stable"]
    NotReady = 0,
    #[doc = "1: PLL is stable"]
    Ready = 1,
}
impl From<Pllstbr> for bool {
    #[inline(always)]
    fn from(variant: Pllstbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTB` reader - PLL Clock Stabilization Flag"]
pub type PllstbR = crate::BitReader<Pllstbr>;
impl PllstbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllstbr {
        match self.bits {
            false => Pllstbr::NotReady,
            true => Pllstbr::Ready,
        }
    }
    #[doc = "PLL is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Pllstbr::NotReady
    }
    #[doc = "PLL is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Pllstbr::Ready
    }
}
#[doc = "Field `PLLI2SEN` reader - PLLI2S enable"]
pub use PllenR as Plli2senR;
#[doc = "Field `PLLSAIEN` reader - PLLSAI enable"]
pub use PllenR as PllsaienR;
#[doc = "Field `PLLI2SEN` writer - PLLI2S enable"]
pub use PllenW as Plli2senW;
#[doc = "Field `PLLSAIEN` writer - PLLSAI enable"]
pub use PllenW as PllsaienW;
#[doc = "Field `PLLI2SSTB` reader - PLLI2S Clock Stabilization Flag"]
pub use PllstbR as Plli2sstbR;
#[doc = "Field `PLLSAISTB` reader - PLLSAI Clock Stabilization Flag"]
pub use PllstbR as PllsaistbR;
impl R {
    #[doc = "Bit 0 - Internal 16MHz RC oscillator Enable"]
    #[inline(always)]
    pub fn irc16men(&self) -> Irc16menR {
        Irc16menR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC16M Internal 16MHz RC Oscillator stabilization Flag"]
    #[inline(always)]
    pub fn irc16mstb(&self) -> Irc16mstbR {
        Irc16mstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Internal 16MHz RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc16madj(&self) -> Irc16madjR {
        Irc16madjR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Internal 16MHz RC Oscillator calibration value register"]
    #[inline(always)]
    pub fn irc16mcalib(&self) -> Irc16mcalibR {
        Irc16mcalibR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    pub fn hxtalen(&self) -> HxtalenR {
        HxtalenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External crystal oscillator (HXTAL) clock stabilization flag"]
    #[inline(always)]
    pub fn hxtalstb(&self) -> HxtalstbR {
        HxtalstbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&self) -> HxtalbpsR {
        HxtalbpsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    pub fn ckmen(&self) -> CkmenR {
        CkmenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PllenR {
        PllenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pllstb(&self) -> PllstbR {
        PllstbR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PLLI2S enable"]
    #[inline(always)]
    pub fn plli2sen(&self) -> Plli2senR {
        Plli2senR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLLI2S Clock Stabilization Flag"]
    #[inline(always)]
    pub fn plli2sstb(&self) -> Plli2sstbR {
        Plli2sstbR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PLLSAI enable"]
    #[inline(always)]
    pub fn pllsaien(&self) -> PllsaienR {
        PllsaienR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLLSAI Clock Stabilization Flag"]
    #[inline(always)]
    pub fn pllsaistb(&self) -> PllsaistbR {
        PllsaistbR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal 16MHz RC oscillator Enable"]
    #[inline(always)]
    pub fn irc16men(&mut self) -> Irc16menW<'_, Ctl0Spec> {
        Irc16menW::new(self, 0)
    }
    #[doc = "Bits 3:7 - Internal 16MHz RC Oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc16madj(&mut self) -> Irc16madjW<'_, Ctl0Spec> {
        Irc16madjW::new(self, 3)
    }
    #[doc = "Bit 16 - External High Speed oscillator Enable"]
    #[inline(always)]
    pub fn hxtalen(&mut self) -> HxtalenW<'_, Ctl0Spec> {
        HxtalenW::new(self, 16)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&mut self) -> HxtalbpsW<'_, Ctl0Spec> {
        HxtalbpsW::new(self, 18)
    }
    #[doc = "Bit 19 - HXTAL Clock Monitor Enable"]
    #[inline(always)]
    pub fn ckmen(&mut self) -> CkmenW<'_, Ctl0Spec> {
        CkmenW::new(self, 19)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PllenW<'_, Ctl0Spec> {
        PllenW::new(self, 24)
    }
    #[doc = "Bit 26 - PLLI2S enable"]
    #[inline(always)]
    pub fn plli2sen(&mut self) -> Plli2senW<'_, Ctl0Spec> {
        Plli2senW::new(self, 26)
    }
    #[doc = "Bit 28 - PLLSAI enable"]
    #[inline(always)]
    pub fn pllsaien(&mut self) -> PllsaienW<'_, Ctl0Spec> {
        PllsaienW::new(self, 28)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL0 to value 0x83"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0x83;
}
