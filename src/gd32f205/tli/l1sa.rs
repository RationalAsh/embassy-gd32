#[doc = "Register `L1SA` reader"]
pub type R = crate::R<L1saSpec>;
#[doc = "Register `L1SA` writer"]
pub type W = crate::W<L1saSpec>;
#[doc = "Field `SA` reader - Specified alpha"]
pub type SaR = crate::FieldReader;
#[doc = "Field `SA` writer - Specified alpha"]
pub type SaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Specified alpha"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specified alpha"]
    #[inline(always)]
    pub fn sa(&mut self) -> SaW<'_, L1saSpec> {
        SaW::new(self, 0)
    }
}
#[doc = "Layer 1 specified alpha register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1saSpec;
impl crate::RegisterSpec for L1saSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1sa::R`](R) reader structure"]
impl crate::Readable for L1saSpec {}
#[doc = "`write(|w| ..)` method takes [`l1sa::W`](W) writer structure"]
impl crate::Writable for L1saSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1SA to value 0xff"]
impl crate::Resettable for L1saSpec {
    const RESET_VALUE: u32 = 0xff;
}
