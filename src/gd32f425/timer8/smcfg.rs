#[doc = "Register `SMCFG` reader"]
pub type R = crate::R<SmcfgSpec>;
#[doc = "Register `SMCFG` writer"]
pub type W = crate::W<SmcfgSpec>;
#[doc = "Slave mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smc {
    #[doc = "0: Slave mode disabled."]
    Disabled = 0,
    #[doc = "1: Quadrature decoder mode 0.The counter counts on CI0FE0 edge, while the direction depends on CI1FE1 level."]
    EncoderMode1 = 1,
    #[doc = "2: Quadrature decoder mode 1.The counter counts on CI1FE1 edge, while the direction depends on CI0FE0 level."]
    EncoderMode2 = 2,
    #[doc = "3: Quadrature decoder mode 2.The counter counts on both CI0FE0 and CI1FE1 edge, while the direction depends on each other."]
    EncoderMode3 = 3,
    #[doc = "4: Restart mode. The counter is reinitialized and an update event is generated on the rising edge of the selected trigger input."]
    RestartMode = 4,
    #[doc = "5: Pause mode. The trigger input enables the counter clock when it is high and disables the counter clock when it is low."]
    PauseMode = 5,
    #[doc = "6: Event mode. A rising edge of the trigger input enables the counter."]
    TriggerMode = 6,
    #[doc = "7: External clock mode 0. The counter counts on the rising edges of the selected trigger."]
    ExternalClockMode1 = 7,
}
impl From<Smc> for u8 {
    #[inline(always)]
    fn from(variant: Smc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smc {
    type Ux = u8;
}
impl crate::IsEnum for Smc {}
#[doc = "Field `SMC` reader - Slave mode control"]
pub type SmcR = crate::FieldReader<Smc>;
impl SmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smc {
        match self.bits {
            0 => Smc::Disabled,
            1 => Smc::EncoderMode1,
            2 => Smc::EncoderMode2,
            3 => Smc::EncoderMode3,
            4 => Smc::RestartMode,
            5 => Smc::PauseMode,
            6 => Smc::TriggerMode,
            7 => Smc::ExternalClockMode1,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave mode disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Smc::Disabled
    }
    #[doc = "Quadrature decoder mode 0.The counter counts on CI0FE0 edge, while the direction depends on CI1FE1 level."]
    #[inline(always)]
    pub fn is_encoder_mode1(&self) -> bool {
        *self == Smc::EncoderMode1
    }
    #[doc = "Quadrature decoder mode 1.The counter counts on CI1FE1 edge, while the direction depends on CI0FE0 level."]
    #[inline(always)]
    pub fn is_encoder_mode2(&self) -> bool {
        *self == Smc::EncoderMode2
    }
    #[doc = "Quadrature decoder mode 2.The counter counts on both CI0FE0 and CI1FE1 edge, while the direction depends on each other."]
    #[inline(always)]
    pub fn is_encoder_mode3(&self) -> bool {
        *self == Smc::EncoderMode3
    }
    #[doc = "Restart mode. The counter is reinitialized and an update event is generated on the rising edge of the selected trigger input."]
    #[inline(always)]
    pub fn is_restart_mode(&self) -> bool {
        *self == Smc::RestartMode
    }
    #[doc = "Pause mode. The trigger input enables the counter clock when it is high and disables the counter clock when it is low."]
    #[inline(always)]
    pub fn is_pause_mode(&self) -> bool {
        *self == Smc::PauseMode
    }
    #[doc = "Event mode. A rising edge of the trigger input enables the counter."]
    #[inline(always)]
    pub fn is_trigger_mode(&self) -> bool {
        *self == Smc::TriggerMode
    }
    #[doc = "External clock mode 0. The counter counts on the rising edges of the selected trigger."]
    #[inline(always)]
    pub fn is_external_clock_mode1(&self) -> bool {
        *self == Smc::ExternalClockMode1
    }
}
#[doc = "Field `SMC` writer - Slave mode control"]
pub type SmcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Smc, crate::Safe>;
impl<'a, REG> SmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::Disabled)
    }
    #[doc = "Quadrature decoder mode 0.The counter counts on CI0FE0 edge, while the direction depends on CI1FE1 level."]
    #[inline(always)]
    pub fn encoder_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::EncoderMode1)
    }
    #[doc = "Quadrature decoder mode 1.The counter counts on CI1FE1 edge, while the direction depends on CI0FE0 level."]
    #[inline(always)]
    pub fn encoder_mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::EncoderMode2)
    }
    #[doc = "Quadrature decoder mode 2.The counter counts on both CI0FE0 and CI1FE1 edge, while the direction depends on each other."]
    #[inline(always)]
    pub fn encoder_mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::EncoderMode3)
    }
    #[doc = "Restart mode. The counter is reinitialized and an update event is generated on the rising edge of the selected trigger input."]
    #[inline(always)]
    pub fn restart_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::RestartMode)
    }
    #[doc = "Pause mode. The trigger input enables the counter clock when it is high and disables the counter clock when it is low."]
    #[inline(always)]
    pub fn pause_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::PauseMode)
    }
    #[doc = "Event mode. A rising edge of the trigger input enables the counter."]
    #[inline(always)]
    pub fn trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::TriggerMode)
    }
    #[doc = "External clock mode 0. The counter counts on the rising edges of the selected trigger."]
    #[inline(always)]
    pub fn external_clock_mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Smc::ExternalClockMode1)
    }
}
#[doc = "Trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgs {
    #[doc = "0: Trigger input is ITI0."]
    Iti0 = 0,
    #[doc = "1: Trigger input is ITI1."]
    Iti1 = 1,
    #[doc = "2: Trigger input is ITI2."]
    Iti2 = 2,
    #[doc = "3: Trigger input is ITI3."]
    Iti3 = 3,
    #[doc = "4: Trigger input is CI0F_EDGE."]
    Ci0fEdge = 4,
    #[doc = "5: Trigger input is CI0FE0."]
    Ci0fe0 = 5,
    #[doc = "6: Trigger input is CI1FE1."]
    Ci1fe1 = 6,
}
impl From<Trgs> for u8 {
    #[inline(always)]
    fn from(variant: Trgs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgs {
    type Ux = u8;
}
impl crate::IsEnum for Trgs {}
#[doc = "Field `TRGS` reader - Trigger selection"]
pub type TrgsR = crate::FieldReader<Trgs>;
impl TrgsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trgs> {
        match self.bits {
            0 => Some(Trgs::Iti0),
            1 => Some(Trgs::Iti1),
            2 => Some(Trgs::Iti2),
            3 => Some(Trgs::Iti3),
            4 => Some(Trgs::Ci0fEdge),
            5 => Some(Trgs::Ci0fe0),
            6 => Some(Trgs::Ci1fe1),
            _ => None,
        }
    }
    #[doc = "Trigger input is ITI0."]
    #[inline(always)]
    pub fn is_iti0(&self) -> bool {
        *self == Trgs::Iti0
    }
    #[doc = "Trigger input is ITI1."]
    #[inline(always)]
    pub fn is_iti1(&self) -> bool {
        *self == Trgs::Iti1
    }
    #[doc = "Trigger input is ITI2."]
    #[inline(always)]
    pub fn is_iti2(&self) -> bool {
        *self == Trgs::Iti2
    }
    #[doc = "Trigger input is ITI3."]
    #[inline(always)]
    pub fn is_iti3(&self) -> bool {
        *self == Trgs::Iti3
    }
    #[doc = "Trigger input is CI0F_EDGE."]
    #[inline(always)]
    pub fn is_ci0f_edge(&self) -> bool {
        *self == Trgs::Ci0fEdge
    }
    #[doc = "Trigger input is CI0FE0."]
    #[inline(always)]
    pub fn is_ci0fe0(&self) -> bool {
        *self == Trgs::Ci0fe0
    }
    #[doc = "Trigger input is CI1FE1."]
    #[inline(always)]
    pub fn is_ci1fe1(&self) -> bool {
        *self == Trgs::Ci1fe1
    }
}
#[doc = "Field `TRGS` writer - Trigger selection"]
pub type TrgsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trgs>;
impl<'a, REG> TrgsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger input is ITI0."]
    #[inline(always)]
    pub fn iti0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Iti0)
    }
    #[doc = "Trigger input is ITI1."]
    #[inline(always)]
    pub fn iti1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Iti1)
    }
    #[doc = "Trigger input is ITI2."]
    #[inline(always)]
    pub fn iti2(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Iti2)
    }
    #[doc = "Trigger input is ITI3."]
    #[inline(always)]
    pub fn iti3(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Iti3)
    }
    #[doc = "Trigger input is CI0F_EDGE."]
    #[inline(always)]
    pub fn ci0f_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Ci0fEdge)
    }
    #[doc = "Trigger input is CI0FE0."]
    #[inline(always)]
    pub fn ci0fe0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Ci0fe0)
    }
    #[doc = "Trigger input is CI1FE1."]
    #[inline(always)]
    pub fn ci1fe1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Ci1fe1)
    }
}
#[doc = "Master-slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msm {
    #[doc = "0: Master/slave mode disabled."]
    Disabled = 0,
    #[doc = "1: Master mode enabled."]
    Master = 1,
}
impl From<Msm> for bool {
    #[inline(always)]
    fn from(variant: Msm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSM` reader - Master-slave mode"]
pub type MsmR = crate::BitReader<Msm>;
impl MsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msm {
        match self.bits {
            false => Msm::Disabled,
            true => Msm::Master,
        }
    }
    #[doc = "Master/slave mode disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msm::Disabled
    }
    #[doc = "Master mode enabled."]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Msm::Master
    }
}
#[doc = "Field `MSM` writer - Master-slave mode"]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG, Msm>;
impl<'a, REG> MsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master/slave mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msm::Disabled)
    }
    #[doc = "Master mode enabled."]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Msm::Master)
    }
}
impl R {
    #[doc = "Bits 0:2 - Slave mode control"]
    #[inline(always)]
    pub fn smc(&self) -> SmcR {
        SmcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&self) -> TrgsR {
        TrgsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master-slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode control"]
    #[inline(always)]
    pub fn smc(&mut self) -> SmcW<'_, SmcfgSpec> {
        SmcW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&mut self) -> TrgsW<'_, SmcfgSpec> {
        TrgsW::new(self, 4)
    }
    #[doc = "Bit 7 - Master-slave mode"]
    #[inline(always)]
    pub fn msm(&mut self) -> MsmW<'_, SmcfgSpec> {
        MsmW::new(self, 7)
    }
}
#[doc = "slave mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcfgSpec;
impl crate::RegisterSpec for SmcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcfg::R`](R) reader structure"]
impl crate::Readable for SmcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`smcfg::W`](W) writer structure"]
impl crate::Writable for SmcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMCFG to value 0"]
impl crate::Resettable for SmcfgSpec {}
