#[doc = "Register `ALRM0SS` reader"]
pub type R = crate::R<Alrm0ssSpec>;
#[doc = "Register `ALRM0SS` writer"]
pub type W = crate::W<Alrm0ssSpec>;
#[doc = "Field `SSC` reader - Alarm sub second value"]
pub type SscR = crate::FieldReader<u16>;
#[doc = "Field `SSC` writer - Alarm sub second value"]
pub type SscW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `MSKSSC` reader - Mask control bit of SSC"]
pub type MsksscR = crate::FieldReader;
#[doc = "Field `MSKSSC` writer - Mask control bit of SSC"]
pub type MsksscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - Alarm sub second value"]
    #[inline(always)]
    pub fn ssc(&self) -> SscR {
        SscR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Mask control bit of SSC"]
    #[inline(always)]
    pub fn mskssc(&self) -> MsksscR {
        MsksscR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Alarm sub second value"]
    #[inline(always)]
    pub fn ssc(&mut self) -> SscW<'_, Alrm0ssSpec> {
        SscW::new(self, 0)
    }
    #[doc = "Bits 24:27 - Mask control bit of SSC"]
    #[inline(always)]
    pub fn mskssc(&mut self) -> MsksscW<'_, Alrm0ssSpec> {
        MsksscW::new(self, 24)
    }
}
#[doc = "alarm A sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrm0ss::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrm0ss::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alrm0ssSpec;
impl crate::RegisterSpec for Alrm0ssSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrm0ss::R`](R) reader structure"]
impl crate::Readable for Alrm0ssSpec {}
#[doc = "`write(|w| ..)` method takes [`alrm0ss::W`](W) writer structure"]
impl crate::Writable for Alrm0ssSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALRM0SS to value 0"]
impl crate::Resettable for Alrm0ssSpec {}
