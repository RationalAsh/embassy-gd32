#[doc = "Register `TMI1` reader"]
pub type R = crate::R<Tmi1Spec>;
#[doc = "Register `TMI1` writer"]
pub type W = crate::W<Tmi1Spec>;
#[doc = "Field `TEN` reader - Transmit enable"]
pub type TenR = crate::BitReader;
#[doc = "Field `TEN` writer - Transmit enable"]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT` reader - Frame type"]
pub type FtR = crate::BitReader;
#[doc = "Field `FT` writer - Frame type"]
pub type FtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FF` reader - Frame format"]
pub type FfR = crate::BitReader;
#[doc = "Field `FF` writer - Frame format"]
pub type FfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFID` reader - The frame identifier"]
pub type EfidR = crate::FieldReader<u32>;
#[doc = "Field `EFID` writer - The frame identifier"]
pub type EfidW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SFID_EFID` reader - The frame identifier"]
pub type SfidEfidR = crate::FieldReader<u16>;
#[doc = "Field `SFID_EFID` writer - The frame identifier"]
pub type SfidEfidW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&self) -> FtR {
        FtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FfR {
        FfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EfidR {
        EfidR::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&self) -> SfidEfidR {
        SfidEfidR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TenW<'_, Tmi1Spec> {
        TenW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&mut self) -> FtW<'_, Tmi1Spec> {
        FtW::new(self, 1)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&mut self) -> FfW<'_, Tmi1Spec> {
        FfW::new(self, 2)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&mut self) -> EfidW<'_, Tmi1Spec> {
        EfidW::new(self, 3)
    }
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&mut self) -> SfidEfidW<'_, Tmi1Spec> {
        SfidEfidW::new(self, 21)
    }
}
#[doc = "Transmit mailbox identifier register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tmi1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmi1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmi1Spec;
impl crate::RegisterSpec for Tmi1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmi1::R`](R) reader structure"]
impl crate::Readable for Tmi1Spec {}
#[doc = "`write(|w| ..)` method takes [`tmi1::W`](W) writer structure"]
impl crate::Writable for Tmi1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMI1 to value 0"]
impl crate::Resettable for Tmi1Spec {}
