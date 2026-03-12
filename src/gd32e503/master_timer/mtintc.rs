#[doc = "Register `MTINTC` writer"]
pub type W = crate::W<MtintcSpec>;
#[doc = "Field `CMP0IFC` writer - Clear compare 0 interrupt flag"]
pub type Cmp0ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1IFC` writer - Clear compare 1 interrupt flag"]
pub type Cmp1ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2IFC` writer - Clear compare 2 interrupt flag"]
pub type Cmp2ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3IFC` writer - Clear compare 3 interrupt flag"]
pub type Cmp3ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPIFC` writer - Clear repetition interrupt flag"]
pub type RepifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNIIFC` writer - Clear synchronization input interrupt flag"]
pub type SyniifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPIFC` writer - Clear update interrupt flag"]
pub type UpifcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear compare 0 interrupt flag"]
    #[inline(always)]
    pub fn cmp0ifc(&mut self) -> Cmp0ifcW<'_, MtintcSpec> {
        Cmp0ifcW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cmp1ifc(&mut self) -> Cmp1ifcW<'_, MtintcSpec> {
        Cmp1ifcW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cmp2ifc(&mut self) -> Cmp2ifcW<'_, MtintcSpec> {
        Cmp2ifcW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cmp3ifc(&mut self) -> Cmp3ifcW<'_, MtintcSpec> {
        Cmp3ifcW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear repetition interrupt flag"]
    #[inline(always)]
    pub fn repifc(&mut self) -> RepifcW<'_, MtintcSpec> {
        RepifcW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear synchronization input interrupt flag"]
    #[inline(always)]
    pub fn syniifc(&mut self) -> SyniifcW<'_, MtintcSpec> {
        SyniifcW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear update interrupt flag"]
    #[inline(always)]
    pub fn upifc(&mut self) -> UpifcW<'_, MtintcSpec> {
        UpifcW::new(self, 6)
    }
}
#[doc = "SHRTIMER Master_TIMER interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtintc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtintcSpec;
impl crate::RegisterSpec for MtintcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mtintc::W`](W) writer structure"]
impl crate::Writable for MtintcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTINTC to value 0"]
impl crate::Resettable for MtintcSpec {}
