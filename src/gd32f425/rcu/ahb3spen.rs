#[doc = "Register `AHB3SPEN` reader"]
pub type R = crate::R<Ahb3spenSpec>;
#[doc = "Register `AHB3SPEN` writer"]
pub type W = crate::W<Ahb3spenSpec>;
#[doc = "EXMC clock enable when sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exmcspen {
    #[doc = "0: Disable the selected module in sleep mode."]
    Disable = 0,
    #[doc = "1: Enable the selected module in sleep mode."]
    Enable = 1,
}
impl From<Exmcspen> for bool {
    #[inline(always)]
    fn from(variant: Exmcspen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXMCSPEN` reader - EXMC clock enable when sleep mode"]
pub type ExmcspenR = crate::BitReader<Exmcspen>;
impl ExmcspenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exmcspen {
        match self.bits {
            false => Exmcspen::Disable,
            true => Exmcspen::Enable,
        }
    }
    #[doc = "Disable the selected module in sleep mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Exmcspen::Disable
    }
    #[doc = "Enable the selected module in sleep mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Exmcspen::Enable
    }
}
#[doc = "Field `EXMCSPEN` writer - EXMC clock enable when sleep mode"]
pub type ExmcspenW<'a, REG> = crate::BitWriter<'a, REG, Exmcspen>;
impl<'a, REG> ExmcspenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selected module in sleep mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Exmcspen::Disable)
    }
    #[doc = "Enable the selected module in sleep mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Exmcspen::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - EXMC clock enable when sleep mode"]
    #[inline(always)]
    pub fn exmcspen(&self) -> ExmcspenR {
        ExmcspenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXMC clock enable when sleep mode"]
    #[inline(always)]
    pub fn exmcspen(&mut self) -> ExmcspenW<'_, Ahb3spenSpec> {
        ExmcspenW::new(self, 0)
    }
}
#[doc = "AHB3 Sleep mode enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3spen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3spen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb3spenSpec;
impl crate::RegisterSpec for Ahb3spenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3spen::R`](R) reader structure"]
impl crate::Readable for Ahb3spenSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb3spen::W`](W) writer structure"]
impl crate::Writable for Ahb3spenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB3SPEN to value 0x01"]
impl crate::Resettable for Ahb3spenSpec {
    const RESET_VALUE: u32 = 0x01;
}
