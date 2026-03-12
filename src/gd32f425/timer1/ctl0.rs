#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cen {
    #[doc = "0: Counter disabled"]
    Disabled = 0,
    #[doc = "1: Counter enabled"]
    Enabled = 1,
}
impl From<Cen> for bool {
    #[inline(always)]
    fn from(variant: Cen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Counter enable"]
pub type CenR = crate::BitReader<Cen>;
impl CenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cen {
        match self.bits {
            false => Cen::Disabled,
            true => Cen::Enabled,
        }
    }
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cen::Disabled
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cen::Enabled
    }
}
#[doc = "Field `CEN` writer - Counter enable"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG, Cen>;
impl<'a, REG> CenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cen::Disabled)
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cen::Enabled)
    }
}
#[doc = "Update disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Updis {
    #[doc = "0: Update event enabled"]
    Enabled = 0,
    #[doc = "1: Update event disabled"]
    Disabled = 1,
}
impl From<Updis> for bool {
    #[inline(always)]
    fn from(variant: Updis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDIS` reader - Update disable"]
pub type UpdisR = crate::BitReader<Updis>;
impl UpdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Updis {
        match self.bits {
            false => Updis::Enabled,
            true => Updis::Disabled,
        }
    }
    #[doc = "Update event enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Updis::Enabled
    }
    #[doc = "Update event disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Updis::Disabled
    }
}
#[doc = "Field `UPDIS` writer - Update disable"]
pub type UpdisW<'a, REG> = crate::BitWriter<'a, REG, Updis>;
impl<'a, REG> UpdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Updis::Enabled)
    }
    #[doc = "Update event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Updis::Disabled)
    }
}
#[doc = "Update source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ups {
    #[doc = "0: Any of counter overflow/underflow, setting UPG, or update through slave mode, generates an update interrupt or DMA request"]
    AnyEvent = 0,
    #[doc = "1: Only counter overflow/underflow generates an update interrupt or DMA request"]
    CounterOnly = 1,
}
impl From<Ups> for bool {
    #[inline(always)]
    fn from(variant: Ups) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPS` reader - Update source"]
pub type UpsR = crate::BitReader<Ups>;
impl UpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ups {
        match self.bits {
            false => Ups::AnyEvent,
            true => Ups::CounterOnly,
        }
    }
    #[doc = "Any of counter overflow/underflow, setting UPG, or update through slave mode, generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn is_any_event(&self) -> bool {
        *self == Ups::AnyEvent
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn is_counter_only(&self) -> bool {
        *self == Ups::CounterOnly
    }
}
#[doc = "Field `UPS` writer - Update source"]
pub type UpsW<'a, REG> = crate::BitWriter<'a, REG, Ups>;
impl<'a, REG> UpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any of counter overflow/underflow, setting UPG, or update through slave mode, generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn any_event(self) -> &'a mut crate::W<REG> {
        self.variant(Ups::AnyEvent)
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn counter_only(self) -> &'a mut crate::W<REG> {
        self.variant(Ups::CounterOnly)
    }
}
#[doc = "Single pulse mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spm {
    #[doc = "0: Single pulse mode disabled."]
    Disabled = 0,
    #[doc = "1: Single pulse mode enabled."]
    Enabled = 1,
}
impl From<Spm> for bool {
    #[inline(always)]
    fn from(variant: Spm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPM` reader - Single pulse mode"]
pub type SpmR = crate::BitReader<Spm>;
impl SpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spm {
        match self.bits {
            false => Spm::Disabled,
            true => Spm::Enabled,
        }
    }
    #[doc = "Single pulse mode disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Spm::Disabled
    }
    #[doc = "Single pulse mode enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Spm::Enabled
    }
}
#[doc = "Field `SPM` writer - Single pulse mode"]
pub type SpmW<'a, REG> = crate::BitWriter<'a, REG, Spm>;
impl<'a, REG> SpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single pulse mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Spm::Disabled)
    }
    #[doc = "Single pulse mode enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Spm::Enabled)
    }
}
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Counter counts up"]
    Up = 0,
    #[doc = "1: Counter counts down"]
    Down = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Direction"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::Up,
            true => Dir::Down,
        }
    }
    #[doc = "Counter counts up"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Dir::Up
    }
    #[doc = "Counter counts down"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Dir::Down
    }
}
#[doc = "Field `DIR` writer - Direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter counts up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Up)
    }
    #[doc = "Counter counts down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Down)
    }
}
#[doc = "Counter aligns mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cam {
    #[doc = "0: No Center aligned mode. Direction specified by DIR bit."]
    NoCenterAlign = 0,
    #[doc = "1: Center-aligned and counting down assert mode. The counter counts under center-aligned and channel is configured in output mode (CHxMS=00 in TIMERx_CHCTL0 register). Only when counting down, CHxF bit can be set."]
    CenterAlignCountDownAssert = 1,
    #[doc = "2: Center-aligned and counting up assert mode. The counter counts under center- aligned and channel is configured in output mode (CHxMS=00 in TIMERx_CHCTL0 register). Only when counting up, CHxF bit can be set."]
    CenterAlignCountUpAssert = 2,
    #[doc = "3: Center-aligned and counting up/down assert mode. The counter counts under center-aligned and channel is configured in output mode (CHxMS=00 in TIMERx_CHCTL0 register). Both when counting up and counting down, CHxF bit can be set. After the counter is enabled, cannot be switched from 0x00 to non 0x00."]
    CenterAlignCountUpDownAssert = 3,
}
impl From<Cam> for u8 {
    #[inline(always)]
    fn from(variant: Cam) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cam {
    type Ux = u8;
}
impl crate::IsEnum for Cam {}
#[doc = "Field `CAM` reader - Counter aligns mode selection"]
pub type CamR = crate::FieldReader<Cam>;
impl CamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cam {
        match self.bits {
            0 => Cam::NoCenterAlign,
            1 => Cam::CenterAlignCountDownAssert,
            2 => Cam::CenterAlignCountUpAssert,
            3 => Cam::CenterAlignCountUpDownAssert,
            _ => unreachable!(),
        }
    }
    #[doc = "No Center aligned mode. Direction specified by DIR bit."]
    #[inline(always)]
    pub fn is_no_center_align(&self) -> bool {
        *self == Cam::NoCenterAlign
    }
    #[doc = "Center-aligned and counting down assert mode. The counter counts under center-aligned and channel is configured in output mode (CHxMS=00 in TIMERx_CHCTL0 register). Only when counting down, CHxF bit can be set."]
    #[inline(always)]
    pub fn is_center_align_count_down_assert(&self) -> bool {
        *self == Cam::CenterAlignCountDownAssert
    }
    #[doc = "Center-aligned and counting up assert mode. The counter counts under center- aligned and channel is configured in output mode (CHxMS=00 in TIMERx_CHCTL0 register). Only when counting up, CHxF bit can be set."]
    #[inline(always)]
    pub fn is_center_align_count_up_assert(&self) -> bool {
        *self == Cam::CenterAlignCountUpAssert
    }
    #[doc = "Center-aligned and counting up/down assert mode. The counter counts under center-aligned and channel is configured in output mode (CHxMS=00 in TIMERx_CHCTL0 register). Both when counting up and counting down, CHxF bit can be set. After the counter is enabled, cannot be switched from 0x00 to non 0x00."]
    #[inline(always)]
    pub fn is_center_align_count_up_down_assert(&self) -> bool {
        *self == Cam::CenterAlignCountUpDownAssert
    }
}
#[doc = "Field `CAM` writer - Counter aligns mode selection"]
pub type CamW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cam, crate::Safe>;
impl<'a, REG> CamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Center aligned mode. Direction specified by DIR bit."]
    #[inline(always)]
    pub fn no_center_align(self) -> &'a mut crate::W<REG> {
        self.variant(Cam::NoCenterAlign)
    }
    #[doc = "Center-aligned and counting down assert mode. The counter counts under center-aligned and channel is configured in output mode (CHxMS=00 in TIMERx_CHCTL0 register). Only when counting down, CHxF bit can be set."]
    #[inline(always)]
    pub fn center_align_count_down_assert(self) -> &'a mut crate::W<REG> {
        self.variant(Cam::CenterAlignCountDownAssert)
    }
    #[doc = "Center-aligned and counting up assert mode. The counter counts under center- aligned and channel is configured in output mode (CHxMS=00 in TIMERx_CHCTL0 register). Only when counting up, CHxF bit can be set."]
    #[inline(always)]
    pub fn center_align_count_up_assert(self) -> &'a mut crate::W<REG> {
        self.variant(Cam::CenterAlignCountUpAssert)
    }
    #[doc = "Center-aligned and counting up/down assert mode. The counter counts under center-aligned and channel is configured in output mode (CHxMS=00 in TIMERx_CHCTL0 register). Both when counting up and counting down, CHxF bit can be set. After the counter is enabled, cannot be switched from 0x00 to non 0x00."]
    #[inline(always)]
    pub fn center_align_count_up_down_assert(self) -> &'a mut crate::W<REG> {
        self.variant(Cam::CenterAlignCountUpDownAssert)
    }
}
#[doc = "Auto-reload shadow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arse {
    #[doc = "0: The shadow register for CAR is disabled"]
    Disabled = 0,
    #[doc = "1: The shadow register for CAR is enabled"]
    Enabled = 1,
}
impl From<Arse> for bool {
    #[inline(always)]
    fn from(variant: Arse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARSE` reader - Auto-reload shadow enable"]
pub type ArseR = crate::BitReader<Arse>;
impl ArseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arse {
        match self.bits {
            false => Arse::Disabled,
            true => Arse::Enabled,
        }
    }
    #[doc = "The shadow register for CAR is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Arse::Disabled
    }
    #[doc = "The shadow register for CAR is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Arse::Enabled
    }
}
#[doc = "Field `ARSE` writer - Auto-reload shadow enable"]
pub type ArseW<'a, REG> = crate::BitWriter<'a, REG, Arse>;
impl<'a, REG> ArseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The shadow register for CAR is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Arse::Disabled)
    }
    #[doc = "The shadow register for CAR is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Arse::Enabled)
    }
}
#[doc = "Clock division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckdiv {
    #[doc = "0: Clock division factor 1"]
    Ckdiv0 = 0,
    #[doc = "1: Clock division factor 2"]
    Ckdiv1 = 1,
    #[doc = "2: Clock division factor 4"]
    Ckdiv2 = 2,
}
impl From<Ckdiv> for u8 {
    #[inline(always)]
    fn from(variant: Ckdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckdiv {
    type Ux = u8;
}
impl crate::IsEnum for Ckdiv {}
#[doc = "Field `CKDIV` reader - Clock division"]
pub type CkdivR = crate::FieldReader<Ckdiv>;
impl CkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckdiv> {
        match self.bits {
            0 => Some(Ckdiv::Ckdiv0),
            1 => Some(Ckdiv::Ckdiv1),
            2 => Some(Ckdiv::Ckdiv2),
            _ => None,
        }
    }
    #[doc = "Clock division factor 1"]
    #[inline(always)]
    pub fn is_ckdiv0(&self) -> bool {
        *self == Ckdiv::Ckdiv0
    }
    #[doc = "Clock division factor 2"]
    #[inline(always)]
    pub fn is_ckdiv1(&self) -> bool {
        *self == Ckdiv::Ckdiv1
    }
    #[doc = "Clock division factor 4"]
    #[inline(always)]
    pub fn is_ckdiv2(&self) -> bool {
        *self == Ckdiv::Ckdiv2
    }
}
#[doc = "Field `CKDIV` writer - Clock division"]
pub type CkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckdiv>;
impl<'a, REG> CkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock division factor 1"]
    #[inline(always)]
    pub fn ckdiv0(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdiv::Ckdiv0)
    }
    #[doc = "Clock division factor 2"]
    #[inline(always)]
    pub fn ckdiv1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdiv::Ckdiv1)
    }
    #[doc = "Clock division factor 4"]
    #[inline(always)]
    pub fn ckdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Ckdiv::Ckdiv2)
    }
}
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&self) -> UpdisR {
        UpdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update source"]
    #[inline(always)]
    pub fn ups(&self) -> UpsR {
        UpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    pub fn spm(&self) -> SpmR {
        SpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Counter aligns mode selection"]
    #[inline(always)]
    pub fn cam(&self) -> CamR {
        CamR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    pub fn arse(&self) -> ArseR {
        ArseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CkdivR {
        CkdivR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<'_, Ctl0Spec> {
        CenW::new(self, 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn updis(&mut self) -> UpdisW<'_, Ctl0Spec> {
        UpdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Update source"]
    #[inline(always)]
    pub fn ups(&mut self) -> UpsW<'_, Ctl0Spec> {
        UpsW::new(self, 2)
    }
    #[doc = "Bit 3 - Single pulse mode"]
    #[inline(always)]
    pub fn spm(&mut self) -> SpmW<'_, Ctl0Spec> {
        SpmW::new(self, 3)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, Ctl0Spec> {
        DirW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Counter aligns mode selection"]
    #[inline(always)]
    pub fn cam(&mut self) -> CamW<'_, Ctl0Spec> {
        CamW::new(self, 5)
    }
    #[doc = "Bit 7 - Auto-reload shadow enable"]
    #[inline(always)]
    pub fn arse(&mut self) -> ArseW<'_, Ctl0Spec> {
        ArseW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CkdivW<'_, Ctl0Spec> {
        CkdivW::new(self, 8)
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
