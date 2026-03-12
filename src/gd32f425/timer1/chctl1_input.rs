#[doc = "Register `CHCTL1_Input` reader"]
pub type R = crate::R<Chctl1InputSpec>;
#[doc = "Register `CHCTL1_Input` writer"]
pub type W = crate::W<Chctl1InputSpec>;
#[doc = "Channel 2 mode selection\n\nValue on reset: 0"]
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
#[doc = "Field `CH2MS` reader - Channel 2 mode selection"]
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
#[doc = "Field `CH2MS` writer - Channel 2 mode selection"]
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
#[doc = "Channel 2 input capture prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch2cappsc {
    #[doc = "0: Channel 2 input capture prescaler disabled."]
    Disabled = 0,
    #[doc = "1: Channel 2 The input capture occurs on every 2 channel input edges"]
    Div2 = 1,
    #[doc = "2: Channel 2 The input capture occurs on every 4 channel input edges"]
    Div4 = 2,
    #[doc = "3: Channel 2 The input capture occurs on every 8 channel input edges"]
    Div8 = 3,
}
impl From<Ch2cappsc> for u8 {
    #[inline(always)]
    fn from(variant: Ch2cappsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch2cappsc {
    type Ux = u8;
}
impl crate::IsEnum for Ch2cappsc {}
#[doc = "Field `CH2CAPPSC` reader - Channel 2 input capture prescaler"]
pub type Ch2cappscR = crate::FieldReader<Ch2cappsc>;
impl Ch2cappscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2cappsc {
        match self.bits {
            0 => Ch2cappsc::Disabled,
            1 => Ch2cappsc::Div2,
            2 => Ch2cappsc::Div4,
            3 => Ch2cappsc::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 2 input capture prescaler disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2cappsc::Disabled
    }
    #[doc = "Channel 2 The input capture occurs on every 2 channel input edges"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ch2cappsc::Div2
    }
    #[doc = "Channel 2 The input capture occurs on every 4 channel input edges"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Ch2cappsc::Div4
    }
    #[doc = "Channel 2 The input capture occurs on every 8 channel input edges"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Ch2cappsc::Div8
    }
}
#[doc = "Field `CH2CAPPSC` writer - Channel 2 input capture prescaler"]
pub type Ch2cappscW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch2cappsc, crate::Safe>;
impl<'a, REG> Ch2cappscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 2 input capture prescaler disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2cappsc::Disabled)
    }
    #[doc = "Channel 2 The input capture occurs on every 2 channel input edges"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2cappsc::Div2)
    }
    #[doc = "Channel 2 The input capture occurs on every 4 channel input edges"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2cappsc::Div4)
    }
    #[doc = "Channel 2 The input capture occurs on every 8 channel input edges"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2cappsc::Div8)
    }
}
#[doc = "Channel 2 input capture filter control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch2capflt {
    #[doc = "0: Channel 2 input filter disabled."]
    Disabled = 0,
    #[doc = "1: Channel 2 input filter capacity is 2, f_samp is fck_timer"]
    Capacity1 = 1,
    #[doc = "2: Channel 2 input filter capacity is 4, f_samp is fck_timer"]
    Capacity2 = 2,
    #[doc = "3: Channel 2 input filter capacity is 8, f_samp is fck_timer"]
    Capacity3 = 3,
    #[doc = "4: Channel 2 input filter capacity is 6, f_samp is f_dts / 2"]
    Capacity4 = 4,
    #[doc = "5: Channel 2 input filter capacity is 8, f_samp is f_dts / 2"]
    Capacity5 = 5,
    #[doc = "6: Channel 2 input filter capacity is 6, f_samp is f_dts / 4"]
    Capacity6 = 6,
    #[doc = "7: Channel 2 input filter capacity is 8, f_samp is f_dts / 4"]
    Capacity7 = 7,
    #[doc = "8: Channel 2 input filter capacity is 6, f_samp is f_dts / 8"]
    Capacity8 = 8,
    #[doc = "9: Channel 2 input filter capacity is 8, f_samp is f_dts / 8"]
    Capacity9 = 9,
    #[doc = "10: Channel 2 input filter capacity is 5, f_samp is f_dts / 16"]
    Capacity10 = 10,
    #[doc = "11: Channel 2 input filter capacity is 6, f_samp is f_dts / 16"]
    Capacity11 = 11,
    #[doc = "12: Channel 2 input filter capacity is 8, f_samp is f_dts / 16"]
    Capacity12 = 12,
    #[doc = "13: Channel 2 input filter capacity is 5, f_samp is f_dts / 32"]
    Capacity13 = 13,
    #[doc = "14: Channel 2 input filter capacity is 6, f_samp is f_dts / 32"]
    Capacity14 = 14,
    #[doc = "15: Channel 2 input filter capacity is 8, f_samp is f_dts / 32"]
    Capacity15 = 15,
}
impl From<Ch2capflt> for u8 {
    #[inline(always)]
    fn from(variant: Ch2capflt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch2capflt {
    type Ux = u8;
}
impl crate::IsEnum for Ch2capflt {}
#[doc = "Field `CH2CAPFLT` reader - Channel 2 input capture filter control"]
pub type Ch2capfltR = crate::FieldReader<Ch2capflt>;
impl Ch2capfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2capflt {
        match self.bits {
            0 => Ch2capflt::Disabled,
            1 => Ch2capflt::Capacity1,
            2 => Ch2capflt::Capacity2,
            3 => Ch2capflt::Capacity3,
            4 => Ch2capflt::Capacity4,
            5 => Ch2capflt::Capacity5,
            6 => Ch2capflt::Capacity6,
            7 => Ch2capflt::Capacity7,
            8 => Ch2capflt::Capacity8,
            9 => Ch2capflt::Capacity9,
            10 => Ch2capflt::Capacity10,
            11 => Ch2capflt::Capacity11,
            12 => Ch2capflt::Capacity12,
            13 => Ch2capflt::Capacity13,
            14 => Ch2capflt::Capacity14,
            15 => Ch2capflt::Capacity15,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 2 input filter disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2capflt::Disabled
    }
    #[doc = "Channel 2 input filter capacity is 2, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity1(&self) -> bool {
        *self == Ch2capflt::Capacity1
    }
    #[doc = "Channel 2 input filter capacity is 4, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity2(&self) -> bool {
        *self == Ch2capflt::Capacity2
    }
    #[doc = "Channel 2 input filter capacity is 8, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity3(&self) -> bool {
        *self == Ch2capflt::Capacity3
    }
    #[doc = "Channel 2 input filter capacity is 6, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn is_capacity4(&self) -> bool {
        *self == Ch2capflt::Capacity4
    }
    #[doc = "Channel 2 input filter capacity is 8, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn is_capacity5(&self) -> bool {
        *self == Ch2capflt::Capacity5
    }
    #[doc = "Channel 2 input filter capacity is 6, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn is_capacity6(&self) -> bool {
        *self == Ch2capflt::Capacity6
    }
    #[doc = "Channel 2 input filter capacity is 8, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn is_capacity7(&self) -> bool {
        *self == Ch2capflt::Capacity7
    }
    #[doc = "Channel 2 input filter capacity is 6, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn is_capacity8(&self) -> bool {
        *self == Ch2capflt::Capacity8
    }
    #[doc = "Channel 2 input filter capacity is 8, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn is_capacity9(&self) -> bool {
        *self == Ch2capflt::Capacity9
    }
    #[doc = "Channel 2 input filter capacity is 5, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn is_capacity10(&self) -> bool {
        *self == Ch2capflt::Capacity10
    }
    #[doc = "Channel 2 input filter capacity is 6, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn is_capacity11(&self) -> bool {
        *self == Ch2capflt::Capacity11
    }
    #[doc = "Channel 2 input filter capacity is 8, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn is_capacity12(&self) -> bool {
        *self == Ch2capflt::Capacity12
    }
    #[doc = "Channel 2 input filter capacity is 5, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn is_capacity13(&self) -> bool {
        *self == Ch2capflt::Capacity13
    }
    #[doc = "Channel 2 input filter capacity is 6, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn is_capacity14(&self) -> bool {
        *self == Ch2capflt::Capacity14
    }
    #[doc = "Channel 2 input filter capacity is 8, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn is_capacity15(&self) -> bool {
        *self == Ch2capflt::Capacity15
    }
}
#[doc = "Field `CH2CAPFLT` writer - Channel 2 input capture filter control"]
pub type Ch2capfltW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ch2capflt, crate::Safe>;
impl<'a, REG> Ch2capfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 2 input filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Disabled)
    }
    #[doc = "Channel 2 input filter capacity is 2, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity1)
    }
    #[doc = "Channel 2 input filter capacity is 4, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity2)
    }
    #[doc = "Channel 2 input filter capacity is 8, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity3)
    }
    #[doc = "Channel 2 input filter capacity is 6, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn capacity4(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity4)
    }
    #[doc = "Channel 2 input filter capacity is 8, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn capacity5(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity5)
    }
    #[doc = "Channel 2 input filter capacity is 6, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn capacity6(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity6)
    }
    #[doc = "Channel 2 input filter capacity is 8, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn capacity7(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity7)
    }
    #[doc = "Channel 2 input filter capacity is 6, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn capacity8(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity8)
    }
    #[doc = "Channel 2 input filter capacity is 8, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn capacity9(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity9)
    }
    #[doc = "Channel 2 input filter capacity is 5, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn capacity10(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity10)
    }
    #[doc = "Channel 2 input filter capacity is 6, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn capacity11(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity11)
    }
    #[doc = "Channel 2 input filter capacity is 8, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn capacity12(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity12)
    }
    #[doc = "Channel 2 input filter capacity is 5, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn capacity13(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity13)
    }
    #[doc = "Channel 2 input filter capacity is 6, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn capacity14(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity14)
    }
    #[doc = "Channel 2 input filter capacity is 8, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn capacity15(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2capflt::Capacity15)
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
    #[doc = "2: Channel 3 is programmed as input mode, IS3 is connected to CI3FE3"]
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
    #[doc = "Channel 3 is programmed as input mode, IS3 is connected to CI3FE3"]
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
    #[doc = "Channel 3 is programmed as input mode, IS3 is connected to CI3FE3"]
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
#[doc = "Channel 3 input capture prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch3cappsc {
    #[doc = "0: Channel 3 input capture prescaler disabled."]
    Disabled = 0,
    #[doc = "1: Channel 3 The input capture occurs on every 2 channel input edges"]
    Div2 = 1,
    #[doc = "2: Channel 3 The input capture occurs on every 4 channel input edges"]
    Div4 = 2,
    #[doc = "3: Channel 3 The input capture occurs on every 8 channel input edges"]
    Div8 = 3,
}
impl From<Ch3cappsc> for u8 {
    #[inline(always)]
    fn from(variant: Ch3cappsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch3cappsc {
    type Ux = u8;
}
impl crate::IsEnum for Ch3cappsc {}
#[doc = "Field `CH3CAPPSC` reader - Channel 3 input capture prescaler"]
pub type Ch3cappscR = crate::FieldReader<Ch3cappsc>;
impl Ch3cappscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3cappsc {
        match self.bits {
            0 => Ch3cappsc::Disabled,
            1 => Ch3cappsc::Div2,
            2 => Ch3cappsc::Div4,
            3 => Ch3cappsc::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 3 input capture prescaler disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3cappsc::Disabled
    }
    #[doc = "Channel 3 The input capture occurs on every 2 channel input edges"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ch3cappsc::Div2
    }
    #[doc = "Channel 3 The input capture occurs on every 4 channel input edges"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Ch3cappsc::Div4
    }
    #[doc = "Channel 3 The input capture occurs on every 8 channel input edges"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Ch3cappsc::Div8
    }
}
#[doc = "Field `CH3CAPPSC` writer - Channel 3 input capture prescaler"]
pub type Ch3cappscW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch3cappsc, crate::Safe>;
impl<'a, REG> Ch3cappscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 3 input capture prescaler disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3cappsc::Disabled)
    }
    #[doc = "Channel 3 The input capture occurs on every 2 channel input edges"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3cappsc::Div2)
    }
    #[doc = "Channel 3 The input capture occurs on every 4 channel input edges"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3cappsc::Div4)
    }
    #[doc = "Channel 3 The input capture occurs on every 8 channel input edges"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3cappsc::Div8)
    }
}
#[doc = "Channel 3 input capture filter control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch3capflt {
    #[doc = "0: Channel 3 input filter disabled."]
    Disabled = 0,
    #[doc = "1: Channel 3 input filter capacity is 2, f_samp is fck_timer"]
    Capacity1 = 1,
    #[doc = "2: Channel 3 input filter capacity is 4, f_samp is fck_timer"]
    Capacity2 = 2,
    #[doc = "3: Channel 3 input filter capacity is 8, f_samp is fck_timer"]
    Capacity3 = 3,
    #[doc = "4: Channel 3 input filter capacity is 6, f_samp is f_dts / 2"]
    Capacity4 = 4,
    #[doc = "5: Channel 3 input filter capacity is 8, f_samp is f_dts / 2"]
    Capacity5 = 5,
    #[doc = "6: Channel 3 input filter capacity is 6, f_samp is f_dts / 4"]
    Capacity6 = 6,
    #[doc = "7: Channel 3 input filter capacity is 8, f_samp is f_dts / 4"]
    Capacity7 = 7,
    #[doc = "8: Channel 3 input filter capacity is 6, f_samp is f_dts / 8"]
    Capacity8 = 8,
    #[doc = "9: Channel 3 input filter capacity is 8, f_samp is f_dts / 8"]
    Capacity9 = 9,
    #[doc = "10: Channel 3 input filter capacity is 5, f_samp is f_dts / 16"]
    Capacity10 = 10,
    #[doc = "11: Channel 3 input filter capacity is 6, f_samp is f_dts / 16"]
    Capacity11 = 11,
    #[doc = "12: Channel 3 input filter capacity is 8, f_samp is f_dts / 16"]
    Capacity12 = 12,
    #[doc = "13: Channel 3 input filter capacity is 5, f_samp is f_dts / 32"]
    Capacity13 = 13,
    #[doc = "14: Channel 3 input filter capacity is 6, f_samp is f_dts / 32"]
    Capacity14 = 14,
    #[doc = "15: Channel 3 input filter capacity is 8, f_samp is f_dts / 32"]
    Capacity15 = 15,
}
impl From<Ch3capflt> for u8 {
    #[inline(always)]
    fn from(variant: Ch3capflt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch3capflt {
    type Ux = u8;
}
impl crate::IsEnum for Ch3capflt {}
#[doc = "Field `CH3CAPFLT` reader - Channel 3 input capture filter control"]
pub type Ch3capfltR = crate::FieldReader<Ch3capflt>;
impl Ch3capfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3capflt {
        match self.bits {
            0 => Ch3capflt::Disabled,
            1 => Ch3capflt::Capacity1,
            2 => Ch3capflt::Capacity2,
            3 => Ch3capflt::Capacity3,
            4 => Ch3capflt::Capacity4,
            5 => Ch3capflt::Capacity5,
            6 => Ch3capflt::Capacity6,
            7 => Ch3capflt::Capacity7,
            8 => Ch3capflt::Capacity8,
            9 => Ch3capflt::Capacity9,
            10 => Ch3capflt::Capacity10,
            11 => Ch3capflt::Capacity11,
            12 => Ch3capflt::Capacity12,
            13 => Ch3capflt::Capacity13,
            14 => Ch3capflt::Capacity14,
            15 => Ch3capflt::Capacity15,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 3 input filter disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3capflt::Disabled
    }
    #[doc = "Channel 3 input filter capacity is 2, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity1(&self) -> bool {
        *self == Ch3capflt::Capacity1
    }
    #[doc = "Channel 3 input filter capacity is 4, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity2(&self) -> bool {
        *self == Ch3capflt::Capacity2
    }
    #[doc = "Channel 3 input filter capacity is 8, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity3(&self) -> bool {
        *self == Ch3capflt::Capacity3
    }
    #[doc = "Channel 3 input filter capacity is 6, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn is_capacity4(&self) -> bool {
        *self == Ch3capflt::Capacity4
    }
    #[doc = "Channel 3 input filter capacity is 8, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn is_capacity5(&self) -> bool {
        *self == Ch3capflt::Capacity5
    }
    #[doc = "Channel 3 input filter capacity is 6, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn is_capacity6(&self) -> bool {
        *self == Ch3capflt::Capacity6
    }
    #[doc = "Channel 3 input filter capacity is 8, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn is_capacity7(&self) -> bool {
        *self == Ch3capflt::Capacity7
    }
    #[doc = "Channel 3 input filter capacity is 6, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn is_capacity8(&self) -> bool {
        *self == Ch3capflt::Capacity8
    }
    #[doc = "Channel 3 input filter capacity is 8, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn is_capacity9(&self) -> bool {
        *self == Ch3capflt::Capacity9
    }
    #[doc = "Channel 3 input filter capacity is 5, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn is_capacity10(&self) -> bool {
        *self == Ch3capflt::Capacity10
    }
    #[doc = "Channel 3 input filter capacity is 6, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn is_capacity11(&self) -> bool {
        *self == Ch3capflt::Capacity11
    }
    #[doc = "Channel 3 input filter capacity is 8, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn is_capacity12(&self) -> bool {
        *self == Ch3capflt::Capacity12
    }
    #[doc = "Channel 3 input filter capacity is 5, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn is_capacity13(&self) -> bool {
        *self == Ch3capflt::Capacity13
    }
    #[doc = "Channel 3 input filter capacity is 6, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn is_capacity14(&self) -> bool {
        *self == Ch3capflt::Capacity14
    }
    #[doc = "Channel 3 input filter capacity is 8, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn is_capacity15(&self) -> bool {
        *self == Ch3capflt::Capacity15
    }
}
#[doc = "Field `CH3CAPFLT` writer - Channel 3 input capture filter control"]
pub type Ch3capfltW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ch3capflt, crate::Safe>;
impl<'a, REG> Ch3capfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 3 input filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Disabled)
    }
    #[doc = "Channel 3 input filter capacity is 2, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity1)
    }
    #[doc = "Channel 3 input filter capacity is 4, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity2)
    }
    #[doc = "Channel 3 input filter capacity is 8, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity3)
    }
    #[doc = "Channel 3 input filter capacity is 6, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn capacity4(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity4)
    }
    #[doc = "Channel 3 input filter capacity is 8, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn capacity5(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity5)
    }
    #[doc = "Channel 3 input filter capacity is 6, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn capacity6(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity6)
    }
    #[doc = "Channel 3 input filter capacity is 8, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn capacity7(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity7)
    }
    #[doc = "Channel 3 input filter capacity is 6, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn capacity8(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity8)
    }
    #[doc = "Channel 3 input filter capacity is 8, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn capacity9(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity9)
    }
    #[doc = "Channel 3 input filter capacity is 5, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn capacity10(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity10)
    }
    #[doc = "Channel 3 input filter capacity is 6, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn capacity11(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity11)
    }
    #[doc = "Channel 3 input filter capacity is 8, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn capacity12(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity12)
    }
    #[doc = "Channel 3 input filter capacity is 5, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn capacity13(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity13)
    }
    #[doc = "Channel 3 input filter capacity is 6, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn capacity14(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity14)
    }
    #[doc = "Channel 3 input filter capacity is 8, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn capacity15(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3capflt::Capacity15)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline(always)]
    pub fn ch2ms(&self) -> Ch2msR {
        Ch2msR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 2 input capture prescaler"]
    #[inline(always)]
    pub fn ch2cappsc(&self) -> Ch2cappscR {
        Ch2cappscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Channel 2 input capture filter control"]
    #[inline(always)]
    pub fn ch2capflt(&self) -> Ch2capfltR {
        Ch2capfltR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&self) -> Ch3msR {
        Ch3msR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 3 input capture prescaler"]
    #[inline(always)]
    pub fn ch3cappsc(&self) -> Ch3cappscR {
        Ch3cappscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Channel 3 input capture filter control"]
    #[inline(always)]
    pub fn ch3capflt(&self) -> Ch3capfltR {
        Ch3capfltR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline(always)]
    pub fn ch2ms(&mut self) -> Ch2msW<'_, Chctl1InputSpec> {
        Ch2msW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 2 input capture prescaler"]
    #[inline(always)]
    pub fn ch2cappsc(&mut self) -> Ch2cappscW<'_, Chctl1InputSpec> {
        Ch2cappscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Channel 2 input capture filter control"]
    #[inline(always)]
    pub fn ch2capflt(&mut self) -> Ch2capfltW<'_, Chctl1InputSpec> {
        Ch2capfltW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&mut self) -> Ch3msW<'_, Chctl1InputSpec> {
        Ch3msW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Channel 3 input capture prescaler"]
    #[inline(always)]
    pub fn ch3cappsc(&mut self) -> Ch3cappscW<'_, Chctl1InputSpec> {
        Ch3cappscW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Channel 3 input capture filter control"]
    #[inline(always)]
    pub fn ch3capflt(&mut self) -> Ch3capfltW<'_, Chctl1InputSpec> {
        Ch3capfltW::new(self, 12)
    }
}
#[doc = "Channel control register 1 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl1_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl1_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chctl1InputSpec;
impl crate::RegisterSpec for Chctl1InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl1_input::R`](R) reader structure"]
impl crate::Readable for Chctl1InputSpec {}
#[doc = "`write(|w| ..)` method takes [`chctl1_input::W`](W) writer structure"]
impl crate::Writable for Chctl1InputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTL1_Input to value 0"]
impl crate::Resettable for Chctl1InputSpec {}
