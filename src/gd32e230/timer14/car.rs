#[doc = "Register `CAR` reader"]
pub type R = crate::R<CarSpec>;
#[doc = "Register `CAR` writer"]
pub type W = crate::W<CarSpec>;
#[doc = "Field `CAR` reader - Auto-reload value"]
pub type CarR = crate::FieldReader<u16>;
#[doc = "Field `CAR` writer - Auto-reload value"]
pub type CarW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Auto-reload value"]
    #[inline(always)]
    pub fn car(&self) -> CarR {
        CarR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auto-reload value"]
    #[inline(always)]
    pub fn car(&mut self) -> CarW<'_, CarSpec> {
        CarW::new(self, 0)
    }
}
#[doc = "auto-reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`car::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`car::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CarSpec;
impl crate::RegisterSpec for CarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`car::R`](R) reader structure"]
impl crate::Readable for CarSpec {}
#[doc = "`write(|w| ..)` method takes [`car::W`](W) writer structure"]
impl crate::Writable for CarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAR to value 0"]
impl crate::Resettable for CarSpec {}
