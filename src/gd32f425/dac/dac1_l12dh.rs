#[doc = "Register `DAC1_L12DH` reader"]
pub type R = crate::R<Dac1L12dhSpec>;
#[doc = "Register `DAC1_L12DH` writer"]
pub type W = crate::W<Dac1L12dhSpec>;
#[doc = "Field `DAC1_DH` reader - DAC1 12-bit left-aligned data"]
pub type Dac1DhR = crate::FieldReader<u16>;
#[doc = "Field `DAC1_DH` writer - DAC1 12-bit left-aligned data"]
pub type Dac1DhW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&self) -> Dac1DhR {
        Dac1DhR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&mut self) -> Dac1DhW<'_, Dac1L12dhSpec> {
        Dac1DhW::new(self, 4)
    }
}
#[doc = "DAC1 12-bit left aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac1_l12dh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac1_l12dh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac1L12dhSpec;
impl crate::RegisterSpec for Dac1L12dhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1_l12dh::R`](R) reader structure"]
impl crate::Readable for Dac1L12dhSpec {}
#[doc = "`write(|w| ..)` method takes [`dac1_l12dh::W`](W) writer structure"]
impl crate::Writable for Dac1L12dhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC1_L12DH to value 0"]
impl crate::Resettable for Dac1L12dhSpec {}
