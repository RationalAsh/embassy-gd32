#[doc = "Register `CH2CV` reader"]
pub type R = crate::R<Ch2cvSpec>;
#[doc = "Register `CH2CV` writer"]
pub type W = crate::W<Ch2cvSpec>;
#[doc = "Field `CH2VAL` reader - Capture or compare value of channel 2"]
pub type Ch2valR = crate::FieldReader<u16>;
#[doc = "Field `CH2VAL` writer - Capture or compare value of channel 2"]
pub type Ch2valW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Capture or compare value of channel 2"]
    #[inline(always)]
    pub fn ch2val(&self) -> Ch2valR {
        Ch2valR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value of channel 2"]
    #[inline(always)]
    pub fn ch2val(&mut self) -> Ch2valW<'_, Ch2cvSpec> {
        Ch2valW::new(self, 0)
    }
}
#[doc = "Channel 2 capture/compare value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2cvSpec;
impl crate::RegisterSpec for Ch2cvSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ch2cv::R`](R) reader structure"]
impl crate::Readable for Ch2cvSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2cv::W`](W) writer structure"]
impl crate::Writable for Ch2cvSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CH2CV to value 0"]
impl crate::Resettable for Ch2cvSpec {}
