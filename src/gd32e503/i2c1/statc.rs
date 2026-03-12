#[doc = "Register `STATC` reader"]
pub type R = crate::R<StatcSpec>;
#[doc = "Register `STATC` writer"]
pub type W = crate::W<StatcSpec>;
#[doc = "Field `SBSENDC` reader - Start send status clear"]
pub type SbsendcR = crate::BitReader;
#[doc = "Field `SBSENDC` writer - Start send status clear"]
pub type SbsendcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDSENDC` reader - ADDSEND status clear"]
pub type AddsendcR = crate::BitReader;
#[doc = "Field `ADDSENDC` writer - ADDSEND status clear"]
pub type AddsendcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTCC` reader - BTC status clear"]
pub type BtccR = crate::BitReader;
#[doc = "Field `BTCC` writer - BTC status clear"]
pub type BtccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD10SENDC` reader - ADD10SEND status clear"]
pub type Add10sendcR = crate::BitReader;
#[doc = "Field `ADD10SENDC` writer - ADD10SEND status clear"]
pub type Add10sendcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPFC` reader - STOPF status clear"]
pub type StopfcR = crate::BitReader;
#[doc = "Field `STOPFC` writer - STOPF status clear"]
pub type StopfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRCEN` reader - Status register clear enable"]
pub type SrcenR = crate::BitReader;
#[doc = "Field `SRCEN` writer - Status register clear enable"]
pub type SrcenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start send status clear"]
    #[inline(always)]
    pub fn sbsendc(&self) -> SbsendcR {
        SbsendcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADDSEND status clear"]
    #[inline(always)]
    pub fn addsendc(&self) -> AddsendcR {
        AddsendcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BTC status clear"]
    #[inline(always)]
    pub fn btcc(&self) -> BtccR {
        BtccR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADD10SEND status clear"]
    #[inline(always)]
    pub fn add10sendc(&self) -> Add10sendcR {
        Add10sendcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STOPF status clear"]
    #[inline(always)]
    pub fn stopfc(&self) -> StopfcR {
        StopfcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 15 - Status register clear enable"]
    #[inline(always)]
    pub fn srcen(&self) -> SrcenR {
        SrcenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start send status clear"]
    #[inline(always)]
    pub fn sbsendc(&mut self) -> SbsendcW<'_, StatcSpec> {
        SbsendcW::new(self, 0)
    }
    #[doc = "Bit 1 - ADDSEND status clear"]
    #[inline(always)]
    pub fn addsendc(&mut self) -> AddsendcW<'_, StatcSpec> {
        AddsendcW::new(self, 1)
    }
    #[doc = "Bit 2 - BTC status clear"]
    #[inline(always)]
    pub fn btcc(&mut self) -> BtccW<'_, StatcSpec> {
        BtccW::new(self, 2)
    }
    #[doc = "Bit 3 - ADD10SEND status clear"]
    #[inline(always)]
    pub fn add10sendc(&mut self) -> Add10sendcW<'_, StatcSpec> {
        Add10sendcW::new(self, 3)
    }
    #[doc = "Bit 4 - STOPF status clear"]
    #[inline(always)]
    pub fn stopfc(&mut self) -> StopfcW<'_, StatcSpec> {
        StopfcW::new(self, 4)
    }
    #[doc = "Bit 15 - Status register clear enable"]
    #[inline(always)]
    pub fn srcen(&mut self) -> SrcenW<'_, StatcSpec> {
        SrcenW::new(self, 15)
    }
}
#[doc = "Status clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`statc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatcSpec;
impl crate::RegisterSpec for StatcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statc::R`](R) reader structure"]
impl crate::Readable for StatcSpec {}
#[doc = "`write(|w| ..)` method takes [`statc::W`](W) writer structure"]
impl crate::Writable for StatcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATC to value 0"]
impl crate::Resettable for StatcSpec {}
