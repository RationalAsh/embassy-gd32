#[doc = "Register `OMODE` reader"]
pub type R = crate::R<OmodeSpec>;
#[doc = "Register `OMODE` writer"]
pub type W = crate::W<OmodeSpec>;
#[doc = "Port 0 output mode bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Om0 {
    #[doc = "0: Output push-pull (reset state)"]
    PushPull = 0,
    #[doc = "1: Output open-drain"]
    OpenDrain = 1,
}
impl From<Om0> for bool {
    #[inline(always)]
    fn from(variant: Om0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OM0` reader - Port 0 output mode bit"]
pub type Om0R = crate::BitReader<Om0>;
impl Om0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Om0 {
        match self.bits {
            false => Om0::PushPull,
            true => Om0::OpenDrain,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == Om0::PushPull
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == Om0::OpenDrain
    }
}
#[doc = "Field `OM0` writer - Port 0 output mode bit"]
pub type Om0W<'a, REG> = crate::BitWriter<'a, REG, Om0>;
impl<'a, REG> Om0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(Om0::PushPull)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(Om0::OpenDrain)
    }
}
#[doc = "Field `OM1` reader - Port 1 output mode bit"]
pub use Om0R as Om1R;
#[doc = "Field `OM2` reader - Port 2 output mode bit"]
pub use Om0R as Om2R;
#[doc = "Field `OM3` reader - Port 3 output mode bit"]
pub use Om0R as Om3R;
#[doc = "Field `OM4` reader - Port 4 output mode bit"]
pub use Om0R as Om4R;
#[doc = "Field `OM5` reader - Port 5 output mode bit"]
pub use Om0R as Om5R;
#[doc = "Field `OM6` reader - Port 6 output mode bit"]
pub use Om0R as Om6R;
#[doc = "Field `OM7` reader - Port 7 output mode bit"]
pub use Om0R as Om7R;
#[doc = "Field `OM8` reader - Port 8 output mode bit"]
pub use Om0R as Om8R;
#[doc = "Field `OM9` reader - Port 9 output mode bit"]
pub use Om0R as Om9R;
#[doc = "Field `OM10` reader - Port 10 output mode bit"]
pub use Om0R as Om10R;
#[doc = "Field `OM11` reader - Port 11 output mode bit"]
pub use Om0R as Om11R;
#[doc = "Field `OM12` reader - Port 12 output mode bit"]
pub use Om0R as Om12R;
#[doc = "Field `OM13` reader - Port 13 output mode bit"]
pub use Om0R as Om13R;
#[doc = "Field `OM14` reader - Port 14 output mode bit"]
pub use Om0R as Om14R;
#[doc = "Field `OM15` reader - Port 15 output mode bit"]
pub use Om0R as Om15R;
#[doc = "Field `OM1` writer - Port 1 output mode bit"]
pub use Om0W as Om1W;
#[doc = "Field `OM2` writer - Port 2 output mode bit"]
pub use Om0W as Om2W;
#[doc = "Field `OM3` writer - Port 3 output mode bit"]
pub use Om0W as Om3W;
#[doc = "Field `OM4` writer - Port 4 output mode bit"]
pub use Om0W as Om4W;
#[doc = "Field `OM5` writer - Port 5 output mode bit"]
pub use Om0W as Om5W;
#[doc = "Field `OM6` writer - Port 6 output mode bit"]
pub use Om0W as Om6W;
#[doc = "Field `OM7` writer - Port 7 output mode bit"]
pub use Om0W as Om7W;
#[doc = "Field `OM8` writer - Port 8 output mode bit"]
pub use Om0W as Om8W;
#[doc = "Field `OM9` writer - Port 9 output mode bit"]
pub use Om0W as Om9W;
#[doc = "Field `OM10` writer - Port 10 output mode bit"]
pub use Om0W as Om10W;
#[doc = "Field `OM11` writer - Port 11 output mode bit"]
pub use Om0W as Om11W;
#[doc = "Field `OM12` writer - Port 12 output mode bit"]
pub use Om0W as Om12W;
#[doc = "Field `OM13` writer - Port 13 output mode bit"]
pub use Om0W as Om13W;
#[doc = "Field `OM14` writer - Port 14 output mode bit"]
pub use Om0W as Om14W;
#[doc = "Field `OM15` writer - Port 15 output mode bit"]
pub use Om0W as Om15W;
impl R {
    #[doc = "Bit 0 - Port 0 output mode bit"]
    #[inline(always)]
    pub fn om0(&self) -> Om0R {
        Om0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port 1 output mode bit"]
    #[inline(always)]
    pub fn om1(&self) -> Om1R {
        Om1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port 2 output mode bit"]
    #[inline(always)]
    pub fn om2(&self) -> Om2R {
        Om2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port 3 output mode bit"]
    #[inline(always)]
    pub fn om3(&self) -> Om3R {
        Om3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port 4 output mode bit"]
    #[inline(always)]
    pub fn om4(&self) -> Om4R {
        Om4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port 5 output mode bit"]
    #[inline(always)]
    pub fn om5(&self) -> Om5R {
        Om5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port 6 output mode bit"]
    #[inline(always)]
    pub fn om6(&self) -> Om6R {
        Om6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port 7 output mode bit"]
    #[inline(always)]
    pub fn om7(&self) -> Om7R {
        Om7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port 8 output mode bit"]
    #[inline(always)]
    pub fn om8(&self) -> Om8R {
        Om8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port 9 output mode bit"]
    #[inline(always)]
    pub fn om9(&self) -> Om9R {
        Om9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port 10 output mode bit"]
    #[inline(always)]
    pub fn om10(&self) -> Om10R {
        Om10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port 11 output mode bit"]
    #[inline(always)]
    pub fn om11(&self) -> Om11R {
        Om11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port 12 output mode bit"]
    #[inline(always)]
    pub fn om12(&self) -> Om12R {
        Om12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port 13 output mode bit"]
    #[inline(always)]
    pub fn om13(&self) -> Om13R {
        Om13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port 14 output mode bit"]
    #[inline(always)]
    pub fn om14(&self) -> Om14R {
        Om14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port 15 output mode bit"]
    #[inline(always)]
    pub fn om15(&self) -> Om15R {
        Om15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 0 output mode bit"]
    #[inline(always)]
    pub fn om0(&mut self) -> Om0W<'_, OmodeSpec> {
        Om0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port 1 output mode bit"]
    #[inline(always)]
    pub fn om1(&mut self) -> Om1W<'_, OmodeSpec> {
        Om1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port 2 output mode bit"]
    #[inline(always)]
    pub fn om2(&mut self) -> Om2W<'_, OmodeSpec> {
        Om2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port 3 output mode bit"]
    #[inline(always)]
    pub fn om3(&mut self) -> Om3W<'_, OmodeSpec> {
        Om3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port 4 output mode bit"]
    #[inline(always)]
    pub fn om4(&mut self) -> Om4W<'_, OmodeSpec> {
        Om4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port 5 output mode bit"]
    #[inline(always)]
    pub fn om5(&mut self) -> Om5W<'_, OmodeSpec> {
        Om5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port 6 output mode bit"]
    #[inline(always)]
    pub fn om6(&mut self) -> Om6W<'_, OmodeSpec> {
        Om6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port 7 output mode bit"]
    #[inline(always)]
    pub fn om7(&mut self) -> Om7W<'_, OmodeSpec> {
        Om7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port 8 output mode bit"]
    #[inline(always)]
    pub fn om8(&mut self) -> Om8W<'_, OmodeSpec> {
        Om8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port 9 output mode bit"]
    #[inline(always)]
    pub fn om9(&mut self) -> Om9W<'_, OmodeSpec> {
        Om9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port 10 output mode bit"]
    #[inline(always)]
    pub fn om10(&mut self) -> Om10W<'_, OmodeSpec> {
        Om10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port 11 output mode bit"]
    #[inline(always)]
    pub fn om11(&mut self) -> Om11W<'_, OmodeSpec> {
        Om11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port 12 output mode bit"]
    #[inline(always)]
    pub fn om12(&mut self) -> Om12W<'_, OmodeSpec> {
        Om12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port 13 output mode bit"]
    #[inline(always)]
    pub fn om13(&mut self) -> Om13W<'_, OmodeSpec> {
        Om13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port 14 output mode bit"]
    #[inline(always)]
    pub fn om14(&mut self) -> Om14W<'_, OmodeSpec> {
        Om14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port 15 output mode bit"]
    #[inline(always)]
    pub fn om15(&mut self) -> Om15W<'_, OmodeSpec> {
        Om15W::new(self, 15)
    }
}
#[doc = "GPIO port output mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`omode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`omode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OmodeSpec;
impl crate::RegisterSpec for OmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`omode::R`](R) reader structure"]
impl crate::Readable for OmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`omode::W`](W) writer structure"]
impl crate::Writable for OmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OMODE to value 0"]
impl crate::Resettable for OmodeSpec {}
