#[doc = "Register `CHCTL0_Output` reader"]
pub type R = crate::R<Chctl0OutputSpec>;
#[doc = "Register `CHCTL0_Output` writer"]
pub type W = crate::W<Chctl0OutputSpec>;
#[doc = "Channel 0 I/O mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch0ms {
    #[doc = "0: Channel 0 output mode."]
    Output = 0,
    #[doc = "1: Channel 0 is programmed as input mode, IS0 is connected to CI0FE0"]
    InputCi1 = 1,
    #[doc = "2: Channel 0 is programmed as input mode, IS0 is connected to CI1FE0"]
    InputCi0 = 2,
    #[doc = "3: Channel 0 is programmed as input mode, IS0 is connected to ITS"]
    InputIts = 3,
}
impl From<Ch0ms> for u8 {
    #[inline(always)]
    fn from(variant: Ch0ms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch0ms {
    type Ux = u8;
}
impl crate::IsEnum for Ch0ms {}
#[doc = "Field `CH0MS` reader - Channel 0 I/O mode selection"]
pub type Ch0msR = crate::FieldReader<Ch0ms>;
impl Ch0msR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0ms {
        match self.bits {
            0 => Ch0ms::Output,
            1 => Ch0ms::InputCi1,
            2 => Ch0ms::InputCi0,
            3 => Ch0ms::InputIts,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 output mode."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Ch0ms::Output
    }
    #[doc = "Channel 0 is programmed as input mode, IS0 is connected to CI0FE0"]
    #[inline(always)]
    pub fn is_input_ci1(&self) -> bool {
        *self == Ch0ms::InputCi1
    }
    #[doc = "Channel 0 is programmed as input mode, IS0 is connected to CI1FE0"]
    #[inline(always)]
    pub fn is_input_ci0(&self) -> bool {
        *self == Ch0ms::InputCi0
    }
    #[doc = "Channel 0 is programmed as input mode, IS0 is connected to ITS"]
    #[inline(always)]
    pub fn is_input_its(&self) -> bool {
        *self == Ch0ms::InputIts
    }
}
#[doc = "Field `CH0MS` writer - Channel 0 I/O mode selection"]
pub type Ch0msW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch0ms, crate::Safe>;
impl<'a, REG> Ch0msW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0ms::Output)
    }
    #[doc = "Channel 0 is programmed as input mode, IS0 is connected to CI0FE0"]
    #[inline(always)]
    pub fn input_ci1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0ms::InputCi1)
    }
    #[doc = "Channel 0 is programmed as input mode, IS0 is connected to CI1FE0"]
    #[inline(always)]
    pub fn input_ci0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0ms::InputCi0)
    }
    #[doc = "Channel 0 is programmed as input mode, IS0 is connected to ITS"]
    #[inline(always)]
    pub fn input_its(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0ms::InputIts)
    }
}
#[doc = "Channel 0 output compare fast enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0comfen {
    #[doc = "0: Channel 0 output compare fast disabled."]
    Disabled = 0,
    #[doc = "1: Channel 0 compare fast enabled."]
    Enabled = 1,
}
impl From<Ch0comfen> for bool {
    #[inline(always)]
    fn from(variant: Ch0comfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0COMFEN` reader - Channel 0 output compare fast enable"]
pub type Ch0comfenR = crate::BitReader<Ch0comfen>;
impl Ch0comfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0comfen {
        match self.bits {
            false => Ch0comfen::Disabled,
            true => Ch0comfen::Enabled,
        }
    }
    #[doc = "Channel 0 output compare fast disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0comfen::Disabled
    }
    #[doc = "Channel 0 compare fast enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0comfen::Enabled
    }
}
#[doc = "Field `CH0COMFEN` writer - Channel 0 output compare fast enable"]
pub type Ch0comfenW<'a, REG> = crate::BitWriter<'a, REG, Ch0comfen>;
impl<'a, REG> Ch0comfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0 output compare fast disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0comfen::Disabled)
    }
    #[doc = "Channel 0 compare fast enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0comfen::Enabled)
    }
}
#[doc = "Channel 0 compare output shadow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0comsen {
    #[doc = "0: Channel 0 output compare shadow disabled."]
    Disabled = 0,
    #[doc = "1: Channel 0 compare shadow enabled."]
    Enabled = 1,
}
impl From<Ch0comsen> for bool {
    #[inline(always)]
    fn from(variant: Ch0comsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0COMSEN` reader - Channel 0 compare output shadow enable"]
pub type Ch0comsenR = crate::BitReader<Ch0comsen>;
impl Ch0comsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0comsen {
        match self.bits {
            false => Ch0comsen::Disabled,
            true => Ch0comsen::Enabled,
        }
    }
    #[doc = "Channel 0 output compare shadow disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0comsen::Disabled
    }
    #[doc = "Channel 0 compare shadow enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0comsen::Enabled
    }
}
#[doc = "Field `CH0COMSEN` writer - Channel 0 compare output shadow enable"]
pub type Ch0comsenW<'a, REG> = crate::BitWriter<'a, REG, Ch0comsen>;
impl<'a, REG> Ch0comsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 0 output compare shadow disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0comsen::Disabled)
    }
    #[doc = "Channel 0 compare shadow enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0comsen::Enabled)
    }
}
#[doc = "Channel 0 compare output control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch0comctl {
    #[doc = "0: Timing mode. The O0CPRE signal keeps stable, independent of the comparison between the register TIMERx_CH0CV and the counter TIMERx_CNT."]
    Mode0 = 0,
    #[doc = "1: Set the channel output. O0CPRE signal is forced high when the counter is equals to the output compare register TIMERx_CH0CV."]
    Mode1 = 1,
    #[doc = "2: Clear the channel output. O0CPRE signal is forced low when the counter is equals to the output compare register TIMERx_CH0CV."]
    Mode2 = 2,
    #[doc = "3: Toggle on match. O0CPRE toggles when the counter is equals to the output compare register TIMERx_CH0CV."]
    Mode3 = 3,
    #[doc = "4: Force low. O0CPRE is forced to low level."]
    Mode4 = 4,
    #[doc = "5: Force high. O0CPRE is forced to high level."]
    Mode5 = 5,
    #[doc = "6: PWM mode0. When counting up, O0CPRE is high when the counter is smaller than TIMERx_CH0CV, and low otherwise."]
    Pwmmode0 = 6,
    #[doc = "7: PWM mode1. When counting up, O0CPRE is low when the counter is smaller than TIMERx_CH0CV, and high otherwise."]
    Pwmmode1 = 7,
}
impl From<Ch0comctl> for u8 {
    #[inline(always)]
    fn from(variant: Ch0comctl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch0comctl {
    type Ux = u8;
}
impl crate::IsEnum for Ch0comctl {}
#[doc = "Field `CH0COMCTL` reader - Channel 0 compare output control"]
pub type Ch0comctlR = crate::FieldReader<Ch0comctl>;
impl Ch0comctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0comctl {
        match self.bits {
            0 => Ch0comctl::Mode0,
            1 => Ch0comctl::Mode1,
            2 => Ch0comctl::Mode2,
            3 => Ch0comctl::Mode3,
            4 => Ch0comctl::Mode4,
            5 => Ch0comctl::Mode5,
            6 => Ch0comctl::Pwmmode0,
            7 => Ch0comctl::Pwmmode1,
            _ => unreachable!(),
        }
    }
    #[doc = "Timing mode. The O0CPRE signal keeps stable, independent of the comparison between the register TIMERx_CH0CV and the counter TIMERx_CNT."]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Ch0comctl::Mode0
    }
    #[doc = "Set the channel output. O0CPRE signal is forced high when the counter is equals to the output compare register TIMERx_CH0CV."]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Ch0comctl::Mode1
    }
    #[doc = "Clear the channel output. O0CPRE signal is forced low when the counter is equals to the output compare register TIMERx_CH0CV."]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Ch0comctl::Mode2
    }
    #[doc = "Toggle on match. O0CPRE toggles when the counter is equals to the output compare register TIMERx_CH0CV."]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Ch0comctl::Mode3
    }
    #[doc = "Force low. O0CPRE is forced to low level."]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == Ch0comctl::Mode4
    }
    #[doc = "Force high. O0CPRE is forced to high level."]
    #[inline(always)]
    pub fn is_mode5(&self) -> bool {
        *self == Ch0comctl::Mode5
    }
    #[doc = "PWM mode0. When counting up, O0CPRE is high when the counter is smaller than TIMERx_CH0CV, and low otherwise."]
    #[inline(always)]
    pub fn is_pwmmode0(&self) -> bool {
        *self == Ch0comctl::Pwmmode0
    }
    #[doc = "PWM mode1. When counting up, O0CPRE is low when the counter is smaller than TIMERx_CH0CV, and high otherwise."]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == Ch0comctl::Pwmmode1
    }
}
#[doc = "Field `CH0COMCTL` writer - Channel 0 compare output control"]
pub type Ch0comctlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ch0comctl, crate::Safe>;
impl<'a, REG> Ch0comctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timing mode. The O0CPRE signal keeps stable, independent of the comparison between the register TIMERx_CH0CV and the counter TIMERx_CNT."]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0comctl::Mode0)
    }
    #[doc = "Set the channel output. O0CPRE signal is forced high when the counter is equals to the output compare register TIMERx_CH0CV."]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0comctl::Mode1)
    }
    #[doc = "Clear the channel output. O0CPRE signal is forced low when the counter is equals to the output compare register TIMERx_CH0CV."]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0comctl::Mode2)
    }
    #[doc = "Toggle on match. O0CPRE toggles when the counter is equals to the output compare register TIMERx_CH0CV."]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0comctl::Mode3)
    }
    #[doc = "Force low. O0CPRE is forced to low level."]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0comctl::Mode4)
    }
    #[doc = "Force high. O0CPRE is forced to high level."]
    #[inline(always)]
    pub fn mode5(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0comctl::Mode5)
    }
    #[doc = "PWM mode0. When counting up, O0CPRE is high when the counter is smaller than TIMERx_CH0CV, and low otherwise."]
    #[inline(always)]
    pub fn pwmmode0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0comctl::Pwmmode0)
    }
    #[doc = "PWM mode1. When counting up, O0CPRE is low when the counter is smaller than TIMERx_CH0CV, and high otherwise."]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0comctl::Pwmmode1)
    }
}
#[doc = "Channel 1 mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch1ms {
    #[doc = "0: Channel 1 output mode."]
    Output = 0,
    #[doc = "1: Channel 1 is programmed as input mode, IS1 is connected to CI1FE1"]
    InputCi1 = 1,
    #[doc = "2: Channel 1 is programmed as input mode, IS1 is connected to CI0FE1"]
    InputCi0 = 2,
    #[doc = "3: Channel 1 is programmed as input mode, IS1 is connected to ITS"]
    InputIts = 3,
}
impl From<Ch1ms> for u8 {
    #[inline(always)]
    fn from(variant: Ch1ms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch1ms {
    type Ux = u8;
}
impl crate::IsEnum for Ch1ms {}
#[doc = "Field `CH1MS` reader - Channel 1 mode selection"]
pub type Ch1msR = crate::FieldReader<Ch1ms>;
impl Ch1msR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1ms {
        match self.bits {
            0 => Ch1ms::Output,
            1 => Ch1ms::InputCi1,
            2 => Ch1ms::InputCi0,
            3 => Ch1ms::InputIts,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 1 output mode."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Ch1ms::Output
    }
    #[doc = "Channel 1 is programmed as input mode, IS1 is connected to CI1FE1"]
    #[inline(always)]
    pub fn is_input_ci1(&self) -> bool {
        *self == Ch1ms::InputCi1
    }
    #[doc = "Channel 1 is programmed as input mode, IS1 is connected to CI0FE1"]
    #[inline(always)]
    pub fn is_input_ci0(&self) -> bool {
        *self == Ch1ms::InputCi0
    }
    #[doc = "Channel 1 is programmed as input mode, IS1 is connected to ITS"]
    #[inline(always)]
    pub fn is_input_its(&self) -> bool {
        *self == Ch1ms::InputIts
    }
}
#[doc = "Field `CH1MS` writer - Channel 1 mode selection"]
pub type Ch1msW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch1ms, crate::Safe>;
impl<'a, REG> Ch1msW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 1 output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1ms::Output)
    }
    #[doc = "Channel 1 is programmed as input mode, IS1 is connected to CI1FE1"]
    #[inline(always)]
    pub fn input_ci1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1ms::InputCi1)
    }
    #[doc = "Channel 1 is programmed as input mode, IS1 is connected to CI0FE1"]
    #[inline(always)]
    pub fn input_ci0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1ms::InputCi0)
    }
    #[doc = "Channel 1 is programmed as input mode, IS1 is connected to ITS"]
    #[inline(always)]
    pub fn input_its(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1ms::InputIts)
    }
}
#[doc = "Channel 1 output compare fast enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1comfen {
    #[doc = "0: Channel 1 output compare fast disabled."]
    Disabled = 0,
    #[doc = "1: Channel 1 compare fast enabled."]
    Enabled = 1,
}
impl From<Ch1comfen> for bool {
    #[inline(always)]
    fn from(variant: Ch1comfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1COMFEN` reader - Channel 1 output compare fast enable"]
pub type Ch1comfenR = crate::BitReader<Ch1comfen>;
impl Ch1comfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1comfen {
        match self.bits {
            false => Ch1comfen::Disabled,
            true => Ch1comfen::Enabled,
        }
    }
    #[doc = "Channel 1 output compare fast disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1comfen::Disabled
    }
    #[doc = "Channel 1 compare fast enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1comfen::Enabled
    }
}
#[doc = "Field `CH1COMFEN` writer - Channel 1 output compare fast enable"]
pub type Ch1comfenW<'a, REG> = crate::BitWriter<'a, REG, Ch1comfen>;
impl<'a, REG> Ch1comfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1 output compare fast disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1comfen::Disabled)
    }
    #[doc = "Channel 1 compare fast enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1comfen::Enabled)
    }
}
#[doc = "Channel 1 output compare shadow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1comsen {
    #[doc = "0: Channel 1 output compare shadow disabled."]
    Disabled = 0,
    #[doc = "1: Channel 1 compare shadow enabled."]
    Enabled = 1,
}
impl From<Ch1comsen> for bool {
    #[inline(always)]
    fn from(variant: Ch1comsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1COMSEN` reader - Channel 1 output compare shadow enable"]
pub type Ch1comsenR = crate::BitReader<Ch1comsen>;
impl Ch1comsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1comsen {
        match self.bits {
            false => Ch1comsen::Disabled,
            true => Ch1comsen::Enabled,
        }
    }
    #[doc = "Channel 1 output compare shadow disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1comsen::Disabled
    }
    #[doc = "Channel 1 compare shadow enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1comsen::Enabled
    }
}
#[doc = "Field `CH1COMSEN` writer - Channel 1 output compare shadow enable"]
pub type Ch1comsenW<'a, REG> = crate::BitWriter<'a, REG, Ch1comsen>;
impl<'a, REG> Ch1comsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel 1 output compare shadow disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1comsen::Disabled)
    }
    #[doc = "Channel 1 compare shadow enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1comsen::Enabled)
    }
}
#[doc = "Channel 1 compare output control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch1comctl {
    #[doc = "0: Timing mode. The O1CPRE signal keeps stable, independent of the comparison between the register TIMERx_CH1CV and the counter TIMERx_CNT."]
    Mode0 = 0,
    #[doc = "1: Set the channel output. O1CPRE signal is forced high when the counter is equals to the output compare register TIMERx_CH1CV."]
    Mode1 = 1,
    #[doc = "2: Clear the channel output. O1CPRE signal is forced low when the counter is equals to the output compare register TIMERx_CH1CV."]
    Mode2 = 2,
    #[doc = "3: Toggle on match. O1CPRE toggles when the counter is equals to the output compare register TIMERx_CH1CV."]
    Mode3 = 3,
    #[doc = "4: Force low. O1CPRE is forced to low level."]
    Mode4 = 4,
    #[doc = "5: Force high. O1CPRE is forced to high level."]
    Mode5 = 5,
    #[doc = "6: PWM mode0. When counting up, O1CPRE is high when the counter is smaller than TIMERx_CH1CV, and low otherwise."]
    Pwmmode0 = 6,
    #[doc = "7: PWM mode1. When counting up, O1CPRE is low when the counter is smaller than TIMERx_CH1CV, and high otherwise."]
    Pwmmode1 = 7,
}
impl From<Ch1comctl> for u8 {
    #[inline(always)]
    fn from(variant: Ch1comctl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch1comctl {
    type Ux = u8;
}
impl crate::IsEnum for Ch1comctl {}
#[doc = "Field `CH1COMCTL` reader - Channel 1 compare output control"]
pub type Ch1comctlR = crate::FieldReader<Ch1comctl>;
impl Ch1comctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1comctl {
        match self.bits {
            0 => Ch1comctl::Mode0,
            1 => Ch1comctl::Mode1,
            2 => Ch1comctl::Mode2,
            3 => Ch1comctl::Mode3,
            4 => Ch1comctl::Mode4,
            5 => Ch1comctl::Mode5,
            6 => Ch1comctl::Pwmmode0,
            7 => Ch1comctl::Pwmmode1,
            _ => unreachable!(),
        }
    }
    #[doc = "Timing mode. The O1CPRE signal keeps stable, independent of the comparison between the register TIMERx_CH1CV and the counter TIMERx_CNT."]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Ch1comctl::Mode0
    }
    #[doc = "Set the channel output. O1CPRE signal is forced high when the counter is equals to the output compare register TIMERx_CH1CV."]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Ch1comctl::Mode1
    }
    #[doc = "Clear the channel output. O1CPRE signal is forced low when the counter is equals to the output compare register TIMERx_CH1CV."]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Ch1comctl::Mode2
    }
    #[doc = "Toggle on match. O1CPRE toggles when the counter is equals to the output compare register TIMERx_CH1CV."]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Ch1comctl::Mode3
    }
    #[doc = "Force low. O1CPRE is forced to low level."]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == Ch1comctl::Mode4
    }
    #[doc = "Force high. O1CPRE is forced to high level."]
    #[inline(always)]
    pub fn is_mode5(&self) -> bool {
        *self == Ch1comctl::Mode5
    }
    #[doc = "PWM mode0. When counting up, O1CPRE is high when the counter is smaller than TIMERx_CH1CV, and low otherwise."]
    #[inline(always)]
    pub fn is_pwmmode0(&self) -> bool {
        *self == Ch1comctl::Pwmmode0
    }
    #[doc = "PWM mode1. When counting up, O1CPRE is low when the counter is smaller than TIMERx_CH1CV, and high otherwise."]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == Ch1comctl::Pwmmode1
    }
}
#[doc = "Field `CH1COMCTL` writer - Channel 1 compare output control"]
pub type Ch1comctlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ch1comctl, crate::Safe>;
impl<'a, REG> Ch1comctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timing mode. The O1CPRE signal keeps stable, independent of the comparison between the register TIMERx_CH1CV and the counter TIMERx_CNT."]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1comctl::Mode0)
    }
    #[doc = "Set the channel output. O1CPRE signal is forced high when the counter is equals to the output compare register TIMERx_CH1CV."]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1comctl::Mode1)
    }
    #[doc = "Clear the channel output. O1CPRE signal is forced low when the counter is equals to the output compare register TIMERx_CH1CV."]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1comctl::Mode2)
    }
    #[doc = "Toggle on match. O1CPRE toggles when the counter is equals to the output compare register TIMERx_CH1CV."]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1comctl::Mode3)
    }
    #[doc = "Force low. O1CPRE is forced to low level."]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1comctl::Mode4)
    }
    #[doc = "Force high. O1CPRE is forced to high level."]
    #[inline(always)]
    pub fn mode5(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1comctl::Mode5)
    }
    #[doc = "PWM mode0. When counting up, O1CPRE is high when the counter is smaller than TIMERx_CH1CV, and low otherwise."]
    #[inline(always)]
    pub fn pwmmode0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1comctl::Pwmmode0)
    }
    #[doc = "PWM mode1. When counting up, O1CPRE is low when the counter is smaller than TIMERx_CH1CV, and high otherwise."]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1comctl::Pwmmode1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 0 I/O mode selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> Ch0msR {
        Ch0msR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Channel 0 output compare fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&self) -> Ch0comfenR {
        Ch0comfenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 compare output shadow enable"]
    #[inline(always)]
    pub fn ch0comsen(&self) -> Ch0comsenR {
        Ch0comsenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 0 compare output control"]
    #[inline(always)]
    pub fn ch0comctl(&self) -> Ch0comctlR {
        Ch0comctlR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    pub fn ch1ms(&self) -> Ch1msR {
        Ch1msR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Channel 1 output compare fast enable"]
    #[inline(always)]
    pub fn ch1comfen(&self) -> Ch1comfenR {
        Ch1comfenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 1 output compare shadow enable"]
    #[inline(always)]
    pub fn ch1comsen(&self) -> Ch1comsenR {
        Ch1comsenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Channel 1 compare output control"]
    #[inline(always)]
    pub fn ch1comctl(&self) -> Ch1comctlR {
        Ch1comctlR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 I/O mode selection"]
    #[inline(always)]
    pub fn ch0ms(&mut self) -> Ch0msW<'_, Chctl0OutputSpec> {
        Ch0msW::new(self, 0)
    }
    #[doc = "Bit 2 - Channel 0 output compare fast enable"]
    #[inline(always)]
    pub fn ch0comfen(&mut self) -> Ch0comfenW<'_, Chctl0OutputSpec> {
        Ch0comfenW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 0 compare output shadow enable"]
    #[inline(always)]
    pub fn ch0comsen(&mut self) -> Ch0comsenW<'_, Chctl0OutputSpec> {
        Ch0comsenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Channel 0 compare output control"]
    #[inline(always)]
    pub fn ch0comctl(&mut self) -> Ch0comctlW<'_, Chctl0OutputSpec> {
        Ch0comctlW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    pub fn ch1ms(&mut self) -> Ch1msW<'_, Chctl0OutputSpec> {
        Ch1msW::new(self, 8)
    }
    #[doc = "Bit 10 - Channel 1 output compare fast enable"]
    #[inline(always)]
    pub fn ch1comfen(&mut self) -> Ch1comfenW<'_, Chctl0OutputSpec> {
        Ch1comfenW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 1 output compare shadow enable"]
    #[inline(always)]
    pub fn ch1comsen(&mut self) -> Ch1comsenW<'_, Chctl0OutputSpec> {
        Ch1comsenW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Channel 1 compare output control"]
    #[inline(always)]
    pub fn ch1comctl(&mut self) -> Ch1comctlW<'_, Chctl0OutputSpec> {
        Ch1comctlW::new(self, 12)
    }
}
#[doc = "Channel control register 0 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl0_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl0_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chctl0OutputSpec;
impl crate::RegisterSpec for Chctl0OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl0_output::R`](R) reader structure"]
impl crate::Readable for Chctl0OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`chctl0_output::W`](W) writer structure"]
impl crate::Writable for Chctl0OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTL0_Output to value 0"]
impl crate::Resettable for Chctl0OutputSpec {}
