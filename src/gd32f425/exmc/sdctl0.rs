#[doc = "Register `SDCTL0` reader"]
pub type R = crate::R<Sdctl0Spec>;
#[doc = "Register `SDCTL0` writer"]
pub type W = crate::W<Sdctl0Spec>;
#[doc = "Field `CAW` reader - Column address bit width"]
pub type CawR = crate::FieldReader;
#[doc = "Field `CAW` writer - Column address bit width"]
pub type CawW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RAW` reader - Row address bit width"]
pub type RawR = crate::FieldReader;
#[doc = "Field `RAW` writer - Row address bit width"]
pub type RawW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDW` reader - SDRAM data bus width"]
pub type SdwR = crate::FieldReader;
#[doc = "Field `SDW` writer - SDRAM data bus width"]
pub type SdwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NBK` reader - Number of banks"]
pub type NbkR = crate::BitReader;
#[doc = "Field `NBK` writer - Number of banks"]
pub type NbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CL` reader - CAS Latency"]
pub type ClR = crate::FieldReader;
#[doc = "Field `CL` writer - CAS Latency"]
pub type ClW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WPEN` reader - Write protection enable"]
pub type WpenR = crate::BitReader;
#[doc = "Field `WPEN` writer - Write protection enable"]
pub type WpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDCLK` reader - SDRAM clock configuration"]
pub type SdclkR = crate::FieldReader;
#[doc = "Field `SDCLK` writer - SDRAM clock configuration"]
pub type SdclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BRSTRD` reader - Burst read"]
pub type BrstrdR = crate::BitReader;
#[doc = "Field `BRSTRD` writer - Burst read"]
pub type BrstrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIPED` reader - Pipeline delay"]
pub type PipedR = crate::FieldReader;
#[doc = "Field `PIPED` writer - Pipeline delay"]
pub type PipedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Column address bit width"]
    #[inline(always)]
    pub fn caw(&self) -> CawR {
        CawR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Row address bit width"]
    #[inline(always)]
    pub fn raw(&self) -> RawR {
        RawR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SDRAM data bus width"]
    #[inline(always)]
    pub fn sdw(&self) -> SdwR {
        SdwR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Number of banks"]
    #[inline(always)]
    pub fn nbk(&self) -> NbkR {
        NbkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - CAS Latency"]
    #[inline(always)]
    pub fn cl(&self) -> ClR {
        ClR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Write protection enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WpenR {
        WpenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - SDRAM clock configuration"]
    #[inline(always)]
    pub fn sdclk(&self) -> SdclkR {
        SdclkR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Burst read"]
    #[inline(always)]
    pub fn brstrd(&self) -> BrstrdR {
        BrstrdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Pipeline delay"]
    #[inline(always)]
    pub fn piped(&self) -> PipedR {
        PipedR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Column address bit width"]
    #[inline(always)]
    pub fn caw(&mut self) -> CawW<'_, Sdctl0Spec> {
        CawW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Row address bit width"]
    #[inline(always)]
    pub fn raw(&mut self) -> RawW<'_, Sdctl0Spec> {
        RawW::new(self, 2)
    }
    #[doc = "Bits 4:5 - SDRAM data bus width"]
    #[inline(always)]
    pub fn sdw(&mut self) -> SdwW<'_, Sdctl0Spec> {
        SdwW::new(self, 4)
    }
    #[doc = "Bit 6 - Number of banks"]
    #[inline(always)]
    pub fn nbk(&mut self) -> NbkW<'_, Sdctl0Spec> {
        NbkW::new(self, 6)
    }
    #[doc = "Bits 7:8 - CAS Latency"]
    #[inline(always)]
    pub fn cl(&mut self) -> ClW<'_, Sdctl0Spec> {
        ClW::new(self, 7)
    }
    #[doc = "Bit 9 - Write protection enable"]
    #[inline(always)]
    pub fn wpen(&mut self) -> WpenW<'_, Sdctl0Spec> {
        WpenW::new(self, 9)
    }
    #[doc = "Bits 10:11 - SDRAM clock configuration"]
    #[inline(always)]
    pub fn sdclk(&mut self) -> SdclkW<'_, Sdctl0Spec> {
        SdclkW::new(self, 10)
    }
    #[doc = "Bit 12 - Burst read"]
    #[inline(always)]
    pub fn brstrd(&mut self) -> BrstrdW<'_, Sdctl0Spec> {
        BrstrdW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Pipeline delay"]
    #[inline(always)]
    pub fn piped(&mut self) -> PipedW<'_, Sdctl0Spec> {
        PipedW::new(self, 13)
    }
}
#[doc = "SDRAM control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sdctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdctl0Spec;
impl crate::RegisterSpec for Sdctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdctl0::R`](R) reader structure"]
impl crate::Readable for Sdctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`sdctl0::W`](W) writer structure"]
impl crate::Writable for Sdctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDCTL0 to value 0x02d0"]
impl crate::Resettable for Sdctl0Spec {
    const RESET_VALUE: u32 = 0x02d0;
}
