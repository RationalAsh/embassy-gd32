#[doc = "Register `SDCMD` reader"]
pub type R = crate::R<SdcmdSpec>;
#[doc = "Register `SDCMD` writer"]
pub type W = crate::W<SdcmdSpec>;
#[doc = "Field `CMD` reader - Command"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `CMD` writer - Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DS1` reader - Device select 1"]
pub type Ds1R = crate::BitReader;
#[doc = "Field `DS1` writer - Device select 1"]
pub type Ds1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DS0` reader - Device select 0"]
pub type Ds0R = crate::BitReader;
#[doc = "Field `DS0` writer - Device select 0"]
pub type Ds0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NARF` reader - Number of successive Auto-refresh"]
pub type NarfR = crate::FieldReader;
#[doc = "Field `NARF` writer - Number of successive Auto-refresh"]
pub type NarfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MRC` reader - Mode register content"]
pub type MrcR = crate::FieldReader<u16>;
#[doc = "Field `MRC` writer - Mode register content"]
pub type MrcW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:2 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Device select 1"]
    #[inline(always)]
    pub fn ds1(&self) -> Ds1R {
        Ds1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Device select 0"]
    #[inline(always)]
    pub fn ds0(&self) -> Ds0R {
        Ds0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Number of successive Auto-refresh"]
    #[inline(always)]
    pub fn narf(&self) -> NarfR {
        NarfR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:21 - Mode register content"]
    #[inline(always)]
    pub fn mrc(&self) -> MrcR {
        MrcR::new(((self.bits >> 9) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, SdcmdSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bit 3 - Device select 1"]
    #[inline(always)]
    pub fn ds1(&mut self) -> Ds1W<'_, SdcmdSpec> {
        Ds1W::new(self, 3)
    }
    #[doc = "Bit 4 - Device select 0"]
    #[inline(always)]
    pub fn ds0(&mut self) -> Ds0W<'_, SdcmdSpec> {
        Ds0W::new(self, 4)
    }
    #[doc = "Bits 5:8 - Number of successive Auto-refresh"]
    #[inline(always)]
    pub fn narf(&mut self) -> NarfW<'_, SdcmdSpec> {
        NarfW::new(self, 5)
    }
    #[doc = "Bits 9:21 - Mode register content"]
    #[inline(always)]
    pub fn mrc(&mut self) -> MrcW<'_, SdcmdSpec> {
        MrcW::new(self, 9)
    }
}
#[doc = "SDRAM command register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdcmdSpec;
impl crate::RegisterSpec for SdcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdcmd::R`](R) reader structure"]
impl crate::Readable for SdcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`sdcmd::W`](W) writer structure"]
impl crate::Writable for SdcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDCMD to value 0"]
impl crate::Resettable for SdcmdSpec {}
