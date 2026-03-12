#[doc = "Register `CMDAGMT` reader"]
pub type R = crate::R<CmdagmtSpec>;
#[doc = "Register `CMDAGMT` writer"]
pub type W = crate::W<CmdagmtSpec>;
#[doc = "Field `CMDAGMT` reader - SDIO card command argument"]
pub type CmdagmtR = crate::FieldReader<u32>;
#[doc = "Field `CMDAGMT` writer - SDIO card command argument"]
pub type CmdagmtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SDIO card command argument"]
    #[inline(always)]
    pub fn cmdagmt(&self) -> CmdagmtR {
        CmdagmtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SDIO card command argument"]
    #[inline(always)]
    pub fn cmdagmt(&mut self) -> CmdagmtW<'_, CmdagmtSpec> {
        CmdagmtW::new(self, 0)
    }
}
#[doc = "Command argument register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdagmt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdagmt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdagmtSpec;
impl crate::RegisterSpec for CmdagmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdagmt::R`](R) reader structure"]
impl crate::Readable for CmdagmtSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdagmt::W`](W) writer structure"]
impl crate::Writable for CmdagmtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDAGMT to value 0"]
impl crate::Resettable for CmdagmtSpec {}
