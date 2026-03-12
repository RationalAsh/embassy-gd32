#[doc = "Register `HCH5LEN` reader"]
pub type R = crate::R<Hch5lenSpec>;
#[doc = "Register `HCH5LEN` writer"]
pub type W = crate::W<Hch5lenSpec>;
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TlenR = crate::FieldReader<u32>;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TlenW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PCNT` reader - Packet count"]
pub type PcntR = crate::FieldReader<u16>;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DpidR = crate::FieldReader;
#[doc = "Field `DPID` writer - Data PID"]
pub type DpidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&mut self) -> TlenW<'_, Hch5lenSpec> {
        TlenW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&mut self) -> PcntW<'_, Hch5lenSpec> {
        PcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&mut self) -> DpidW<'_, Hch5lenSpec> {
        DpidW::new(self, 29)
    }
}
#[doc = "host channel-5 transfer length register\n\nYou can [`read`](crate::Reg::read) this register and get [`hch5len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hch5len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hch5lenSpec;
impl crate::RegisterSpec for Hch5lenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch5len::R`](R) reader structure"]
impl crate::Readable for Hch5lenSpec {}
#[doc = "`write(|w| ..)` method takes [`hch5len::W`](W) writer structure"]
impl crate::Writable for Hch5lenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCH5LEN to value 0"]
impl crate::Resettable for Hch5lenSpec {}
