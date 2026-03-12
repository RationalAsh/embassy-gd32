#[doc = "Register `IRMP` reader"]
pub type R = crate::R<IrmpSpec>;
#[doc = "Register `IRMP` writer"]
pub type W = crate::W<IrmpSpec>;
#[doc = "Field `ITI1_RMP` reader - Internal trigger input1 remap"]
pub type Iti1RmpR = crate::FieldReader;
#[doc = "Field `ITI1_RMP` writer - Internal trigger input1 remap"]
pub type Iti1RmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 10:11 - Internal trigger input1 remap"]
    #[inline(always)]
    pub fn iti1_rmp(&self) -> Iti1RmpR {
        Iti1RmpR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 10:11 - Internal trigger input1 remap"]
    #[inline(always)]
    pub fn iti1_rmp(&mut self) -> Iti1RmpW<'_, IrmpSpec> {
        Iti1RmpW::new(self, 10)
    }
}
#[doc = "channel input remap register\n\nYou can [`read`](crate::Reg::read) this register and get [`irmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrmpSpec;
impl crate::RegisterSpec for IrmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irmp::R`](R) reader structure"]
impl crate::Readable for IrmpSpec {}
#[doc = "`write(|w| ..)` method takes [`irmp::W`](W) writer structure"]
impl crate::Writable for IrmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRMP to value 0"]
impl crate::Resettable for IrmpSpec {}
