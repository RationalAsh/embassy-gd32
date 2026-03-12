#[doc = "Register `DACC_R12DH` reader"]
pub type R = crate::R<DaccR12dhSpec>;
#[doc = "Register `DACC_R12DH` writer"]
pub type W = crate::W<DaccR12dhSpec>;
#[doc = "Field `OUT0_DH` reader - DAC_OUT0 12-bit right-aligned data"]
pub type Out0DhR = crate::FieldReader<u16>;
#[doc = "Field `OUT0_DH` writer - DAC_OUT0 12-bit right-aligned data"]
pub type Out0DhW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `OUT1_DH` reader - DAC_OUT1 12-bit right-aligned data"]
pub type Out1DhR = crate::FieldReader<u16>;
#[doc = "Field `OUT1_DH` writer - DAC_OUT1 12-bit right-aligned data"]
pub type Out1DhW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC_OUT0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn out0_dh(&self) -> Out0DhR {
        Out0DhR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC_OUT1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn out1_dh(&self) -> Out1DhR {
        Out1DhR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC_OUT0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn out0_dh(&mut self) -> Out0DhW<'_, DaccR12dhSpec> {
        Out0DhW::new(self, 0)
    }
    #[doc = "Bits 16:27 - DAC_OUT1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn out1_dh(&mut self) -> Out1DhW<'_, DaccR12dhSpec> {
        Out1DhW::new(self, 16)
    }
}
#[doc = "DAC concurrent mode 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dacc_r12dh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacc_r12dh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaccR12dhSpec;
impl crate::RegisterSpec for DaccR12dhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacc_r12dh::R`](R) reader structure"]
impl crate::Readable for DaccR12dhSpec {}
#[doc = "`write(|w| ..)` method takes [`dacc_r12dh::W`](W) writer structure"]
impl crate::Writable for DaccR12dhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DACC_R12DH to value 0"]
impl crate::Resettable for DaccR12dhSpec {}
