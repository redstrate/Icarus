#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ParamGrowSheet {
exd: EXD,
exh: EXH,
}
impl ParamGrowSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "ParamGrow")?;let exd = read_excel_sheet(resource, "ParamGrow", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<ParamGrowRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(ParamGrowRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<ParamGrowRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ParamGrowRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct ParamGrowRow {
columns: Vec<ColumnData>,
}
impl ParamGrowRow {
pub fn ExpToNext(&self) -> &ColumnData {
&self.columns[0]
}
pub fn MpModifier(&self) -> &ColumnData {
&self.columns[1]
}
pub fn BaseSpeed(&self) -> &ColumnData {
&self.columns[2]
}
pub fn LevelModifier(&self) -> &ColumnData {
&self.columns[3]
}
pub fn HuntingLogExpReward(&self) -> &ColumnData {
&self.columns[4]
}
pub fn MonsterNoteSeals(&self) -> &ColumnData {
&self.columns[5]
}
pub fn ScaledQuestXP(&self) -> &ColumnData {
&self.columns[6]
}
pub fn HpModifier(&self) -> &ColumnData {
&self.columns[7]
}
pub fn ItemLevelSync(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ProperDungeon(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ProperGuildOrder(&self) -> &ColumnData {
&self.columns[10]
}
pub fn CraftingLevel(&self) -> &ColumnData {
&self.columns[11]
}
pub fn AdditionalActions(&self) -> &ColumnData {
&self.columns[12]
}
pub fn ApplyAction(&self) -> &ColumnData {
&self.columns[13]
}
pub fn QuestExpModifier(&self) -> &ColumnData {
&self.columns[14]
}
}
