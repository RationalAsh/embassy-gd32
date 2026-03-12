#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `SLCD_DECA` reader - Decoupling capacitance connection for SLCD"]
pub type SlcdDecaR = crate::FieldReader;
#[doc = "Field `SLCD_DECA` writer - Decoupling capacitance connection for SLCD"]
pub type SlcdDecaW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Bits 1:3 - Decoupling capacitance connection for SLCD"]
    #[inline(always)]
    pub fn slcd_deca(&self) -> SlcdDecaR {
        SlcdDecaR::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 1:3 - Decoupling capacitance connection for SLCD"]
    #[inline(always)]
    pub fn slcd_deca(&mut self) -> SlcdDecaW<'_, Cfg1Spec> {
        SlcdDecaW::new(self, 1)
    }
}
#[doc = "System configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
