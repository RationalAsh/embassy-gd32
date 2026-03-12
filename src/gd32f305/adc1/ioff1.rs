#[doc = "Register `IOFF1` reader"]
pub type R = crate::R<Ioff1Spec>;
#[doc = "Register `IOFF1` writer"]
pub type W = crate::W<Ioff1Spec>;
#[doc = "Field `IOFF` reader - Data offset for inserted channel 1"]
pub type IoffR = crate::FieldReader<u16>;
#[doc = "Field `IOFF` writer - Data offset for inserted channel 1"]
pub type IoffW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:11 - Data offset for inserted channel 1"]
    #[inline(always)]
    pub fn ioff(&self) -> IoffR {
        IoffR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for inserted channel 1"]
    #[inline(always)]
    pub fn ioff(&mut self) -> IoffW<'_, Ioff1Spec> {
        IoffW::new(self, 0)
    }
}
#[doc = "Inserted channel data offset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ioff1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioff1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioff1Spec;
impl crate::RegisterSpec for Ioff1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioff1::R`](R) reader structure"]
impl crate::Readable for Ioff1Spec {}
#[doc = "`write(|w| ..)` method takes [`ioff1::W`](W) writer structure"]
impl crate::Writable for Ioff1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOFF1 to value 0"]
impl crate::Resettable for Ioff1Spec {}
