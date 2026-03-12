#[doc = "Register `ST4CMP1V` reader"]
pub type R = crate::R<St4cmp1vSpec>;
#[doc = "Register `ST4CMP1V` writer"]
pub type W = crate::W<St4cmp1vSpec>;
#[doc = "Field `CMP1VAL` reader - Compare 1 value"]
pub type Cmp1valR = crate::FieldReader<u16>;
#[doc = "Field `CMP1VAL` writer - Compare 1 value"]
pub type Cmp1valW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 1 value"]
    #[inline(always)]
    pub fn cmp1val(&self) -> Cmp1valR {
        Cmp1valR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 1 value"]
    #[inline(always)]
    pub fn cmp1val(&mut self) -> Cmp1valW<'_, St4cmp1vSpec> {
        Cmp1valW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMERx compare 1 value register\n\nYou can [`read`](crate::Reg::read) this register and get [`st4cmp1v::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st4cmp1v::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St4cmp1vSpec;
impl crate::RegisterSpec for St4cmp1vSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st4cmp1v::R`](R) reader structure"]
impl crate::Readable for St4cmp1vSpec {}
#[doc = "`write(|w| ..)` method takes [`st4cmp1v::W`](W) writer structure"]
impl crate::Writable for St4cmp1vSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST4CMP1V to value 0"]
impl crate::Resettable for St4cmp1vSpec {}
