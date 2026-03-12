#[doc = "Register `OUT1_L12DH` reader"]
pub type R = crate::R<Out1L12dhSpec>;
#[doc = "Register `OUT1_L12DH` writer"]
pub type W = crate::W<Out1L12dhSpec>;
#[doc = "Field `OUT1_DH` reader - DAC_OUT1 12-bit left-aligned data"]
pub type Out1DhR = crate::FieldReader<u16>;
#[doc = "Field `OUT1_DH` writer - DAC_OUT1 12-bit left-aligned data"]
pub type Out1DhW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC_OUT1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn out1_dh(&self) -> Out1DhR {
        Out1DhR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC_OUT1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn out1_dh(&mut self) -> Out1DhW<'_, Out1L12dhSpec> {
        Out1DhW::new(self, 4)
    }
}
#[doc = "DAC_OUT1 12-bit left aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`out1_l12dh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1_l12dh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Out1L12dhSpec;
impl crate::RegisterSpec for Out1L12dhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out1_l12dh::R`](R) reader structure"]
impl crate::Readable for Out1L12dhSpec {}
#[doc = "`write(|w| ..)` method takes [`out1_l12dh::W`](W) writer structure"]
impl crate::Writable for Out1L12dhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT1_L12DH to value 0"]
impl crate::Resettable for Out1L12dhSpec {}
