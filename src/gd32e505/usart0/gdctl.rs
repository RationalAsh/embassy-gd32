#[doc = "Register `GDCTL` reader"]
pub type R = crate::R<GdctlSpec>;
#[doc = "Register `GDCTL` writer"]
pub type W = crate::W<GdctlSpec>;
#[doc = "Field `CDEN` reader - Collision detection enable"]
pub type CdenR = crate::BitReader;
#[doc = "Field `CDEN` writer - Collision detection enable"]
pub type CdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD` writer - Collision detected status"]
pub type CdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDIE` reader - Collision detected interrupt enable"]
pub type CdieR = crate::BitReader;
#[doc = "Field `CDIE` writer - Collision detected interrupt enable"]
pub type CdieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Collision detection enable"]
    #[inline(always)]
    pub fn cden(&self) -> CdenR {
        CdenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Collision detected interrupt enable"]
    #[inline(always)]
    pub fn cdie(&self) -> CdieR {
        CdieR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Collision detection enable"]
    #[inline(always)]
    pub fn cden(&mut self) -> CdenW<'_, GdctlSpec> {
        CdenW::new(self, 1)
    }
    #[doc = "Bit 8 - Collision detected status"]
    #[inline(always)]
    pub fn cd(&mut self) -> CdW<'_, GdctlSpec> {
        CdW::new(self, 8)
    }
    #[doc = "Bit 16 - Collision detected interrupt enable"]
    #[inline(always)]
    pub fn cdie(&mut self) -> CdieW<'_, GdctlSpec> {
        CdieW::new(self, 16)
    }
}
#[doc = "GD control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdctlSpec;
impl crate::RegisterSpec for GdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdctl::R`](R) reader structure"]
impl crate::Readable for GdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gdctl::W`](W) writer structure"]
impl crate::Writable for GdctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GDCTL to value 0"]
impl crate::Resettable for GdctlSpec {}
