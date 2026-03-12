#[doc = "Register `DAC0_L12DH` reader"]
pub type R = crate::R<Dac0L12dhSpec>;
#[doc = "Register `DAC0_L12DH` writer"]
pub type W = crate::W<Dac0L12dhSpec>;
#[doc = "Field `DAC0_DH` reader - DAC0 12-bit left-aligned data"]
pub type Dac0DhR = crate::FieldReader<u16>;
#[doc = "Field `DAC0_DH` writer - DAC0 12-bit left-aligned data"]
pub type Dac0DhW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    #[doc = "Bits 4:15 - DAC0 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&self) -> Dac0DhR {
        Dac0DhR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC0 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&mut self) -> Dac0DhW<'_, Dac0L12dhSpec> {
        Dac0DhW::new(self, 4)
    }
}
#[doc = "DAC0 12-bit left-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dac0_l12dh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac0_l12dh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac0L12dhSpec;
impl crate::RegisterSpec for Dac0L12dhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0_l12dh::R`](R) reader structure"]
impl crate::Readable for Dac0L12dhSpec {}
#[doc = "`write(|w| ..)` method takes [`dac0_l12dh::W`](W) writer structure"]
impl crate::Writable for Dac0L12dhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC0_L12DH to value 0"]
impl crate::Resettable for Dac0L12dhSpec {}
