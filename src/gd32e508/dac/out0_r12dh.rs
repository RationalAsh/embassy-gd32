#[doc = "Register `OUT0_R12DH` reader"]
pub type R = crate::R<Out0R12dhSpec>;
#[doc = "Register `OUT0_R12DH` writer"]
pub type W = crate::W<Out0R12dhSpec>;
#[doc = "Field `OUT0_DH` reader - DAC_OUT0 12-bit right-aligned data"]
pub type Out0DhR = crate::FieldReader<u16>;
#[doc = "Field `OUT0_DH` writer - DAC_OUT0 12-bit right-aligned data"]
pub type Out0DhW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC_OUT0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn out0_dh(&self) -> Out0DhR {
        Out0DhR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC_OUT0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn out0_dh(&mut self) -> Out0DhW<'_, Out0R12dhSpec> {
        Out0DhW::new(self, 0)
    }
}
#[doc = "DAC_OUT0 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`out0_r12dh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out0_r12dh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Out0R12dhSpec;
impl crate::RegisterSpec for Out0R12dhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out0_r12dh::R`](R) reader structure"]
impl crate::Readable for Out0R12dhSpec {}
#[doc = "`write(|w| ..)` method takes [`out0_r12dh::W`](W) writer structure"]
impl crate::Writable for Out0R12dhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT0_R12DH to value 0"]
impl crate::Resettable for Out0R12dhSpec {}
