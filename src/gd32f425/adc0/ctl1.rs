#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "ADC on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcon {
    #[doc = "0: ADC disabled"]
    Disabled = 0,
    #[doc = "1: ADC enabled"]
    Enabled = 1,
}
impl From<Adcon> for bool {
    #[inline(always)]
    fn from(variant: Adcon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCON` reader - ADC on"]
pub type AdconR = crate::BitReader<Adcon>;
impl AdconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcon {
        match self.bits {
            false => Adcon::Disabled,
            true => Adcon::Enabled,
        }
    }
    #[doc = "ADC disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Adcon::Disabled
    }
    #[doc = "ADC enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Adcon::Enabled
    }
}
#[doc = "Field `ADCON` writer - ADC on"]
pub type AdconW<'a, REG> = crate::BitWriter<'a, REG, Adcon>;
impl<'a, REG> AdconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adcon::Disabled)
    }
    #[doc = "ADC enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adcon::Enabled)
    }
}
#[doc = "Continuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctn {
    #[doc = "0: Continuous conversion disabled"]
    Disabled = 0,
    #[doc = "1: Continuous conversion enabled"]
    Enabled = 1,
}
impl From<Ctn> for bool {
    #[inline(always)]
    fn from(variant: Ctn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTN` reader - Continuous mode"]
pub type CtnR = crate::BitReader<Ctn>;
impl CtnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctn {
        match self.bits {
            false => Ctn::Disabled,
            true => Ctn::Enabled,
        }
    }
    #[doc = "Continuous conversion disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctn::Disabled
    }
    #[doc = "Continuous conversion enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctn::Enabled
    }
}
#[doc = "Field `CTN` writer - Continuous mode"]
pub type CtnW<'a, REG> = crate::BitWriter<'a, REG, Ctn>;
impl<'a, REG> CtnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous conversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctn::Disabled)
    }
    #[doc = "Continuous conversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctn::Enabled)
    }
}
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clbr {
    #[doc = "0: Calibration complete"]
    Done = 0,
    #[doc = "1: Calibration in progress"]
    Busy = 1,
}
impl From<Clbr> for bool {
    #[inline(always)]
    fn from(variant: Clbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLB` reader - ADC calibration"]
pub type ClbR = crate::BitReader<Clbr>;
impl ClbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clbr {
        match self.bits {
            false => Clbr::Done,
            true => Clbr::Busy,
        }
    }
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == Clbr::Done
    }
    #[doc = "Calibration in progress"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Clbr::Busy
    }
}
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClbwWO {
    #[doc = "1: Start calibration"]
    Start = 1,
}
impl From<ClbwWO> for bool {
    #[inline(always)]
    fn from(variant: ClbwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLB` writer - ADC calibration"]
pub type ClbW<'a, REG> = crate::BitWriter<'a, REG, ClbwWO>;
impl<'a, REG> ClbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start calibration"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(ClbwWO::Start)
    }
}
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstclbr {
    #[doc = "0: Calibration register initialized."]
    Done = 0,
    #[doc = "1: Calibration register initializing."]
    Busy = 1,
}
impl From<Rstclbr> for bool {
    #[inline(always)]
    fn from(variant: Rstclbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCLB` reader - Reset calibration"]
pub type RstclbR = crate::BitReader<Rstclbr>;
impl RstclbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstclbr {
        match self.bits {
            false => Rstclbr::Done,
            true => Rstclbr::Busy,
        }
    }
    #[doc = "Calibration register initialized."]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == Rstclbr::Done
    }
    #[doc = "Calibration register initializing."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Rstclbr::Busy
    }
}
#[doc = "Reset calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstclbwWO {
    #[doc = "1: Initialize calibration register start."]
    Reset = 1,
}
impl From<RstclbwWO> for bool {
    #[inline(always)]
    fn from(variant: RstclbwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCLB` writer - Reset calibration"]
pub type RstclbW<'a, REG> = crate::BitWriter<'a, REG, RstclbwWO>;
impl<'a, REG> RstclbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initialize calibration register start."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RstclbwWO::Reset)
    }
}
#[doc = "DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "0: DMA request disabled"]
    Disabled = 0,
    #[doc = "1: DMA request enabled"]
    Enabled = 1,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA request enable"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            false => Dma::Disabled,
            true => Dma::Enabled,
        }
    }
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dma::Disabled
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dma::Enabled
    }
}
#[doc = "Field `DMA` writer - DMA request enable"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, Dma>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::Disabled)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::Enabled)
    }
}
#[doc = "DMA disable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddm {
    #[doc = "0: DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DMA mode enabled"]
    Enabled = 1,
}
impl From<Ddm> for bool {
    #[inline(always)]
    fn from(variant: Ddm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDM` reader - DMA disable mode"]
pub type DdmR = crate::BitReader<Ddm>;
impl DdmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddm {
        match self.bits {
            false => Ddm::Disabled,
            true => Ddm::Enabled,
        }
    }
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ddm::Disabled
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ddm::Enabled
    }
}
#[doc = "Field `DDM` writer - DMA disable mode"]
pub type DdmW<'a, REG> = crate::BitWriter<'a, REG, Ddm>;
impl<'a, REG> DdmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ddm::Disabled)
    }
    #[doc = "DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ddm::Enabled)
    }
}
#[doc = "End of conversion mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eocm {
    #[doc = "0: EOC bit is set at the end of a sequence of conversions"]
    EndOfSequence = 0,
    #[doc = "1: EOC bit is set at the end of each conversion"]
    EndOfEach = 1,
}
impl From<Eocm> for bool {
    #[inline(always)]
    fn from(variant: Eocm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCM` reader - End of conversion mode"]
pub type EocmR = crate::BitReader<Eocm>;
impl EocmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eocm {
        match self.bits {
            false => Eocm::EndOfSequence,
            true => Eocm::EndOfEach,
        }
    }
    #[doc = "EOC bit is set at the end of a sequence of conversions"]
    #[inline(always)]
    pub fn is_end_of_sequence(&self) -> bool {
        *self == Eocm::EndOfSequence
    }
    #[doc = "EOC bit is set at the end of each conversion"]
    #[inline(always)]
    pub fn is_end_of_each(&self) -> bool {
        *self == Eocm::EndOfEach
    }
}
#[doc = "Field `EOCM` writer - End of conversion mode"]
pub type EocmW<'a, REG> = crate::BitWriter<'a, REG, Eocm>;
impl<'a, REG> EocmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOC bit is set at the end of a sequence of conversions"]
    #[inline(always)]
    pub fn end_of_sequence(self) -> &'a mut crate::W<REG> {
        self.variant(Eocm::EndOfSequence)
    }
    #[doc = "EOC bit is set at the end of each conversion"]
    #[inline(always)]
    pub fn end_of_each(self) -> &'a mut crate::W<REG> {
        self.variant(Eocm::EndOfEach)
    }
}
#[doc = "Data alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dal {
    #[doc = "0: Right alignment"]
    Right = 0,
    #[doc = "1: Left alignment"]
    Left = 1,
}
impl From<Dal> for bool {
    #[inline(always)]
    fn from(variant: Dal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAL` reader - Data alignment"]
pub type DalR = crate::BitReader<Dal>;
impl DalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dal {
        match self.bits {
            false => Dal::Right,
            true => Dal::Left,
        }
    }
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == Dal::Right
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == Dal::Left
    }
}
#[doc = "Field `DAL` writer - Data alignment"]
pub type DalW<'a, REG> = crate::BitWriter<'a, REG, Dal>;
impl<'a, REG> DalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(Dal::Right)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(Dal::Left)
    }
}
#[doc = "Field `ETSIC` reader - External trigger select for inserted channel"]
pub type EtsicR = crate::FieldReader;
#[doc = "Field `ETSIC` writer - External trigger select for inserted channel"]
pub type EtsicW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETMIC` reader - External trigger mode for inserted channel"]
pub type EtmicR = crate::FieldReader;
#[doc = "Field `ETMIC` writer - External trigger mode for inserted channel"]
pub type EtmicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWICST` reader - Software start on inserted channel"]
pub type SwicstR = crate::BitReader;
#[doc = "Field `SWICST` writer - Software start on inserted channel"]
pub type SwicstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "External trigger select for regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etsrc {
    #[doc = "0: Timer 0 channel 0 event"]
    Timer0ch0 = 0,
    #[doc = "1: Timer 0 channel 1 event"]
    Timer0ch1 = 1,
    #[doc = "2: Timer 0 channel 2 event"]
    Timer0ch2 = 2,
    #[doc = "3: Timer 1 channel 1 event"]
    Timer1ch1 = 3,
    #[doc = "4: Timer 1 channel 2 event"]
    Timer1ch2 = 4,
    #[doc = "5: Timer 1 channel 3 event"]
    Timer1ch3 = 5,
    #[doc = "6: Timer 1 TRGO event"]
    Timer1trgo = 6,
    #[doc = "7: Timer 2 channel 0 event"]
    Timer2ch0 = 7,
    #[doc = "8: Timer 2 TRGO event"]
    Timer2trgo = 8,
    #[doc = "9: Timer 3 channel 3 event"]
    Timer3ch3 = 9,
    #[doc = "10: Timer 4 channel 0 event"]
    Timer4ch0 = 10,
    #[doc = "11: Timer 4 channel 1 event"]
    Timer4ch1 = 11,
    #[doc = "12: Timer 4 channel 2 event"]
    Timer4ch2 = 12,
    #[doc = "13: Timer 7 channel 0 event"]
    Timer7ch0 = 13,
    #[doc = "14: Timer 7 TRGO event"]
    Timer7trgo = 14,
    #[doc = "15: EXTI line 11 event"]
    Exti11 = 15,
}
impl From<Etsrc> for u8 {
    #[inline(always)]
    fn from(variant: Etsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etsrc {
    type Ux = u8;
}
impl crate::IsEnum for Etsrc {}
#[doc = "Field `ETSRC` reader - External trigger select for regular channel"]
pub type EtsrcR = crate::FieldReader<Etsrc>;
impl EtsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etsrc {
        match self.bits {
            0 => Etsrc::Timer0ch0,
            1 => Etsrc::Timer0ch1,
            2 => Etsrc::Timer0ch2,
            3 => Etsrc::Timer1ch1,
            4 => Etsrc::Timer1ch2,
            5 => Etsrc::Timer1ch3,
            6 => Etsrc::Timer1trgo,
            7 => Etsrc::Timer2ch0,
            8 => Etsrc::Timer2trgo,
            9 => Etsrc::Timer3ch3,
            10 => Etsrc::Timer4ch0,
            11 => Etsrc::Timer4ch1,
            12 => Etsrc::Timer4ch2,
            13 => Etsrc::Timer7ch0,
            14 => Etsrc::Timer7trgo,
            15 => Etsrc::Exti11,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer 0 channel 0 event"]
    #[inline(always)]
    pub fn is_timer0ch0(&self) -> bool {
        *self == Etsrc::Timer0ch0
    }
    #[doc = "Timer 0 channel 1 event"]
    #[inline(always)]
    pub fn is_timer0ch1(&self) -> bool {
        *self == Etsrc::Timer0ch1
    }
    #[doc = "Timer 0 channel 2 event"]
    #[inline(always)]
    pub fn is_timer0ch2(&self) -> bool {
        *self == Etsrc::Timer0ch2
    }
    #[doc = "Timer 1 channel 1 event"]
    #[inline(always)]
    pub fn is_timer1ch1(&self) -> bool {
        *self == Etsrc::Timer1ch1
    }
    #[doc = "Timer 1 channel 2 event"]
    #[inline(always)]
    pub fn is_timer1ch2(&self) -> bool {
        *self == Etsrc::Timer1ch2
    }
    #[doc = "Timer 1 channel 3 event"]
    #[inline(always)]
    pub fn is_timer1ch3(&self) -> bool {
        *self == Etsrc::Timer1ch3
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn is_timer1trgo(&self) -> bool {
        *self == Etsrc::Timer1trgo
    }
    #[doc = "Timer 2 channel 0 event"]
    #[inline(always)]
    pub fn is_timer2ch0(&self) -> bool {
        *self == Etsrc::Timer2ch0
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn is_timer2trgo(&self) -> bool {
        *self == Etsrc::Timer2trgo
    }
    #[doc = "Timer 3 channel 3 event"]
    #[inline(always)]
    pub fn is_timer3ch3(&self) -> bool {
        *self == Etsrc::Timer3ch3
    }
    #[doc = "Timer 4 channel 0 event"]
    #[inline(always)]
    pub fn is_timer4ch0(&self) -> bool {
        *self == Etsrc::Timer4ch0
    }
    #[doc = "Timer 4 channel 1 event"]
    #[inline(always)]
    pub fn is_timer4ch1(&self) -> bool {
        *self == Etsrc::Timer4ch1
    }
    #[doc = "Timer 4 channel 2 event"]
    #[inline(always)]
    pub fn is_timer4ch2(&self) -> bool {
        *self == Etsrc::Timer4ch2
    }
    #[doc = "Timer 7 channel 0 event"]
    #[inline(always)]
    pub fn is_timer7ch0(&self) -> bool {
        *self == Etsrc::Timer7ch0
    }
    #[doc = "Timer 7 TRGO event"]
    #[inline(always)]
    pub fn is_timer7trgo(&self) -> bool {
        *self == Etsrc::Timer7trgo
    }
    #[doc = "EXTI line 11 event"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == Etsrc::Exti11
    }
}
#[doc = "Field `ETSRC` writer - External trigger select for regular channel"]
pub type EtsrcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Etsrc, crate::Safe>;
impl<'a, REG> EtsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 0 channel 0 event"]
    #[inline(always)]
    pub fn timer0ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer0ch0)
    }
    #[doc = "Timer 0 channel 1 event"]
    #[inline(always)]
    pub fn timer0ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer0ch1)
    }
    #[doc = "Timer 0 channel 2 event"]
    #[inline(always)]
    pub fn timer0ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer0ch2)
    }
    #[doc = "Timer 1 channel 1 event"]
    #[inline(always)]
    pub fn timer1ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer1ch1)
    }
    #[doc = "Timer 1 channel 2 event"]
    #[inline(always)]
    pub fn timer1ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer1ch2)
    }
    #[doc = "Timer 1 channel 3 event"]
    #[inline(always)]
    pub fn timer1ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer1ch3)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn timer1trgo(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer1trgo)
    }
    #[doc = "Timer 2 channel 0 event"]
    #[inline(always)]
    pub fn timer2ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer2ch0)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn timer2trgo(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer2trgo)
    }
    #[doc = "Timer 3 channel 3 event"]
    #[inline(always)]
    pub fn timer3ch3(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer3ch3)
    }
    #[doc = "Timer 4 channel 0 event"]
    #[inline(always)]
    pub fn timer4ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer4ch0)
    }
    #[doc = "Timer 4 channel 1 event"]
    #[inline(always)]
    pub fn timer4ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer4ch1)
    }
    #[doc = "Timer 4 channel 2 event"]
    #[inline(always)]
    pub fn timer4ch2(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer4ch2)
    }
    #[doc = "Timer 7 channel 0 event"]
    #[inline(always)]
    pub fn timer7ch0(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer7ch0)
    }
    #[doc = "Timer 7 TRGO event"]
    #[inline(always)]
    pub fn timer7trgo(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Timer7trgo)
    }
    #[doc = "EXTI line 11 event"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut crate::W<REG> {
        self.variant(Etsrc::Exti11)
    }
}
#[doc = "External trigger mode for regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etmrc {
    #[doc = "0: Disable external trigger conversion of regular channels"]
    Disable = 0,
    #[doc = "1: Enable external trigger conversion of regular channels on rising edge"]
    RisingEdge = 1,
    #[doc = "2: Enable external trigger conversion of regular channels on falling edge"]
    FallingEdge = 2,
    #[doc = "3: Enable external trigger conversion of regular channels on both edges"]
    BothEdges = 3,
}
impl From<Etmrc> for u8 {
    #[inline(always)]
    fn from(variant: Etmrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etmrc {
    type Ux = u8;
}
impl crate::IsEnum for Etmrc {}
#[doc = "Field `ETMRC` reader - External trigger mode for regular channel"]
pub type EtmrcR = crate::FieldReader<Etmrc>;
impl EtmrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etmrc {
        match self.bits {
            0 => Etmrc::Disable,
            1 => Etmrc::RisingEdge,
            2 => Etmrc::FallingEdge,
            3 => Etmrc::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable external trigger conversion of regular channels"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Etmrc::Disable
    }
    #[doc = "Enable external trigger conversion of regular channels on rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Etmrc::RisingEdge
    }
    #[doc = "Enable external trigger conversion of regular channels on falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Etmrc::FallingEdge
    }
    #[doc = "Enable external trigger conversion of regular channels on both edges"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == Etmrc::BothEdges
    }
}
#[doc = "Field `ETMRC` writer - External trigger mode for regular channel"]
pub type EtmrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Etmrc, crate::Safe>;
impl<'a, REG> EtmrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable external trigger conversion of regular channels"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Etmrc::Disable)
    }
    #[doc = "Enable external trigger conversion of regular channels on rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Etmrc::RisingEdge)
    }
    #[doc = "Enable external trigger conversion of regular channels on falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Etmrc::FallingEdge)
    }
    #[doc = "Enable external trigger conversion of regular channels on both edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(Etmrc::BothEdges)
    }
}
#[doc = "Software start on regular channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrcst {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Start the conversion of a regular channel"]
    Reset = 1,
}
impl From<Swrcst> for bool {
    #[inline(always)]
    fn from(variant: Swrcst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRCST` reader - Software start on regular channel"]
pub type SwrcstR = crate::BitReader<Swrcst>;
impl SwrcstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrcst {
        match self.bits {
            false => Swrcst::NoEffect,
            true => Swrcst::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Swrcst::NoEffect
    }
    #[doc = "Start the conversion of a regular channel"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Swrcst::Reset
    }
}
#[doc = "Field `SWRCST` writer - Software start on regular channel"]
pub type SwrcstW<'a, REG> = crate::BitWriter<'a, REG, Swrcst>;
impl<'a, REG> SwrcstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Swrcst::NoEffect)
    }
    #[doc = "Start the conversion of a regular channel"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Swrcst::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - ADC on"]
    #[inline(always)]
    pub fn adcon(&self) -> AdconR {
        AdconR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    pub fn ctn(&self) -> CtnR {
        CtnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    pub fn clb(&self) -> ClbR {
        ClbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstclb(&self) -> RstclbR {
        RstclbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA disable mode"]
    #[inline(always)]
    pub fn ddm(&self) -> DdmR {
        DdmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - End of conversion mode"]
    #[inline(always)]
    pub fn eocm(&self) -> EocmR {
        EocmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dal(&self) -> DalR {
        DalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - External trigger select for inserted channel"]
    #[inline(always)]
    pub fn etsic(&self) -> EtsicR {
        EtsicR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - External trigger mode for inserted channel"]
    #[inline(always)]
    pub fn etmic(&self) -> EtmicR {
        EtmicR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Software start on inserted channel"]
    #[inline(always)]
    pub fn swicst(&self) -> SwicstR {
        SwicstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - External trigger select for regular channel"]
    #[inline(always)]
    pub fn etsrc(&self) -> EtsrcR {
        EtsrcR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - External trigger mode for regular channel"]
    #[inline(always)]
    pub fn etmrc(&self) -> EtmrcR {
        EtmrcR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Software start on regular channel"]
    #[inline(always)]
    pub fn swrcst(&self) -> SwrcstR {
        SwrcstR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC on"]
    #[inline(always)]
    pub fn adcon(&mut self) -> AdconW<'_, Ctl1Spec> {
        AdconW::new(self, 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    pub fn ctn(&mut self) -> CtnW<'_, Ctl1Spec> {
        CtnW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    pub fn clb(&mut self) -> ClbW<'_, Ctl1Spec> {
        ClbW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstclb(&mut self) -> RstclbW<'_, Ctl1Spec> {
        RstclbW::new(self, 3)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, Ctl1Spec> {
        DmaW::new(self, 8)
    }
    #[doc = "Bit 9 - DMA disable mode"]
    #[inline(always)]
    pub fn ddm(&mut self) -> DdmW<'_, Ctl1Spec> {
        DdmW::new(self, 9)
    }
    #[doc = "Bit 10 - End of conversion mode"]
    #[inline(always)]
    pub fn eocm(&mut self) -> EocmW<'_, Ctl1Spec> {
        EocmW::new(self, 10)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dal(&mut self) -> DalW<'_, Ctl1Spec> {
        DalW::new(self, 11)
    }
    #[doc = "Bits 16:19 - External trigger select for inserted channel"]
    #[inline(always)]
    pub fn etsic(&mut self) -> EtsicW<'_, Ctl1Spec> {
        EtsicW::new(self, 16)
    }
    #[doc = "Bits 20:21 - External trigger mode for inserted channel"]
    #[inline(always)]
    pub fn etmic(&mut self) -> EtmicW<'_, Ctl1Spec> {
        EtmicW::new(self, 20)
    }
    #[doc = "Bit 22 - Software start on inserted channel"]
    #[inline(always)]
    pub fn swicst(&mut self) -> SwicstW<'_, Ctl1Spec> {
        SwicstW::new(self, 22)
    }
    #[doc = "Bits 24:27 - External trigger select for regular channel"]
    #[inline(always)]
    pub fn etsrc(&mut self) -> EtsrcW<'_, Ctl1Spec> {
        EtsrcW::new(self, 24)
    }
    #[doc = "Bits 28:29 - External trigger mode for regular channel"]
    #[inline(always)]
    pub fn etmrc(&mut self) -> EtmrcW<'_, Ctl1Spec> {
        EtmrcW::new(self, 28)
    }
    #[doc = "Bit 30 - Software start on regular channel"]
    #[inline(always)]
    pub fn swrcst(&mut self) -> SwrcstW<'_, Ctl1Spec> {
        SwrcstW::new(self, 30)
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
