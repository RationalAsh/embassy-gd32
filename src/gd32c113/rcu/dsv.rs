#[doc = "Register `DSV` reader"]
pub type R = crate::R<DsvSpec>;
#[doc = "Register `DSV` writer"]
pub type W = crate::W<DsvSpec>;
#[doc = "Field `DSLPVS` reader - Deep-sleep mode voltage select"]
pub type DslpvsR = crate::FieldReader;
#[doc = "Field `DSLPVS` writer - Deep-sleep mode voltage select"]
pub type DslpvsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Deep-sleep mode voltage select"]
    #[inline(always)]
    pub fn dslpvs(&self) -> DslpvsR {
        DslpvsR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Deep-sleep mode voltage select"]
    #[inline(always)]
    pub fn dslpvs(&mut self) -> DslpvsW<'_, DsvSpec> {
        DslpvsW::new(self, 0)
    }
}
#[doc = "Deep sleep mode Voltage register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsvSpec;
impl crate::RegisterSpec for DsvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsv::R`](R) reader structure"]
impl crate::Readable for DsvSpec {}
#[doc = "`write(|w| ..)` method takes [`dsv::W`](W) writer structure"]
impl crate::Writable for DsvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSV to value 0"]
impl crate::Resettable for DsvSpec {}
