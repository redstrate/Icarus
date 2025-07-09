#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct MirageStoreSetItemSheet {
exd: EXD,
exh: EXH,
}
impl MirageStoreSetItemSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "MirageStoreSetItem")?;let exd = read_excel_sheet(resource, "MirageStoreSetItem", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<MirageStoreSetItemRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(MirageStoreSetItemRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<MirageStoreSetItemRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<MirageStoreSetItemRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct MirageStoreSetItemRow {
columns: Vec<ColumnData>,
}
impl MirageStoreSetItemRow {
pub fn MainHand(&self) -> &ColumnData {
&self.columns[0]
}
pub fn OffHand(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Head(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Body(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Hands(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Legs(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Feet(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Earrings(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Necklace(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Bracelets(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Ring(&self) -> &ColumnData {
&self.columns[10]
}
}
