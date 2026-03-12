#[doc = "Register `PLL` reader"]
pub type R = crate::R<PllSpec>;
#[doc = "Register `PLL` writer"]
pub type W = crate::W<PllSpec>;
#[doc = "Field `PLLPSC` reader - The PLL VCO source clock prescaler"]
pub type PllpscR = crate::FieldReader;
#[doc = "Field `PLLPSC` writer - The PLL VCO source clock prescaler"]
pub type PllpscW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PLLN` reader - The PLL VCO clock multi factor"]
pub type PllnR = crate::FieldReader<u16>;
#[doc = "Field `PLLN` writer - The PLL VCO clock multi factor"]
pub type PllnW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "The PLLP output frequency division factor from PLL VCO clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllp {
    #[doc = "0: PLL division factor P set to 2"]
    Div2 = 0,
    #[doc = "1: PLL division factor P set to 4"]
    Div4 = 1,
    #[doc = "2: PLL division factor P set to 8"]
    Div8 = 2,
}
impl From<Pllp> for u8 {
    #[inline(always)]
    fn from(variant: Pllp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllp {
    type Ux = u8;
}
impl crate::IsEnum for Pllp {}
#[doc = "Field `PLLP` reader - The PLLP output frequency division factor from PLL VCO clock"]
pub type PllpR = crate::FieldReader<Pllp>;
impl PllpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pllp> {
        match self.bits {
            0 => Some(Pllp::Div2),
            1 => Some(Pllp::Div4),
            2 => Some(Pllp::Div8),
            _ => None,
        }
    }
    #[doc = "PLL division factor P set to 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pllp::Div2
    }
    #[doc = "PLL division factor P set to 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Pllp::Div4
    }
    #[doc = "PLL division factor P set to 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Pllp::Div8
    }
}
#[doc = "Field `PLLP` writer - The PLLP output frequency division factor from PLL VCO clock"]
pub type PllpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pllp>;
impl<'a, REG> PllpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL division factor P set to 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllp::Div2)
    }
    #[doc = "PLL division factor P set to 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Pllp::Div4)
    }
    #[doc = "PLL division factor P set to 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Pllp::Div8)
    }
}
#[doc = "PLL Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllsel {
    #[doc = "0: IRC16M selected as the source clock of the PLL, PLLSAI, PLLI2S"]
    Irc16m = 0,
    #[doc = "1: HXTAL selected as the source clock of PLL, PLLSAI, PLLI2S"]
    Hxtal = 1,
}
impl From<Pllsel> for bool {
    #[inline(always)]
    fn from(variant: Pllsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSEL` reader - PLL Clock Source Selection"]
pub type PllselR = crate::BitReader<Pllsel>;
impl PllselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllsel {
        match self.bits {
            false => Pllsel::Irc16m,
            true => Pllsel::Hxtal,
        }
    }
    #[doc = "IRC16M selected as the source clock of the PLL, PLLSAI, PLLI2S"]
    #[inline(always)]
    pub fn is_irc16m(&self) -> bool {
        *self == Pllsel::Irc16m
    }
    #[doc = "HXTAL selected as the source clock of PLL, PLLSAI, PLLI2S"]
    #[inline(always)]
    pub fn is_hxtal(&self) -> bool {
        *self == Pllsel::Hxtal
    }
}
#[doc = "Field `PLLSEL` writer - PLL Clock Source Selection"]
pub type PllselW<'a, REG> = crate::BitWriter<'a, REG, Pllsel>;
impl<'a, REG> PllselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IRC16M selected as the source clock of the PLL, PLLSAI, PLLI2S"]
    #[inline(always)]
    pub fn irc16m(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsel::Irc16m)
    }
    #[doc = "HXTAL selected as the source clock of PLL, PLLSAI, PLLI2S"]
    #[inline(always)]
    pub fn hxtal(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsel::Hxtal)
    }
}
#[doc = "The PLL Q output frequency division factor from PLL VCO clock\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllq {
    #[doc = "2: PLL divisor Q set to 2."]
    Div2 = 2,
    #[doc = "3: PLL divisor Q set to 3."]
    Div3 = 3,
    #[doc = "4: PLL divisor Q set to 4."]
    Div4 = 4,
    #[doc = "5: PLL divisor Q set to 5."]
    Div5 = 5,
    #[doc = "6: PLL divisor Q set to 6."]
    Div6 = 6,
    #[doc = "7: PLL divisor Q set to 7."]
    Div7 = 7,
    #[doc = "8: PLL divisor Q set to 8."]
    Div8 = 8,
    #[doc = "9: PLL divisor Q set to 9."]
    Div9 = 9,
    #[doc = "10: PLL divisor Q set to 10."]
    Div10 = 10,
    #[doc = "11: PLL divisor Q set to 11."]
    Div11 = 11,
    #[doc = "12: PLL divisor Q set to 12."]
    Div12 = 12,
    #[doc = "13: PLL divisor Q set to 13."]
    Div13 = 13,
    #[doc = "14: PLL divisor Q set to 14."]
    Div14 = 14,
    #[doc = "15: PLL divisor Q set to 15."]
    Div15 = 15,
}
impl From<Pllq> for u8 {
    #[inline(always)]
    fn from(variant: Pllq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllq {
    type Ux = u8;
}
impl crate::IsEnum for Pllq {}
#[doc = "Field `PLLQ` reader - The PLL Q output frequency division factor from PLL VCO clock"]
pub type PllqR = crate::FieldReader<Pllq>;
impl PllqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pllq> {
        match self.bits {
            2 => Some(Pllq::Div2),
            3 => Some(Pllq::Div3),
            4 => Some(Pllq::Div4),
            5 => Some(Pllq::Div5),
            6 => Some(Pllq::Div6),
            7 => Some(Pllq::Div7),
            8 => Some(Pllq::Div8),
            9 => Some(Pllq::Div9),
            10 => Some(Pllq::Div10),
            11 => Some(Pllq::Div11),
            12 => Some(Pllq::Div12),
            13 => Some(Pllq::Div13),
            14 => Some(Pllq::Div14),
            15 => Some(Pllq::Div15),
            _ => None,
        }
    }
    #[doc = "PLL divisor Q set to 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pllq::Div2
    }
    #[doc = "PLL divisor Q set to 3."]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Pllq::Div3
    }
    #[doc = "PLL divisor Q set to 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Pllq::Div4
    }
    #[doc = "PLL divisor Q set to 5."]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Pllq::Div5
    }
    #[doc = "PLL divisor Q set to 6."]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Pllq::Div6
    }
    #[doc = "PLL divisor Q set to 7."]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Pllq::Div7
    }
    #[doc = "PLL divisor Q set to 8."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Pllq::Div8
    }
    #[doc = "PLL divisor Q set to 9."]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == Pllq::Div9
    }
    #[doc = "PLL divisor Q set to 10."]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Pllq::Div10
    }
    #[doc = "PLL divisor Q set to 11."]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == Pllq::Div11
    }
    #[doc = "PLL divisor Q set to 12."]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Pllq::Div12
    }
    #[doc = "PLL divisor Q set to 13."]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == Pllq::Div13
    }
    #[doc = "PLL divisor Q set to 14."]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Pllq::Div14
    }
    #[doc = "PLL divisor Q set to 15."]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == Pllq::Div15
    }
}
#[doc = "Field `PLLQ` writer - The PLL Q output frequency division factor from PLL VCO clock"]
pub type PllqW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pllq>;
impl<'a, REG> PllqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLL divisor Q set to 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div2)
    }
    #[doc = "PLL divisor Q set to 3."]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div3)
    }
    #[doc = "PLL divisor Q set to 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div4)
    }
    #[doc = "PLL divisor Q set to 5."]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div5)
    }
    #[doc = "PLL divisor Q set to 6."]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div6)
    }
    #[doc = "PLL divisor Q set to 7."]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div7)
    }
    #[doc = "PLL divisor Q set to 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div8)
    }
    #[doc = "PLL divisor Q set to 9."]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div9)
    }
    #[doc = "PLL divisor Q set to 10."]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div10)
    }
    #[doc = "PLL divisor Q set to 11."]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div11)
    }
    #[doc = "PLL divisor Q set to 12."]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div12)
    }
    #[doc = "PLL divisor Q set to 13."]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div13)
    }
    #[doc = "PLL divisor Q set to 14."]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div14)
    }
    #[doc = "PLL divisor Q set to 15."]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(Pllq::Div15)
    }
}
impl R {
    #[doc = "Bits 0:5 - The PLL VCO source clock prescaler"]
    #[inline(always)]
    pub fn pllpsc(&self) -> PllpscR {
        PllpscR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - The PLL VCO clock multi factor"]
    #[inline(always)]
    pub fn plln(&self) -> PllnR {
        PllnR::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - The PLLP output frequency division factor from PLL VCO clock"]
    #[inline(always)]
    pub fn pllp(&self) -> PllpR {
        PllpR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&self) -> PllselR {
        PllselR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - The PLL Q output frequency division factor from PLL VCO clock"]
    #[inline(always)]
    pub fn pllq(&self) -> PllqR {
        PllqR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The PLL VCO source clock prescaler"]
    #[inline(always)]
    pub fn pllpsc(&mut self) -> PllpscW<'_, PllSpec> {
        PllpscW::new(self, 0)
    }
    #[doc = "Bits 6:14 - The PLL VCO clock multi factor"]
    #[inline(always)]
    pub fn plln(&mut self) -> PllnW<'_, PllSpec> {
        PllnW::new(self, 6)
    }
    #[doc = "Bits 16:17 - The PLLP output frequency division factor from PLL VCO clock"]
    #[inline(always)]
    pub fn pllp(&mut self) -> PllpW<'_, PllSpec> {
        PllpW::new(self, 16)
    }
    #[doc = "Bit 22 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&mut self) -> PllselW<'_, PllSpec> {
        PllselW::new(self, 22)
    }
    #[doc = "Bits 24:27 - The PLL Q output frequency division factor from PLL VCO clock"]
    #[inline(always)]
    pub fn pllq(&mut self) -> PllqW<'_, PllSpec> {
        PllqW::new(self, 24)
    }
}
#[doc = "PLL register (RCU_PLL)\n\nYou can [`read`](crate::Reg::read) this register and get [`pll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllSpec;
impl crate::RegisterSpec for PllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll::R`](R) reader structure"]
impl crate::Readable for PllSpec {}
#[doc = "`write(|w| ..)` method takes [`pll::W`](W) writer structure"]
impl crate::Writable for PllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL to value 0x2400_3010"]
impl crate::Resettable for PllSpec {
    const RESET_VALUE: u32 = 0x2400_3010;
}
