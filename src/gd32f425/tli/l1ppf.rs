#[doc = "Register `L1PPF` reader"]
pub type R = crate::R<L1ppfSpec>;
#[doc = "Register `L1PPF` writer"]
pub type W = crate::W<L1ppfSpec>;
#[doc = "Field `PPF` reader - Packeted Pixel Format"]
pub type PpfR = crate::FieldReader;
#[doc = "Field `PPF` writer - Packeted Pixel Format"]
pub type PpfW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Packeted Pixel Format"]
    #[inline(always)]
    pub fn ppf(&self) -> PpfR {
        PpfR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Packeted Pixel Format"]
    #[inline(always)]
    pub fn ppf(&mut self) -> PpfW<'_, L1ppfSpec> {
        PpfW::new(self, 0)
    }
}
#[doc = "Layer 1 packeted pixel format register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1ppf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1ppf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1ppfSpec;
impl crate::RegisterSpec for L1ppfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1ppf::R`](R) reader structure"]
impl crate::Readable for L1ppfSpec {}
#[doc = "`write(|w| ..)` method takes [`l1ppf::W`](W) writer structure"]
impl crate::Writable for L1ppfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1PPF to value 0"]
impl crate::Resettable for L1ppfSpec {}
