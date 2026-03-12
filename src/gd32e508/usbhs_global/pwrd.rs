#[doc = "Register `PWRD` reader"]
pub type R = crate::R<PwrdSpec>;
#[doc = "Register `PWRD` writer"]
pub type W = crate::W<PwrdSpec>;
#[doc = "Field `ADPMEN` reader - ADP module enable"]
pub type AdpmenR = crate::BitReader;
#[doc = "Field `ADPMEN` writer - ADP module enable"]
pub type AdpmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADPF` reader - ADP event interrupt flag"]
pub type AdpfR = crate::BitReader;
#[doc = "Field `ADPF` writer - ADP event interrupt flag"]
pub type AdpfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    pub fn adpmen(&self) -> AdpmenR {
        AdpmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 23 - ADP event interrupt flag"]
    #[inline(always)]
    pub fn adpf(&self) -> AdpfR {
        AdpfR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    pub fn adpmen(&mut self) -> AdpmenW<'_, PwrdSpec> {
        AdpmenW::new(self, 0)
    }
    #[doc = "Bit 23 - ADP event interrupt flag"]
    #[inline(always)]
    pub fn adpf(&mut self) -> AdpfW<'_, PwrdSpec> {
        AdpfW::new(self, 23)
    }
}
#[doc = "Power down register (USBHS_PWRD)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrdSpec;
impl crate::RegisterSpec for PwrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrd::R`](R) reader structure"]
impl crate::Readable for PwrdSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrd::W`](W) writer structure"]
impl crate::Writable for PwrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRD to value 0"]
impl crate::Resettable for PwrdSpec {}
