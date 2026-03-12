#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `WDCHSEL` reader - Analog watchdog channel select"]
pub type WdchselR = crate::FieldReader;
#[doc = "Field `WDCHSEL` writer - Analog watchdog channel select"]
pub type WdchselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Interrupt enable for EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eocie {
    #[doc = "0: EOC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: EOC interrupt enabled"]
    Enabled = 1,
}
impl From<Eocie> for bool {
    #[inline(always)]
    fn from(variant: Eocie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOCIE` reader - Interrupt enable for EOC"]
pub type EocieR = crate::BitReader<Eocie>;
impl EocieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eocie {
        match self.bits {
            false => Eocie::Disabled,
            true => Eocie::Enabled,
        }
    }
    #[doc = "EOC interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Eocie::Disabled
    }
    #[doc = "EOC interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Eocie::Enabled
    }
}
#[doc = "Field `EOCIE` writer - Interrupt enable for EOC"]
pub type EocieW<'a, REG> = crate::BitWriter<'a, REG, Eocie>;
impl<'a, REG> EocieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eocie::Disabled)
    }
    #[doc = "EOC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eocie::Enabled)
    }
}
#[doc = "Analog watchdog WDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdeie {
    #[doc = "0: WDE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: WDE interrupt enabled"]
    Enabled = 1,
}
impl From<Wdeie> for bool {
    #[inline(always)]
    fn from(variant: Wdeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDEIE` reader - Analog watchdog WDE"]
pub type WdeieR = crate::BitReader<Wdeie>;
impl WdeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdeie {
        match self.bits {
            false => Wdeie::Disabled,
            true => Wdeie::Enabled,
        }
    }
    #[doc = "WDE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wdeie::Disabled
    }
    #[doc = "WDE interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wdeie::Enabled
    }
}
#[doc = "Field `WDEIE` writer - Analog watchdog WDE"]
pub type WdeieW<'a, REG> = crate::BitWriter<'a, REG, Wdeie>;
impl<'a, REG> WdeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdeie::Disabled)
    }
    #[doc = "WDE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdeie::Enabled)
    }
}
#[doc = "Field `EOICIE` reader - Interrupt enable for EOIC"]
pub type EoicieR = crate::BitReader;
#[doc = "Field `EOICIE` writer - Interrupt enable for EOIC"]
pub type EoicieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Scan mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sm {
    #[doc = "0: Scan mode disabled"]
    Disabled = 0,
    #[doc = "1: Scan mode enabled"]
    Enabled = 1,
}
impl From<Sm> for bool {
    #[inline(always)]
    fn from(variant: Sm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SM` reader - Scan mode"]
pub type SmR = crate::BitReader<Sm>;
impl SmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sm {
        match self.bits {
            false => Sm::Disabled,
            true => Sm::Enabled,
        }
    }
    #[doc = "Scan mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sm::Disabled
    }
    #[doc = "Scan mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sm::Enabled
    }
}
#[doc = "Field `SM` writer - Scan mode"]
pub type SmW<'a, REG> = crate::BitWriter<'a, REG, Sm>;
impl<'a, REG> SmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scan mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::Disabled)
    }
    #[doc = "Scan mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::Enabled)
    }
}
#[doc = "When in scan mode, analog watchdog is effective on a single channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdsc {
    #[doc = "0: Analog watchdog enabled on all channels"]
    All = 0,
    #[doc = "1: Analog watchdog enabled on a single channel"]
    Single = 1,
}
impl From<Wdsc> for bool {
    #[inline(always)]
    fn from(variant: Wdsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDSC` reader - When in scan mode, analog watchdog is effective on a single channel"]
pub type WdscR = crate::BitReader<Wdsc>;
impl WdscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdsc {
        match self.bits {
            false => Wdsc::All,
            true => Wdsc::Single,
        }
    }
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Wdsc::All
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Wdsc::Single
    }
}
#[doc = "Field `WDSC` writer - When in scan mode, analog watchdog is effective on a single channel"]
pub type WdscW<'a, REG> = crate::BitWriter<'a, REG, Wdsc>;
impl<'a, REG> WdscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog enabled on all channels"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Wdsc::All)
    }
    #[doc = "Analog watchdog enabled on a single channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Wdsc::Single)
    }
}
#[doc = "Field `ICA` reader - Inserted channel group convert automatically"]
pub type IcaR = crate::BitReader;
#[doc = "Field `ICA` writer - Inserted channel group convert automatically"]
pub type IcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Discontinuous mode on regular channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disrc {
    #[doc = "0: Discontinuous mode on regular channels disabled"]
    Disabled = 0,
    #[doc = "1: Discontinuous mode on regular channels enabled"]
    Enabled = 1,
}
impl From<Disrc> for bool {
    #[inline(always)]
    fn from(variant: Disrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISRC` reader - Discontinuous mode on regular channels"]
pub type DisrcR = crate::BitReader<Disrc>;
impl DisrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disrc {
        match self.bits {
            false => Disrc::Disabled,
            true => Disrc::Enabled,
        }
    }
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Disrc::Disabled
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Disrc::Enabled
    }
}
#[doc = "Field `DISRC` writer - Discontinuous mode on regular channels"]
pub type DisrcW<'a, REG> = crate::BitWriter<'a, REG, Disrc>;
impl<'a, REG> DisrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Disrc::Disabled)
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Disrc::Enabled)
    }
}
#[doc = "Field `DISIC` reader - Discontinuous mode on inserted channels"]
pub type DisicR = crate::BitReader;
#[doc = "Field `DISIC` writer - Discontinuous mode on inserted channels"]
pub type DisicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISNUM` reader - Number of conversions in discontinuous mode"]
pub type DisnumR = crate::FieldReader;
#[doc = "Field `DISNUM` writer - Number of conversions in discontinuous mode"]
pub type DisnumW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `IWDEN` reader - Inserted channel analog watchdog enable"]
pub type IwdenR = crate::BitReader;
#[doc = "Field `IWDEN` writer - Inserted channel analog watchdog enable"]
pub type IwdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Regular channel analog watchdog enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwden {
    #[doc = "0: Analog watchdog disabled"]
    Disabled = 0,
    #[doc = "1: Analog watchdog enabled"]
    Enabled = 1,
}
impl From<Rwden> for bool {
    #[inline(always)]
    fn from(variant: Rwden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWDEN` reader - Regular channel analog watchdog enable"]
pub type RwdenR = crate::BitReader<Rwden>;
impl RwdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwden {
        match self.bits {
            false => Rwden::Disabled,
            true => Rwden::Enabled,
        }
    }
    #[doc = "Analog watchdog disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rwden::Disabled
    }
    #[doc = "Analog watchdog enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rwden::Enabled
    }
}
#[doc = "Field `RWDEN` writer - Regular channel analog watchdog enable"]
pub type RwdenW<'a, REG> = crate::BitWriter<'a, REG, Rwden>;
impl<'a, REG> RwdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog watchdog disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rwden::Disabled)
    }
    #[doc = "Analog watchdog enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rwden::Enabled)
    }
}
#[doc = "ADC data resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dres {
    #[doc = "0: 12-bit resolution"]
    Bits12 = 0,
    #[doc = "1: 10-bit resolution"]
    Bits10 = 1,
    #[doc = "2: 8-bit resolution"]
    Bits8 = 2,
    #[doc = "3: 6-bit resolution"]
    Bits6 = 3,
}
impl From<Dres> for u8 {
    #[inline(always)]
    fn from(variant: Dres) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dres {
    type Ux = u8;
}
impl crate::IsEnum for Dres {}
#[doc = "Field `DRES` reader - ADC data resolution"]
pub type DresR = crate::FieldReader<Dres>;
impl DresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dres {
        match self.bits {
            0 => Dres::Bits12,
            1 => Dres::Bits10,
            2 => Dres::Bits8,
            3 => Dres::Bits6,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        *self == Dres::Bits12
    }
    #[doc = "10-bit resolution"]
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == Dres::Bits10
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == Dres::Bits8
    }
    #[doc = "6-bit resolution"]
    #[inline(always)]
    pub fn is_bits6(&self) -> bool {
        *self == Dres::Bits6
    }
}
#[doc = "Field `DRES` writer - ADC data resolution"]
pub type DresW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dres, crate::Safe>;
impl<'a, REG> DresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn bits12(self) -> &'a mut crate::W<REG> {
        self.variant(Dres::Bits12)
    }
    #[doc = "10-bit resolution"]
    #[inline(always)]
    pub fn bits10(self) -> &'a mut crate::W<REG> {
        self.variant(Dres::Bits10)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(Dres::Bits8)
    }
    #[doc = "6-bit resolution"]
    #[inline(always)]
    pub fn bits6(self) -> &'a mut crate::W<REG> {
        self.variant(Dres::Bits6)
    }
}
#[doc = "Interrupt enable for ROVF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rovfie {
    #[doc = "0: ROVF interrupt disabled"]
    Disabled = 0,
    #[doc = "1: ROVF interrupt enabled."]
    Enabled = 1,
}
impl From<Rovfie> for bool {
    #[inline(always)]
    fn from(variant: Rovfie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROVFIE` reader - Interrupt enable for ROVF"]
pub type RovfieR = crate::BitReader<Rovfie>;
impl RovfieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rovfie {
        match self.bits {
            false => Rovfie::Disabled,
            true => Rovfie::Enabled,
        }
    }
    #[doc = "ROVF interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rovfie::Disabled
    }
    #[doc = "ROVF interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rovfie::Enabled
    }
}
#[doc = "Field `ROVFIE` writer - Interrupt enable for ROVF"]
pub type RovfieW<'a, REG> = crate::BitWriter<'a, REG, Rovfie>;
impl<'a, REG> RovfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ROVF interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rovfie::Disabled)
    }
    #[doc = "ROVF interrupt enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rovfie::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog watchdog channel select"]
    #[inline(always)]
    pub fn wdchsel(&self) -> WdchselR {
        WdchselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&self) -> EocieR {
        EocieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog watchdog WDE"]
    #[inline(always)]
    pub fn wdeie(&self) -> WdeieR {
        WdeieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    pub fn eoicie(&self) -> EoicieR {
        EoicieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn sm(&self) -> SmR {
        SmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog is effective on a single channel"]
    #[inline(always)]
    pub fn wdsc(&self) -> WdscR {
        WdscR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Inserted channel group convert automatically"]
    #[inline(always)]
    pub fn ica(&self) -> IcaR {
        IcaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn disrc(&self) -> DisrcR {
        DisrcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Discontinuous mode on inserted channels"]
    #[inline(always)]
    pub fn disic(&self) -> DisicR {
        DisicR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    pub fn disnum(&self) -> DisnumR {
        DisnumR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog enable"]
    #[inline(always)]
    pub fn iwden(&self) -> IwdenR {
        IwdenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Regular channel analog watchdog enable"]
    #[inline(always)]
    pub fn rwden(&self) -> RwdenR {
        RwdenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - ADC data resolution"]
    #[inline(always)]
    pub fn dres(&self) -> DresR {
        DresR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Interrupt enable for ROVF"]
    #[inline(always)]
    pub fn rovfie(&self) -> RovfieR {
        RovfieR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog watchdog channel select"]
    #[inline(always)]
    pub fn wdchsel(&mut self) -> WdchselW<'_, Ctl0Spec> {
        WdchselW::new(self, 0)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EocieW<'_, Ctl0Spec> {
        EocieW::new(self, 5)
    }
    #[doc = "Bit 6 - Analog watchdog WDE"]
    #[inline(always)]
    pub fn wdeie(&mut self) -> WdeieW<'_, Ctl0Spec> {
        WdeieW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    pub fn eoicie(&mut self) -> EoicieW<'_, Ctl0Spec> {
        EoicieW::new(self, 7)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn sm(&mut self) -> SmW<'_, Ctl0Spec> {
        SmW::new(self, 8)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog is effective on a single channel"]
    #[inline(always)]
    pub fn wdsc(&mut self) -> WdscW<'_, Ctl0Spec> {
        WdscW::new(self, 9)
    }
    #[doc = "Bit 10 - Inserted channel group convert automatically"]
    #[inline(always)]
    pub fn ica(&mut self) -> IcaW<'_, Ctl0Spec> {
        IcaW::new(self, 10)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn disrc(&mut self) -> DisrcW<'_, Ctl0Spec> {
        DisrcW::new(self, 11)
    }
    #[doc = "Bit 12 - Discontinuous mode on inserted channels"]
    #[inline(always)]
    pub fn disic(&mut self) -> DisicW<'_, Ctl0Spec> {
        DisicW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    pub fn disnum(&mut self) -> DisnumW<'_, Ctl0Spec> {
        DisnumW::new(self, 13)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog enable"]
    #[inline(always)]
    pub fn iwden(&mut self) -> IwdenW<'_, Ctl0Spec> {
        IwdenW::new(self, 22)
    }
    #[doc = "Bit 23 - Regular channel analog watchdog enable"]
    #[inline(always)]
    pub fn rwden(&mut self) -> RwdenW<'_, Ctl0Spec> {
        RwdenW::new(self, 23)
    }
    #[doc = "Bits 24:25 - ADC data resolution"]
    #[inline(always)]
    pub fn dres(&mut self) -> DresW<'_, Ctl0Spec> {
        DresW::new(self, 24)
    }
    #[doc = "Bit 26 - Interrupt enable for ROVF"]
    #[inline(always)]
    pub fn rovfie(&mut self) -> RovfieW<'_, Ctl0Spec> {
        RovfieW::new(self, 26)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {}
