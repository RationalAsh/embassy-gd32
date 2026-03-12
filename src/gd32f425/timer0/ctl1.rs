#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Commutation control shadow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccse {
    #[doc = "0: Shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits disabled"]
    NotPreloaded = 0,
    #[doc = "1: Shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits enabled"]
    Preloaded = 1,
}
impl From<Ccse> for bool {
    #[inline(always)]
    fn from(variant: Ccse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCSE` reader - Commutation control shadow enable"]
pub type CcseR = crate::BitReader<Ccse>;
impl CcseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccse {
        match self.bits {
            false => Ccse::NotPreloaded,
            true => Ccse::Preloaded,
        }
    }
    #[doc = "Shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits disabled"]
    #[inline(always)]
    pub fn is_not_preloaded(&self) -> bool {
        *self == Ccse::NotPreloaded
    }
    #[doc = "Shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits enabled"]
    #[inline(always)]
    pub fn is_preloaded(&self) -> bool {
        *self == Ccse::Preloaded
    }
}
#[doc = "Field `CCSE` writer - Commutation control shadow enable"]
pub type CcseW<'a, REG> = crate::BitWriter<'a, REG, Ccse>;
impl<'a, REG> CcseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits disabled"]
    #[inline(always)]
    pub fn not_preloaded(self) -> &'a mut crate::W<REG> {
        self.variant(Ccse::NotPreloaded)
    }
    #[doc = "Shadow registers for CHxEN, CHxNEN and CHxCOMCTL bits enabled"]
    #[inline(always)]
    pub fn preloaded(self) -> &'a mut crate::W<REG> {
        self.variant(Ccse::Preloaded)
    }
}
#[doc = "Commutation control shadow register update control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccuc {
    #[doc = "0: Capture/compare shadow registers updated only by setting CMTG bit"]
    Default = 0,
    #[doc = "1: Capture/compare shadow registers updated by setting CMTG bit or when a rising edge occurs on TRGI"]
    WithRisingEdge = 1,
}
impl From<Ccuc> for bool {
    #[inline(always)]
    fn from(variant: Ccuc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUC` reader - Commutation control shadow register update control"]
pub type CcucR = crate::BitReader<Ccuc>;
impl CcucR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccuc {
        match self.bits {
            false => Ccuc::Default,
            true => Ccuc::WithRisingEdge,
        }
    }
    #[doc = "Capture/compare shadow registers updated only by setting CMTG bit"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Ccuc::Default
    }
    #[doc = "Capture/compare shadow registers updated by setting CMTG bit or when a rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn is_with_rising_edge(&self) -> bool {
        *self == Ccuc::WithRisingEdge
    }
}
#[doc = "Field `CCUC` writer - Commutation control shadow register update control"]
pub type CcucW<'a, REG> = crate::BitWriter<'a, REG, Ccuc>;
impl<'a, REG> CcucW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare shadow registers updated only by setting CMTG bit"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Ccuc::Default)
    }
    #[doc = "Capture/compare shadow registers updated by setting CMTG bit or when a rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn with_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ccuc::WithRisingEdge)
    }
}
#[doc = "Field `DMAS` reader - DMA request source selection"]
pub type DmasR = crate::BitReader;
#[doc = "Field `DMAS` writer - DMA request source selection"]
pub type DmasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMC` reader - Master mode control"]
pub type MmcR = crate::FieldReader;
#[doc = "Field `MMC` writer - Master mode control"]
pub type MmcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TI0S` reader - Channel 0 trigger input selection"]
pub type Ti0sR = crate::BitReader;
#[doc = "Field `TI0S` writer - Channel 0 trigger input selection"]
pub type Ti0sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Idle state of channel 0 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iso0 {
    #[doc = "0: CH0_O is low (after dead-time if CH0_ON is implemented) when POEN is reset"]
    Low = 0,
    #[doc = "1: CH0_O is high (after dead-time if CH0_ON is implemented) when POEN is reset"]
    High = 1,
}
impl From<Iso0> for bool {
    #[inline(always)]
    fn from(variant: Iso0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISO0` reader - Idle state of channel 0 output"]
pub type Iso0R = crate::BitReader<Iso0>;
impl Iso0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iso0 {
        match self.bits {
            false => Iso0::Low,
            true => Iso0::High,
        }
    }
    #[doc = "CH0_O is low (after dead-time if CH0_ON is implemented) when POEN is reset"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iso0::Low
    }
    #[doc = "CH0_O is high (after dead-time if CH0_ON is implemented) when POEN is reset"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iso0::High
    }
}
#[doc = "Field `ISO0` writer - Idle state of channel 0 output"]
pub type Iso0W<'a, REG> = crate::BitWriter<'a, REG, Iso0>;
impl<'a, REG> Iso0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CH0_O is low (after dead-time if CH0_ON is implemented) when POEN is reset"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iso0::Low)
    }
    #[doc = "CH0_O is high (after dead-time if CH0_ON is implemented) when POEN is reset"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iso0::High)
    }
}
#[doc = "Idle state of channel 0 complementary output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iso0n {
    #[doc = "0: CH0_ON is low when POEN is reset"]
    Low = 0,
    #[doc = "1: CH0_ON is high when POEN is reset"]
    High = 1,
}
impl From<Iso0n> for bool {
    #[inline(always)]
    fn from(variant: Iso0n) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISO0N` reader - Idle state of channel 0 complementary output"]
pub type Iso0nR = crate::BitReader<Iso0n>;
impl Iso0nR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iso0n {
        match self.bits {
            false => Iso0n::Low,
            true => Iso0n::High,
        }
    }
    #[doc = "CH0_ON is low when POEN is reset"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iso0n::Low
    }
    #[doc = "CH0_ON is high when POEN is reset"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iso0n::High
    }
}
#[doc = "Field `ISO0N` writer - Idle state of channel 0 complementary output"]
pub type Iso0nW<'a, REG> = crate::BitWriter<'a, REG, Iso0n>;
impl<'a, REG> Iso0nW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CH0_ON is low when POEN is reset"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iso0n::Low)
    }
    #[doc = "CH0_ON is high when POEN is reset"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iso0n::High)
    }
}
#[doc = "Idle state of channel 1 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iso1 {
    #[doc = "0: CH1_O is low (after dead-time if CH1_ON is implemented) when POEN is reset"]
    Low = 0,
    #[doc = "1: CH1_O is high (after dead-time if CH1_ON is implemented) when POEN is reset"]
    High = 1,
}
impl From<Iso1> for bool {
    #[inline(always)]
    fn from(variant: Iso1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISO1` reader - Idle state of channel 1 output"]
pub type Iso1R = crate::BitReader<Iso1>;
impl Iso1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iso1 {
        match self.bits {
            false => Iso1::Low,
            true => Iso1::High,
        }
    }
    #[doc = "CH1_O is low (after dead-time if CH1_ON is implemented) when POEN is reset"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iso1::Low
    }
    #[doc = "CH1_O is high (after dead-time if CH1_ON is implemented) when POEN is reset"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iso1::High
    }
}
#[doc = "Field `ISO1` writer - Idle state of channel 1 output"]
pub type Iso1W<'a, REG> = crate::BitWriter<'a, REG, Iso1>;
impl<'a, REG> Iso1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CH1_O is low (after dead-time if CH1_ON is implemented) when POEN is reset"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iso1::Low)
    }
    #[doc = "CH1_O is high (after dead-time if CH1_ON is implemented) when POEN is reset"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iso1::High)
    }
}
#[doc = "Idle state of channel 1 complementary output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iso1n {
    #[doc = "0: CH1_ON is low when POEN is reset"]
    Low = 0,
    #[doc = "1: CH1_ON is high when POEN is reset"]
    High = 1,
}
impl From<Iso1n> for bool {
    #[inline(always)]
    fn from(variant: Iso1n) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISO1N` reader - Idle state of channel 1 complementary output"]
pub type Iso1nR = crate::BitReader<Iso1n>;
impl Iso1nR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iso1n {
        match self.bits {
            false => Iso1n::Low,
            true => Iso1n::High,
        }
    }
    #[doc = "CH1_ON is low when POEN is reset"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iso1n::Low
    }
    #[doc = "CH1_ON is high when POEN is reset"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iso1n::High
    }
}
#[doc = "Field `ISO1N` writer - Idle state of channel 1 complementary output"]
pub type Iso1nW<'a, REG> = crate::BitWriter<'a, REG, Iso1n>;
impl<'a, REG> Iso1nW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CH1_ON is low when POEN is reset"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iso1n::Low)
    }
    #[doc = "CH1_ON is high when POEN is reset"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iso1n::High)
    }
}
#[doc = "Idle state of channel 2 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iso2 {
    #[doc = "0: CH2_O is low (after dead-time if CH2_ON is implemented) when POEN is reset"]
    Low = 0,
    #[doc = "1: CH2_O is high (after dead-time if CH2_ON is implemented) when POEN is reset"]
    High = 1,
}
impl From<Iso2> for bool {
    #[inline(always)]
    fn from(variant: Iso2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISO2` reader - Idle state of channel 2 output"]
pub type Iso2R = crate::BitReader<Iso2>;
impl Iso2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iso2 {
        match self.bits {
            false => Iso2::Low,
            true => Iso2::High,
        }
    }
    #[doc = "CH2_O is low (after dead-time if CH2_ON is implemented) when POEN is reset"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iso2::Low
    }
    #[doc = "CH2_O is high (after dead-time if CH2_ON is implemented) when POEN is reset"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iso2::High
    }
}
#[doc = "Field `ISO2` writer - Idle state of channel 2 output"]
pub type Iso2W<'a, REG> = crate::BitWriter<'a, REG, Iso2>;
impl<'a, REG> Iso2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CH2_O is low (after dead-time if CH2_ON is implemented) when POEN is reset"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iso2::Low)
    }
    #[doc = "CH2_O is high (after dead-time if CH2_ON is implemented) when POEN is reset"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iso2::High)
    }
}
#[doc = "Idle state of channel 2 complementary output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iso2n {
    #[doc = "0: CH2_ON is low when POEN is reset"]
    Low = 0,
    #[doc = "1: CH2_ON is high when POEN is reset"]
    High = 1,
}
impl From<Iso2n> for bool {
    #[inline(always)]
    fn from(variant: Iso2n) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISO2N` reader - Idle state of channel 2 complementary output"]
pub type Iso2nR = crate::BitReader<Iso2n>;
impl Iso2nR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iso2n {
        match self.bits {
            false => Iso2n::Low,
            true => Iso2n::High,
        }
    }
    #[doc = "CH2_ON is low when POEN is reset"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iso2n::Low
    }
    #[doc = "CH2_ON is high when POEN is reset"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iso2n::High
    }
}
#[doc = "Field `ISO2N` writer - Idle state of channel 2 complementary output"]
pub type Iso2nW<'a, REG> = crate::BitWriter<'a, REG, Iso2n>;
impl<'a, REG> Iso2nW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CH2_ON is low when POEN is reset"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iso2n::Low)
    }
    #[doc = "CH2_ON is high when POEN is reset"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iso2n::High)
    }
}
#[doc = "Idle state of channel 3 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iso3 {
    #[doc = "0: CH3_O is low when POEN is reset"]
    Low = 0,
    #[doc = "1: CH3_O is high when POEN is reset"]
    High = 1,
}
impl From<Iso3> for bool {
    #[inline(always)]
    fn from(variant: Iso3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISO3` reader - Idle state of channel 3 output"]
pub type Iso3R = crate::BitReader<Iso3>;
impl Iso3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iso3 {
        match self.bits {
            false => Iso3::Low,
            true => Iso3::High,
        }
    }
    #[doc = "CH3_O is low when POEN is reset"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Iso3::Low
    }
    #[doc = "CH3_O is high when POEN is reset"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Iso3::High
    }
}
#[doc = "Field `ISO3` writer - Idle state of channel 3 output"]
pub type Iso3W<'a, REG> = crate::BitWriter<'a, REG, Iso3>;
impl<'a, REG> Iso3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CH3_O is low when POEN is reset"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Iso3::Low)
    }
    #[doc = "CH3_O is high when POEN is reset"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Iso3::High)
    }
}
impl R {
    #[doc = "Bit 0 - Commutation control shadow enable"]
    #[inline(always)]
    pub fn ccse(&self) -> CcseR {
        CcseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    pub fn ccuc(&self) -> CcucR {
        CcucR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&self) -> DmasR {
        DmasR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    pub fn mmc(&self) -> MmcR {
        MmcR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 0 trigger input selection"]
    #[inline(always)]
    pub fn ti0s(&self) -> Ti0sR {
        Ti0sR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    pub fn iso0(&self) -> Iso0R {
        Iso0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Idle state of channel 0 complementary output"]
    #[inline(always)]
    pub fn iso0n(&self) -> Iso0nR {
        Iso0nR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso1(&self) -> Iso1R {
        Iso1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Idle state of channel 1 complementary output"]
    #[inline(always)]
    pub fn iso1n(&self) -> Iso1nR {
        Iso1nR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Idle state of channel 2 output"]
    #[inline(always)]
    pub fn iso2(&self) -> Iso2R {
        Iso2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Idle state of channel 2 complementary output"]
    #[inline(always)]
    pub fn iso2n(&self) -> Iso2nR {
        Iso2nR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Idle state of channel 3 output"]
    #[inline(always)]
    pub fn iso3(&self) -> Iso3R {
        Iso3R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Commutation control shadow enable"]
    #[inline(always)]
    pub fn ccse(&mut self) -> CcseW<'_, Ctl1Spec> {
        CcseW::new(self, 0)
    }
    #[doc = "Bit 2 - Commutation control shadow register update control"]
    #[inline(always)]
    pub fn ccuc(&mut self) -> CcucW<'_, Ctl1Spec> {
        CcucW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA request source selection"]
    #[inline(always)]
    pub fn dmas(&mut self) -> DmasW<'_, Ctl1Spec> {
        DmasW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Master mode control"]
    #[inline(always)]
    pub fn mmc(&mut self) -> MmcW<'_, Ctl1Spec> {
        MmcW::new(self, 4)
    }
    #[doc = "Bit 7 - Channel 0 trigger input selection"]
    #[inline(always)]
    pub fn ti0s(&mut self) -> Ti0sW<'_, Ctl1Spec> {
        Ti0sW::new(self, 7)
    }
    #[doc = "Bit 8 - Idle state of channel 0 output"]
    #[inline(always)]
    pub fn iso0(&mut self) -> Iso0W<'_, Ctl1Spec> {
        Iso0W::new(self, 8)
    }
    #[doc = "Bit 9 - Idle state of channel 0 complementary output"]
    #[inline(always)]
    pub fn iso0n(&mut self) -> Iso0nW<'_, Ctl1Spec> {
        Iso0nW::new(self, 9)
    }
    #[doc = "Bit 10 - Idle state of channel 1 output"]
    #[inline(always)]
    pub fn iso1(&mut self) -> Iso1W<'_, Ctl1Spec> {
        Iso1W::new(self, 10)
    }
    #[doc = "Bit 11 - Idle state of channel 1 complementary output"]
    #[inline(always)]
    pub fn iso1n(&mut self) -> Iso1nW<'_, Ctl1Spec> {
        Iso1nW::new(self, 11)
    }
    #[doc = "Bit 12 - Idle state of channel 2 output"]
    #[inline(always)]
    pub fn iso2(&mut self) -> Iso2W<'_, Ctl1Spec> {
        Iso2W::new(self, 12)
    }
    #[doc = "Bit 13 - Idle state of channel 2 complementary output"]
    #[inline(always)]
    pub fn iso2n(&mut self) -> Iso2nW<'_, Ctl1Spec> {
        Iso2nW::new(self, 13)
    }
    #[doc = "Bit 14 - Idle state of channel 3 output"]
    #[inline(always)]
    pub fn iso3(&mut self) -> Iso3W<'_, Ctl1Spec> {
        Iso3W::new(self, 14)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
