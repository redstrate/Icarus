#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct FishParameterSheet {
exd: EXD,
exh: EXH,
}
impl FishParameterSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "FishParameter")?;let exd = read_excel_sheet(resource, "FishParameter", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<FishParameterRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(FishParameterRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<FishParameterRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<FishParameterRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct FishParameterRow {
columns: Vec<ColumnData>,
}
impl FishParameterRow {
pub fn Text(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown_70_3(&self) -> &ColumnData {
&self.columns[3]
}
pub fn AchievementCredit(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[5]
}
pub fn GatheringItemLevel(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[7]
}
pub fn FishingSpot(&self) -> &ColumnData {
&self.columns[8]
}
pub fn GatheringSubCategory(&self) -> &ColumnData {
&self.columns[9]
}
pub fn OceanStars(&self) -> &ColumnData {
&self.columns[10]
}
pub fn FishingRecordType(&self) -> &ColumnData {
&self.columns[11]
}
pub fn IsHidden(&self) -> &ColumnData {
&self.columns[12]
}
pub fn IsInLog(&self) -> &ColumnData {
&self.columns[13]
}
}
