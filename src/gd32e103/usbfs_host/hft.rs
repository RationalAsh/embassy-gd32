#[doc = "Register `HFT` reader"]
pub type R = crate::R<HftSpec>;
#[doc = "Register `HFT` writer"]
pub type W = crate::W<HftSpec>;
#[doc = "Field `FRI` reader - Frame interval"]
pub type FriR = crate::FieldReader<u16>;
#[doc = "Field `FRI` writer - Frame interval"]
pub type FriW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Frame interval"]
    #[inline(always)]
    pub fn fri(&self) -> FriR {
        FriR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame interval"]
    #[inline(always)]
    pub fn fri(&mut self) -> FriW<'_, HftSpec> {
        FriW::new(self, 0)
    }
}
#[doc = "Host frame interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`hft::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hft::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HftSpec;
impl crate::RegisterSpec for HftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hft::R`](R) reader structure"]
impl crate::Readable for HftSpec {}
#[doc = "`write(|w| ..)` method takes [`hft::W`](W) writer structure"]
impl crate::Writable for HftSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFT to value 0xbb80"]
impl crate::Resettable for HftSpec {
    const RESET_VALUE: u32 = 0xbb80;
}
