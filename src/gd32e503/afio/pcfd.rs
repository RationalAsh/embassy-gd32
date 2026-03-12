#[doc = "Register `PCFD` reader"]
pub type R = crate::R<PcfdSpec>;
#[doc = "Register `PCFD` writer"]
pub type W = crate::W<PcfdSpec>;
#[doc = "Field `PD4_AFCFG` reader - PD4 AF function configuration bitse"]
pub type Pd4AfcfgR = crate::BitReader;
#[doc = "Field `PD4_AFCFG` writer - PD4 AF function configuration bitse"]
pub type Pd4AfcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5_AFCFG` reader - PD5 AF function configuration bitse"]
pub type Pd5AfcfgR = crate::BitReader;
#[doc = "Field `PD5_AFCFG` writer - PD5 AF function configuration bitse"]
pub type Pd5AfcfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - PD4 AF function configuration bitse"]
    #[inline(always)]
    pub fn pd4_afcfg(&self) -> Pd4AfcfgR {
        Pd4AfcfgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - PD5 AF function configuration bitse"]
    #[inline(always)]
    pub fn pd5_afcfg(&self) -> Pd5AfcfgR {
        Pd5AfcfgR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - PD4 AF function configuration bitse"]
    #[inline(always)]
    pub fn pd4_afcfg(&mut self) -> Pd4AfcfgW<'_, PcfdSpec> {
        Pd4AfcfgW::new(self, 8)
    }
    #[doc = "Bit 10 - PD5 AF function configuration bitse"]
    #[inline(always)]
    pub fn pd5_afcfg(&mut self) -> Pd5AfcfgW<'_, PcfdSpec> {
        Pd5AfcfgW::new(self, 10)
    }
}
#[doc = "AFIO port configuration register D\n\nYou can [`read`](crate::Reg::read) this register and get [`pcfd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcfdSpec;
impl crate::RegisterSpec for PcfdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcfd::R`](R) reader structure"]
impl crate::Readable for PcfdSpec {}
#[doc = "`write(|w| ..)` method takes [`pcfd::W`](W) writer structure"]
impl crate::Writable for PcfdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCFD to value 0"]
impl crate::Resettable for PcfdSpec {}
