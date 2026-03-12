#[doc = "Register `IV0H` reader"]
pub type R = crate::R<Iv0hSpec>;
#[doc = "Register `IV0H` writer"]
pub type W = crate::W<Iv0hSpec>;
#[doc = "Field `IV0H` reader - The initialization vector for DES,TDES,AES"]
pub type Iv0hR = crate::FieldReader<u32>;
#[doc = "Field `IV0H` writer - The initialization vector for DES,TDES,AES"]
pub type Iv0hW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv0h(&self) -> Iv0hR {
        Iv0hR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The initialization vector for DES,TDES,AES"]
    #[inline(always)]
    pub fn iv0h(&mut self) -> Iv0hW<'_, Iv0hSpec> {
        Iv0hW::new(self, 0)
    }
}
#[doc = "CAU initialization register\n\nYou can [`read`](crate::Reg::read) this register and get [`iv0h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv0h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iv0hSpec;
impl crate::RegisterSpec for Iv0hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv0h::R`](R) reader structure"]
impl crate::Readable for Iv0hSpec {}
#[doc = "`write(|w| ..)` method takes [`iv0h::W`](W) writer structure"]
impl crate::Writable for Iv0hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IV0H to value 0"]
impl crate::Resettable for Iv0hSpec {}
