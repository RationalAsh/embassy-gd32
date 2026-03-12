#[doc = "Register `DATATO` reader"]
pub type R = crate::R<DatatoSpec>;
#[doc = "Register `DATATO` writer"]
pub type W = crate::W<DatatoSpec>;
#[doc = "Field `DATATO` reader - Data timeout period"]
pub type DatatoR = crate::FieldReader<u32>;
#[doc = "Field `DATATO` writer - Data timeout period"]
pub type DatatoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    pub fn datato(&self) -> DatatoR {
        DatatoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    pub fn datato(&mut self) -> DatatoW<'_, DatatoSpec> {
        DatatoW::new(self, 0)
    }
}
#[doc = "Data timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`datato::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datato::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatatoSpec;
impl crate::RegisterSpec for DatatoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datato::R`](R) reader structure"]
impl crate::Readable for DatatoSpec {}
#[doc = "`write(|w| ..)` method takes [`datato::W`](W) writer structure"]
impl crate::Writable for DatatoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATATO to value 0"]
impl crate::Resettable for DatatoSpec {}
