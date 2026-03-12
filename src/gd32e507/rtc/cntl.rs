#[doc = "Register `CNTL` reader"]
pub type R = crate::R<CntlSpec>;
#[doc = "Register `CNTL` writer"]
pub type W = crate::W<CntlSpec>;
#[doc = "Field `CNT` reader - RTC conuter value low"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - RTC conuter value low"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC conuter value low"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC conuter value low"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, CntlSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "RTC counter low register\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntlSpec;
impl crate::RegisterSpec for CntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl::R`](R) reader structure"]
impl crate::Readable for CntlSpec {}
#[doc = "`write(|w| ..)` method takes [`cntl::W`](W) writer structure"]
impl crate::Writable for CntlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTL to value 0"]
impl crate::Resettable for CntlSpec {}
