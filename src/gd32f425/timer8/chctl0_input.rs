#[doc = "Register `CHCTL0_Input` reader"]
pub type R = crate::R<Chctl0InputSpec>;
#[doc = "Register `CHCTL0_Input` writer"]
pub type W = crate::W<Chctl0InputSpec>;
#[doc = "Channel 0 mode selection\n\nValue on reset: 0"]
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
#[doc = "Field `CH0MS` reader - Channel 0 mode selection"]
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
#[doc = "Field `CH0MS` writer - Channel 0 mode selection"]
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
#[doc = "Channel 0 input capture prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch0cappsc {
    #[doc = "0: Channel 0 input capture prescaler disabled."]
    Disabled = 0,
    #[doc = "1: Channel 0 The input capture occurs on every 2 channel input edges"]
    Div2 = 1,
    #[doc = "2: Channel 0 The input capture occurs on every 4 channel input edges"]
    Div4 = 2,
    #[doc = "3: Channel 0 The input capture occurs on every 8 channel input edges"]
    Div8 = 3,
}
impl From<Ch0cappsc> for u8 {
    #[inline(always)]
    fn from(variant: Ch0cappsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch0cappsc {
    type Ux = u8;
}
impl crate::IsEnum for Ch0cappsc {}
#[doc = "Field `CH0CAPPSC` reader - Channel 0 input capture prescaler"]
pub type Ch0cappscR = crate::FieldReader<Ch0cappsc>;
impl Ch0cappscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0cappsc {
        match self.bits {
            0 => Ch0cappsc::Disabled,
            1 => Ch0cappsc::Div2,
            2 => Ch0cappsc::Div4,
            3 => Ch0cappsc::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 input capture prescaler disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0cappsc::Disabled
    }
    #[doc = "Channel 0 The input capture occurs on every 2 channel input edges"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ch0cappsc::Div2
    }
    #[doc = "Channel 0 The input capture occurs on every 4 channel input edges"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Ch0cappsc::Div4
    }
    #[doc = "Channel 0 The input capture occurs on every 8 channel input edges"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Ch0cappsc::Div8
    }
}
#[doc = "Field `CH0CAPPSC` writer - Channel 0 input capture prescaler"]
pub type Ch0cappscW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch0cappsc, crate::Safe>;
impl<'a, REG> Ch0cappscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 input capture prescaler disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0cappsc::Disabled)
    }
    #[doc = "Channel 0 The input capture occurs on every 2 channel input edges"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0cappsc::Div2)
    }
    #[doc = "Channel 0 The input capture occurs on every 4 channel input edges"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0cappsc::Div4)
    }
    #[doc = "Channel 0 The input capture occurs on every 8 channel input edges"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0cappsc::Div8)
    }
}
#[doc = "Channel 0 input capture filter control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch0capflt {
    #[doc = "0: Channel 0 input filter disabled."]
    Disabled = 0,
    #[doc = "1: Channel 0 input filter capacity is 2, f_samp is fck_timer"]
    Capacity1 = 1,
    #[doc = "2: Channel 0 input filter capacity is 4, f_samp is fck_timer"]
    Capacity2 = 2,
    #[doc = "3: Channel 0 input filter capacity is 8, f_samp is fck_timer"]
    Capacity3 = 3,
    #[doc = "4: Channel 0 input filter capacity is 6, f_samp is f_dts / 2"]
    Capacity4 = 4,
    #[doc = "5: Channel 0 input filter capacity is 8, f_samp is f_dts / 2"]
    Capacity5 = 5,
    #[doc = "6: Channel 0 input filter capacity is 6, f_samp is f_dts / 4"]
    Capacity6 = 6,
    #[doc = "7: Channel 0 input filter capacity is 8, f_samp is f_dts / 4"]
    Capacity7 = 7,
    #[doc = "8: Channel 0 input filter capacity is 6, f_samp is f_dts / 8"]
    Capacity8 = 8,
    #[doc = "9: Channel 0 input filter capacity is 8, f_samp is f_dts / 8"]
    Capacity9 = 9,
    #[doc = "10: Channel 0 input filter capacity is 5, f_samp is f_dts / 16"]
    Capacity10 = 10,
    #[doc = "11: Channel 0 input filter capacity is 6, f_samp is f_dts / 16"]
    Capacity11 = 11,
    #[doc = "12: Channel 0 input filter capacity is 8, f_samp is f_dts / 16"]
    Capacity12 = 12,
    #[doc = "13: Channel 0 input filter capacity is 5, f_samp is f_dts / 32"]
    Capacity13 = 13,
    #[doc = "14: Channel 0 input filter capacity is 6, f_samp is f_dts / 32"]
    Capacity14 = 14,
    #[doc = "15: Channel 0 input filter capacity is 8, f_samp is f_dts / 32"]
    Capacity15 = 15,
}
impl From<Ch0capflt> for u8 {
    #[inline(always)]
    fn from(variant: Ch0capflt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch0capflt {
    type Ux = u8;
}
impl crate::IsEnum for Ch0capflt {}
#[doc = "Field `CH0CAPFLT` reader - Channel 0 input capture filter control"]
pub type Ch0capfltR = crate::FieldReader<Ch0capflt>;
impl Ch0capfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0capflt {
        match self.bits {
            0 => Ch0capflt::Disabled,
            1 => Ch0capflt::Capacity1,
            2 => Ch0capflt::Capacity2,
            3 => Ch0capflt::Capacity3,
            4 => Ch0capflt::Capacity4,
            5 => Ch0capflt::Capacity5,
            6 => Ch0capflt::Capacity6,
            7 => Ch0capflt::Capacity7,
            8 => Ch0capflt::Capacity8,
            9 => Ch0capflt::Capacity9,
            10 => Ch0capflt::Capacity10,
            11 => Ch0capflt::Capacity11,
            12 => Ch0capflt::Capacity12,
            13 => Ch0capflt::Capacity13,
            14 => Ch0capflt::Capacity14,
            15 => Ch0capflt::Capacity15,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 0 input filter disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0capflt::Disabled
    }
    #[doc = "Channel 0 input filter capacity is 2, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity1(&self) -> bool {
        *self == Ch0capflt::Capacity1
    }
    #[doc = "Channel 0 input filter capacity is 4, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity2(&self) -> bool {
        *self == Ch0capflt::Capacity2
    }
    #[doc = "Channel 0 input filter capacity is 8, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity3(&self) -> bool {
        *self == Ch0capflt::Capacity3
    }
    #[doc = "Channel 0 input filter capacity is 6, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn is_capacity4(&self) -> bool {
        *self == Ch0capflt::Capacity4
    }
    #[doc = "Channel 0 input filter capacity is 8, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn is_capacity5(&self) -> bool {
        *self == Ch0capflt::Capacity5
    }
    #[doc = "Channel 0 input filter capacity is 6, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn is_capacity6(&self) -> bool {
        *self == Ch0capflt::Capacity6
    }
    #[doc = "Channel 0 input filter capacity is 8, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn is_capacity7(&self) -> bool {
        *self == Ch0capflt::Capacity7
    }
    #[doc = "Channel 0 input filter capacity is 6, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn is_capacity8(&self) -> bool {
        *self == Ch0capflt::Capacity8
    }
    #[doc = "Channel 0 input filter capacity is 8, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn is_capacity9(&self) -> bool {
        *self == Ch0capflt::Capacity9
    }
    #[doc = "Channel 0 input filter capacity is 5, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn is_capacity10(&self) -> bool {
        *self == Ch0capflt::Capacity10
    }
    #[doc = "Channel 0 input filter capacity is 6, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn is_capacity11(&self) -> bool {
        *self == Ch0capflt::Capacity11
    }
    #[doc = "Channel 0 input filter capacity is 8, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn is_capacity12(&self) -> bool {
        *self == Ch0capflt::Capacity12
    }
    #[doc = "Channel 0 input filter capacity is 5, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn is_capacity13(&self) -> bool {
        *self == Ch0capflt::Capacity13
    }
    #[doc = "Channel 0 input filter capacity is 6, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn is_capacity14(&self) -> bool {
        *self == Ch0capflt::Capacity14
    }
    #[doc = "Channel 0 input filter capacity is 8, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn is_capacity15(&self) -> bool {
        *self == Ch0capflt::Capacity15
    }
}
#[doc = "Field `CH0CAPFLT` writer - Channel 0 input capture filter control"]
pub type Ch0capfltW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ch0capflt, crate::Safe>;
impl<'a, REG> Ch0capfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 input filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Disabled)
    }
    #[doc = "Channel 0 input filter capacity is 2, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity1)
    }
    #[doc = "Channel 0 input filter capacity is 4, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity2)
    }
    #[doc = "Channel 0 input filter capacity is 8, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity3)
    }
    #[doc = "Channel 0 input filter capacity is 6, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn capacity4(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity4)
    }
    #[doc = "Channel 0 input filter capacity is 8, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn capacity5(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity5)
    }
    #[doc = "Channel 0 input filter capacity is 6, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn capacity6(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity6)
    }
    #[doc = "Channel 0 input filter capacity is 8, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn capacity7(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity7)
    }
    #[doc = "Channel 0 input filter capacity is 6, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn capacity8(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity8)
    }
    #[doc = "Channel 0 input filter capacity is 8, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn capacity9(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity9)
    }
    #[doc = "Channel 0 input filter capacity is 5, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn capacity10(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity10)
    }
    #[doc = "Channel 0 input filter capacity is 6, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn capacity11(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity11)
    }
    #[doc = "Channel 0 input filter capacity is 8, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn capacity12(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity12)
    }
    #[doc = "Channel 0 input filter capacity is 5, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn capacity13(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity13)
    }
    #[doc = "Channel 0 input filter capacity is 6, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn capacity14(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity14)
    }
    #[doc = "Channel 0 input filter capacity is 8, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn capacity15(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0capflt::Capacity15)
    }
}
#[doc = "Channel 1 mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch1ms {
    #[doc = "0: Channel 1 output mode."]
    Output = 0,
    #[doc = "1: Channel 1 is programmed as input mode, IS1 is connected to CI0FE1"]
    InputCi1 = 1,
    #[doc = "2: Channel 1 is programmed as input mode, IS1 is connected to CI1FE1"]
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
    #[doc = "Channel 1 is programmed as input mode, IS1 is connected to CI0FE1"]
    #[inline(always)]
    pub fn is_input_ci1(&self) -> bool {
        *self == Ch1ms::InputCi1
    }
    #[doc = "Channel 1 is programmed as input mode, IS1 is connected to CI1FE1"]
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
    #[doc = "Channel 1 is programmed as input mode, IS1 is connected to CI0FE1"]
    #[inline(always)]
    pub fn input_ci1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1ms::InputCi1)
    }
    #[doc = "Channel 1 is programmed as input mode, IS1 is connected to CI1FE1"]
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
#[doc = "Channel 1 input capture prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch1cappsc {
    #[doc = "0: Channel 1 input capture prescaler disabled."]
    Disabled = 0,
    #[doc = "1: Channel 1 The input capture occurs on every 2 channel input edges"]
    Div2 = 1,
    #[doc = "2: Channel 1 The input capture occurs on every 4 channel input edges"]
    Div4 = 2,
    #[doc = "3: Channel 1 The input capture occurs on every 8 channel input edges"]
    Div8 = 3,
}
impl From<Ch1cappsc> for u8 {
    #[inline(always)]
    fn from(variant: Ch1cappsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch1cappsc {
    type Ux = u8;
}
impl crate::IsEnum for Ch1cappsc {}
#[doc = "Field `CH1CAPPSC` reader - Channel 1 input capture prescaler"]
pub type Ch1cappscR = crate::FieldReader<Ch1cappsc>;
impl Ch1cappscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1cappsc {
        match self.bits {
            0 => Ch1cappsc::Disabled,
            1 => Ch1cappsc::Div2,
            2 => Ch1cappsc::Div4,
            3 => Ch1cappsc::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 1 input capture prescaler disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1cappsc::Disabled
    }
    #[doc = "Channel 1 The input capture occurs on every 2 channel input edges"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ch1cappsc::Div2
    }
    #[doc = "Channel 1 The input capture occurs on every 4 channel input edges"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Ch1cappsc::Div4
    }
    #[doc = "Channel 1 The input capture occurs on every 8 channel input edges"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Ch1cappsc::Div8
    }
}
#[doc = "Field `CH1CAPPSC` writer - Channel 1 input capture prescaler"]
pub type Ch1cappscW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch1cappsc, crate::Safe>;
impl<'a, REG> Ch1cappscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 1 input capture prescaler disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1cappsc::Disabled)
    }
    #[doc = "Channel 1 The input capture occurs on every 2 channel input edges"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1cappsc::Div2)
    }
    #[doc = "Channel 1 The input capture occurs on every 4 channel input edges"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1cappsc::Div4)
    }
    #[doc = "Channel 1 The input capture occurs on every 8 channel input edges"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1cappsc::Div8)
    }
}
#[doc = "Channel 1 input capture filter control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch1capflt {
    #[doc = "0: Channel 1 input filter disabled."]
    Disabled = 0,
    #[doc = "1: Channel 1 input filter capacity is 2, f_samp is fck_timer"]
    Capacity1 = 1,
    #[doc = "2: Channel 1 input filter capacity is 4, f_samp is fck_timer"]
    Capacity2 = 2,
    #[doc = "3: Channel 1 input filter capacity is 8, f_samp is fck_timer"]
    Capacity3 = 3,
    #[doc = "4: Channel 1 input filter capacity is 6, f_samp is f_dts / 2"]
    Capacity4 = 4,
    #[doc = "5: Channel 1 input filter capacity is 8, f_samp is f_dts / 2"]
    Capacity5 = 5,
    #[doc = "6: Channel 1 input filter capacity is 6, f_samp is f_dts / 4"]
    Capacity6 = 6,
    #[doc = "7: Channel 1 input filter capacity is 8, f_samp is f_dts / 4"]
    Capacity7 = 7,
    #[doc = "8: Channel 1 input filter capacity is 6, f_samp is f_dts / 8"]
    Capacity8 = 8,
    #[doc = "9: Channel 1 input filter capacity is 8, f_samp is f_dts / 8"]
    Capacity9 = 9,
    #[doc = "10: Channel 1 input filter capacity is 5, f_samp is f_dts / 16"]
    Capacity10 = 10,
    #[doc = "11: Channel 1 input filter capacity is 6, f_samp is f_dts / 16"]
    Capacity11 = 11,
    #[doc = "12: Channel 1 input filter capacity is 8, f_samp is f_dts / 16"]
    Capacity12 = 12,
    #[doc = "13: Channel 1 input filter capacity is 5, f_samp is f_dts / 32"]
    Capacity13 = 13,
    #[doc = "14: Channel 1 input filter capacity is 6, f_samp is f_dts / 32"]
    Capacity14 = 14,
    #[doc = "15: Channel 1 input filter capacity is 8, f_samp is f_dts / 32"]
    Capacity15 = 15,
}
impl From<Ch1capflt> for u8 {
    #[inline(always)]
    fn from(variant: Ch1capflt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch1capflt {
    type Ux = u8;
}
impl crate::IsEnum for Ch1capflt {}
#[doc = "Field `CH1CAPFLT` reader - Channel 1 input capture filter control"]
pub type Ch1capfltR = crate::FieldReader<Ch1capflt>;
impl Ch1capfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1capflt {
        match self.bits {
            0 => Ch1capflt::Disabled,
            1 => Ch1capflt::Capacity1,
            2 => Ch1capflt::Capacity2,
            3 => Ch1capflt::Capacity3,
            4 => Ch1capflt::Capacity4,
            5 => Ch1capflt::Capacity5,
            6 => Ch1capflt::Capacity6,
            7 => Ch1capflt::Capacity7,
            8 => Ch1capflt::Capacity8,
            9 => Ch1capflt::Capacity9,
            10 => Ch1capflt::Capacity10,
            11 => Ch1capflt::Capacity11,
            12 => Ch1capflt::Capacity12,
            13 => Ch1capflt::Capacity13,
            14 => Ch1capflt::Capacity14,
            15 => Ch1capflt::Capacity15,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel 1 input filter disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1capflt::Disabled
    }
    #[doc = "Channel 1 input filter capacity is 2, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity1(&self) -> bool {
        *self == Ch1capflt::Capacity1
    }
    #[doc = "Channel 1 input filter capacity is 4, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity2(&self) -> bool {
        *self == Ch1capflt::Capacity2
    }
    #[doc = "Channel 1 input filter capacity is 8, f_samp is fck_timer"]
    #[inline(always)]
    pub fn is_capacity3(&self) -> bool {
        *self == Ch1capflt::Capacity3
    }
    #[doc = "Channel 1 input filter capacity is 6, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn is_capacity4(&self) -> bool {
        *self == Ch1capflt::Capacity4
    }
    #[doc = "Channel 1 input filter capacity is 8, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn is_capacity5(&self) -> bool {
        *self == Ch1capflt::Capacity5
    }
    #[doc = "Channel 1 input filter capacity is 6, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn is_capacity6(&self) -> bool {
        *self == Ch1capflt::Capacity6
    }
    #[doc = "Channel 1 input filter capacity is 8, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn is_capacity7(&self) -> bool {
        *self == Ch1capflt::Capacity7
    }
    #[doc = "Channel 1 input filter capacity is 6, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn is_capacity8(&self) -> bool {
        *self == Ch1capflt::Capacity8
    }
    #[doc = "Channel 1 input filter capacity is 8, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn is_capacity9(&self) -> bool {
        *self == Ch1capflt::Capacity9
    }
    #[doc = "Channel 1 input filter capacity is 5, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn is_capacity10(&self) -> bool {
        *self == Ch1capflt::Capacity10
    }
    #[doc = "Channel 1 input filter capacity is 6, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn is_capacity11(&self) -> bool {
        *self == Ch1capflt::Capacity11
    }
    #[doc = "Channel 1 input filter capacity is 8, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn is_capacity12(&self) -> bool {
        *self == Ch1capflt::Capacity12
    }
    #[doc = "Channel 1 input filter capacity is 5, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn is_capacity13(&self) -> bool {
        *self == Ch1capflt::Capacity13
    }
    #[doc = "Channel 1 input filter capacity is 6, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn is_capacity14(&self) -> bool {
        *self == Ch1capflt::Capacity14
    }
    #[doc = "Channel 1 input filter capacity is 8, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn is_capacity15(&self) -> bool {
        *self == Ch1capflt::Capacity15
    }
}
#[doc = "Field `CH1CAPFLT` writer - Channel 1 input capture filter control"]
pub type Ch1capfltW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ch1capflt, crate::Safe>;
impl<'a, REG> Ch1capfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 1 input filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Disabled)
    }
    #[doc = "Channel 1 input filter capacity is 2, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity1)
    }
    #[doc = "Channel 1 input filter capacity is 4, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity2)
    }
    #[doc = "Channel 1 input filter capacity is 8, f_samp is fck_timer"]
    #[inline(always)]
    pub fn capacity3(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity3)
    }
    #[doc = "Channel 1 input filter capacity is 6, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn capacity4(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity4)
    }
    #[doc = "Channel 1 input filter capacity is 8, f_samp is f_dts / 2"]
    #[inline(always)]
    pub fn capacity5(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity5)
    }
    #[doc = "Channel 1 input filter capacity is 6, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn capacity6(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity6)
    }
    #[doc = "Channel 1 input filter capacity is 8, f_samp is f_dts / 4"]
    #[inline(always)]
    pub fn capacity7(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity7)
    }
    #[doc = "Channel 1 input filter capacity is 6, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn capacity8(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity8)
    }
    #[doc = "Channel 1 input filter capacity is 8, f_samp is f_dts / 8"]
    #[inline(always)]
    pub fn capacity9(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity9)
    }
    #[doc = "Channel 1 input filter capacity is 5, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn capacity10(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity10)
    }
    #[doc = "Channel 1 input filter capacity is 6, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn capacity11(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity11)
    }
    #[doc = "Channel 1 input filter capacity is 8, f_samp is f_dts / 16"]
    #[inline(always)]
    pub fn capacity12(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity12)
    }
    #[doc = "Channel 1 input filter capacity is 5, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn capacity13(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity13)
    }
    #[doc = "Channel 1 input filter capacity is 6, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn capacity14(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity14)
    }
    #[doc = "Channel 1 input filter capacity is 8, f_samp is f_dts / 32"]
    #[inline(always)]
    pub fn capacity15(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1capflt::Capacity15)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> Ch0msR {
        Ch0msR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&self) -> Ch0cappscR {
        Ch0cappscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline(always)]
    pub fn ch0capflt(&self) -> Ch0capfltR {
        Ch0capfltR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    pub fn ch1ms(&self) -> Ch1msR {
        Ch1msR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 1 input capture prescaler"]
    #[inline(always)]
    pub fn ch1cappsc(&self) -> Ch1cappscR {
        Ch1cappscR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Channel 1 input capture filter control"]
    #[inline(always)]
    pub fn ch1capflt(&self) -> Ch1capfltR {
        Ch1capfltR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    pub fn ch0ms(&mut self) -> Ch0msW<'_, Chctl0InputSpec> {
        Ch0msW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&mut self) -> Ch0cappscW<'_, Chctl0InputSpec> {
        Ch0cappscW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline(always)]
    pub fn ch0capflt(&mut self) -> Ch0capfltW<'_, Chctl0InputSpec> {
        Ch0capfltW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    pub fn ch1ms(&mut self) -> Ch1msW<'_, Chctl0InputSpec> {
        Ch1msW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Channel 1 input capture prescaler"]
    #[inline(always)]
    pub fn ch1cappsc(&mut self) -> Ch1cappscW<'_, Chctl0InputSpec> {
        Ch1cappscW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Channel 1 input capture filter control"]
    #[inline(always)]
    pub fn ch1capflt(&mut self) -> Ch1capfltW<'_, Chctl0InputSpec> {
        Ch1capfltW::new(self, 12)
    }
}
#[doc = "Channel control register 0 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl0_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl0_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chctl0InputSpec;
impl crate::RegisterSpec for Chctl0InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chctl0_input::R`](R) reader structure"]
impl crate::Readable for Chctl0InputSpec {}
#[doc = "`write(|w| ..)` method takes [`chctl0_input::W`](W) writer structure"]
impl crate::Writable for Chctl0InputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHCTL0_Input to value 0"]
impl crate::Resettable for Chctl0InputSpec {}
