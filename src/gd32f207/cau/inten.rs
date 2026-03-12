#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `IINTEN` reader - In FIFO interrupt enable"]
pub type IintenR = crate::BitReader;
#[doc = "Field `IINTEN` writer - In FIFO interrupt enable"]
pub type IintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OINTEN` reader - Out FIFO interrupt enable"]
pub type OintenR = crate::BitReader;
#[doc = "Field `OINTEN` writer - Out FIFO interrupt enable"]
pub type OintenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - In FIFO interrupt enable"]
    #[inline(always)]
    pub fn iinten(&self) -> IintenR {
        IintenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Out FIFO interrupt enable"]
    #[inline(always)]
    pub fn ointen(&self) -> OintenR {
        OintenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - In FIFO interrupt enable"]
    #[inline(always)]
    pub fn iinten(&mut self) -> IintenW<'_, IntenSpec> {
        IintenW::new(self, 0)
    }
    #[doc = "Bit 1 - Out FIFO interrupt enable"]
    #[inline(always)]
    pub fn ointen(&mut self) -> OintenW<'_, IntenSpec> {
        OintenW::new(self, 1)
    }
}
#[doc = "CAU interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
