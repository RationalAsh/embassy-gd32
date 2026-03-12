#[doc = "Register `ST4CMP2V` reader"]
pub type R = crate::R<St4cmp2vSpec>;
#[doc = "Register `ST4CMP2V` writer"]
pub type W = crate::W<St4cmp2vSpec>;
#[doc = "Field `CMP2VAL` reader - Compare 2 value"]
pub type Cmp2valR = crate::FieldReader<u16>;
#[doc = "Field `CMP2VAL` writer - Compare 2 value"]
pub type Cmp2valW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 2 value"]
    #[inline(always)]
    pub fn cmp2val(&self) -> Cmp2valR {
        Cmp2valR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 2 value"]
    #[inline(always)]
    pub fn cmp2val(&mut self) -> Cmp2valW<'_, St4cmp2vSpec> {
        Cmp2valW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMERx compare 2 value register\n\nYou can [`read`](crate::Reg::read) this register and get [`st4cmp2v::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st4cmp2v::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St4cmp2vSpec;
impl crate::RegisterSpec for St4cmp2vSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st4cmp2v::R`](R) reader structure"]
impl crate::Readable for St4cmp2vSpec {}
#[doc = "`write(|w| ..)` method takes [`st4cmp2v::W`](W) writer structure"]
impl crate::Writable for St4cmp2vSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST4CMP2V to value 0"]
impl crate::Resettable for St4cmp2vSpec {}
