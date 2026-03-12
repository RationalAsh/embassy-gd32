#[doc = "Register `ST2CMP3V` reader"]
pub type R = crate::R<St2cmp3vSpec>;
#[doc = "Register `ST2CMP3V` writer"]
pub type W = crate::W<St2cmp3vSpec>;
#[doc = "Field `CMP3VAL` reader - Compare 3 value"]
pub type Cmp3valR = crate::FieldReader<u16>;
#[doc = "Field `CMP3VAL` writer - Compare 3 value"]
pub type Cmp3valW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare 3 value"]
    #[inline(always)]
    pub fn cmp3val(&self) -> Cmp3valR {
        Cmp3valR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare 3 value"]
    #[inline(always)]
    pub fn cmp3val(&mut self) -> Cmp3valW<'_, St2cmp3vSpec> {
        Cmp3valW::new(self, 0)
    }
}
#[doc = "SHRTIMER Slave_TIMERx compare 3 value register\n\nYou can [`read`](crate::Reg::read) this register and get [`st2cmp3v::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st2cmp3v::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St2cmp3vSpec;
impl crate::RegisterSpec for St2cmp3vSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2cmp3v::R`](R) reader structure"]
impl crate::Readable for St2cmp3vSpec {}
#[doc = "`write(|w| ..)` method takes [`st2cmp3v::W`](W) writer structure"]
impl crate::Writable for St2cmp3vSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST2CMP3V to value 0"]
impl crate::Resettable for St2cmp3vSpec {}
