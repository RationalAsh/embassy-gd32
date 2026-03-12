#[doc = "Register `PLLSSCTL` reader"]
pub type R = crate::R<PllssctlSpec>;
#[doc = "Register `PLLSSCTL` writer"]
pub type W = crate::W<PllssctlSpec>;
#[doc = "Field `MODCNT` reader - configure PLL spread spectrum modulation profile amplitude and frequency"]
pub type ModcntR = crate::FieldReader<u16>;
#[doc = "Field `MODCNT` writer - configure PLL spread spectrum modulation profile amplitude and frequency"]
pub type ModcntW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `MODSTEP` reader - configure PLL spread spectrum modulation profile amplitude and frequency"]
pub type ModstepR = crate::FieldReader<u16>;
#[doc = "Field `MODSTEP` writer - configure PLL spread spectrum modulation profile amplitude and frequency"]
pub type ModstepW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "PLL spread spectrum modulation type select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsType {
    #[doc = "0: Center spread."]
    Center = 0,
    #[doc = "1: Down spread."]
    Down = 1,
}
impl From<SsType> for bool {
    #[inline(always)]
    fn from(variant: SsType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS_TYPE` reader - PLL spread spectrum modulation type select"]
pub type SsTypeR = crate::BitReader<SsType>;
impl SsTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsType {
        match self.bits {
            false => SsType::Center,
            true => SsType::Down,
        }
    }
    #[doc = "Center spread."]
    #[inline(always)]
    pub fn is_center(&self) -> bool {
        *self == SsType::Center
    }
    #[doc = "Down spread."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == SsType::Down
    }
}
#[doc = "Field `SS_TYPE` writer - PLL spread spectrum modulation type select"]
pub type SsTypeW<'a, REG> = crate::BitWriter<'a, REG, SsType>;
impl<'a, REG> SsTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Center spread."]
    #[inline(always)]
    pub fn center(self) -> &'a mut crate::W<REG> {
        self.variant(SsType::Center)
    }
    #[doc = "Down spread."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(SsType::Down)
    }
}
#[doc = "PLL spread spectrum modulation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sscgon {
    #[doc = "0: Enable the module."]
    Enable = 0,
    #[doc = "1: Disable the module."]
    Disable = 1,
}
impl From<Sscgon> for bool {
    #[inline(always)]
    fn from(variant: Sscgon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSCGON` reader - PLL spread spectrum modulation enable"]
pub type SscgonR = crate::BitReader<Sscgon>;
impl SscgonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sscgon {
        match self.bits {
            false => Sscgon::Enable,
            true => Sscgon::Disable,
        }
    }
    #[doc = "Enable the module."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sscgon::Enable
    }
    #[doc = "Disable the module."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sscgon::Disable
    }
}
#[doc = "Field `SSCGON` writer - PLL spread spectrum modulation enable"]
pub type SscgonW<'a, REG> = crate::BitWriter<'a, REG, Sscgon>;
impl<'a, REG> SscgonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the module."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sscgon::Enable)
    }
    #[doc = "Disable the module."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sscgon::Disable)
    }
}
impl R {
    #[doc = "Bits 0:12 - configure PLL spread spectrum modulation profile amplitude and frequency"]
    #[inline(always)]
    pub fn modcnt(&self) -> ModcntR {
        ModcntR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:27 - configure PLL spread spectrum modulation profile amplitude and frequency"]
    #[inline(always)]
    pub fn modstep(&self) -> ModstepR {
        ModstepR::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    #[doc = "Bit 30 - PLL spread spectrum modulation type select"]
    #[inline(always)]
    pub fn ss_type(&self) -> SsTypeR {
        SsTypeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PLL spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgon(&self) -> SscgonR {
        SscgonR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - configure PLL spread spectrum modulation profile amplitude and frequency"]
    #[inline(always)]
    pub fn modcnt(&mut self) -> ModcntW<'_, PllssctlSpec> {
        ModcntW::new(self, 0)
    }
    #[doc = "Bits 13:27 - configure PLL spread spectrum modulation profile amplitude and frequency"]
    #[inline(always)]
    pub fn modstep(&mut self) -> ModstepW<'_, PllssctlSpec> {
        ModstepW::new(self, 13)
    }
    #[doc = "Bit 30 - PLL spread spectrum modulation type select"]
    #[inline(always)]
    pub fn ss_type(&mut self) -> SsTypeW<'_, PllssctlSpec> {
        SsTypeW::new(self, 30)
    }
    #[doc = "Bit 31 - PLL spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgon(&mut self) -> SscgonW<'_, PllssctlSpec> {
        SscgonW::new(self, 31)
    }
}
#[doc = "PLL clock spread spectrum control register (RCU_PLLSSCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`pllssctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllssctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllssctlSpec;
impl crate::RegisterSpec for PllssctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllssctl::R`](R) reader structure"]
impl crate::Readable for PllssctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pllssctl::W`](W) writer structure"]
impl crate::Writable for PllssctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLSSCTL to value 0"]
impl crate::Resettable for PllssctlSpec {}
