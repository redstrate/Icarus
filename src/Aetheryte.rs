#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct AetheryteSheet {
exd: EXD,
exh: EXH,
}
impl AetheryteSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "Aetheryte")?;let exd = read_excel_sheet(resource, "Aetheryte", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<AetheryteRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(AetheryteRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<AetheryteRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<AetheryteRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct AetheryteRow {
columns: Vec<ColumnData>,
}
impl AetheryteRow {
pub fn Singular(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Plural(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Adjective(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PossessivePronoun(&self) -> &ColumnData {
&self.columns[3]
}
pub fn StartsWithVowel(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Pronoun(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Article(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Level(&self) -> [&ColumnData; 4] {
[&self.columns[9],&self.columns[10],&self.columns[11],&self.columns[12],]
}
pub fn RequiredQuest(&self) -> &ColumnData {
&self.columns[13]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[14]
}
pub fn AethernetName(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Territory(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Map(&self) -> &ColumnData {
&self.columns[17]
}
pub fn AetherstreamX(&self) -> &ColumnData {
&self.columns[18]
}
pub fn AetherstreamY(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[20]
}
pub fn AethernetGroup(&self) -> &ColumnData {
&self.columns[21]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[22]
}
pub fn IsAetheryte(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Invisible(&self) -> &ColumnData {
&self.columns[24]
}
}
