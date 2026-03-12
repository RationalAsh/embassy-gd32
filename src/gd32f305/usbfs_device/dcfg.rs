#[doc = "Register `DCFG` reader"]
pub type R = crate::R<DcfgSpec>;
#[doc = "Register `DCFG` writer"]
pub type W = crate::W<DcfgSpec>;
#[doc = "Field `DS` reader - Device speed"]
pub type DsR = crate::FieldReader;
#[doc = "Field `DS` writer - Device speed"]
pub type DsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NZLSOH` reader - Non-zero-length status OUT handshake"]
pub type NzlsohR = crate::BitReader;
#[doc = "Field `NZLSOH` writer - Non-zero-length status OUT handshake"]
pub type NzlsohW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAR` reader - Device address"]
pub type DarR = crate::FieldReader;
#[doc = "Field `DAR` writer - Device address"]
pub type DarW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EOPFT` reader - end of periodic frame time"]
pub type EopftR = crate::FieldReader;
#[doc = "Field `EOPFT` writer - end of periodic frame time"]
pub type EopftW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsoh(&self) -> NzlsohR {
        NzlsohR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dar(&self) -> DarR {
        DarR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - end of periodic frame time"]
    #[inline(always)]
    pub fn eopft(&self) -> EopftR {
        EopftR::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn ds(&mut self) -> DsW<'_, DcfgSpec> {
        DsW::new(self, 0)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsoh(&mut self) -> NzlsohW<'_, DcfgSpec> {
        NzlsohW::new(self, 2)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dar(&mut self) -> DarW<'_, DcfgSpec> {
        DarW::new(self, 4)
    }
    #[doc = "Bits 11:12 - end of periodic frame time"]
    #[inline(always)]
    pub fn eopft(&mut self) -> EopftW<'_, DcfgSpec> {
        EopftW::new(self, 11)
    }
}
#[doc = "device configuration register (DCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcfgSpec;
impl crate::RegisterSpec for DcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCFG to value 0"]
impl crate::Resettable for DcfgSpec {}
