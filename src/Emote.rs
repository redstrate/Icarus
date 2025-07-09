#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct EmoteSheet {
exd: EXD,
exh: EXH,
}
impl EmoteSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "Emote")?;let exd = read_excel_sheet(resource, "Emote", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<EmoteRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(EmoteRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<EmoteRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<EmoteRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct EmoteRow {
columns: Vec<ColumnData>,
}
impl EmoteRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn UnlockLink(&self) -> &ColumnData {
&self.columns[1]
}
pub fn TextCommand(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ActionTimeline(&self) -> [&ColumnData; 7] {
[&self.columns[3],&self.columns[4],&self.columns[5],&self.columns[6],&self.columns[7],&self.columns[8],&self.columns[9],]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Icon(&self) -> &ColumnData {
&self.columns[11]
}
pub fn LogMessageTargeted(&self) -> &ColumnData {
&self.columns[12]
}
pub fn LogMessageUntargeted(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[14]
}
pub fn EmoteCategory(&self) -> &ColumnData {
&self.columns[15]
}
pub fn EmoteMode(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[21]
}
pub fn HasCancelEmote(&self) -> &ColumnData {
&self.columns[22]
}
pub fn DrawsWeapon(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[24]
}
}
