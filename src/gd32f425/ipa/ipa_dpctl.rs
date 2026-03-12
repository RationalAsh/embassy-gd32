#[doc = "Register `IPA_DPCTL` reader"]
pub type R = crate::R<IpaDpctlSpec>;
#[doc = "Register `IPA_DPCTL` writer"]
pub type W = crate::W<IpaDpctlSpec>;
#[doc = "Field `DPF` reader - Destination pixel format"]
pub type DpfR = crate::FieldReader;
#[doc = "Field `DPF` writer - Destination pixel format"]
pub type DpfW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Destination pixel format"]
    #[inline(always)]
    pub fn dpf(&self) -> DpfR {
        DpfR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Destination pixel format"]
    #[inline(always)]
    pub fn dpf(&mut self) -> DpfW<'_, IpaDpctlSpec> {
        DpfW::new(self, 0)
    }
}
#[doc = "Destination pixel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipa_dpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipa_dpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaDpctlSpec;
impl crate::RegisterSpec for IpaDpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_dpctl::R`](R) reader structure"]
impl crate::Readable for IpaDpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_dpctl::W`](W) writer structure"]
impl crate::Writable for IpaDpctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPA_DPCTL to value 0"]
impl crate::Resettable for IpaDpctlSpec {}
