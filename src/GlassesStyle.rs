#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct GlassesStyleSheet {
exd: EXD,
exh: EXH,
}
impl GlassesStyleSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "GlassesStyle")?;let exd = read_excel_sheet(resource, "GlassesStyle", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<GlassesStyleRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(GlassesStyleRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<GlassesStyleRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<GlassesStyleRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct GlassesStyleRow {
columns: Vec<ColumnData>,
}
impl GlassesStyleRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Name(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown_70_1(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown_70_2(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown_70_3(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown_70_4(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown_70_5(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown_70_6(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Glasses(&self) -> [&ColumnData; 12] {
[&self.columns[11],&self.columns[12],&self.columns[13],&self.columns[14],&self.columns[15],&self.columns[16],&self.columns[17],&self.columns[18],&self.columns[19],&self.columns[20],&self.columns[21],&self.columns[22],]
}
pub fn Unknown_70_7(&self) -> &ColumnData {
&self.columns[23]
}
}
