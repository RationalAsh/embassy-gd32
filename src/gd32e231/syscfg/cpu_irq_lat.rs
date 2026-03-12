#[doc = "Register `CPU_IRQ_LAT` reader"]
pub type R = crate::R<CpuIrqLatSpec>;
#[doc = "Register `CPU_IRQ_LAT` writer"]
pub type W = crate::W<CpuIrqLatSpec>;
#[doc = "Field `IRQ_LATENCY` reader - specifies the minimum number of cycles between an interrupt"]
pub type IrqLatencyR = crate::FieldReader;
#[doc = "Field `IRQ_LATENCY` writer - specifies the minimum number of cycles between an interrupt"]
pub type IrqLatencyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - specifies the minimum number of cycles between an interrupt"]
    #[inline(always)]
    pub fn irq_latency(&self) -> IrqLatencyR {
        IrqLatencyR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - specifies the minimum number of cycles between an interrupt"]
    #[inline(always)]
    pub fn irq_latency(&mut self) -> IrqLatencyW<'_, CpuIrqLatSpec> {
        IrqLatencyW::new(self, 0)
    }
}
#[doc = "IRQ Latency register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_irq_lat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_irq_lat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuIrqLatSpec;
impl crate::RegisterSpec for CpuIrqLatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_irq_lat::R`](R) reader structure"]
impl crate::Readable for CpuIrqLatSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_irq_lat::W`](W) writer structure"]
impl crate::Writable for CpuIrqLatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_IRQ_LAT to value 0"]
impl crate::Resettable for CpuIrqLatSpec {}
