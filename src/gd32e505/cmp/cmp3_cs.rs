#[doc = "Register `CMP3_CS` reader"]
pub type R = crate::R<Cmp3CsSpec>;
#[doc = "Register `CMP3_CS` writer"]
pub type W = crate::W<Cmp3CsSpec>;
#[doc = "Field `CMP3EN` reader - Comparator 3 enable"]
pub type Cmp3enR = crate::BitReader;
#[doc = "Field `CMP3EN` writer - Comparator 3 enable"]
pub type Cmp3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3MSEL` reader - Comparator 3 input selection"]
pub type Cmp3mselR = crate::FieldReader;
#[doc = "Field `CMP3MSEL` writer - Comparator 3 input selection"]
pub type Cmp3mselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP3OSEL` reader - Comparator 3 output selection"]
pub type Cmp3oselR = crate::FieldReader;
#[doc = "Field `CMP3OSEL` writer - Comparator 3 output selection"]
pub type Cmp3oselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CMP3PL` reader - Polarity of comparator 3 output"]
pub type Cmp3plR = crate::BitReader;
#[doc = "Field `CMP3PL` writer - Polarity of comparator 3 output"]
pub type Cmp3plW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3BLK` reader - CMP3 output blanking source"]
pub type Cmp3blkR = crate::FieldReader;
#[doc = "Field `CMP3BLK` writer - CMP3 output blanking source"]
pub type Cmp3blkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP3MSEL_3` reader - CMP3_IM input selection"]
pub type Cmp3msel3R = crate::BitReader;
#[doc = "Field `CMP3MSEL_3` writer - CMP3_IM input selection"]
pub type Cmp3msel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP3O` reader - CMP3 output"]
pub type Cmp3oR = crate::BitReader;
#[doc = "Field `CMP3LK` reader - Comparator 3 lock"]
pub type Cmp3lkR = crate::BitReader;
#[doc = "Field `CMP3LK` writer - Comparator 3 lock"]
pub type Cmp3lkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 3 enable"]
    #[inline(always)]
    pub fn cmp3en(&self) -> Cmp3enR {
        Cmp3enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 3 input selection"]
    #[inline(always)]
    pub fn cmp3msel(&self) -> Cmp3mselR {
        Cmp3mselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 3 output selection"]
    #[inline(always)]
    pub fn cmp3osel(&self) -> Cmp3oselR {
        Cmp3oselR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Polarity of comparator 3 output"]
    #[inline(always)]
    pub fn cmp3pl(&self) -> Cmp3plR {
        Cmp3plR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:20 - CMP3 output blanking source"]
    #[inline(always)]
    pub fn cmp3blk(&self) -> Cmp3blkR {
        Cmp3blkR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - CMP3_IM input selection"]
    #[inline(always)]
    pub fn cmp3msel_3(&self) -> Cmp3msel3R {
        Cmp3msel3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 30 - CMP3 output"]
    #[inline(always)]
    pub fn cmp3o(&self) -> Cmp3oR {
        Cmp3oR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 3 lock"]
    #[inline(always)]
    pub fn cmp3lk(&self) -> Cmp3lkR {
        Cmp3lkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 3 enable"]
    #[inline(always)]
    pub fn cmp3en(&mut self) -> Cmp3enW<'_, Cmp3CsSpec> {
        Cmp3enW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Comparator 3 input selection"]
    #[inline(always)]
    pub fn cmp3msel(&mut self) -> Cmp3mselW<'_, Cmp3CsSpec> {
        Cmp3mselW::new(self, 4)
    }
    #[doc = "Bits 10:13 - Comparator 3 output selection"]
    #[inline(always)]
    pub fn cmp3osel(&mut self) -> Cmp3oselW<'_, Cmp3CsSpec> {
        Cmp3oselW::new(self, 10)
    }
    #[doc = "Bit 15 - Polarity of comparator 3 output"]
    #[inline(always)]
    pub fn cmp3pl(&mut self) -> Cmp3plW<'_, Cmp3CsSpec> {
        Cmp3plW::new(self, 15)
    }
    #[doc = "Bits 18:20 - CMP3 output blanking source"]
    #[inline(always)]
    pub fn cmp3blk(&mut self) -> Cmp3blkW<'_, Cmp3CsSpec> {
        Cmp3blkW::new(self, 18)
    }
    #[doc = "Bit 22 - CMP3_IM input selection"]
    #[inline(always)]
    pub fn cmp3msel_3(&mut self) -> Cmp3msel3W<'_, Cmp3CsSpec> {
        Cmp3msel3W::new(self, 22)
    }
    #[doc = "Bit 31 - Comparator 3 lock"]
    #[inline(always)]
    pub fn cmp3lk(&mut self) -> Cmp3lkW<'_, Cmp3CsSpec> {
        Cmp3lkW::new(self, 31)
    }
}
#[doc = "CMP3 control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp3_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp3_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp3CsSpec;
impl crate::RegisterSpec for Cmp3CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp3_cs::R`](R) reader structure"]
impl crate::Readable for Cmp3CsSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp3_cs::W`](W) writer structure"]
impl crate::Writable for Cmp3CsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP3_CS to value 0"]
impl crate::Resettable for Cmp3CsSpec {}
