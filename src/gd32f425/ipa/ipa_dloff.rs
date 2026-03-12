#[doc = "Register `IPA_DLOFF` reader"]
pub type R = crate::R<IpaDloffSpec>;
#[doc = "Register `IPA_DLOFF` writer"]
pub type W = crate::W<IpaDloffSpec>;
#[doc = "Field `DLOFF` reader - Destination line offset"]
pub type DloffR = crate::FieldReader<u16>;
#[doc = "Field `DLOFF` writer - Destination line offset"]
pub type DloffW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Destination line offset"]
    #[inline(always)]
    pub fn dloff(&self) -> DloffR {
        DloffR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Destination line offset"]
    #[inline(always)]
    pub fn dloff(&mut self) -> DloffW<'_, IpaDloffSpec> {
        DloffW::new(self, 0)
    }
}
#[doc = "Destination line offset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipa_dloff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipa_dloff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpaDloffSpec;
impl crate::RegisterSpec for IpaDloffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipa_dloff::R`](R) reader structure"]
impl crate::Readable for IpaDloffSpec {}
#[doc = "`write(|w| ..)` method takes [`ipa_dloff::W`](W) writer structure"]
impl crate::Writable for IpaDloffSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPA_DLOFF to value 0"]
impl crate::Resettable for IpaDloffSpec {}
