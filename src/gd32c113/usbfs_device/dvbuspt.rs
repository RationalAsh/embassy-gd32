#[doc = "Register `DVBUSPT` reader"]
pub type R = crate::R<DvbusptSpec>;
#[doc = "Register `DVBUSPT` writer"]
pub type W = crate::W<DvbusptSpec>;
#[doc = "Field `DVBUSPT` reader - Device VBUS pulsing time"]
pub type DvbusptR = crate::FieldReader<u16>;
#[doc = "Field `DVBUSPT` writer - Device VBUS pulsing time"]
pub type DvbusptW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbuspt(&self) -> DvbusptR {
        DvbusptR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbuspt(&mut self) -> DvbusptW<'_, DvbusptSpec> {
        DvbusptW::new(self, 0)
    }
}
#[doc = "device VBUS pulsing time register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbuspt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbuspt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DvbusptSpec;
impl crate::RegisterSpec for DvbusptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvbuspt::R`](R) reader structure"]
impl crate::Readable for DvbusptSpec {}
#[doc = "`write(|w| ..)` method takes [`dvbuspt::W`](W) writer structure"]
impl crate::Writable for DvbusptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVBUSPT to value 0x05b8"]
impl crate::Resettable for DvbusptSpec {
    const RESET_VALUE: u32 = 0x05b8;
}
