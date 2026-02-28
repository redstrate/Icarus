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
    index_mapping: Vec<usize>,
}
impl ItemLevelSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ItemLevel")?;
        let sheet = resolver.read_excel_sheet(&exh, "ItemLevel", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
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
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
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
    index_mapping: Vec<usize>,
}
impl<'a> ItemLevelRow<'a> {
    pub fn Strength(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Dexterity(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Vitality(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn Intelligence(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn Mind(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn Piety(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn HP(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn MP(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn TP(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn GP(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn CP(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn PhysicalDamage(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn MagicalDamage(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn Delay(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn AdditionalEffect(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn AttackSpeed(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn BlockRate(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn BlockStrength(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn Tenacity(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn AttackPower(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn Defense(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn DirectHitRate(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn Evasion(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn MagicDefense(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn CriticalHitPower(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn CriticalHitResilience(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn CriticalHit(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn CriticalHitEvasion(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn SlashingResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn PiercingResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn BluntResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn ProjectileResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn AttackMagicPotency(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn HealingMagicPotency(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn EnhancementMagicPotency(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn EnfeeblingMagicPotency(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn FireResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn IceResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn WindResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn EarthResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn LightningResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn WaterResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn MagicResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn Determination(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    pub fn SkillSpeed(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    pub fn SpellSpeed(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn Haste(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
    pub fn Morale(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[47]]
    }
    pub fn Enmity(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[48]]
    }
    pub fn EnmityReduction(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[49]]
    }
    pub fn CarefulDesynthesis(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[50]]
    }
    pub fn EXPBonus(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[51]]
    }
    pub fn Regen(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[52]]
    }
    pub fn Refresh(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[53]]
    }
    pub fn MovementSpeed(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[54]]
    }
    pub fn Spikes(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[55]]
    }
    pub fn SlowResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[56]]
    }
    pub fn PetrificationResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[57]]
    }
    pub fn ParalysisResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[58]]
    }
    pub fn SilenceResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[59]]
    }
    pub fn BlindResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[60]]
    }
    pub fn PoisonResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[61]]
    }
    pub fn StunResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[62]]
    }
    pub fn SleepResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[63]]
    }
    pub fn BindResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[64]]
    }
    pub fn HeavyResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[65]]
    }
    pub fn DoomResistance(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[66]]
    }
    pub fn ReducedDurabilityLoss(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[67]]
    }
    pub fn IncreasedSpiritbondGain(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[68]]
    }
    pub fn Craftsmanship(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[69]]
    }
    pub fn Control(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[70]]
    }
    pub fn Gathering(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[71]]
    }
    pub fn Perception(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[72]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[73]]
    }
}
