#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "The divider factor from PLLSAIR clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pllsairdiv {
    #[doc = "0: TLI clock is CK_PLLSAIR divided by 2."]
    Div2 = 0,
    #[doc = "1: TLI clock is CK_PLLSAIR divided by 4."]
    Div4 = 1,
    #[doc = "2: TLI clock is CK_PLLSAIR divided by 8."]
    Div8 = 2,
    #[doc = "3: TLI clock is CK_PLLSAIR divided by 16."]
    Div16 = 3,
}
impl From<Pllsairdiv> for u8 {
    #[inline(always)]
    fn from(variant: Pllsairdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pllsairdiv {
    type Ux = u8;
}
impl crate::IsEnum for Pllsairdiv {}
#[doc = "Field `PLLSAIRDIV` reader - The divider factor from PLLSAIR clock"]
pub type PllsairdivR = crate::FieldReader<Pllsairdiv>;
impl PllsairdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllsairdiv {
        match self.bits {
            0 => Pllsairdiv::Div2,
            1 => Pllsairdiv::Div4,
            2 => Pllsairdiv::Div8,
            3 => Pllsairdiv::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "TLI clock is CK_PLLSAIR divided by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pllsairdiv::Div2
    }
    #[doc = "TLI clock is CK_PLLSAIR divided by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Pllsairdiv::Div4
    }
    #[doc = "TLI clock is CK_PLLSAIR divided by 8."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Pllsairdiv::Div8
    }
    #[doc = "TLI clock is CK_PLLSAIR divided by 16."]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Pllsairdiv::Div16
    }
}
#[doc = "Field `PLLSAIRDIV` writer - The divider factor from PLLSAIR clock"]
pub type PllsairdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pllsairdiv, crate::Safe>;
impl<'a, REG> PllsairdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TLI clock is CK_PLLSAIR divided by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsairdiv::Div2)
    }
    #[doc = "TLI clock is CK_PLLSAIR divided by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsairdiv::Div4)
    }
    #[doc = "TLI clock is CK_PLLSAIR divided by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsairdiv::Div8)
    }
    #[doc = "TLI clock is CK_PLLSAIR divided by 16."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Pllsairdiv::Div16)
    }
}
#[doc = "TIMER clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timersel {
    #[doc = "0: If APB1PSC/APB2PSC in RCU_CFG0 register is 0b0xx(CK_APBx = CK_AHB) or 0b100(CK_APBx = CK_AHB/2), the TIMER clock is equal to CK_AHB(CK_TIMERx = CK_AHB). Or else, the TIMER clock is twice the corresponding APB clock (TIMER in APB1 domain: CK_TIMERx = 2 x CK_APB1; TIMER in APB2 domain: CK_TIMERx = 2 x CK_APB2)."]
    Reset = 0,
    #[doc = "1: If APB1PSC/APB2PSC in RCU_CFG0 register is 0b0xx(CK_APBx = CK_AHB), 0b100(CK_APBx = CK_AHB/2), or 0b101(CK_APBx = CK_AHB/4), the TIMER clock is equal to CK_AHB(CK_TIMERx = CK_AHB). Or else, the TIMER clock is four times the corresponding APB clock (TIMER in APB1 domain: CK_TIMERx = 4x CK_APB1, TIMER in APB2 domain: CK_TIMERx = 4 x CK_APB2)."]
    Set = 1,
}
impl From<Timersel> for bool {
    #[inline(always)]
    fn from(variant: Timersel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMERSEL` reader - TIMER clock selection"]
pub type TimerselR = crate::BitReader<Timersel>;
impl TimerselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timersel {
        match self.bits {
            false => Timersel::Reset,
            true => Timersel::Set,
        }
    }
    #[doc = "If APB1PSC/APB2PSC in RCU_CFG0 register is 0b0xx(CK_APBx = CK_AHB) or 0b100(CK_APBx = CK_AHB/2), the TIMER clock is equal to CK_AHB(CK_TIMERx = CK_AHB). Or else, the TIMER clock is twice the corresponding APB clock (TIMER in APB1 domain: CK_TIMERx = 2 x CK_APB1; TIMER in APB2 domain: CK_TIMERx = 2 x CK_APB2)."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Timersel::Reset
    }
    #[doc = "If APB1PSC/APB2PSC in RCU_CFG0 register is 0b0xx(CK_APBx = CK_AHB), 0b100(CK_APBx = CK_AHB/2), or 0b101(CK_APBx = CK_AHB/4), the TIMER clock is equal to CK_AHB(CK_TIMERx = CK_AHB). Or else, the TIMER clock is four times the corresponding APB clock (TIMER in APB1 domain: CK_TIMERx = 4x CK_APB1, TIMER in APB2 domain: CK_TIMERx = 4 x CK_APB2)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Timersel::Set
    }
}
#[doc = "Field `TIMERSEL` writer - TIMER clock selection"]
pub type TimerselW<'a, REG> = crate::BitWriter<'a, REG, Timersel>;
impl<'a, REG> TimerselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If APB1PSC/APB2PSC in RCU_CFG0 register is 0b0xx(CK_APBx = CK_AHB) or 0b100(CK_APBx = CK_AHB/2), the TIMER clock is equal to CK_AHB(CK_TIMERx = CK_AHB). Or else, the TIMER clock is twice the corresponding APB clock (TIMER in APB1 domain: CK_TIMERx = 2 x CK_APB1; TIMER in APB2 domain: CK_TIMERx = 2 x CK_APB2)."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Timersel::Reset)
    }
    #[doc = "If APB1PSC/APB2PSC in RCU_CFG0 register is 0b0xx(CK_APBx = CK_AHB), 0b100(CK_APBx = CK_AHB/2), or 0b101(CK_APBx = CK_AHB/4), the TIMER clock is equal to CK_AHB(CK_TIMERx = CK_AHB). Or else, the TIMER clock is four times the corresponding APB clock (TIMER in APB1 domain: CK_TIMERx = 4x CK_APB1, TIMER in APB2 domain: CK_TIMERx = 4 x CK_APB2)."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Timersel::Set)
    }
}
impl R {
    #[doc = "Bits 16:17 - The divider factor from PLLSAIR clock"]
    #[inline(always)]
    pub fn pllsairdiv(&self) -> PllsairdivR {
        PllsairdivR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - TIMER clock selection"]
    #[inline(always)]
    pub fn timersel(&self) -> TimerselR {
        TimerselR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - The divider factor from PLLSAIR clock"]
    #[inline(always)]
    pub fn pllsairdiv(&mut self) -> PllsairdivW<'_, Cfg1Spec> {
        PllsairdivW::new(self, 16)
    }
    #[doc = "Bit 24 - TIMER clock selection"]
    #[inline(always)]
    pub fn timersel(&mut self) -> TimerselW<'_, Cfg1Spec> {
        TimerselW::new(self, 24)
    }
}
#[doc = "Clock Configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {}
