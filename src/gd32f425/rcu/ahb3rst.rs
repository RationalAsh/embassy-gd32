#[doc = "Register `AHB3RST` reader"]
pub type R = crate::R<Ahb3rstSpec>;
#[doc = "Register `AHB3RST` writer"]
pub type W = crate::W<Ahb3rstSpec>;
#[doc = "EXMC reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exmcrst {
    #[doc = "1: Reset the selected module."]
    Reset = 1,
}
impl From<Exmcrst> for bool {
    #[inline(always)]
    fn from(variant: Exmcrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXMCRST` reader - EXMC reset"]
pub type ExmcrstR = crate::BitReader<Exmcrst>;
impl ExmcrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Exmcrst> {
        match self.bits {
            true => Some(Exmcrst::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Exmcrst::Reset
    }
}
#[doc = "Field `EXMCRST` writer - EXMC reset"]
pub type ExmcrstW<'a, REG> = crate::BitWriter<'a, REG, Exmcrst>;
impl<'a, REG> ExmcrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Exmcrst::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - EXMC reset"]
    #[inline(always)]
    pub fn exmcrst(&self) -> ExmcrstR {
        ExmcrstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXMC reset"]
    #[inline(always)]
    pub fn exmcrst(&mut self) -> ExmcrstW<'_, Ahb3rstSpec> {
        ExmcrstW::new(self, 0)
    }
}
#[doc = "AHB3 reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb3rstSpec;
impl crate::RegisterSpec for Ahb3rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3rst::R`](R) reader structure"]
impl crate::Readable for Ahb3rstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb3rst::W`](W) writer structure"]
impl crate::Writable for Ahb3rstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB3RST to value 0"]
impl crate::Resettable for Ahb3rstSpec {}
