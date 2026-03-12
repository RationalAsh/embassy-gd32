#[doc = "Register `SCUMSK` reader"]
pub type R = crate::R<ScumskSpec>;
#[doc = "Register `SCUMSK` writer"]
pub type W = crate::W<ScumskSpec>;
#[doc = "Field `FSM` reader - Frame start code mask bits in embedded synchronous mode"]
pub type FsmR = crate::FieldReader;
#[doc = "Field `FSM` writer - Frame start code mask bits in embedded synchronous mode"]
pub type FsmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LSM` reader - Line start code mask bits in embedded synchronous mode"]
pub type LsmR = crate::FieldReader;
#[doc = "Field `LSM` writer - Line start code mask bits in embedded synchronous mode"]
pub type LsmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LEM` reader - Line end code mask bits in embedded synchronous mode"]
pub type LemR = crate::FieldReader;
#[doc = "Field `LEM` writer - Line end code mask bits in embedded synchronous mode"]
pub type LemW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FEM` reader - Frame end code mask bits in embedded synchronous mode"]
pub type FemR = crate::FieldReader;
#[doc = "Field `FEM` writer - Frame end code mask bits in embedded synchronous mode"]
pub type FemW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame start code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn fsm(&self) -> FsmR {
        FsmR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn lsm(&self) -> LsmR {
        LsmR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn lem(&self) -> LemR {
        LemR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn fem(&self) -> FemR {
        FemR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn fsm(&mut self) -> FsmW<'_, ScumskSpec> {
        FsmW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Line start code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn lsm(&mut self) -> LsmW<'_, ScumskSpec> {
        LsmW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Line end code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn lem(&mut self) -> LemW<'_, ScumskSpec> {
        LemW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Frame end code mask bits in embedded synchronous mode"]
    #[inline(always)]
    pub fn fem(&mut self) -> FemW<'_, ScumskSpec> {
        FemW::new(self, 24)
    }
}
#[doc = "DCI Synchronization codes unmask register\n\nYou can [`read`](crate::Reg::read) this register and get [`scumsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scumsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScumskSpec;
impl crate::RegisterSpec for ScumskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scumsk::R`](R) reader structure"]
impl crate::Readable for ScumskSpec {}
#[doc = "`write(|w| ..)` method takes [`scumsk::W`](W) writer structure"]
impl crate::Writable for ScumskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUMSK to value 0"]
impl crate::Resettable for ScumskSpec {}
