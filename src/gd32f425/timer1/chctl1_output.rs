#[doc = "Register `CHCTL1_Output` reader"]
pub type R = crate::R<Chctl1OutputSpec>;
#[doc = "Register `CHCTL1_Output` writer"]
pub type W = crate::W<Chctl1OutputSpec>;
#[doc = "Channel 2 I/O mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch2ms {
    #[doc = "0: Channel 2 output mode."]
    Output = 0,
    #[doc = "1: Channel 2 is programmed as input mode, IS2 is connected to CI2FE2"]
    InputCi1 = 1,
    #[doc = "2: Channel 2 is programmed as input mode, IS2 is connected to CI3FE2"]
    InputCi0 = 2,
    #[doc = "3: Channel 2 is programmed as input mode, IS2 is connected to ITS"]
    InputIts = 3,
}
impl From<Ch2ms> for u8 {
    #[inline(always)]
    fn from(variant: Ch2ms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch2ms {
    type Ux = u8;
}
impl crate::IsEnum for Ch2ms {}
#[doc = "Field `CH2MS` reader - Channel 2 I/O mode selection"]
pub type Ch2msR = crate::FieldReader<Ch2ms>;
impl Ch2msR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2ms {
        match self.bits {
            0 => Ch2ms::Output,
            1 => Ch2ms::InputCi1,
            2 => Ch2ms::InputCi0,
            3 => Ch2ms::InputIts,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 2 output mode."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Ch2ms::Output
    }
    #[doc = "Channel 2 is programmed as input mode, IS2 is connected to CI2FE2"]
    #[inline(always)]
    pub fn is_input_ci1(&self) -> bool {
        *self == Ch2ms::InputCi1
    }
    #[doc = "Channel 2 is programmed as input mode, IS2 is connected to CI3FE2"]
    #[inline(always)]
    pub fn is_input_ci0(&self) -> bool {
        *self == Ch2ms::InputCi0
    }
    #[doc = "Channel 2 is programmed as input mode, IS2 is connected to ITS"]
    #[inline(always)]
    pub fn is_input_its(&self) -> bool {
        *self == Ch2ms::InputIts
    }
}
#[doc = "Field `CH2MS` writer - Channel 2 I/O mode selection"]
pub type Ch2msW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch2ms, crate::Safe>;
impl<'a, REG> Ch2msW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 2 output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2ms::Output)
    }
    #[doc = "Channel 2 is programmed as input mode, IS2 is connected to CI2FE2"]
    #[inline(always)]
    pub fn input_ci1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2ms::InputCi1)
    }
    #[doc = "Channel 2 is programmed as input mode, IS2 is connected to CI3FE2"]
    #[inline(always)]
    pub fn input_ci0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2ms::InputCi0)
    }
    #[doc = "Channel 2 is programmed as input mode, IS2 is connected to ITS"]
    #[inline(always)]
    pub fn input_its(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2ms::InputIts)
    }
}
#[doc = "Channel 2 output compare fast enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2comfen {
    #[doc = "0: Channel 2 output compare fast disabled."]
    Disabled = 0,
    #[doc = "1: Channel 2 compare fast enabled."]
    Enabled = 1,
}
impl From<Ch2comfen> for bool {
    #[inline(always)]
    fn from(variant: Ch2comfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2COMFEN` reader - Channel 2 output compare fast enable"]
pub type Ch2comfenR = crate::BitReader<Ch2comfen>;
impl Ch2comfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2comfen {
        match self.bits {
            false => Ch2comfen::Disabled,
            true => Ch2comfen::Enabled,
        }
    }
    #[doc = "Channel 2 output compare fast disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2comfen::Disabled
    }
    #[doc = "Channel 2 compare fast enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2comfen::Enabled
    }
}
#[doc = "Field `CH2COMFEN` writer - Channel 2 output compare fast enable"]
pub type Ch2comfenW<'a, REG> = crate::BitWriter<'a, REG, Ch2comfen>;
impl<'a, REG> Ch2comfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 output compare fast disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comfen::Disabled)
    }
    #[doc = "Channel 2 compare fast enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comfen::Enabled)
    }
}
#[doc = "Channel 2 compare output shadow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2comsen {
    #[doc = "0: Channel 2 output compare shadow disabled."]
    Disabled = 0,
    #[doc = "1: Channel 2 compare shadow enabled."]
    Enabled = 1,
}
impl From<Ch2comsen> for bool {
    #[inline(always)]
    fn from(variant: Ch2comsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2COMSEN` reader - Channel 2 compare output shadow enable"]
pub type Ch2comsenR = crate::BitReader<Ch2comsen>;
impl Ch2comsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2comsen {
        match self.bits {
            false => Ch2comsen::Disabled,
            true => Ch2comsen::Enabled,
        }
    }
    #[doc = "Channel 2 output compare shadow disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2comsen::Disabled
    }
    #[doc = "Channel 2 compare shadow enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2comsen::Enabled
    }
}
#[doc = "Field `CH2COMSEN` writer - Channel 2 compare output shadow enable"]
pub type Ch2comsenW<'a, REG> = crate::BitWriter<'a, REG, Ch2comsen>;
impl<'a, REG> Ch2comsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 output compare shadow disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comsen::Disabled)
    }
    #[doc = "Channel 2 compare shadow enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comsen::Enabled)
    }
}
#[doc = "Channel 2 compare output control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch2comctl {
    #[doc = "0: Timing mode. The O2CPRE signal keeps stable, independent of the comparison between the register TIMERx_CH2CV and the counter TIMERx_CNT."]
    Mode0 = 0,
    #[doc = "1: Set the channel output. O2CPRE signal is forced high when the counter is equals to the output compare register TIMERx_CH2CV."]
    Mode1 = 1,
    #[doc = "2: Clear the channel output. O2CPRE signal is forced low when the counter is equals to the output compare register TIMERx_CH2CV."]
    Mode2 = 2,
    #[doc = "3: Toggle on match. O2CPRE toggles when the counter is equals to the output compare register TIMERx_CH2CV."]
    Mode3 = 3,
    #[doc = "4: Force low. O2CPRE is forced to low level."]
    Mode4 = 4,
    #[doc = "5: Force high. O2CPRE is forced to high level."]
    Mode5 = 5,
    #[doc = "6: PWM mode0. When counting up, O2CPRE is high when the counter is smaller than TIMERx_CH2CV, and low otherwise. When counting down, O2CPRE is low when the counter is larger than TIMERx_CH2CV, and high otherwise."]
    Pwmmode0 = 6,
    #[doc = "7: PWM mode1. When counting up, O2CPRE is low when the counter is smaller than TIMERx_CH2CV, and high otherwise. When counting down, O2CPRE is high when the counter is larger than TIMERx_CH2CV, and low otherwise."]
    Pwmmode1 = 7,
}
impl From<Ch2comctl> for u8 {
    #[inline(always)]
    fn from(variant: Ch2comctl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch2comctl {
    type Ux = u8;
}
impl crate::IsEnum for Ch2comctl {}
#[doc = "Field `CH2COMCTL` reader - Channel 2 compare output control"]
pub type Ch2comctlR = crate::FieldReader<Ch2comctl>;
impl Ch2comctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2comctl {
        match self.bits {
            0 => Ch2comctl::Mode0,
            1 => Ch2comctl::Mode1,
            2 => Ch2comctl::Mode2,
            3 => Ch2comctl::Mode3,
            4 => Ch2comctl::Mode4,
            5 => Ch2comctl::Mode5,
            6 => Ch2comctl::Pwmmode0,
            7 => Ch2comctl::Pwmmode1,
            _ => unreachable!(),
        }
    }
    #[doc = "Timing mode. The O2CPRE signal keeps stable, independent of the comparison between the register TIMERx_CH2CV and the counter TIMERx_CNT."]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Ch2comctl::Mode0
    }
    #[doc = "Set the channel output. O2CPRE signal is forced high when the counter is equals to the output compare register TIMERx_CH2CV."]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Ch2comctl::Mode1
    }
    #[doc = "Clear the channel output. O2CPRE signal is forced low when the counter is equals to the output compare register TIMERx_CH2CV."]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Ch2comctl::Mode2
    }
    #[doc = "Toggle on match. O2CPRE toggles when the counter is equals to the output compare register TIMERx_CH2CV."]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Ch2comctl::Mode3
    }
    #[doc = "Force low. O2CPRE is forced to low level."]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == Ch2comctl::Mode4
    }
    #[doc = "Force high. O2CPRE is forced to high level."]
    #[inline(always)]
    pub fn is_mode5(&self) -> bool {
        *self == Ch2comctl::Mode5
    }
    #[doc = "PWM mode0. When counting up, O2CPRE is high when the counter is smaller than TIMERx_CH2CV, and low otherwise. When counting down, O2CPRE is low when the counter is larger than TIMERx_CH2CV, and high otherwise."]
    #[inline(always)]
    pub fn is_pwmmode0(&self) -> bool {
        *self == Ch2comctl::Pwmmode0
    }
    #[doc = "PWM mode1. When counting up, O2CPRE is low when the counter is smaller than TIMERx_CH2CV, and high otherwise. When counting down, O2CPRE is high when the counter is larger than TIMERx_CH2CV, and low otherwise."]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == Ch2comctl::Pwmmode1
    }
}
#[doc = "Field `CH2COMCTL` writer - Channel 2 compare output control"]
pub type Ch2comctlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ch2comctl, crate::Safe>;
impl<'a, REG> Ch2comctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timing mode. The O2CPRE signal keeps stable, independent of the comparison between the register TIMERx_CH2CV and the counter TIMERx_CNT."]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comctl::Mode0)
    }
    #[doc = "Set the channel output. O2CPRE signal is forced high when the counter is equals to the output compare register TIMERx_CH2CV."]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comctl::Mode1)
    }
    #[doc = "Clear the channel output. O2CPRE signal is forced low when the counter is equals to the output compare register TIMERx_CH2CV."]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comctl::Mode2)
    }
    #[doc = "Toggle on match. O2CPRE toggles when the counter is equals to the output compare register TIMERx_CH2CV."]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comctl::Mode3)
    }
    #[doc = "Force low. O2CPRE is forced to low level."]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comctl::Mode4)
    }
    #[doc = "Force high. O2CPRE is forced to high level."]
    #[inline(always)]
    pub fn mode5(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comctl::Mode5)
    }
    #[doc = "PWM mode0. When counting up, O2CPRE is high when the counter is smaller than TIMERx_CH2CV, and low otherwise. When counting down, O2CPRE is low when the counter is larger than TIMERx_CH2CV, and high otherwise."]
    #[inline(always)]
    pub fn pwmmode0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comctl::Pwmmode0)
    }
    #[doc = "PWM mode1. When counting up, O2CPRE is low when the counter is smaller than TIMERx_CH2CV, and high otherwise. When counting down, O2CPRE is high when the counter is larger than TIMERx_CH2CV, and low otherwise."]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comctl::Pwmmode1)
    }
}
#[doc = "Channel 2 output compare clear enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2comcen {
    #[doc = "0: Channel 2 output compare clear disabled."]
    Disabled = 0,
    #[doc = "1: Channel 2 compare clear enabled."]
    Enabled = 1,
}
impl From<Ch2comcen> for bool {
    #[inline(always)]
    fn from(variant: Ch2comcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2COMCEN` reader - Channel 2 output compare clear enable"]
pub type Ch2comcenR = crate::BitReader<Ch2comcen>;
impl Ch2comcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2comcen {
        match self.bits {
            false => Ch2comcen::Disabled,
            true => Ch2comcen::Enabled,
        }
    }
    #[doc = "Channel 2 output compare clear disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2comcen::Disabled
    }
    #[doc = "Channel 2 compare clear enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2comcen::Enabled
    }
}
#[doc = "Field `CH2COMCEN` writer - Channel 2 output compare clear enable"]
pub type Ch2comcenW<'a, REG> = crate::BitWriter<'a, REG, Ch2comcen>;
impl<'a, REG> Ch2comcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 2 output compare clear disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comcen::Disabled)
    }
    #[doc = "Channel 2 compare clear enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2comcen::Enabled)
    }
}
#[doc = "Channel 3 mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch3ms {
    #[doc = "0: Channel 3 output mode."]
    Output = 0,
    #[doc = "1: Channel 3 is programmed as input mode, IS3 is connected to CI2FE3"]
    InputCi1 = 1,
    #[doc = "2: Channel 3 is programmed as input mode, IS3 is connected to CI2FE3"]
    InputCi0 = 2,
    #[doc = "3: Channel 3 is programmed as input mode, IS3 is connected to ITS"]
    InputIts = 3,
}
impl From<Ch3ms> for u8 {
    #[inline(always)]
    fn from(variant: Ch3ms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch3ms {
    type Ux = u8;
}
impl crate::IsEnum for Ch3ms {}
#[doc = "Field `CH3MS` reader - Channel 3 mode selection"]
pub type Ch3msR = crate::FieldReader<Ch3ms>;
impl Ch3msR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3ms {
        match self.bits {
            0 => Ch3ms::Output,
            1 => Ch3ms::InputCi1,
            2 => Ch3ms::InputCi0,
            3 => Ch3ms::InputIts,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 3 output mode."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Ch3ms::Output
    }
    #[doc = "Channel 3 is programmed as input mode, IS3 is connected to CI2FE3"]
    #[inline(always)]
    pub fn is_input_ci1(&self) -> bool {
        *self == Ch3ms::InputCi1
    }
    #[doc = "Channel 3 is programmed as input mode, IS3 is connected to CI2FE3"]
    #[inline(always)]
    pub fn is_input_ci0(&self) -> bool {
        *self == Ch3ms::InputCi0
    }
    #[doc = "Channel 3 is programmed as input mode, IS3 is connected to ITS"]
    #[inline(always)]
    pub fn is_input_its(&self) -> bool {
        *self == Ch3ms::InputIts
    }
}
#[doc = "Field `CH3MS` writer - Channel 3 mode selection"]
pub type Ch3msW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch3ms, crate::Safe>;
impl<'a, REG> Ch3msW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 3 output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3ms::Output)
    }
    #[doc = "Channel 3 is programmed as input mode, IS3 is connected to CI2FE3"]
    #[inline(always)]
    pub fn input_ci1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3ms::InputCi1)
    }
    #[doc = "Channel 3 is programmed as input mode, IS3 is connected to CI2FE3"]
    #[inline(always)]
    pub fn input_ci0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3ms::InputCi0)
    }
    #[doc = "Channel 3 is programmed as input mode, IS3 is connected to ITS"]
    #[inline(always)]
    pub fn input_its(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3ms::InputIts)
    }
}
#[doc = "Channel 3 output compare fast enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3comfen {
    #[doc = "0: Channel 3 output compare fast disabled."]
    Disabled = 0,
    #[doc = "1: Channel 3 compare fast enabled."]
    Enabled = 1,
}
impl From<Ch3comfen> for bool {
    #[inline(always)]
    fn from(variant: Ch3comfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3COMFEN` reader - Channel 3 output compare fast enable"]
pub type Ch3comfenR = crate::BitReader<Ch3comfen>;
impl Ch3comfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3comfen {
        match self.bits {
            false => Ch3comfen::Disabled,
            true => Ch3comfen::Enabled,
        }
    }
    #[doc = "Channel 3 output compare fast disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3comfen::Disabled
    }
    #[doc = "Channel 3 compare fast enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3comfen::Enabled
    }
}
#[doc = "Field `CH3COMFEN` writer - Channel 3 output compare fast enable"]
pub type Ch3comfenW<'a, REG> = crate::BitWriter<'a, REG, Ch3comfen>;
impl<'a, REG> Ch3comfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 3 output compare fast disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comfen::Disabled)
    }
    #[doc = "Channel 3 compare fast enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comfen::Enabled)
    }
}
#[doc = "Channel 3 output compare shadow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3comsen {
    #[doc = "0: Channel 3 output compare shadow disabled."]
    Disabled = 0,
    #[doc = "1: Channel 3 compare shadow enabled."]
    Enabled = 1,
}
impl From<Ch3comsen> for bool {
    #[inline(always)]
    fn from(variant: Ch3comsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3COMSEN` reader - Channel 3 output compare shadow enable"]
pub type Ch3comsenR = crate::BitReader<Ch3comsen>;
impl Ch3comsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3comsen {
        match self.bits {
            false => Ch3comsen::Disabled,
            true => Ch3comsen::Enabled,
        }
    }
    #[doc = "Channel 3 output compare shadow disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3comsen::Disabled
    }
    #[doc = "Channel 3 compare shadow enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3comsen::Enabled
    }
}
#[doc = "Field `CH3COMSEN` writer - Channel 3 output compare shadow enable"]
pub type Ch3comsenW<'a, REG> = crate::BitWriter<'a, REG, Ch3comsen>;
impl<'a, REG> Ch3comsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 3 output compare shadow disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comsen::Disabled)
    }
    #[doc = "Channel 3 compare shadow enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comsen::Enabled)
    }
}
#[doc = "Channel 3 compare output control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch3comctl {
    #[doc = "0: Timing mode. The O3CPRE signal keeps stable, independent of the comparison between the register TIMERx_CH3CV and the counter TIMERx_CNT."]
    Mode0 = 0,
    #[doc = "1: Set the channel output. O3CPRE signal is forced high when the counter is equals to the output compare register TIMERx_CH3CV."]
    Mode1 = 1,
    #[doc = "2: Clear the channel output. O3CPRE signal is forced low when the counter is equals to the output compare register TIMERx_CH3CV."]
    Mode2 = 2,
    #[doc = "3: Toggle on match. O3CPRE toggles when the counter is equals to the output compare register TIMERx_CH3CV."]
    Mode3 = 3,
    #[doc = "4: Force low. O3CPRE is forced to low level."]
    Mode4 = 4,
    #[doc = "5: Force high. O3CPRE is forced to high level."]
    Mode5 = 5,
    #[doc = "6: PWM mode0. When counting up, O3CPRE is high when the counter is smaller than TIMERx_CH3CV, and low otherwise. When counting down, O3CPRE is low when the counter is larger than TIMERx_CH3CV, and high otherwise."]
    Pwmmode0 = 6,
    #[doc = "7: PWM mode1. When counting up, O3CPRE is low when the counter is smaller than TIMERx_CH3CV, and high otherwise. When counting down, O3CPRE is high when the counter is larger than TIMERx_CH3CV, and low otherwise."]
    Pwmmode1 = 7,
}
impl From<Ch3comctl> for u8 {
    #[inline(always)]
    fn from(variant: Ch3comctl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch3comctl {
    type Ux = u8;
}
impl crate::IsEnum for Ch3comctl {}
#[doc = "Field `CH3COMCTL` reader - Channel 3 compare output control"]
pub type Ch3comctlR = crate::FieldReader<Ch3comctl>;
impl Ch3comctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3comctl {
        match self.bits {
            0 => Ch3comctl::Mode0,
            1 => Ch3comctl::Mode1,
            2 => Ch3comctl::Mode2,
            3 => Ch3comctl::Mode3,
            4 => Ch3comctl::Mode4,
            5 => Ch3comctl::Mode5,
            6 => Ch3comctl::Pwmmode0,
            7 => Ch3comctl::Pwmmode1,
            _ => unreachable!(),
        }
    }
    #[doc = "Timing mode. The O3CPRE signal keeps stable, independent of the comparison between the register TIMERx_CH3CV and the counter TIMERx_CNT."]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Ch3comctl::Mode0
    }
    #[doc = "Set the channel output. O3CPRE signal is forced high when the counter is equals to the output compare register TIMERx_CH3CV."]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Ch3comctl::Mode1
    }
    #[doc = "Clear the channel output. O3CPRE signal is forced low when the counter is equals to the output compare register TIMERx_CH3CV."]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Ch3comctl::Mode2
    }
    #[doc = "Toggle on match. O3CPRE toggles when the counter is equals to the output compare register TIMERx_CH3CV."]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Ch3comctl::Mode3
    }
    #[doc = "Force low. O3CPRE is forced to low level."]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == Ch3comctl::Mode4
    }
    #[doc = "Force high. O3CPRE is forced to high level."]
    #[inline(always)]
    pub fn is_mode5(&self) -> bool {
        *self == Ch3comctl::Mode5
    }
    #[doc = "PWM mode0. When counting up, O3CPRE is high when the counter is smaller than TIMERx_CH3CV, and low otherwise. When counting down, O3CPRE is low when the counter is larger than TIMERx_CH3CV, and high otherwise."]
    #[inline(always)]
    pub fn is_pwmmode0(&self) -> bool {
        *self == Ch3comctl::Pwmmode0
    }
    #[doc = "PWM mode1. When counting up, O3CPRE is low when the counter is smaller than TIMERx_CH3CV, and high otherwise. When counting down, O3CPRE is high when the counter is larger than TIMERx_CH3CV, and low otherwise."]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == Ch3comctl::Pwmmode1
    }
}
#[doc = "Field `CH3COMCTL` writer - Channel 3 compare output control"]
pub type Ch3comctlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ch3comctl, crate::Safe>;
impl<'a, REG> Ch3comctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timing mode. The O3CPRE signal keeps stable, independent of the comparison between the register TIMERx_CH3CV and the counter TIMERx_CNT."]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comctl::Mode0)
    }
    #[doc = "Set the channel output. O3CPRE signal is forced high when the counter is equals to the output compare register TIMERx_CH3CV."]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comctl::Mode1)
    }
    #[doc = "Clear the channel output. O3CPRE signal is forced low when the counter is equals to the output compare register TIMERx_CH3CV."]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comctl::Mode2)
    }
    #[doc = "Toggle on match. O3CPRE toggles when the counter is equals to the output compare register TIMERx_CH3CV."]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comctl::Mode3)
    }
    #[doc = "Force low. O3CPRE is forced to low level."]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comctl::Mode4)
    }
    #[doc = "Force high. O3CPRE is forced to high level."]
    #[inline(always)]
    pub fn mode5(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comctl::Mode5)
    }
    #[doc = "PWM mode0. When counting up, O3CPRE is high when the counter is smaller than TIMERx_CH3CV, and low otherwise. When counting down, O3CPRE is low when the counter is larger than TIMERx_CH3CV, and high otherwise."]
    #[inline(always)]
    pub fn pwmmode0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comctl::Pwmmode0)
    }
    #[doc = "PWM mode1. When counting up, O3CPRE is low when the counter is smaller than TIMERx_CH3CV, and high otherwise. When counting down, O3CPRE is high when the counter is larger than TIMERx_CH3CV, and low otherwise."]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comctl::Pwmmode1)
    }
}
#[doc = "Channel 3 output compare clear enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3comcen {
    #[doc = "0: Channel 3 output compare clear disabled."]
    Disabled = 0,
    #[doc = "1: Channel 3 compare clear enabled."]
    Enabled = 1,
}
impl From<Ch3comcen> for bool {
    #[inline(always)]
    fn from(variant: Ch3comcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3COMCEN` reader - Channel 3 output compare clear enable"]
pub type Ch3comcenR = crate::BitReader<Ch3comcen>;
impl Ch3comcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3comcen {
        match self.bits {
            false => Ch3comcen::Disabled,
            true => Ch3comcen::Enabled,
        }
    }
    #[doc = "Channel 3 output compare clear disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3comcen::Disabled
    }
    #[doc = "Channel 3 compare clear enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3comcen::Enabled
    }
}
#[doc = "Field `CH3COMCEN` writer - Channel 3 output compare clear enable"]
pub type Ch3comcenW<'a, REG> = crate::BitWriter<'a, REG, Ch3comcen>;
impl<'a, REG> Ch3comcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 3 output compare clear disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comcen::Disabled)
    }
    #[doc = "Channel 3 compare clear enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3comcen::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 2 I/O mode selection"]
    #[inline(always)]
    pub fn ch2ms(&self) -> Ch2msR {
        Ch2msR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Channel 2 output compare fast enable"]
    #[inline(always)]
    pub fn ch2comfen(&self) -> Ch2comfenR {
        Ch2comfenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 compare output shadow enable"]
    #[inline(always)]
    pub fn ch2comsen(&self) -> Ch2comsenR {
        Ch2comsenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 2 compare output control"]
    #[inline(always)]
    pub fn ch2comctl(&self) -> Ch2comctlR {
        Ch2comctlR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 2 output compare clear enable"]
    #[inline(always)]
    pub fn ch2comcen(&self) -> Ch2comcenR {
        Ch2comcenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&self) -> Ch3msR {
        Ch3msR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Channel 3 output compare fast enable"]
    #[inline(always)]
    pub fn ch3comfen(&self) -> Ch3comfenR {
        Ch3comfenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 output compare shadow enable"]
    #[inline(always)]
    pub fn ch3comsen(&self) -> Ch3comsenR {
        Ch3comsenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Channel 3 compare output control"]
    #[inline(always)]
    pub fn ch3comctl(&self) -> Ch3comctlR {
        Ch3comctlR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Channel 3 output compare clear enable"]
    #[inline(always)]
    pub fn ch3comcen(&self) -> Ch3comcenR {
        Ch3comcenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 2 I/O mode selection"]
    #[inline(always)]
    pub fn ch2ms(&mut self) -> Ch2msW<'_, Chctl1OutputSpec> {
        Ch2msW::new(self, 0)
    }
    #[doc = "Bit 2 - Channel 2 output compare fast enable"]
    #[inline(always)]
    pub fn ch2comfen(&mut self) -> Ch2comfenW<'_, Chctl1OutputSpec> {
        Ch2comfenW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 2 compare output shadow enable"]
    #[inline(always)]
    pub fn ch2comsen(&mut self) -> Ch2comsenW<'_, Chctl1OutputSpec> {
        Ch2comsenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Channel 2 compare output control"]
    #[inline(always)]
    pub fn ch2comctl(&mut self) -> Ch2comctlW<'_, Chctl1OutputSpec> {
        Ch2comctlW::new(self, 4)
    }
    #[doc = "Bit 7 - Channel 2 output compare clear enable"]
    #[inline(always)]
    pub fn ch2comcen(&mut self) -> Ch2comcenW<'_, Chctl1OutputSpec> {
        Ch2comcenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&mut self) -> Ch3msW<'_, Chctl1OutputSpec> {
        Ch3msW::new(self, 8)
    }
    #[doc = "Bit 10 - Channel 3 output compare fast enable"]
    #[inline(always)]
    pub fn ch3comfen(&mut self) -> Ch3comfenW<'_, Chctl1OutputSpec> {
        Ch3comfenW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 output compare shadow enable"]
    #[inline(always)]
    pub fn ch3comsen(&mut self) -> Ch3comsenW<'_, Chctl1OutputSpec> {
        Ch3comsenW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Channel 3 compare output control"]
    #[inline(always)]
    pub fn ch3comctl(&mut self) -> Ch3comctlW<'_, Chctl1OutputSpec> {
        Ch3comctlW::new(self, 12)
    }
    #[doc = "Bit 15 - Channel 3 output compare clear enable"]
    #[inline(always)]
    pub fn ch3comcen(&mut self) -> Ch3comcenW<'_, Chctl1OutputSpec> {
        Ch3comcenW::new(self, 15)
    }
}
#[doc = "Channel control register 1 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chctl1OutputSpec;
impl crate::RegisterSpec for Chctl1OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl1_output::R`](R) reader structure"]
impl crate::Readable for Chctl1OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`chctl1_output::W`](W) writer structure"]
impl crate::Writable for Chctl1OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTL1_Output to value 0"]
impl crate::Resettable for Chctl1OutputSpec {}
