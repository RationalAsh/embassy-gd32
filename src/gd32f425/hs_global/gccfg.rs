#[doc = "Register `GCCFG` reader"]
pub type R = crate::R<GccfgSpec>;
#[doc = "Register `GCCFG` writer"]
pub type W = crate::W<GccfgSpec>;
#[doc = "Field `PWRON` reader - Power on"]
pub type PwronR = crate::BitReader;
#[doc = "Field `PWRON` writer - Power on"]
pub type PwronW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSACEN` reader - The VBUS A-device Comparer enable"]
pub type VbusacenR = crate::BitReader;
#[doc = "Field `VBUSACEN` writer - The VBUS A-device Comparer enable"]
pub type VbusacenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSBCEN` reader - The VBUS B-device Comparer enable"]
pub type VbusbcenR = crate::BitReader;
#[doc = "Field `VBUSBCEN` writer - The VBUS B-device Comparer enable"]
pub type VbusbcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFOEN` reader - SOF output enable"]
pub type SofoenR = crate::BitReader;
#[doc = "Field `SOFOEN` writer - SOF output enable"]
pub type SofoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSIG` reader - VBUS ignored"]
pub type VbusigR = crate::BitReader;
#[doc = "Field `VBUSIG` writer - VBUS ignored"]
pub type VbusigW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Power on"]
    #[inline(always)]
    pub fn pwron(&self) -> PwronR {
        PwronR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - The VBUS A-device Comparer enable"]
    #[inline(always)]
    pub fn vbusacen(&self) -> VbusacenR {
        VbusacenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The VBUS B-device Comparer enable"]
    #[inline(always)]
    pub fn vbusbcen(&self) -> VbusbcenR {
        VbusbcenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofoen(&self) -> SofoenR {
        SofoenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VBUS ignored"]
    #[inline(always)]
    pub fn vbusig(&self) -> VbusigR {
        VbusigR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Power on"]
    #[inline(always)]
    pub fn pwron(&mut self) -> PwronW<'_, GccfgSpec> {
        PwronW::new(self, 16)
    }
    #[doc = "Bit 18 - The VBUS A-device Comparer enable"]
    #[inline(always)]
    pub fn vbusacen(&mut self) -> VbusacenW<'_, GccfgSpec> {
        VbusacenW::new(self, 18)
    }
    #[doc = "Bit 19 - The VBUS B-device Comparer enable"]
    #[inline(always)]
    pub fn vbusbcen(&mut self) -> VbusbcenW<'_, GccfgSpec> {
        VbusbcenW::new(self, 19)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofoen(&mut self) -> SofoenW<'_, GccfgSpec> {
        SofoenW::new(self, 20)
    }
    #[doc = "Bit 21 - VBUS ignored"]
    #[inline(always)]
    pub fn vbusig(&mut self) -> VbusigW<'_, GccfgSpec> {
        VbusigW::new(self, 21)
    }
}
#[doc = "Global core configuration register (GCCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`gccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GccfgSpec;
impl crate::RegisterSpec for GccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gccfg::R`](R) reader structure"]
impl crate::Readable for GccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gccfg::W`](W) writer structure"]
impl crate::Writable for GccfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCCFG to value 0"]
impl crate::Resettable for GccfgSpec {}
