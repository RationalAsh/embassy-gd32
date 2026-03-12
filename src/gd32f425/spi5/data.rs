#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `SPI_DATA` reader - Data transfer register"]
pub type SpiDataR = crate::FieldReader<u16>;
#[doc = "Field `SPI_DATA` writer - Data transfer register"]
pub type SpiDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data transfer register"]
    #[inline(always)]
    pub fn spi_data(&self) -> SpiDataR {
        SpiDataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data transfer register"]
    #[inline(always)]
    pub fn spi_data(&mut self) -> SpiDataW<'_, DataSpec> {
        SpiDataW::new(self, 0)
    }
}
#[doc = "data register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {}
