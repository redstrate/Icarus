#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct SpearfishingItemSheet {
exd: EXD,
exh: EXH,
}
impl SpearfishingItemSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "SpearfishingItem")?;let exd = read_excel_sheet(resource, "SpearfishingItem", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<SpearfishingItemRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(SpearfishingItemRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<SpearfishingItemRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<SpearfishingItemRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct SpearfishingItemRow {
columns: Vec<ColumnData>,
}
impl SpearfishingItemRow {
pub fn Description(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Item(&self) -> &ColumnData {
&self.columns[1]
}
pub fn GatheringItemLevel(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[3]
}
pub fn TerritoryType(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn FishingRecordType(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[7]
}
pub fn IsVisible(&self) -> &ColumnData {
&self.columns[8]
}
}
