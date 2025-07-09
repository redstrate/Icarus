#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct QuestClassJobRewardSheet {
exd: EXD,
exh: EXH,
}
impl QuestClassJobRewardSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "QuestClassJobReward")?;let exd = read_excel_sheet(resource, "QuestClassJobReward", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<QuestClassJobRewardRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(QuestClassJobRewardRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<QuestClassJobRewardRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestClassJobRewardRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct QuestClassJobRewardRow {
columns: Vec<ColumnData>,
}
impl QuestClassJobRewardRow {
pub fn RewardItem(&self) -> [&ColumnData; 4] {
[&self.columns[0],&self.columns[1],&self.columns[2],&self.columns[3],]
}
pub fn RequiredItem(&self) -> [&ColumnData; 4] {
[&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[10]
}
pub fn RewardAmount(&self) -> [&ColumnData; 4] {
[&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],]
}
pub fn RequiredAmount(&self) -> [&ColumnData; 4] {
[&self.columns[15],&self.columns[16],&self.columns[17],&self.columns[18],]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[25]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[26]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[27]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[30]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[31]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[32]
}
}
