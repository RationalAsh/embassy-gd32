#[doc = "Register `DIEP3LEN` reader"]
pub type R = crate::R<Diep3lenSpec>;
#[doc = "Register `DIEP3LEN` writer"]
pub type W = crate::W<Diep3lenSpec>;
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TlenR = crate::FieldReader<u32>;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TlenW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PCNT` reader - Packet count"]
pub type PcntR = crate::FieldReader<u16>;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&self) -> TlenR {
        TlenR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PcntR {
        PcntR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&mut self) -> TlenW<'_, Diep3lenSpec> {
        TlenW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&mut self) -> PcntW<'_, Diep3lenSpec> {
        PcntW::new(self, 19)
    }
}
#[doc = "device IN endpoint-3 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep3len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep3len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diep3lenSpec;
impl crate::RegisterSpec for Diep3lenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep3len::R`](R) reader structure"]
impl crate::Readable for Diep3lenSpec {}
#[doc = "`write(|w| ..)` method takes [`diep3len::W`](W) writer structure"]
impl crate::Writable for Diep3lenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEP3LEN to value 0"]
impl crate::Resettable for Diep3lenSpec {}
