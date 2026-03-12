#[doc = "Register `DI` reader"]
pub type R = crate::R<DiSpec>;
#[doc = "Register `DI` writer"]
pub type W = crate::W<DiSpec>;
#[doc = "Field `DI` reader - Data input"]
pub type DiR = crate::FieldReader<u32>;
#[doc = "Field `DI` writer - Data input"]
pub type DiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data input"]
    #[inline(always)]
    pub fn di(&self) -> DiR {
        DiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data input"]
    #[inline(always)]
    pub fn di(&mut self) -> DiW<'_, DiSpec> {
        DiW::new(self, 0)
    }
}
#[doc = "CAU data input register\n\nYou can [`read`](crate::Reg::read) this register and get [`di::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`di::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiSpec;
impl crate::RegisterSpec for DiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`di::R`](R) reader structure"]
impl crate::Readable for DiSpec {}
#[doc = "`write(|w| ..)` method takes [`di::W`](W) writer structure"]
impl crate::Writable for DiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DI to value 0"]
impl crate::Resettable for DiSpec {}
