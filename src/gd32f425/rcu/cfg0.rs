#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "System clock switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scs {
    #[doc = "0: Select IRC16M as the system clock source."]
    Irc16m = 0,
    #[doc = "1: Select the HXTAL as the system clock source."]
    Hxtal = 1,
    #[doc = "2: Select the PLLP as the system clock source."]
    Pllp = 2,
}
impl From<Scs> for u8 {
    #[inline(always)]
    fn from(variant: Scs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scs {
    type Ux = u8;
}
impl crate::IsEnum for Scs {}
#[doc = "Field `SCS` reader - System clock switch"]
pub type ScsR = crate::FieldReader<Scs>;
impl ScsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Scs> {
        match self.bits {
            0 => Some(Scs::Irc16m),
            1 => Some(Scs::Hxtal),
            2 => Some(Scs::Pllp),
            _ => None,
        }
    }
    #[doc = "Select IRC16M as the system clock source."]
    #[inline(always)]
    pub fn is_irc16m(&self) -> bool {
        *self == Scs::Irc16m
    }
    #[doc = "Select the HXTAL as the system clock source."]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == Scs::Hxtal
    }
    #[doc = "Select the PLLP as the system clock source."]
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == Scs::Pllp
    }
}
#[doc = "Field `SCS` writer - System clock switch"]
pub type ScsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Scs>;
impl<'a, REG> ScsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select IRC16M as the system clock source."]
    #[inline(always)]
    pub fn irc16m(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Irc16m)
    }
    #[doc = "Select the HXTAL as the system clock source."]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Hxtal)
    }
    #[doc = "Select the PLLP as the system clock source."]
    #[inline(always)]
    pub fn pllp(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Pllp)
    }
}
#[doc = "System clock switch status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scss {
    #[doc = "0: IRC16M selected as the system clock"]
    Irc16m = 0,
    #[doc = "1: HXTAL selected as the system clock."]
    Hxtal = 1,
    #[doc = "2: Select CK_PLLP as the system clock."]
    Ppllp = 2,
}
impl From<Scss> for u8 {
    #[inline(always)]
    fn from(variant: Scss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scss {
    type Ux = u8;
}
impl crate::IsEnum for Scss {}
#[doc = "Field `SCSS` reader - System clock switch status"]
pub type ScssR = crate::FieldReader<Scss>;
impl ScssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Scss> {
        match self.bits {
            0 => Some(Scss::Irc16m),
            1 => Some(Scss::Hxtal),
            2 => Some(Scss::Ppllp),
            _ => None,
        }
    }
    #[doc = "IRC16M selected as the system clock"]
    #[inline(always)]
    pub fn is_irc16m(&self) -> bool {
        *self == Scss::Irc16m
    }
    #[doc = "HXTAL selected as the system clock."]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == Scss::Hxtal
    }
    #[doc = "Select CK_PLLP as the system clock."]
    #[inline(always)]
    pub fn is_ppllp(&self) -> bool {
        *self == Scss::Ppllp
    }
}
#[doc = "AHB prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ahbpsc {
    #[doc = "0: CK_SYS"]
    Div1 = 0,
    #[doc = "8: CK_SYS divided by 2"]
    Div2 = 8,
    #[doc = "9: CK_SYS divided by 4"]
    Div4 = 9,
    #[doc = "10: CK_SYS divided by 8"]
    Div8 = 10,
    #[doc = "11: CK_SYS divided by 16"]
    Div16 = 11,
    #[doc = "12: CK_SYS divided by 64"]
    Div64 = 12,
    #[doc = "13: CK_SYS divided by 128"]
    Div128 = 13,
    #[doc = "14: CK_SYS divided by 256"]
    Div256 = 14,
    #[doc = "15: CK_SYS divided by 512"]
    Div512 = 15,
}
impl From<Ahbpsc> for u8 {
    #[inline(always)]
    fn from(variant: Ahbpsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ahbpsc {
    type Ux = u8;
}
impl crate::IsEnum for Ahbpsc {}
#[doc = "Field `AHBPSC` reader - AHB prescaler selection"]
pub type AhbpscR = crate::FieldReader<Ahbpsc>;
impl AhbpscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ahbpsc> {
        match self.bits {
            0 => Some(Ahbpsc::Div1),
            8 => Some(Ahbpsc::Div2),
            9 => Some(Ahbpsc::Div4),
            10 => Some(Ahbpsc::Div8),
            11 => Some(Ahbpsc::Div16),
            12 => Some(Ahbpsc::Div64),
            13 => Some(Ahbpsc::Div128),
            14 => Some(Ahbpsc::Div256),
            15 => Some(Ahbpsc::Div512),
            _ => None,
        }
    }
    #[doc = "CK_SYS"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Ahbpsc::Div1
    }
    #[doc = "CK_SYS divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ahbpsc::Div2
    }
    #[doc = "CK_SYS divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Ahbpsc::Div4
    }
    #[doc = "CK_SYS divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Ahbpsc::Div8
    }
    #[doc = "CK_SYS divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Ahbpsc::Div16
    }
    #[doc = "CK_SYS divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Ahbpsc::Div64
    }
    #[doc = "CK_SYS divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Ahbpsc::Div128
    }
    #[doc = "CK_SYS divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Ahbpsc::Div256
    }
    #[doc = "CK_SYS divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Ahbpsc::Div512
    }
}
#[doc = "Field `AHBPSC` writer - AHB prescaler selection"]
pub type AhbpscW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ahbpsc>;
impl<'a, REG> AhbpscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CK_SYS"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div1)
    }
    #[doc = "CK_SYS divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div2)
    }
    #[doc = "CK_SYS divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div4)
    }
    #[doc = "CK_SYS divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div8)
    }
    #[doc = "CK_SYS divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div16)
    }
    #[doc = "CK_SYS divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div64)
    }
    #[doc = "CK_SYS divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div128)
    }
    #[doc = "CK_SYS divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div256)
    }
    #[doc = "CK_SYS divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbpsc::Div512)
    }
}
#[doc = "APB1 prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Apb1psc {
    #[doc = "0: CK_AHB selected"]
    Div1 = 0,
    #[doc = "4: CK_AHB/2 selected"]
    Div2 = 4,
    #[doc = "5: CK_AHB/4 selected"]
    Div4 = 5,
    #[doc = "6: CK_AHB/8 selected"]
    Div8 = 6,
    #[doc = "7: CK_AHB/16 selected"]
    Div16 = 7,
}
impl From<Apb1psc> for u8 {
    #[inline(always)]
    fn from(variant: Apb1psc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Apb1psc {
    type Ux = u8;
}
impl crate::IsEnum for Apb1psc {}
#[doc = "Field `APB1PSC` reader - APB1 prescaler selection"]
pub type Apb1pscR = crate::FieldReader<Apb1psc>;
impl Apb1pscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Apb1psc> {
        match self.bits {
            0 => Some(Apb1psc::Div1),
            4 => Some(Apb1psc::Div2),
            5 => Some(Apb1psc::Div4),
            6 => Some(Apb1psc::Div8),
            7 => Some(Apb1psc::Div16),
            _ => None,
        }
    }
    #[doc = "CK_AHB selected"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Apb1psc::Div1
    }
    #[doc = "CK_AHB/2 selected"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Apb1psc::Div2
    }
    #[doc = "CK_AHB/4 selected"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Apb1psc::Div4
    }
    #[doc = "CK_AHB/8 selected"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Apb1psc::Div8
    }
    #[doc = "CK_AHB/16 selected"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Apb1psc::Div16
    }
}
#[doc = "Field `APB1PSC` writer - APB1 prescaler selection"]
pub type Apb1pscW<'a, REG> = crate::FieldWriter<'a, REG, 3, Apb1psc>;
impl<'a, REG> Apb1pscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CK_AHB selected"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Apb1psc::Div1)
    }
    #[doc = "CK_AHB/2 selected"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Apb1psc::Div2)
    }
    #[doc = "CK_AHB/4 selected"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Apb1psc::Div4)
    }
    #[doc = "CK_AHB/8 selected"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Apb1psc::Div8)
    }
    #[doc = "CK_AHB/16 selected"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Apb1psc::Div16)
    }
}
#[doc = "Field `APB2PSC` reader - APB2 prescaler selection"]
pub use Apb1pscR as Apb2pscR;
#[doc = "Field `APB2PSC` writer - APB2 prescaler selection"]
pub use Apb1pscW as Apb2pscW;
#[doc = "RTC clock divider factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcdiv {
    #[doc = "0: No clock selected for RTC"]
    NoClk = 0,
    #[doc = "2: CK_HXTAL is divided by 2"]
    Div2 = 2,
    #[doc = "3: CK_HXTAL is divided by 3"]
    Div3 = 3,
    #[doc = "4: CK_HXTAL is divided by 4"]
    Div4 = 4,
    #[doc = "5: CK_HXTAL is divided by 5"]
    Div5 = 5,
    #[doc = "6: CK_HXTAL is divided by 6"]
    Div6 = 6,
    #[doc = "7: CK_HXTAL is divided by 7"]
    Div7 = 7,
    #[doc = "8: CK_HXTAL is divided by 8"]
    Div8 = 8,
    #[doc = "9: CK_HXTAL is divided by 9"]
    Div9 = 9,
    #[doc = "10: CK_HXTAL is divided by 10"]
    Div10 = 10,
    #[doc = "11: CK_HXTAL is divided by 11"]
    Div11 = 11,
    #[doc = "12: CK_HXTAL is divided by 12"]
    Div12 = 12,
    #[doc = "13: CK_HXTAL is divided by 13"]
    Div13 = 13,
    #[doc = "14: CK_HXTAL is divided by 14"]
    Div14 = 14,
    #[doc = "15: CK_HXTAL is divided by 15"]
    Div15 = 15,
    #[doc = "16: CK_HXTAL is divided by 16"]
    Div16 = 16,
    #[doc = "17: CK_HXTAL is divided by 17"]
    Div17 = 17,
    #[doc = "18: CK_HXTAL is divided by 18"]
    Div18 = 18,
    #[doc = "19: CK_HXTAL is divided by 19"]
    Div19 = 19,
    #[doc = "20: CK_HXTAL is divided by 20"]
    Div20 = 20,
    #[doc = "21: CK_HXTAL is divided by 21"]
    Div21 = 21,
    #[doc = "22: CK_HXTAL is divided by 22"]
    Div22 = 22,
    #[doc = "23: CK_HXTAL is divided by 23"]
    Div23 = 23,
    #[doc = "24: CK_HXTAL is divided by 24"]
    Div24 = 24,
    #[doc = "25: CK_HXTAL is divided by 25"]
    Div25 = 25,
    #[doc = "26: CK_HXTAL is divided by 26"]
    Div26 = 26,
    #[doc = "27: CK_HXTAL is divided by 27"]
    Div27 = 27,
    #[doc = "28: CK_HXTAL is divided by 28"]
    Div28 = 28,
    #[doc = "29: CK_HXTAL is divided by 29"]
    Div29 = 29,
    #[doc = "30: CK_HXTAL is divided by 30"]
    Div30 = 30,
    #[doc = "31: CK_HXTAL is divided by 31"]
    Div31 = 31,
}
impl From<Rtcdiv> for u8 {
    #[inline(always)]
    fn from(variant: Rtcdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcdiv {
    type Ux = u8;
}
impl crate::IsEnum for Rtcdiv {}
#[doc = "Field `RTCDIV` reader - RTC clock divider factor"]
pub type RtcdivR = crate::FieldReader<Rtcdiv>;
impl RtcdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rtcdiv> {
        match self.bits {
            0 => Some(Rtcdiv::NoClk),
            2 => Some(Rtcdiv::Div2),
            3 => Some(Rtcdiv::Div3),
            4 => Some(Rtcdiv::Div4),
            5 => Some(Rtcdiv::Div5),
            6 => Some(Rtcdiv::Div6),
            7 => Some(Rtcdiv::Div7),
            8 => Some(Rtcdiv::Div8),
            9 => Some(Rtcdiv::Div9),
            10 => Some(Rtcdiv::Div10),
            11 => Some(Rtcdiv::Div11),
            12 => Some(Rtcdiv::Div12),
            13 => Some(Rtcdiv::Div13),
            14 => Some(Rtcdiv::Div14),
            15 => Some(Rtcdiv::Div15),
            16 => Some(Rtcdiv::Div16),
            17 => Some(Rtcdiv::Div17),
            18 => Some(Rtcdiv::Div18),
            19 => Some(Rtcdiv::Div19),
            20 => Some(Rtcdiv::Div20),
            21 => Some(Rtcdiv::Div21),
            22 => Some(Rtcdiv::Div22),
            23 => Some(Rtcdiv::Div23),
            24 => Some(Rtcdiv::Div24),
            25 => Some(Rtcdiv::Div25),
            26 => Some(Rtcdiv::Div26),
            27 => Some(Rtcdiv::Div27),
            28 => Some(Rtcdiv::Div28),
            29 => Some(Rtcdiv::Div29),
            30 => Some(Rtcdiv::Div30),
            31 => Some(Rtcdiv::Div31),
            _ => None,
        }
    }
    #[doc = "No clock selected for RTC"]
    #[inline(always)]
    pub fn is_no_clk(&self) -> bool {
        *self == Rtcdiv::NoClk
    }
    #[doc = "CK_HXTAL is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Rtcdiv::Div2
    }
    #[doc = "CK_HXTAL is divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Rtcdiv::Div3
    }
    #[doc = "CK_HXTAL is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Rtcdiv::Div4
    }
    #[doc = "CK_HXTAL is divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Rtcdiv::Div5
    }
    #[doc = "CK_HXTAL is divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Rtcdiv::Div6
    }
    #[doc = "CK_HXTAL is divided by 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Rtcdiv::Div7
    }
    #[doc = "CK_HXTAL is divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Rtcdiv::Div8
    }
    #[doc = "CK_HXTAL is divided by 9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == Rtcdiv::Div9
    }
    #[doc = "CK_HXTAL is divided by 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Rtcdiv::Div10
    }
    #[doc = "CK_HXTAL is divided by 11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == Rtcdiv::Div11
    }
    #[doc = "CK_HXTAL is divided by 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Rtcdiv::Div12
    }
    #[doc = "CK_HXTAL is divided by 13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == Rtcdiv::Div13
    }
    #[doc = "CK_HXTAL is divided by 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Rtcdiv::Div14
    }
    #[doc = "CK_HXTAL is divided by 15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == Rtcdiv::Div15
    }
    #[doc = "CK_HXTAL is divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Rtcdiv::Div16
    }
    #[doc = "CK_HXTAL is divided by 17"]
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == Rtcdiv::Div17
    }
    #[doc = "CK_HXTAL is divided by 18"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == Rtcdiv::Div18
    }
    #[doc = "CK_HXTAL is divided by 19"]
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == Rtcdiv::Div19
    }
    #[doc = "CK_HXTAL is divided by 20"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == Rtcdiv::Div20
    }
    #[doc = "CK_HXTAL is divided by 21"]
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == Rtcdiv::Div21
    }
    #[doc = "CK_HXTAL is divided by 22"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == Rtcdiv::Div22
    }
    #[doc = "CK_HXTAL is divided by 23"]
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == Rtcdiv::Div23
    }
    #[doc = "CK_HXTAL is divided by 24"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == Rtcdiv::Div24
    }
    #[doc = "CK_HXTAL is divided by 25"]
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == Rtcdiv::Div25
    }
    #[doc = "CK_HXTAL is divided by 26"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == Rtcdiv::Div26
    }
    #[doc = "CK_HXTAL is divided by 27"]
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == Rtcdiv::Div27
    }
    #[doc = "CK_HXTAL is divided by 28"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == Rtcdiv::Div28
    }
    #[doc = "CK_HXTAL is divided by 29"]
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == Rtcdiv::Div29
    }
    #[doc = "CK_HXTAL is divided by 30"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == Rtcdiv::Div30
    }
    #[doc = "CK_HXTAL is divided by 31"]
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == Rtcdiv::Div31
    }
}
#[doc = "Field `RTCDIV` writer - RTC clock divider factor"]
pub type RtcdivW<'a, REG> = crate::FieldWriter<'a, REG, 5, Rtcdiv>;
impl<'a, REG> RtcdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock selected for RTC"]
    #[inline(always)]
    pub fn no_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::NoClk)
    }
    #[doc = "CK_HXTAL is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div2)
    }
    #[doc = "CK_HXTAL is divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div3)
    }
    #[doc = "CK_HXTAL is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div4)
    }
    #[doc = "CK_HXTAL is divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div5)
    }
    #[doc = "CK_HXTAL is divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div6)
    }
    #[doc = "CK_HXTAL is divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div7)
    }
    #[doc = "CK_HXTAL is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div8)
    }
    #[doc = "CK_HXTAL is divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div9)
    }
    #[doc = "CK_HXTAL is divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div10)
    }
    #[doc = "CK_HXTAL is divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div11)
    }
    #[doc = "CK_HXTAL is divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div12)
    }
    #[doc = "CK_HXTAL is divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div13)
    }
    #[doc = "CK_HXTAL is divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div14)
    }
    #[doc = "CK_HXTAL is divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div15)
    }
    #[doc = "CK_HXTAL is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div16)
    }
    #[doc = "CK_HXTAL is divided by 17"]
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div17)
    }
    #[doc = "CK_HXTAL is divided by 18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div18)
    }
    #[doc = "CK_HXTAL is divided by 19"]
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div19)
    }
    #[doc = "CK_HXTAL is divided by 20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div20)
    }
    #[doc = "CK_HXTAL is divided by 21"]
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div21)
    }
    #[doc = "CK_HXTAL is divided by 22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div22)
    }
    #[doc = "CK_HXTAL is divided by 23"]
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div23)
    }
    #[doc = "CK_HXTAL is divided by 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div24)
    }
    #[doc = "CK_HXTAL is divided by 25"]
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div25)
    }
    #[doc = "CK_HXTAL is divided by 26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div26)
    }
    #[doc = "CK_HXTAL is divided by 27"]
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div27)
    }
    #[doc = "CK_HXTAL is divided by 28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div28)
    }
    #[doc = "CK_HXTAL is divided by 29"]
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div29)
    }
    #[doc = "CK_HXTAL is divided by 30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div30)
    }
    #[doc = "CK_HXTAL is divided by 31"]
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcdiv::Div31)
    }
}
#[doc = "CKOUT0 Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckout0sel {
    #[doc = "0: Internal 16 MHz RC oscillator clock selected"]
    Irc16m = 0,
    #[doc = "1: External low speed oscillator clock selected"]
    Lxtal = 1,
    #[doc = "2: External high speed oscillator clock selected"]
    Hxtal = 2,
    #[doc = "3: PLL clock selected"]
    CkPllp = 3,
}
impl From<Ckout0sel> for u8 {
    #[inline(always)]
    fn from(variant: Ckout0sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckout0sel {
    type Ux = u8;
}
impl crate::IsEnum for Ckout0sel {}
#[doc = "Field `CKOUT0SEL` reader - CKOUT0 Clock Source Selection"]
pub type Ckout0selR = crate::FieldReader<Ckout0sel>;
impl Ckout0selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckout0sel {
        match self.bits {
            0 => Ckout0sel::Irc16m,
            1 => Ckout0sel::Lxtal,
            2 => Ckout0sel::Hxtal,
            3 => Ckout0sel::CkPllp,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal 16 MHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn is_irc16m(&self) -> bool {
        *self == Ckout0sel::Irc16m
    }
    #[doc = "External low speed oscillator clock selected"]
    #[inline(always)]
    pub fn is_lxtal(&self) -> bool {
        *self == Ckout0sel::Lxtal
    }
    #[doc = "External high speed oscillator clock selected"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == Ckout0sel::Hxtal
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn is_ck_pllp(&self) -> bool {
        *self == Ckout0sel::CkPllp
    }
}
#[doc = "Field `CKOUT0SEL` writer - CKOUT0 Clock Source Selection"]
pub type Ckout0selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckout0sel, crate::Safe>;
impl<'a, REG> Ckout0selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 16 MHz RC oscillator clock selected"]
    #[inline(always)]
    pub fn irc16m(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout0sel::Irc16m)
    }
    #[doc = "External low speed oscillator clock selected"]
    #[inline(always)]
    pub fn lxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout0sel::Lxtal)
    }
    #[doc = "External high speed oscillator clock selected"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout0sel::Hxtal)
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn ck_pllp(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout0sel::CkPllp)
    }
}
#[doc = "I2S Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2ssel {
    #[doc = "0: PLLI2S output clock selected as the I2S clock source."]
    CkPlli2s = 0,
    #[doc = "1: External I2S_CKIN pin selected as I2S source clock."]
    CkI2s = 1,
}
impl From<I2ssel> for bool {
    #[inline(always)]
    fn from(variant: I2ssel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SSEL` reader - I2S Clock Source Selection"]
pub type I2sselR = crate::BitReader<I2ssel>;
impl I2sselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2ssel {
        match self.bits {
            false => I2ssel::CkPlli2s,
            true => I2ssel::CkI2s,
        }
    }
    #[doc = "PLLI2S output clock selected as the I2S clock source."]
    #[inline(always)]
    pub fn is_ck_plli2s(&self) -> bool {
        *self == I2ssel::CkPlli2s
    }
    #[doc = "External I2S_CKIN pin selected as I2S source clock."]
    #[inline(always)]
    pub fn is_ck_i2s(&self) -> bool {
        *self == I2ssel::CkI2s
    }
}
#[doc = "Field `I2SSEL` writer - I2S Clock Source Selection"]
pub type I2sselW<'a, REG> = crate::BitWriter<'a, REG, I2ssel>;
impl<'a, REG> I2sselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLLI2S output clock selected as the I2S clock source."]
    #[inline(always)]
    pub fn ck_plli2s(self) -> &'a mut crate::W<REG> {
        self.variant(I2ssel::CkPlli2s)
    }
    #[doc = "External I2S_CKIN pin selected as I2S source clock."]
    #[inline(always)]
    pub fn ck_i2s(self) -> &'a mut crate::W<REG> {
        self.variant(I2ssel::CkI2s)
    }
}
#[doc = "The CK_OUT0 divider which the CK_OUT0 frequency can be reduced\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckout0div {
    #[doc = "0: CK_OUT0 is divided by 1"]
    Div1 = 0,
    #[doc = "4: CK_OUT0 is divided by 2"]
    Div2 = 4,
    #[doc = "5: CK_OUT0 is divided by 3"]
    Div3 = 5,
    #[doc = "6: CK_OUT0 is divided by 4"]
    Div4 = 6,
    #[doc = "7: CK_OUT0 is divided by 5"]
    Div5 = 7,
}
impl From<Ckout0div> for u8 {
    #[inline(always)]
    fn from(variant: Ckout0div) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckout0div {
    type Ux = u8;
}
impl crate::IsEnum for Ckout0div {}
#[doc = "Field `CKOUT0DIV` reader - The CK_OUT0 divider which the CK_OUT0 frequency can be reduced"]
pub type Ckout0divR = crate::FieldReader<Ckout0div>;
impl Ckout0divR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckout0div> {
        match self.bits {
            0 => Some(Ckout0div::Div1),
            4 => Some(Ckout0div::Div2),
            5 => Some(Ckout0div::Div3),
            6 => Some(Ckout0div::Div4),
            7 => Some(Ckout0div::Div5),
            _ => None,
        }
    }
    #[doc = "CK_OUT0 is divided by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Ckout0div::Div1
    }
    #[doc = "CK_OUT0 is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ckout0div::Div2
    }
    #[doc = "CK_OUT0 is divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Ckout0div::Div3
    }
    #[doc = "CK_OUT0 is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Ckout0div::Div4
    }
    #[doc = "CK_OUT0 is divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Ckout0div::Div5
    }
}
#[doc = "Field `CKOUT0DIV` writer - The CK_OUT0 divider which the CK_OUT0 frequency can be reduced"]
pub type Ckout0divW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ckout0div>;
impl<'a, REG> Ckout0divW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CK_OUT0 is divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout0div::Div1)
    }
    #[doc = "CK_OUT0 is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout0div::Div2)
    }
    #[doc = "CK_OUT0 is divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout0div::Div3)
    }
    #[doc = "CK_OUT0 is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout0div::Div4)
    }
    #[doc = "CK_OUT0 is divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout0div::Div5)
    }
}
#[doc = "The CK_OUT1 divider which the CK_OUT1 frequency can be reduced\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckout1div {
    #[doc = "0: CK_OUT1 is divided by 1"]
    Div1 = 0,
    #[doc = "4: CK_OUT1 is divided by 2"]
    Div2 = 4,
    #[doc = "5: CK_OUT1 is divided by 3"]
    Div3 = 5,
    #[doc = "6: CK_OUT1 is divided by 4"]
    Div4 = 6,
    #[doc = "7: CK_OUT1 is divided by 5"]
    Div5 = 7,
}
impl From<Ckout1div> for u8 {
    #[inline(always)]
    fn from(variant: Ckout1div) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckout1div {
    type Ux = u8;
}
impl crate::IsEnum for Ckout1div {}
#[doc = "Field `CKOUT1DIV` reader - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced"]
pub type Ckout1divR = crate::FieldReader<Ckout1div>;
impl Ckout1divR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckout1div> {
        match self.bits {
            0 => Some(Ckout1div::Div1),
            4 => Some(Ckout1div::Div2),
            5 => Some(Ckout1div::Div3),
            6 => Some(Ckout1div::Div4),
            7 => Some(Ckout1div::Div5),
            _ => None,
        }
    }
    #[doc = "CK_OUT1 is divided by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Ckout1div::Div1
    }
    #[doc = "CK_OUT1 is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ckout1div::Div2
    }
    #[doc = "CK_OUT1 is divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Ckout1div::Div3
    }
    #[doc = "CK_OUT1 is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Ckout1div::Div4
    }
    #[doc = "CK_OUT1 is divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Ckout1div::Div5
    }
}
#[doc = "Field `CKOUT1DIV` writer - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced"]
pub type Ckout1divW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ckout1div>;
impl<'a, REG> Ckout1divW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CK_OUT1 is divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1div::Div1)
    }
    #[doc = "CK_OUT1 is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1div::Div2)
    }
    #[doc = "CK_OUT1 is divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1div::Div3)
    }
    #[doc = "CK_OUT1 is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1div::Div4)
    }
    #[doc = "CK_OUT1 is divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1div::Div5)
    }
}
#[doc = "CKOUT1 Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckout1sel {
    #[doc = "0: System clock selected"]
    Sysclk = 0,
    #[doc = "1: PLL I2S clock selected"]
    Plli2sr = 1,
    #[doc = "2: External high speed oscillator clock selected"]
    Hxtal = 2,
    #[doc = "3: PLL clock selected"]
    Pllp = 3,
}
impl From<Ckout1sel> for u8 {
    #[inline(always)]
    fn from(variant: Ckout1sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckout1sel {
    type Ux = u8;
}
impl crate::IsEnum for Ckout1sel {}
#[doc = "Field `CKOUT1SEL` reader - CKOUT1 Clock Source Selection"]
pub type Ckout1selR = crate::FieldReader<Ckout1sel>;
impl Ckout1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckout1sel {
        match self.bits {
            0 => Ckout1sel::Sysclk,
            1 => Ckout1sel::Plli2sr,
            2 => Ckout1sel::Hxtal,
            3 => Ckout1sel::Pllp,
            _ => unreachable!(),
        }
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == Ckout1sel::Sysclk
    }
    #[doc = "PLL I2S clock selected"]
    #[inline(always)]
    pub fn is_plli2sr(&self) -> bool {
        *self == Ckout1sel::Plli2sr
    }
    #[doc = "External high speed oscillator clock selected"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == Ckout1sel::Hxtal
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == Ckout1sel::Pllp
    }
}
#[doc = "Field `CKOUT1SEL` writer - CKOUT1 Clock Source Selection"]
pub type Ckout1selW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckout1sel, crate::Safe>;
impl<'a, REG> Ckout1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Sysclk)
    }
    #[doc = "PLL I2S clock selected"]
    #[inline(always)]
    pub fn plli2sr(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Plli2sr)
    }
    #[doc = "External high speed oscillator clock selected"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Hxtal)
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn pllp(self) -> &'a mut crate::W<REG> {
        self.variant(Ckout1sel::Pllp)
    }
}
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&self) -> ScsR {
        ScsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn scss(&self) -> ScssR {
        ScssR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&self) -> AhbpscR {
        AhbpscR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 10:12 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&self) -> Apb1pscR {
        Apb1pscR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&self) -> Apb2pscR {
        Apb2pscR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - RTC clock divider factor"]
    #[inline(always)]
    pub fn rtcdiv(&self) -> RtcdivR {
        RtcdivR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - CKOUT0 Clock Source Selection"]
    #[inline(always)]
    pub fn ckout0sel(&self) -> Ckout0selR {
        Ckout0selR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - I2S Clock Source Selection"]
    #[inline(always)]
    pub fn i2ssel(&self) -> I2sselR {
        I2sselR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - The CK_OUT0 divider which the CK_OUT0 frequency can be reduced"]
    #[inline(always)]
    pub fn ckout0div(&self) -> Ckout0divR {
        Ckout0divR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced"]
    #[inline(always)]
    pub fn ckout1div(&self) -> Ckout1divR {
        Ckout1divR::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - CKOUT1 Clock Source Selection"]
    #[inline(always)]
    pub fn ckout1sel(&self) -> Ckout1selR {
        Ckout1selR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&mut self) -> ScsW<'_, Cfg0Spec> {
        ScsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&mut self) -> AhbpscW<'_, Cfg0Spec> {
        AhbpscW::new(self, 4)
    }
    #[doc = "Bits 10:12 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&mut self) -> Apb1pscW<'_, Cfg0Spec> {
        Apb1pscW::new(self, 10)
    }
    #[doc = "Bits 13:15 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&mut self) -> Apb2pscW<'_, Cfg0Spec> {
        Apb2pscW::new(self, 13)
    }
    #[doc = "Bits 16:20 - RTC clock divider factor"]
    #[inline(always)]
    pub fn rtcdiv(&mut self) -> RtcdivW<'_, Cfg0Spec> {
        RtcdivW::new(self, 16)
    }
    #[doc = "Bits 21:22 - CKOUT0 Clock Source Selection"]
    #[inline(always)]
    pub fn ckout0sel(&mut self) -> Ckout0selW<'_, Cfg0Spec> {
        Ckout0selW::new(self, 21)
    }
    #[doc = "Bit 23 - I2S Clock Source Selection"]
    #[inline(always)]
    pub fn i2ssel(&mut self) -> I2sselW<'_, Cfg0Spec> {
        I2sselW::new(self, 23)
    }
    #[doc = "Bits 24:26 - The CK_OUT0 divider which the CK_OUT0 frequency can be reduced"]
    #[inline(always)]
    pub fn ckout0div(&mut self) -> Ckout0divW<'_, Cfg0Spec> {
        Ckout0divW::new(self, 24)
    }
    #[doc = "Bits 27:29 - The CK_OUT1 divider which the CK_OUT1 frequency can be reduced"]
    #[inline(always)]
    pub fn ckout1div(&mut self) -> Ckout1divW<'_, Cfg0Spec> {
        Ckout1divW::new(self, 27)
    }
    #[doc = "Bits 30:31 - CKOUT1 Clock Source Selection"]
    #[inline(always)]
    pub fn ckout1sel(&mut self) -> Ckout1selW<'_, Cfg0Spec> {
        Ckout1selW::new(self, 30)
    }
}
#[doc = "Clock configuration register 0 (RCU_CFG0)\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for Cfg0Spec {}
