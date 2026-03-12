#[doc = "Register `L0SA` reader"]
pub type R = crate::R<L0saSpec>;
#[doc = "Register `L0SA` writer"]
pub type W = crate::W<L0saSpec>;
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
    pub fn sa(&mut self) -> SaW<'_, L0saSpec> {
        SaW::new(self, 0)
    }
}
#[doc = "Layer 0 specified alpha register\n\nYou can [`read`](crate::Reg::read) this register and get [`l0sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l0sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L0saSpec;
impl crate::RegisterSpec for L0saSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l0sa::R`](R) reader structure"]
impl crate::Readable for L0saSpec {}
#[doc = "`write(|w| ..)` method takes [`l0sa::W`](W) writer structure"]
impl crate::Writable for L0saSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L0SA to value 0xff"]
impl crate::Resettable for L0saSpec {
    const RESET_VALUE: u32 = 0xff;
}
