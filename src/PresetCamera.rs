#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct PresetCameraSheet {
exd: EXD,
exh: EXH,
}
impl PresetCameraSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "PresetCamera")?;let exd = read_excel_sheet(resource, "PresetCamera", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<PresetCameraRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(PresetCameraRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<PresetCameraRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<PresetCameraRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct PresetCameraRow {
columns: Vec<ColumnData>,
}
impl PresetCameraRow {
pub fn PosX(&self) -> &ColumnData {
&self.columns[0]
}
pub fn PosY(&self) -> &ColumnData {
&self.columns[1]
}
pub fn PosZ(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Elezen(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Lalafell(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Miqote(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Roe(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Hrothgar(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Viera(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Hyur_F(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Elezen_F(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Lalafell_F(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Miqote_F(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Roe_F(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Hrothgar_F(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Viera_F(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[17]
}
pub fn EID(&self) -> &ColumnData {
&self.columns[18]
}
}
