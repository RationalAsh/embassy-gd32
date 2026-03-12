#[doc = "Register `MTCMP0V` reader"]
pub type R = crate::R<Mtcmp0vSpec>;
#[doc = "Register `MTCMP0V` writer"]
pub type W = crate::W<Mtcmp0vSpec>;
#[doc = "Field `CMP0VAL` reader - Compare 0 value"]
pub type Cmp0valR = crate::FieldReader<u16>;
#[doc = "Field `CMP0VAL` writer - Compare 0 value"]
pub type Cmp0valW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 0 value"]
    #[inline(always)]
    pub fn cmp0val(&self) -> Cmp0valR {
        Cmp0valR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 0 value"]
    #[inline(always)]
    pub fn cmp0val(&mut self) -> Cmp0valW<'_, Mtcmp0vSpec> {
        Cmp0valW::new(self, 0)
    }
}
#[doc = "SHRTIMER Master_TIMER compare 0 value register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtcmp0v::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtcmp0v::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mtcmp0vSpec;
impl crate::RegisterSpec for Mtcmp0vSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtcmp0v::R`](R) reader structure"]
impl crate::Readable for Mtcmp0vSpec {}
#[doc = "`write(|w| ..)` method takes [`mtcmp0v::W`](W) writer structure"]
impl crate::Writable for Mtcmp0vSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTCMP0V to value 0"]
impl crate::Resettable for Mtcmp0vSpec {}
