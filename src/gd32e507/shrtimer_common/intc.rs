#[doc = "Register `INTC` writer"]
pub type W = crate::W<IntcSpec>;
#[doc = "Field `FLT0IFC` writer - Fault 0 interrupt flag"]
pub type Flt0ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1IF` writer - Fault 1 interrupt flag"]
pub type Flt1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2IFC` writer - Clear fault 2 interrupt flag"]
pub type Flt2ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3IFC` writer - Clear fault 3 interrupt flag"]
pub type Flt3ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4IFC` writer - Clear fault 4 interrupt flag"]
pub type Flt4ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSFLTIFC` writer - Clear system fault interrupt flag"]
pub type SysfltifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLCALIF` writer - Clear DLL calibration completed interrupt flag"]
pub type DllcalifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMPERIFC` writer - Clear bunch mode period interrupt flag"]
pub type BmperifcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Fault 0 interrupt flag"]
    #[inline(always)]
    pub fn flt0ifc(&mut self) -> Flt0ifcW<'_, IntcSpec> {
        Flt0ifcW::new(self, 0)
    }
    #[doc = "Bit 1 - Fault 1 interrupt flag"]
    #[inline(always)]
    pub fn flt1if(&mut self) -> Flt1ifW<'_, IntcSpec> {
        Flt1ifW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear fault 2 interrupt flag"]
    #[inline(always)]
    pub fn flt2ifc(&mut self) -> Flt2ifcW<'_, IntcSpec> {
        Flt2ifcW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear fault 3 interrupt flag"]
    #[inline(always)]
    pub fn flt3ifc(&mut self) -> Flt3ifcW<'_, IntcSpec> {
        Flt3ifcW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear fault 4 interrupt flag"]
    #[inline(always)]
    pub fn flt4ifc(&mut self) -> Flt4ifcW<'_, IntcSpec> {
        Flt4ifcW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear system fault interrupt flag"]
    #[inline(always)]
    pub fn sysfltifc(&mut self) -> SysfltifcW<'_, IntcSpec> {
        SysfltifcW::new(self, 5)
    }
    #[doc = "Bit 16 - Clear DLL calibration completed interrupt flag"]
    #[inline(always)]
    pub fn dllcalif(&mut self) -> DllcalifW<'_, IntcSpec> {
        DllcalifW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear bunch mode period interrupt flag"]
    #[inline(always)]
    pub fn bmperifc(&mut self) -> BmperifcW<'_, IntcSpec> {
        BmperifcW::new(self, 17)
    }
}
#[doc = "SHRTIMER interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcSpec;
impl crate::RegisterSpec for IntcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for IntcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for IntcSpec {}
