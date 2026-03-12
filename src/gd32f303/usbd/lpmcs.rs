#[doc = "Register `LPMCS` reader"]
pub type R = crate::R<LpmcsSpec>;
#[doc = "Register `LPMCS` writer"]
pub type W = crate::W<LpmcsSpec>;
#[doc = "Field `LPMEN` reader - LPM support enable"]
pub type LpmenR = crate::BitReader;
#[doc = "Field `LPMEN` writer - LPM support enable"]
pub type LpmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMACK` reader - LPM token acknowledge enable"]
pub type LpmackR = crate::BitReader;
#[doc = "Field `LPMACK` writer - LPM token acknowledge enable"]
pub type LpmackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMWK` reader - bRemoteWake value"]
pub type RemwkR = crate::BitReader;
#[doc = "Field `REMWK` writer - bRemoteWake value"]
pub type RemwkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLSTAT` reader - bLinkState value"]
pub type BlstatR = crate::FieldReader;
#[doc = "Field `BLSTAT` writer - bLinkState value"]
pub type BlstatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&self) -> LpmenR {
        LpmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&self) -> LpmackR {
        LpmackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwk(&self) -> RemwkR {
        RemwkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - bLinkState value"]
    #[inline(always)]
    pub fn blstat(&self) -> BlstatR {
        BlstatR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&mut self) -> LpmenW<'_, LpmcsSpec> {
        LpmenW::new(self, 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&mut self) -> LpmackW<'_, LpmcsSpec> {
        LpmackW::new(self, 1)
    }
    #[doc = "Bit 3 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwk(&mut self) -> RemwkW<'_, LpmcsSpec> {
        RemwkW::new(self, 3)
    }
    #[doc = "Bits 4:7 - bLinkState value"]
    #[inline(always)]
    pub fn blstat(&mut self) -> BlstatW<'_, LpmcsSpec> {
        BlstatW::new(self, 4)
    }
}
#[doc = "USB LPM control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmcsSpec;
impl crate::RegisterSpec for LpmcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpmcs::R`](R) reader structure"]
impl crate::Readable for LpmcsSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmcs::W`](W) writer structure"]
impl crate::Writable for LpmcsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPMCS to value 0"]
impl crate::Resettable for LpmcsSpec {}
