#[doc = "Register `L1FLLEN` reader"]
pub type R = crate::R<L1fllenSpec>;
#[doc = "Register `L1FLLEN` writer"]
pub type W = crate::W<L1fllenSpec>;
#[doc = "Field `FLL` reader - Frame Line Length"]
pub type FllR = crate::FieldReader<u16>;
#[doc = "Field `FLL` writer - Frame Line Length"]
pub type FllW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `STDOFF` reader - Frame Buffer Stride Offset"]
pub type StdoffR = crate::FieldReader<u16>;
#[doc = "Field `STDOFF` writer - Frame Buffer Stride Offset"]
pub type StdoffW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Frame Line Length"]
    #[inline(always)]
    pub fn fll(&self) -> FllR {
        FllR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Frame Buffer Stride Offset"]
    #[inline(always)]
    pub fn stdoff(&self) -> StdoffR {
        StdoffR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Line Length"]
    #[inline(always)]
    pub fn fll(&mut self) -> FllW<'_, L1fllenSpec> {
        FllW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Frame Buffer Stride Offset"]
    #[inline(always)]
    pub fn stdoff(&mut self) -> StdoffW<'_, L1fllenSpec> {
        StdoffW::new(self, 16)
    }
}
#[doc = "Layer 1 frame line length register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1fllen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1fllen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1fllenSpec;
impl crate::RegisterSpec for L1fllenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1fllen::R`](R) reader structure"]
impl crate::Readable for L1fllenSpec {}
#[doc = "`write(|w| ..)` method takes [`l1fllen::W`](W) writer structure"]
impl crate::Writable for L1fllenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1FLLEN to value 0"]
impl crate::Resettable for L1fllenSpec {}
