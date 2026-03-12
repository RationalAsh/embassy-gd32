#[doc = "Register `CAR` reader"]
pub type R = crate::R<CarSpec>;
#[doc = "Register `CAR` writer"]
pub type W = crate::W<CarSpec>;
#[doc = "Field `CAR` reader - Low Auto-reload value"]
pub type CarR = crate::FieldReader<u32>;
#[doc = "Field `CAR` writer - Low Auto-reload value"]
pub type CarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:31 - Low Auto-reload value"]
    #[inline(always)]
    pub fn car(&self) -> CarR {
        CarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low Auto-reload value"]
    #[inline(always)]
    pub fn car(&mut self) -> CarW<'_, CarSpec> {
        CarW::new(self, 0)
    }
}
#[doc = "Counter auto reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`car::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`car::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CarSpec;
impl crate::RegisterSpec for CarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`car::R`](R) reader structure"]
impl crate::Readable for CarSpec {}
#[doc = "`write(|w| ..)` method takes [`car::W`](W) writer structure"]
impl crate::Writable for CarSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CAR to value 0"]
impl crate::Resettable for CarSpec {}
