#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ItemLevelSheet {
exd: EXD,
exh: EXH,
}
impl ItemLevelSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "ItemLevel")?;let exd = read_excel_sheet(resource, "ItemLevel", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<ItemLevelRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(ItemLevelRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<ItemLevelRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ItemLevelRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct ItemLevelRow {
columns: Vec<ColumnData>,
}
impl ItemLevelRow {
pub fn Strength(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Dexterity(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Vitality(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Intelligence(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Mind(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Piety(&self) -> &ColumnData {
&self.columns[5]
}
pub fn HP(&self) -> &ColumnData {
&self.columns[6]
}
pub fn MP(&self) -> &ColumnData {
&self.columns[7]
}
pub fn TP(&self) -> &ColumnData {
&self.columns[8]
}
pub fn GP(&self) -> &ColumnData {
&self.columns[9]
}
pub fn CP(&self) -> &ColumnData {
&self.columns[10]
}
pub fn PhysicalDamage(&self) -> &ColumnData {
&self.columns[11]
}
pub fn MagicalDamage(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Delay(&self) -> &ColumnData {
&self.columns[13]
}
pub fn AdditionalEffect(&self) -> &ColumnData {
&self.columns[14]
}
pub fn AttackSpeed(&self) -> &ColumnData {
&self.columns[15]
}
pub fn BlockRate(&self) -> &ColumnData {
&self.columns[16]
}
pub fn BlockStrength(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Tenacity(&self) -> &ColumnData {
&self.columns[18]
}
pub fn AttackPower(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Defense(&self) -> &ColumnData {
&self.columns[20]
}
pub fn DirectHitRate(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Evasion(&self) -> &ColumnData {
&self.columns[22]
}
pub fn MagicDefense(&self) -> &ColumnData {
&self.columns[23]
}
pub fn CriticalHitPower(&self) -> &ColumnData {
&self.columns[24]
}
pub fn CriticalHitResilience(&self) -> &ColumnData {
&self.columns[25]
}
pub fn CriticalHit(&self) -> &ColumnData {
&self.columns[26]
}
pub fn CriticalHitEvasion(&self) -> &ColumnData {
&self.columns[27]
}
pub fn SlashingResistance(&self) -> &ColumnData {
&self.columns[28]
}
pub fn PiercingResistance(&self) -> &ColumnData {
&self.columns[29]
}
pub fn BluntResistance(&self) -> &ColumnData {
&self.columns[30]
}
pub fn ProjectileResistance(&self) -> &ColumnData {
&self.columns[31]
}
pub fn AttackMagicPotency(&self) -> &ColumnData {
&self.columns[32]
}
pub fn HealingMagicPotency(&self) -> &ColumnData {
&self.columns[33]
}
pub fn EnhancementMagicPotency(&self) -> &ColumnData {
&self.columns[34]
}
pub fn EnfeeblingMagicPotency(&self) -> &ColumnData {
&self.columns[35]
}
pub fn FireResistance(&self) -> &ColumnData {
&self.columns[36]
}
pub fn IceResistance(&self) -> &ColumnData {
&self.columns[37]
}
pub fn WindResistance(&self) -> &ColumnData {
&self.columns[38]
}
pub fn EarthResistance(&self) -> &ColumnData {
&self.columns[39]
}
pub fn LightningResistance(&self) -> &ColumnData {
&self.columns[40]
}
pub fn WaterResistance(&self) -> &ColumnData {
&self.columns[41]
}
pub fn MagicResistance(&self) -> &ColumnData {
&self.columns[42]
}
pub fn Determination(&self) -> &ColumnData {
&self.columns[43]
}
pub fn SkillSpeed(&self) -> &ColumnData {
&self.columns[44]
}
pub fn SpellSpeed(&self) -> &ColumnData {
&self.columns[45]
}
pub fn Haste(&self) -> &ColumnData {
&self.columns[46]
}
pub fn Morale(&self) -> &ColumnData {
&self.columns[47]
}
pub fn Enmity(&self) -> &ColumnData {
&self.columns[48]
}
pub fn EnmityReduction(&self) -> &ColumnData {
&self.columns[49]
}
pub fn CarefulDesynthesis(&self) -> &ColumnData {
&self.columns[50]
}
pub fn EXPBonus(&self) -> &ColumnData {
&self.columns[51]
}
pub fn Regen(&self) -> &ColumnData {
&self.columns[52]
}
pub fn Refresh(&self) -> &ColumnData {
&self.columns[53]
}
pub fn MovementSpeed(&self) -> &ColumnData {
&self.columns[54]
}
pub fn Spikes(&self) -> &ColumnData {
&self.columns[55]
}
pub fn SlowResistance(&self) -> &ColumnData {
&self.columns[56]
}
pub fn PetrificationResistance(&self) -> &ColumnData {
&self.columns[57]
}
pub fn ParalysisResistance(&self) -> &ColumnData {
&self.columns[58]
}
pub fn SilenceResistance(&self) -> &ColumnData {
&self.columns[59]
}
pub fn BlindResistance(&self) -> &ColumnData {
&self.columns[60]
}
pub fn PoisonResistance(&self) -> &ColumnData {
&self.columns[61]
}
pub fn StunResistance(&self) -> &ColumnData {
&self.columns[62]
}
pub fn SleepResistance(&self) -> &ColumnData {
&self.columns[63]
}
pub fn BindResistance(&self) -> &ColumnData {
&self.columns[64]
}
pub fn HeavyResistance(&self) -> &ColumnData {
&self.columns[65]
}
pub fn DoomResistance(&self) -> &ColumnData {
&self.columns[66]
}
pub fn ReducedDurabilityLoss(&self) -> &ColumnData {
&self.columns[67]
}
pub fn IncreasedSpiritbondGain(&self) -> &ColumnData {
&self.columns[68]
}
pub fn Craftsmanship(&self) -> &ColumnData {
&self.columns[69]
}
pub fn Control(&self) -> &ColumnData {
&self.columns[70]
}
pub fn Gathering(&self) -> &ColumnData {
&self.columns[71]
}
pub fn Perception(&self) -> &ColumnData {
&self.columns[72]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[73]
}
}
