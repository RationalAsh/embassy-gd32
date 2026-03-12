#[doc = "Register `WND` reader"]
pub type R = crate::R<WndSpec>;
#[doc = "Register `WND` writer"]
pub type W = crate::W<WndSpec>;
#[doc = "Field `WND` reader - Watchdog counter window value"]
pub type WndR = crate::FieldReader<u16>;
#[doc = "Field `WND` writer - Watchdog counter window value"]
pub type WndW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn wnd(&self) -> WndR {
        WndR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value"]
    #[inline(always)]
    pub fn wnd(&mut self) -> WndW<'_, WndSpec> {
        WndW::new(self, 0)
    }
}
#[doc = "Window register\n\nYou can [`read`](crate::Reg::read) this register and get [`wnd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wnd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WndSpec;
impl crate::RegisterSpec for WndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wnd::R`](R) reader structure"]
impl crate::Readable for WndSpec {}
#[doc = "`write(|w| ..)` method takes [`wnd::W`](W) writer structure"]
impl crate::Writable for WndSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WND to value 0x0fff"]
impl crate::Resettable for WndSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
