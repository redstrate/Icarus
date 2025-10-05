#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct EmjDaniSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl EmjDaniSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "EmjDani")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "EmjDani", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<EmjDaniRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(EmjDaniRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<EmjDaniRow> {
for page in &self.pages {
let Some(row) = &page.get_row(row_id) else { continue; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
return self.read_row(row);
}
None
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<EmjDaniRow> {
for page in &self.pages {
let Some(row) = &page.get_row(row_id) else { continue; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
return self.read_row(row);
}
None
}
/// Returns the number of rows in this sheet.
pub fn row_count(&self) -> u32 {
self.row_count
}
}
pub struct EmjDaniRow {
columns: Vec<ColumnData>,
}
impl EmjDaniRow {
pub fn Icon(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[11]
}
}
