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
    #[doc = "7: Trigger input is ETIFP."]
    Etifp = 7,
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
    pub const fn variant(&self) -> Trgs {
        match self.bits {
            0 => Trgs::Iti0,
            1 => Trgs::Iti1,
            2 => Trgs::Iti2,
            3 => Trgs::Iti3,
            4 => Trgs::Ci0fEdge,
            5 => Trgs::Ci0fe0,
            6 => Trgs::Ci1fe1,
            7 => Trgs::Etifp,
            _ => unreachable!(),
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
    #[doc = "Trigger input is ETIFP."]
    #[inline(always)]
    pub fn is_etifp(&self) -> bool {
        *self == Trgs::Etifp
    }
}
#[doc = "Field `TRGS` writer - Trigger selection"]
pub type TrgsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trgs, crate::Safe>;
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
    #[doc = "Trigger input is ETIFP."]
    #[inline(always)]
    pub fn etifp(self) -> &'a mut crate::W<REG> {
        self.variant(Trgs::Etifp)
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
#[doc = "External trigger filter control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etfc {
    #[doc = "0: ETIFP filter disabled."]
    Disabled = 0,
    #[doc = "1: ETIFP filter capacity is 2, f_samp is fck_timer"]
    Capacity1 = 1,
    #[doc = "2: ETIFP filter capacity is 4, f_samp is fck_timer"]
    Capacity2 = 2,
    #[doc = "3: ETIFP filter capacity is 8, f_samp is fck_timer"]
    Capacity3 = 3,
    #[doc = "4: ETIFP filter capacity is 6, f_samp is fck_timer / 2"]
    Capacity4 = 4,
    #[doc = "5: ETIFP filter capacity is 8, f_samp is fck_timer / 2"]
    Capacity5 = 5,
    #[doc = "6: ETIFP filter capacity is 6, f_samp is fck_timer / 4"]
    Capacity6 = 6,
    #[doc = "7: ETIFP filter capacity is 8, f_samp is fck_timer / 4"]
    Capacity7 = 7,
    #[doc = "8: ETIFP filter capacity is 6, f_samp is fck_timer / 8"]
    Capacity8 = 8,
    #[doc = "9: ETIFP filter capacity is 8, f_samp is fck_timer / 8"]
    Capacity9 = 9,
    #[doc = "10: ETIFP filter capacity is 5, f_samp is fck_timer / 16"]
    Capacity10 = 10,
    #[doc = "11: ETIFP filter capacity is 6, f_samp is fck_timer / 16"]
    Capacity11 = 11,
    #[doc = "12: ETIFP filter capacity is 8, f_samp is fck_timer / 16"]
    Capacity12 = 12,
    #[doc = "13: ETIFP filter capacity is 5, f_samp is fck_timer / 32"]
    Capacity13 = 13,
    #[doc = "14: ETIFP filter capacity is 6, f_samp is fck_timer / 32"]
    Capacity14 = 14,
    #[doc = "15: ETIFP filter capacity is 8, f_samp is fck_timer / 32"]
    Capacity15 = 15,
}
impl From<Etfc> for u8 {
    #[inline(always)]
    fn from(variant: Etfc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etfc {
    type Ux = u8;
}
impl crate::IsEnum for Etfc {}
#[doc = "Field `ETFC` reader - External trigger filter control"]
pub type EtfcR = crate::FieldReader<Etfc>;
impl EtfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etfc {
        match self.bits {
            0 => Etfc::Disabled,
            1 => Etfc::Capacity1,
            2 => Etfc::Capacity2,
            3 => Etfc::Capacity3,
            4 => Etfc::Capacity4,
            5 => Etfc::Capacity5,
            6 => Etfc::Capacity6,
            7 => Etfc::Capacity7,
            8 => Etfc::Capacity8,
            9 => Etfc::Capacity9,
            10 => Etfc::Capacity10,
            11 => Etfc::Capacity11,
            12 => Etfc::Capacity12,
            13 => Etfc::Capacity13,
            14 => Etfc::Capacity14,
            15 => Etfc::Capacity15,
            _ => unreachable!(),
        }
    }
    #[doc = "ETIFP filter disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Etfc::Disabled
    }
    #[doc = "ETIFP filter capacity is 2, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity1(&self) -> bool {
        *self == Etfc::Capacity1
    }
    #[doc = "ETIFP filter capacity is 4, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity2(&self) -> bool {
        *self == Etfc::Capacity2
    }
    #[doc = "ETIFP filter capacity is 8, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity3(&self) -> bool {
        *self == Etfc::Capacity3
    }
    #[doc = "ETIFP filter capacity is 6, f_samp is fck_timer / 2"]
    #[inline(always)]
    pub fn is_capacity4(&self) -> bool {
        *self == Etfc::Capacity4
    }
    #[doc = "ETIFP filter capacity is 8, f_samp is fck_timer / 2"]
    #[inline(always)]
    pub fn is_capacity5(&self) -> bool {
        *self == Etfc::Capacity5
    }
    #[doc = "ETIFP filter capacity is 6, f_samp is fck_timer / 4"]
    #[inline(always)]
    pub fn is_capacity6(&self) -> bool {
        *self == Etfc::Capacity6
    }
    #[doc = "ETIFP filter capacity is 8, f_samp is fck_timer / 4"]
    #[inline(always)]
    pub fn is_capacity7(&self) -> bool {
        *self == Etfc::Capacity7
    }
    #[doc = "ETIFP filter capacity is 6, f_samp is fck_timer / 8"]
    #[inline(always)]
    pub fn is_capacity8(&self) -> bool {
        *self == Etfc::Capacity8
    }
    #[doc = "ETIFP filter capacity is 8, f_samp is fck_timer / 8"]
    #[inline(always)]
    pub fn is_capacity9(&self) -> bool {
        *self == Etfc::Capacity9
    }
    #[doc = "ETIFP filter capacity is 5, f_samp is fck_timer / 16"]
    #[inline(always)]
    pub fn is_capacity10(&self) -> bool {
        *self == Etfc::Capacity10
    }
    #[doc = "ETIFP filter capacity is 6, f_samp is fck_timer / 16"]
    #[inline(always)]
    pub fn is_capacity11(&self) -> bool {
        *self == Etfc::Capacity11
    }
    #[doc = "ETIFP filter capacity is 8, f_samp is fck_timer / 16"]
    #[inline(always)]
    pub fn is_capacity12(&self) -> bool {
        *self == Etfc::Capacity12
    }
    #[doc = "ETIFP filter capacity is 5, f_samp is fck_timer / 32"]
    #[inline(always)]
    pub fn is_capacity13(&self) -> bool {
        *self == Etfc::Capacity13
    }
    #[doc = "ETIFP filter capacity is 6, f_samp is fck_timer / 32"]
    #[inline(always)]
    pub fn is_capacity14(&self) -> bool {
        *self == Etfc::Capacity14
    }
    #[doc = "ETIFP filter capacity is 8, f_samp is fck_timer / 32"]
    #[inline(always)]
    pub fn is_capacity15(&self) -> bool {
        *self == Etfc::Capacity15
    }
}
#[doc = "Field `ETFC` writer - External trigger filter control"]
pub type EtfcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Etfc, crate::Safe>;
impl<'a, REG> EtfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ETIFP filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Disabled)
    }
    #[doc = "ETIFP filter capacity is 2, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity1(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity1)
    }
    #[doc = "ETIFP filter capacity is 4, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity2(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity2)
    }
    #[doc = "ETIFP filter capacity is 8, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity3(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity3)
    }
    #[doc = "ETIFP filter capacity is 6, f_samp is fck_timer / 2"]
    #[inline(always)]
    pub fn capacity4(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity4)
    }
    #[doc = "ETIFP filter capacity is 8, f_samp is fck_timer / 2"]
    #[inline(always)]
    pub fn capacity5(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity5)
    }
    #[doc = "ETIFP filter capacity is 6, f_samp is fck_timer / 4"]
    #[inline(always)]
    pub fn capacity6(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity6)
    }
    #[doc = "ETIFP filter capacity is 8, f_samp is fck_timer / 4"]
    #[inline(always)]
    pub fn capacity7(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity7)
    }
    #[doc = "ETIFP filter capacity is 6, f_samp is fck_timer / 8"]
    #[inline(always)]
    pub fn capacity8(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity8)
    }
    #[doc = "ETIFP filter capacity is 8, f_samp is fck_timer / 8"]
    #[inline(always)]
    pub fn capacity9(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity9)
    }
    #[doc = "ETIFP filter capacity is 5, f_samp is fck_timer / 16"]
    #[inline(always)]
    pub fn capacity10(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity10)
    }
    #[doc = "ETIFP filter capacity is 6, f_samp is fck_timer / 16"]
    #[inline(always)]
    pub fn capacity11(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity11)
    }
    #[doc = "ETIFP filter capacity is 8, f_samp is fck_timer / 16"]
    #[inline(always)]
    pub fn capacity12(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity12)
    }
    #[doc = "ETIFP filter capacity is 5, f_samp is fck_timer / 32"]
    #[inline(always)]
    pub fn capacity13(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity13)
    }
    #[doc = "ETIFP filter capacity is 6, f_samp is fck_timer / 32"]
    #[inline(always)]
    pub fn capacity14(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity14)
    }
    #[doc = "ETIFP filter capacity is 8, f_samp is fck_timer / 32"]
    #[inline(always)]
    pub fn capacity15(self) -> &'a mut crate::W<REG> {
        self.variant(Etfc::Capacity15)
    }
}
#[doc = "External trigger prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Etpsc {
    #[doc = "0: Prescale is disabled."]
    Psc0 = 0,
    #[doc = "1: Prescale is divided by 2."]
    Psc1 = 1,
    #[doc = "2: Prescale is divided by 4."]
    Psc2 = 2,
    #[doc = "3: Prescale is divided by 8."]
    Psc3 = 3,
}
impl From<Etpsc> for u8 {
    #[inline(always)]
    fn from(variant: Etpsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Etpsc {
    type Ux = u8;
}
impl crate::IsEnum for Etpsc {}
#[doc = "Field `ETPSC` reader - External trigger prescaler"]
pub type EtpscR = crate::FieldReader<Etpsc>;
impl EtpscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etpsc {
        match self.bits {
            0 => Etpsc::Psc0,
            1 => Etpsc::Psc1,
            2 => Etpsc::Psc2,
            3 => Etpsc::Psc3,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescale is disabled."]
    #[inline(always)]
    pub fn is_psc0(&self) -> bool {
        *self == Etpsc::Psc0
    }
    #[doc = "Prescale is divided by 2."]
    #[inline(always)]
    pub fn is_psc1(&self) -> bool {
        *self == Etpsc::Psc1
    }
    #[doc = "Prescale is divided by 4."]
    #[inline(always)]
    pub fn is_psc2(&self) -> bool {
        *self == Etpsc::Psc2
    }
    #[doc = "Prescale is divided by 8."]
    #[inline(always)]
    pub fn is_psc3(&self) -> bool {
        *self == Etpsc::Psc3
    }
}
#[doc = "Field `ETPSC` writer - External trigger prescaler"]
pub type EtpscW<'a, REG> = crate::FieldWriter<'a, REG, 2, Etpsc, crate::Safe>;
impl<'a, REG> EtpscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescale is disabled."]
    #[inline(always)]
    pub fn psc0(self) -> &'a mut crate::W<REG> {
        self.variant(Etpsc::Psc0)
    }
    #[doc = "Prescale is divided by 2."]
    #[inline(always)]
    pub fn psc1(self) -> &'a mut crate::W<REG> {
        self.variant(Etpsc::Psc1)
    }
    #[doc = "Prescale is divided by 4."]
    #[inline(always)]
    pub fn psc2(self) -> &'a mut crate::W<REG> {
        self.variant(Etpsc::Psc2)
    }
    #[doc = "Prescale is divided by 8."]
    #[inline(always)]
    pub fn psc3(self) -> &'a mut crate::W<REG> {
        self.variant(Etpsc::Psc3)
    }
}
#[doc = "Part of SMC for enable External clock mode1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smc1 {
    #[doc = "0: External clock mode 1 disabled."]
    Disabled = 0,
    #[doc = "1: External clock mode 1 enabled. Counter is clocked by any active edge of the ETIFP signal."]
    Enabled = 1,
}
impl From<Smc1> for bool {
    #[inline(always)]
    fn from(variant: Smc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMC1` reader - Part of SMC for enable External clock mode1"]
pub type Smc1R = crate::BitReader<Smc1>;
impl Smc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smc1 {
        match self.bits {
            false => Smc1::Disabled,
            true => Smc1::Enabled,
        }
    }
    #[doc = "External clock mode 1 disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Smc1::Disabled
    }
    #[doc = "External clock mode 1 enabled. Counter is clocked by any active edge of the ETIFP signal."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Smc1::Enabled
    }
}
#[doc = "Field `SMC1` writer - Part of SMC for enable External clock mode1"]
pub type Smc1W<'a, REG> = crate::BitWriter<'a, REG, Smc1>;
impl<'a, REG> Smc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External clock mode 1 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Smc1::Disabled)
    }
    #[doc = "External clock mode 1 enabled. Counter is clocked by any active edge of the ETIFP signal."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Smc1::Enabled)
    }
}
#[doc = "External trigger polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Etp {
    #[doc = "0: ETI is active at rising edge or high level."]
    RisingEdge = 0,
    #[doc = "1: ETI is active at falling edge or low level."]
    FallingEdge = 1,
}
impl From<Etp> for bool {
    #[inline(always)]
    fn from(variant: Etp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETP` reader - External trigger polarity"]
pub type EtpR = crate::BitReader<Etp>;
impl EtpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Etp {
        match self.bits {
            false => Etp::RisingEdge,
            true => Etp::FallingEdge,
        }
    }
    #[doc = "ETI is active at rising edge or high level."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Etp::RisingEdge
    }
    #[doc = "ETI is active at falling edge or low level."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Etp::FallingEdge
    }
}
#[doc = "Field `ETP` writer - External trigger polarity"]
pub type EtpW<'a, REG> = crate::BitWriter<'a, REG, Etp>;
impl<'a, REG> EtpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ETI is active at rising edge or high level."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Etp::RisingEdge)
    }
    #[doc = "ETI is active at falling edge or low level."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Etp::FallingEdge)
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
    #[doc = "Bits 8:11 - External trigger filter control"]
    #[inline(always)]
    pub fn etfc(&self) -> EtfcR {
        EtfcR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etpsc(&self) -> EtpscR {
        EtpscR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Part of SMC for enable External clock mode1"]
    #[inline(always)]
    pub fn smc1(&self) -> Smc1R {
        Smc1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> EtpR {
        EtpR::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bits 8:11 - External trigger filter control"]
    #[inline(always)]
    pub fn etfc(&mut self) -> EtfcW<'_, SmcfgSpec> {
        EtfcW::new(self, 8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etpsc(&mut self) -> EtpscW<'_, SmcfgSpec> {
        EtpscW::new(self, 12)
    }
    #[doc = "Bit 14 - Part of SMC for enable External clock mode1"]
    #[inline(always)]
    pub fn smc1(&mut self) -> Smc1W<'_, SmcfgSpec> {
        Smc1W::new(self, 14)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&mut self) -> EtpW<'_, SmcfgSpec> {
        EtpW::new(self, 15)
    }
}
#[doc = "slave mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
