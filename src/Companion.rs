#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct CompanionSheet {
exd: EXD,
exh: EXH,
}
impl CompanionSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "Companion")?;let exd = read_excel_sheet(resource, "Companion", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<CompanionRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(CompanionRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<CompanionRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<CompanionRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct CompanionRow {
columns: Vec<ColumnData>,
}
impl CompanionRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Adjective(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PossessivePronoun(&self) -> &ColumnData {
&self.columns[3]
}
pub fn StartsWithVowel(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Pronoun(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Article(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Model(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Priority(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Enemy(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[12]
}
pub fn HP(&self) -> &ColumnData {
&self.columns[13]
}
pub fn SkillAngle(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Scale(&self) -> &ColumnData {
&self.columns[16]
}
pub fn InactiveIdle0(&self) -> &ColumnData {
&self.columns[17]
}
pub fn InactiveIdle1(&self) -> &ColumnData {
&self.columns[18]
}
pub fn InactiveBattle(&self) -> &ColumnData {
&self.columns[19]
}
pub fn InactiveWandering(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Behavior(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Special(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[24]
}
pub fn WanderingWait(&self) -> &ColumnData {
&self.columns[25]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[26]
}
pub fn Cost(&self) -> &ColumnData {
&self.columns[27]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[28]
}
pub fn SkillCost(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[30]
}
pub fn MinionRace(&self) -> &ColumnData {
&self.columns[31]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[34]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[35]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[36]
}
pub fn Battle(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Roulette(&self) -> &ColumnData {
&self.columns[38]
}
pub fn IdleAnimation(&self) -> &ColumnData {
&self.columns[39]
}
}
