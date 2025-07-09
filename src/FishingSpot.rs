#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct FishingSpotSheet {
exd: EXD,
exh: EXH,
}
impl FishingSpotSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "FishingSpot")?;let exd = read_excel_sheet(resource, "FishingSpot", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<FishingSpotRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(FishingSpotRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<FishingSpotRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<FishingSpotRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct FishingSpotRow {
columns: Vec<ColumnData>,
}
impl FishingSpotRow {
pub fn BigFishOnReach(&self) -> &ColumnData {
&self.columns[0]
}
pub fn BigFishOnEnd(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Item(&self) -> [&ColumnData; 10] {
[&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],]
}
pub fn TerritoryType(&self) -> &ColumnData {
&self.columns[13]
}
pub fn PlaceNameMain(&self) -> &ColumnData {
&self.columns[14]
}
pub fn PlaceNameSub(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Radius(&self) -> &ColumnData {
&self.columns[16]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[18]
}
pub fn X(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Z(&self) -> &ColumnData {
&self.columns[20]
}
pub fn GatheringLevel(&self) -> &ColumnData {
&self.columns[21]
}
pub fn FishingSpotCategory(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Rare(&self) -> &ColumnData {
&self.columns[24]
}
}
