//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Debug, Clone)]
pub struct ItemLevelSheet {
    sheet: Sheet,
}
impl ItemLevelSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ItemLevel")?;
        let sheet = resolver.read_excel_sheet(&exh, "ItemLevel", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ItemLevelRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ItemLevelRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ItemLevelSheet {
    type Row = ItemLevelRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a ItemLevelSheet {
    type Item = (u32, Vec<(u16, ItemLevelRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ItemLevelSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ItemLevelSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ItemLevelRow<'a> {
    row: &'a Row,
}
impl<'a> ItemLevelRow<'a> {
    pub fn Strength(&'a self) -> u16 {
        self.row.columns[0].into_u16().copied().unwrap()
    }
    pub fn Dexterity(&'a self) -> u16 {
        self.row.columns[1].into_u16().copied().unwrap()
    }
    pub fn Vitality(&'a self) -> u16 {
        self.row.columns[2].into_u16().copied().unwrap()
    }
    pub fn Intelligence(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn Mind(&'a self) -> u16 {
        self.row.columns[4].into_u16().copied().unwrap()
    }
    pub fn Piety(&'a self) -> u16 {
        self.row.columns[5].into_u16().copied().unwrap()
    }
    pub fn HP(&'a self) -> u16 {
        self.row.columns[6].into_u16().copied().unwrap()
    }
    pub fn MP(&'a self) -> u16 {
        self.row.columns[7].into_u16().copied().unwrap()
    }
    pub fn TP(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn GP(&'a self) -> u16 {
        self.row.columns[9].into_u16().copied().unwrap()
    }
    pub fn CP(&'a self) -> u16 {
        self.row.columns[10].into_u16().copied().unwrap()
    }
    pub fn PhysicalDamage(&'a self) -> u16 {
        self.row.columns[11].into_u16().copied().unwrap()
    }
    pub fn MagicalDamage(&'a self) -> u16 {
        self.row.columns[12].into_u16().copied().unwrap()
    }
    pub fn Delay(&'a self) -> u16 {
        self.row.columns[13].into_u16().copied().unwrap()
    }
    pub fn AdditionalEffect(&'a self) -> u16 {
        self.row.columns[14].into_u16().copied().unwrap()
    }
    pub fn AttackSpeed(&'a self) -> u16 {
        self.row.columns[15].into_u16().copied().unwrap()
    }
    pub fn BlockRate(&'a self) -> u16 {
        self.row.columns[16].into_u16().copied().unwrap()
    }
    pub fn BlockStrength(&'a self) -> u16 {
        self.row.columns[17].into_u16().copied().unwrap()
    }
    pub fn Tenacity(&'a self) -> u16 {
        self.row.columns[18].into_u16().copied().unwrap()
    }
    pub fn AttackPower(&'a self) -> u16 {
        self.row.columns[19].into_u16().copied().unwrap()
    }
    pub fn Defense(&'a self) -> u16 {
        self.row.columns[20].into_u16().copied().unwrap()
    }
    pub fn DirectHitRate(&'a self) -> u16 {
        self.row.columns[21].into_u16().copied().unwrap()
    }
    pub fn Evasion(&'a self) -> u16 {
        self.row.columns[22].into_u16().copied().unwrap()
    }
    pub fn MagicDefense(&'a self) -> u16 {
        self.row.columns[23].into_u16().copied().unwrap()
    }
    pub fn CriticalHitPower(&'a self) -> u16 {
        self.row.columns[24].into_u16().copied().unwrap()
    }
    pub fn CriticalHitResilience(&'a self) -> u16 {
        self.row.columns[25].into_u16().copied().unwrap()
    }
    pub fn CriticalHit(&'a self) -> u16 {
        self.row.columns[26].into_u16().copied().unwrap()
    }
    pub fn CriticalHitEvasion(&'a self) -> u16 {
        self.row.columns[27].into_u16().copied().unwrap()
    }
    pub fn SlashingResistance(&'a self) -> u16 {
        self.row.columns[28].into_u16().copied().unwrap()
    }
    pub fn PiercingResistance(&'a self) -> u16 {
        self.row.columns[29].into_u16().copied().unwrap()
    }
    pub fn BluntResistance(&'a self) -> u16 {
        self.row.columns[30].into_u16().copied().unwrap()
    }
    pub fn ProjectileResistance(&'a self) -> u16 {
        self.row.columns[31].into_u16().copied().unwrap()
    }
    pub fn AttackMagicPotency(&'a self) -> u16 {
        self.row.columns[32].into_u16().copied().unwrap()
    }
    pub fn HealingMagicPotency(&'a self) -> u16 {
        self.row.columns[33].into_u16().copied().unwrap()
    }
    pub fn EnhancementMagicPotency(&'a self) -> u16 {
        self.row.columns[34].into_u16().copied().unwrap()
    }
    pub fn EnfeeblingMagicPotency(&'a self) -> u16 {
        self.row.columns[35].into_u16().copied().unwrap()
    }
    pub fn FireResistance(&'a self) -> u16 {
        self.row.columns[36].into_u16().copied().unwrap()
    }
    pub fn IceResistance(&'a self) -> u16 {
        self.row.columns[37].into_u16().copied().unwrap()
    }
    pub fn WindResistance(&'a self) -> u16 {
        self.row.columns[38].into_u16().copied().unwrap()
    }
    pub fn EarthResistance(&'a self) -> u16 {
        self.row.columns[39].into_u16().copied().unwrap()
    }
    pub fn LightningResistance(&'a self) -> u16 {
        self.row.columns[40].into_u16().copied().unwrap()
    }
    pub fn WaterResistance(&'a self) -> u16 {
        self.row.columns[41].into_u16().copied().unwrap()
    }
    pub fn MagicResistance(&'a self) -> u16 {
        self.row.columns[42].into_u16().copied().unwrap()
    }
    pub fn Determination(&'a self) -> u16 {
        self.row.columns[43].into_u16().copied().unwrap()
    }
    pub fn SkillSpeed(&'a self) -> u16 {
        self.row.columns[44].into_u16().copied().unwrap()
    }
    pub fn SpellSpeed(&'a self) -> u16 {
        self.row.columns[45].into_u16().copied().unwrap()
    }
    pub fn Haste(&'a self) -> u16 {
        self.row.columns[46].into_u16().copied().unwrap()
    }
    pub fn Morale(&'a self) -> u16 {
        self.row.columns[47].into_u16().copied().unwrap()
    }
    pub fn Enmity(&'a self) -> u16 {
        self.row.columns[48].into_u16().copied().unwrap()
    }
    pub fn EnmityReduction(&'a self) -> u16 {
        self.row.columns[49].into_u16().copied().unwrap()
    }
    pub fn CarefulDesynthesis(&'a self) -> u16 {
        self.row.columns[50].into_u16().copied().unwrap()
    }
    pub fn EXPBonus(&'a self) -> u16 {
        self.row.columns[51].into_u16().copied().unwrap()
    }
    pub fn Regen(&'a self) -> u16 {
        self.row.columns[52].into_u16().copied().unwrap()
    }
    pub fn Refresh(&'a self) -> u16 {
        self.row.columns[53].into_u16().copied().unwrap()
    }
    pub fn MovementSpeed(&'a self) -> u16 {
        self.row.columns[54].into_u16().copied().unwrap()
    }
    pub fn Spikes(&'a self) -> u16 {
        self.row.columns[55].into_u16().copied().unwrap()
    }
    pub fn SlowResistance(&'a self) -> u16 {
        self.row.columns[56].into_u16().copied().unwrap()
    }
    pub fn PetrificationResistance(&'a self) -> u16 {
        self.row.columns[57].into_u16().copied().unwrap()
    }
    pub fn ParalysisResistance(&'a self) -> u16 {
        self.row.columns[58].into_u16().copied().unwrap()
    }
    pub fn SilenceResistance(&'a self) -> u16 {
        self.row.columns[59].into_u16().copied().unwrap()
    }
    pub fn BlindResistance(&'a self) -> u16 {
        self.row.columns[60].into_u16().copied().unwrap()
    }
    pub fn PoisonResistance(&'a self) -> u16 {
        self.row.columns[61].into_u16().copied().unwrap()
    }
    pub fn StunResistance(&'a self) -> u16 {
        self.row.columns[62].into_u16().copied().unwrap()
    }
    pub fn SleepResistance(&'a self) -> u16 {
        self.row.columns[63].into_u16().copied().unwrap()
    }
    pub fn BindResistance(&'a self) -> u16 {
        self.row.columns[64].into_u16().copied().unwrap()
    }
    pub fn HeavyResistance(&'a self) -> u16 {
        self.row.columns[65].into_u16().copied().unwrap()
    }
    pub fn DoomResistance(&'a self) -> u16 {
        self.row.columns[66].into_u16().copied().unwrap()
    }
    pub fn ReducedDurabilityLoss(&'a self) -> u16 {
        self.row.columns[67].into_u16().copied().unwrap()
    }
    pub fn IncreasedSpiritbondGain(&'a self) -> u16 {
        self.row.columns[68].into_u16().copied().unwrap()
    }
    pub fn Craftsmanship(&'a self) -> u16 {
        self.row.columns[69].into_u16().copied().unwrap()
    }
    pub fn Control(&'a self) -> u16 {
        self.row.columns[70].into_u16().copied().unwrap()
    }
    pub fn Gathering(&'a self) -> u16 {
        self.row.columns[71].into_u16().copied().unwrap()
    }
    pub fn Perception(&'a self) -> u16 {
        self.row.columns[72].into_u16().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u16 {
        self.row.columns[73].into_u16().copied().unwrap()
    }
}
