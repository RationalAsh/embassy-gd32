#[doc = "Register `TDATA` writer"]
pub type W = crate::W<TdataSpec>;
#[doc = "Field `TXDATA` writer - Tx Data register"]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Tx Data register"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TxdataW<'_, TdataSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "Transmit data register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdataSpec;
impl crate::RegisterSpec for TdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdata::W`](W) writer structure"]
impl crate::Writable for TdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDATA to value 0"]
impl crate::Resettable for TdataSpec {}
