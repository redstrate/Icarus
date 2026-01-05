//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct ItemLevelSheet {
    sheet: ExcelSheet,
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
    fn read_row(&self, row: &ExcelSingleRow) -> Option<ItemLevelRow> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(ItemLevelRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<ItemLevelRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ItemLevelRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => return None,
            ExcelRowKind::SubRows(subrows) => {
                &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
            }
        };
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
pub struct ItemLevelRow {
    columns: Vec<ColumnData>,
}
impl ItemLevelRow {
    pub fn Strength<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Dexterity<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Vitality<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Intelligence<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn Mind<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Piety<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn HP<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn MP<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn TP<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn GP<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn CP<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn PhysicalDamage<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn MagicalDamage<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn Delay<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn AdditionalEffect<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn AttackSpeed<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn BlockRate<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn BlockStrength<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Tenacity<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn AttackPower<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn Defense<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn DirectHitRate<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn Evasion<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn MagicDefense<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn CriticalHitPower<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn CriticalHitResilience<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn CriticalHit<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn CriticalHitEvasion<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn SlashingResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn PiercingResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn BluntResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn ProjectileResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn AttackMagicPotency<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn HealingMagicPotency<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn EnhancementMagicPotency<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn EnfeeblingMagicPotency<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn FireResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn IceResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn WindResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn EarthResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn LightningResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn WaterResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn MagicResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
    pub fn Determination<'a>(&'a self) -> &'a ColumnData {
        &self.columns[43]
    }
    pub fn SkillSpeed<'a>(&'a self) -> &'a ColumnData {
        &self.columns[44]
    }
    pub fn SpellSpeed<'a>(&'a self) -> &'a ColumnData {
        &self.columns[45]
    }
    pub fn Haste<'a>(&'a self) -> &'a ColumnData {
        &self.columns[46]
    }
    pub fn Morale<'a>(&'a self) -> &'a ColumnData {
        &self.columns[47]
    }
    pub fn Enmity<'a>(&'a self) -> &'a ColumnData {
        &self.columns[48]
    }
    pub fn EnmityReduction<'a>(&'a self) -> &'a ColumnData {
        &self.columns[49]
    }
    pub fn CarefulDesynthesis<'a>(&'a self) -> &'a ColumnData {
        &self.columns[50]
    }
    pub fn EXPBonus<'a>(&'a self) -> &'a ColumnData {
        &self.columns[51]
    }
    pub fn Regen<'a>(&'a self) -> &'a ColumnData {
        &self.columns[52]
    }
    pub fn Refresh<'a>(&'a self) -> &'a ColumnData {
        &self.columns[53]
    }
    pub fn MovementSpeed<'a>(&'a self) -> &'a ColumnData {
        &self.columns[54]
    }
    pub fn Spikes<'a>(&'a self) -> &'a ColumnData {
        &self.columns[55]
    }
    pub fn SlowResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[56]
    }
    pub fn PetrificationResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[57]
    }
    pub fn ParalysisResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[58]
    }
    pub fn SilenceResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[59]
    }
    pub fn BlindResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[60]
    }
    pub fn PoisonResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[61]
    }
    pub fn StunResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[62]
    }
    pub fn SleepResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[63]
    }
    pub fn BindResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[64]
    }
    pub fn HeavyResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[65]
    }
    pub fn DoomResistance<'a>(&'a self) -> &'a ColumnData {
        &self.columns[66]
    }
    pub fn ReducedDurabilityLoss<'a>(&'a self) -> &'a ColumnData {
        &self.columns[67]
    }
    pub fn IncreasedSpiritbondGain<'a>(&'a self) -> &'a ColumnData {
        &self.columns[68]
    }
    pub fn Craftsmanship<'a>(&'a self) -> &'a ColumnData {
        &self.columns[69]
    }
    pub fn Control<'a>(&'a self) -> &'a ColumnData {
        &self.columns[70]
    }
    pub fn Gathering<'a>(&'a self) -> &'a ColumnData {
        &self.columns[71]
    }
    pub fn Perception<'a>(&'a self) -> &'a ColumnData {
        &self.columns[72]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[73]
    }
}
