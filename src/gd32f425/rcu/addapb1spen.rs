#[doc = "Register `ADDAPB1SPEN` reader"]
pub type R = crate::R<Addapb1spenSpec>;
#[doc = "Register `ADDAPB1SPEN` writer"]
pub type W = crate::W<Addapb1spenSpec>;
#[doc = "CTC enable when sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctcspen {
    #[doc = "0: Disable the selected module in sleep mode."]
    Disable = 0,
    #[doc = "1: Enable the selected module in sleep mode."]
    Enable = 1,
}
impl From<Ctcspen> for bool {
    #[inline(always)]
    fn from(variant: Ctcspen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCSPEN` reader - CTC enable when sleep mode"]
pub type CtcspenR = crate::BitReader<Ctcspen>;
impl CtcspenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctcspen {
        match self.bits {
            false => Ctcspen::Disable,
            true => Ctcspen::Enable,
        }
    }
    #[doc = "Disable the selected module in sleep mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctcspen::Disable
    }
    #[doc = "Enable the selected module in sleep mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctcspen::Enable
    }
}
#[doc = "Field `CTCSPEN` writer - CTC enable when sleep mode"]
pub type CtcspenW<'a, REG> = crate::BitWriter<'a, REG, Ctcspen>;
impl<'a, REG> CtcspenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selected module in sleep mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctcspen::Disable)
    }
    #[doc = "Enable the selected module in sleep mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctcspen::Enable)
    }
}
#[doc = "Field `IREFSPEN` reader - IREF enable when sleep mode"]
pub use CtcspenR as IrefspenR;
#[doc = "Field `IREFSPEN` writer - IREF enable when sleep mode"]
pub use CtcspenW as IrefspenW;
impl R {
    #[doc = "Bit 27 - CTC enable when sleep mode"]
    #[inline(always)]
    pub fn ctcspen(&self) -> CtcspenR {
        CtcspenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - IREF enable when sleep mode"]
    #[inline(always)]
    pub fn irefspen(&self) -> IrefspenR {
        IrefspenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - CTC enable when sleep mode"]
    #[inline(always)]
    pub fn ctcspen(&mut self) -> CtcspenW<'_, Addapb1spenSpec> {
        CtcspenW::new(self, 27)
    }
    #[doc = "Bit 31 - IREF enable when sleep mode"]
    #[inline(always)]
    pub fn irefspen(&mut self) -> IrefspenW<'_, Addapb1spenSpec> {
        IrefspenW::new(self, 31)
    }
}
#[doc = "APB1 additional sleep mode enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`addapb1spen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addapb1spen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addapb1spenSpec;
impl crate::RegisterSpec for Addapb1spenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addapb1spen::R`](R) reader structure"]
impl crate::Readable for Addapb1spenSpec {}
#[doc = "`write(|w| ..)` method takes [`addapb1spen::W`](W) writer structure"]
impl crate::Writable for Addapb1spenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDAPB1SPEN to value 0x8800_0000"]
impl crate::Resettable for Addapb1spenSpec {
    const RESET_VALUE: u32 = 0x8800_0000;
}
