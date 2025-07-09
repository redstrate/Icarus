#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ItemFoodSheet {
exd: EXD,
exh: EXH,
}
impl ItemFoodSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "ItemFood")?;let exd = read_excel_sheet(resource, "ItemFood", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<ItemFoodRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(ItemFoodRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<ItemFoodRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ItemFoodRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct ItemFoodRow {
columns: Vec<ColumnData>,
}
impl ItemFoodRow {
pub fn Max(&self) -> [&ColumnData; 3] {
[&self.columns[0],&self.columns[1],&self.columns[2],]
}
pub fn MaxHQ(&self) -> [&ColumnData; 3] {
[&self.columns[3],&self.columns[4],&self.columns[5],]
}
pub fn EXPBonusPercent(&self) -> &ColumnData {
&self.columns[6]
}
pub fn BaseParam(&self) -> [&ColumnData; 3] {
[&self.columns[7],&self.columns[8],&self.columns[9],]
}
pub fn Value(&self) -> [&ColumnData; 3] {
[&self.columns[10],&self.columns[11],&self.columns[12],]
}
pub fn ValueHQ(&self) -> [&ColumnData; 3] {
[&self.columns[13],&self.columns[14],&self.columns[15],]
}
pub fn IsRelative(&self) -> [&ColumnData; 3] {
[&self.columns[16],&self.columns[17],&self.columns[18],]
}
}
