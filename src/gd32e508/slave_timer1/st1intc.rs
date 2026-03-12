#[doc = "Register `ST1INTC` writer"]
pub type W = crate::W<St1intcSpec>;
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
#[doc = "Field `UPIFC` writer - Clear update interrupt flag"]
pub type UpifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0IFC` writer - Clear capture 0 interrupt flag"]
pub type Cap0ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1IFC` writer - Clear capture 1 interrupt flag"]
pub type Cap1ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0OAIFC` writer - Clear channel 0 output active interrupt flag"]
pub type Ch0oaifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0ONAIFC` writer - Clear channel 0 output inactive interrupt flag"]
pub type Ch0onaifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OAIFC` writer - Clear channel 1 output active interrupt flag"]
pub type Ch1oaifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1ONAIFC` writer - Clear channel 1 output inactive interrupt flag"]
pub type Ch1onaifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIFC` writer - Clear counter reset interrupt flag"]
pub type RstifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLYIIFC` writer - Clear delayed IDLE mode entry interrupt flag"]
pub type DlyiifcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear compare 0 interrupt flag"]
    #[inline(always)]
    pub fn cmp0ifc(&mut self) -> Cmp0ifcW<'_, St1intcSpec> {
        Cmp0ifcW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cmp1ifc(&mut self) -> Cmp1ifcW<'_, St1intcSpec> {
        Cmp1ifcW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cmp2ifc(&mut self) -> Cmp2ifcW<'_, St1intcSpec> {
        Cmp2ifcW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cmp3ifc(&mut self) -> Cmp3ifcW<'_, St1intcSpec> {
        Cmp3ifcW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear repetition interrupt flag"]
    #[inline(always)]
    pub fn repifc(&mut self) -> RepifcW<'_, St1intcSpec> {
        RepifcW::new(self, 4)
    }
    #[doc = "Bit 6 - Clear update interrupt flag"]
    #[inline(always)]
    pub fn upifc(&mut self) -> UpifcW<'_, St1intcSpec> {
        UpifcW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear capture 0 interrupt flag"]
    #[inline(always)]
    pub fn cap0ifc(&mut self) -> Cap0ifcW<'_, St1intcSpec> {
        Cap0ifcW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear capture 1 interrupt flag"]
    #[inline(always)]
    pub fn cap1ifc(&mut self) -> Cap1ifcW<'_, St1intcSpec> {
        Cap1ifcW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear channel 0 output active interrupt flag"]
    #[inline(always)]
    pub fn ch0oaifc(&mut self) -> Ch0oaifcW<'_, St1intcSpec> {
        Ch0oaifcW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear channel 0 output inactive interrupt flag"]
    #[inline(always)]
    pub fn ch0onaifc(&mut self) -> Ch0onaifcW<'_, St1intcSpec> {
        Ch0onaifcW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear channel 1 output active interrupt flag"]
    #[inline(always)]
    pub fn ch1oaifc(&mut self) -> Ch1oaifcW<'_, St1intcSpec> {
        Ch1oaifcW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear channel 1 output inactive interrupt flag"]
    #[inline(always)]
    pub fn ch1onaifc(&mut self) -> Ch1onaifcW<'_, St1intcSpec> {
        Ch1onaifcW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear counter reset interrupt flag"]
    #[inline(always)]
    pub fn rstifc(&mut self) -> RstifcW<'_, St1intcSpec> {
        RstifcW::new(self, 13)
    }
    #[doc = "Bit 14 - Clear delayed IDLE mode entry interrupt flag"]
    #[inline(always)]
    pub fn dlyiifc(&mut self) -> DlyiifcW<'_, St1intcSpec> {
        DlyiifcW::new(self, 14)
    }
}
#[doc = "SHRTIMER Slave_TIMER1 interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st1intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St1intcSpec;
impl crate::RegisterSpec for St1intcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`st1intc::W`](W) writer structure"]
impl crate::Writable for St1intcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ST1INTC to value 0"]
impl crate::Resettable for St1intcSpec {}
