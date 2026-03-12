#[doc = "Register `DATACTL` reader"]
pub type R = crate::R<DatactlSpec>;
#[doc = "Register `DATACTL` writer"]
pub type W = crate::W<DatactlSpec>;
#[doc = "Field `DATAEN` reader - Data transfer enabled bit"]
pub type DataenR = crate::BitReader;
#[doc = "Field `DATAEN` writer - Data transfer enabled bit"]
pub type DataenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATADIR` reader - Data transfer direction"]
pub type DatadirR = crate::BitReader;
#[doc = "Field `DATADIR` writer - Data transfer direction"]
pub type DatadirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMOD` reader - Data transfer mode"]
pub type TransmodR = crate::BitReader;
#[doc = "Field `TRANSMOD` writer - Data transfer mode"]
pub type TransmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA enable bit"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA enable bit"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKSZ` reader - Data block size"]
pub type BlkszR = crate::FieldReader;
#[doc = "Field `BLKSZ` writer - Data block size"]
pub type BlkszW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RWEN` reader - Read wait mode enabled"]
pub type RwenR = crate::BitReader;
#[doc = "Field `RWEN` writer - Read wait mode enabled"]
pub type RwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWSTOP` reader - Read wait stop"]
pub type RwstopR = crate::BitReader;
#[doc = "Field `RWSTOP` writer - Read wait stop"]
pub type RwstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWTYPE` reader - Read wait type"]
pub type RwtypeR = crate::BitReader;
#[doc = "Field `RWTYPE` writer - Read wait type"]
pub type RwtypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOEN` reader - SD I/O specific function enable"]
pub type IoenR = crate::BitReader;
#[doc = "Field `IOEN` writer - SD I/O specific function enable"]
pub type IoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data transfer enabled bit"]
    #[inline(always)]
    pub fn dataen(&self) -> DataenR {
        DataenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    pub fn datadir(&self) -> DatadirR {
        DatadirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    pub fn transmod(&self) -> TransmodR {
        TransmodR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    pub fn blksz(&self) -> BlkszR {
        BlkszR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Read wait mode enabled"]
    #[inline(always)]
    pub fn rwen(&self) -> RwenR {
        RwenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    pub fn rwstop(&self) -> RwstopR {
        RwstopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read wait type"]
    #[inline(always)]
    pub fn rwtype(&self) -> RwtypeR {
        RwtypeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O specific function enable"]
    #[inline(always)]
    pub fn ioen(&self) -> IoenR {
        IoenR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data transfer enabled bit"]
    #[inline(always)]
    pub fn dataen(&mut self) -> DataenW<'_, DatactlSpec> {
        DataenW::new(self, 0)
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    pub fn datadir(&mut self) -> DatadirW<'_, DatactlSpec> {
        DatadirW::new(self, 1)
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    pub fn transmod(&mut self) -> TransmodW<'_, DatactlSpec> {
        TransmodW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, DatactlSpec> {
        DmaenW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    pub fn blksz(&mut self) -> BlkszW<'_, DatactlSpec> {
        BlkszW::new(self, 4)
    }
    #[doc = "Bit 8 - Read wait mode enabled"]
    #[inline(always)]
    pub fn rwen(&mut self) -> RwenW<'_, DatactlSpec> {
        RwenW::new(self, 8)
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    pub fn rwstop(&mut self) -> RwstopW<'_, DatactlSpec> {
        RwstopW::new(self, 9)
    }
    #[doc = "Bit 10 - Read wait type"]
    #[inline(always)]
    pub fn rwtype(&mut self) -> RwtypeW<'_, DatactlSpec> {
        RwtypeW::new(self, 10)
    }
    #[doc = "Bit 11 - SD I/O specific function enable"]
    #[inline(always)]
    pub fn ioen(&mut self) -> IoenW<'_, DatactlSpec> {
        IoenW::new(self, 11)
    }
}
#[doc = "Data control register\n\nYou can [`read`](crate::Reg::read) this register and get [`datactl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datactl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatactlSpec;
impl crate::RegisterSpec for DatactlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datactl::R`](R) reader structure"]
impl crate::Readable for DatactlSpec {}
#[doc = "`write(|w| ..)` method takes [`datactl::W`](W) writer structure"]
impl crate::Writable for DatactlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATACTL to value 0"]
impl crate::Resettable for DatactlSpec {}
