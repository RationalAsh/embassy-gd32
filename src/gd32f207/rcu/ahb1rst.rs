#[doc = "Register `AHB1RST` reader"]
pub type R = crate::R<Ahb1rstSpec>;
#[doc = "Register `AHB1RST` writer"]
pub type W = crate::W<Ahb1rstSpec>;
#[doc = "Field `USBFSRST` reader - USBFS reset"]
pub type UsbfsrstR = crate::BitReader;
#[doc = "Field `USBFSRST` writer - USBFS reset"]
pub type UsbfsrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENETRST` reader - ENET reset"]
pub type EnetrstR = crate::BitReader;
#[doc = "Field `ENETRST` writer - ENET reset"]
pub type EnetrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    pub fn usbfsrst(&self) -> UsbfsrstR {
        UsbfsrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - ENET reset"]
    #[inline(always)]
    pub fn enetrst(&self) -> EnetrstR {
        EnetrstR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    pub fn usbfsrst(&mut self) -> UsbfsrstW<'_, Ahb1rstSpec> {
        UsbfsrstW::new(self, 12)
    }
    #[doc = "Bit 14 - ENET reset"]
    #[inline(always)]
    pub fn enetrst(&mut self) -> EnetrstW<'_, Ahb1rstSpec> {
        EnetrstW::new(self, 14)
    }
}
#[doc = "AHB1 reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1rstSpec;
impl crate::RegisterSpec for Ahb1rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rst::R`](R) reader structure"]
impl crate::Readable for Ahb1rstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1rst::W`](W) writer structure"]
impl crate::Writable for Ahb1rstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1RST to value 0"]
impl crate::Resettable for Ahb1rstSpec {}
