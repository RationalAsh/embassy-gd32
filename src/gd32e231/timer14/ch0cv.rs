#[doc = "Register `CH0CV` reader"]
pub type R = crate::R<Ch0cvSpec>;
#[doc = "Register `CH0CV` writer"]
pub type W = crate::W<Ch0cvSpec>;
#[doc = "Field `CH0VAL` reader - Capture/Compare 0 value"]
pub type Ch0valR = crate::FieldReader<u16>;
#[doc = "Field `CH0VAL` writer - Capture/Compare 0 value"]
pub type Ch0valW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 0 value"]
    #[inline(always)]
    pub fn ch0val(&self) -> Ch0valR {
        Ch0valR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 0 value"]
    #[inline(always)]
    pub fn ch0val(&mut self) -> Ch0valW<'_, Ch0cvSpec> {
        Ch0valW::new(self, 0)
    }
}
#[doc = "capture/compare register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0cvSpec;
impl crate::RegisterSpec for Ch0cvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0cv::R`](R) reader structure"]
impl crate::Readable for Ch0cvSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0cv::W`](W) writer structure"]
impl crate::Writable for Ch0cvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH0CV to value 0"]
impl crate::Resettable for Ch0cvSpec {}
