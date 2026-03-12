#[doc = "Register `AHB3EN` reader"]
pub type R = crate::R<Ahb3enSpec>;
#[doc = "Register `AHB3EN` writer"]
pub type W = crate::W<Ahb3enSpec>;
#[doc = "EXMC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exmcen {
    #[doc = "0: Disable the selected module clock."]
    Disable = 0,
    #[doc = "1: Enable the selected module clock."]
    Enable = 1,
}
impl From<Exmcen> for bool {
    #[inline(always)]
    fn from(variant: Exmcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXMCEN` reader - EXMC clock enable"]
pub type ExmcenR = crate::BitReader<Exmcen>;
impl ExmcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exmcen {
        match self.bits {
            false => Exmcen::Disable,
            true => Exmcen::Enable,
        }
    }
    #[doc = "Disable the selected module clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Exmcen::Disable
    }
    #[doc = "Enable the selected module clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Exmcen::Enable
    }
}
#[doc = "Field `EXMCEN` writer - EXMC clock enable"]
pub type ExmcenW<'a, REG> = crate::BitWriter<'a, REG, Exmcen>;
impl<'a, REG> ExmcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the selected module clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Exmcen::Disable)
    }
    #[doc = "Enable the selected module clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Exmcen::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - EXMC clock enable"]
    #[inline(always)]
    pub fn exmcen(&self) -> ExmcenR {
        ExmcenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXMC clock enable"]
    #[inline(always)]
    pub fn exmcen(&mut self) -> ExmcenW<'_, Ahb3enSpec> {
        ExmcenW::new(self, 0)
    }
}
#[doc = "AHB3 clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb3enSpec;
impl crate::RegisterSpec for Ahb3enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3en::R`](R) reader structure"]
impl crate::Readable for Ahb3enSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb3en::W`](W) writer structure"]
impl crate::Writable for Ahb3enSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB3EN to value 0"]
impl crate::Resettable for Ahb3enSpec {}
