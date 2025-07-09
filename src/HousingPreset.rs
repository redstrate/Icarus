#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct HousingPresetSheet {
exd: EXD,
exh: EXH,
}
impl HousingPresetSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "HousingPreset")?;let exd = read_excel_sheet(resource, "HousingPreset", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<HousingPresetRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(HousingPresetRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<HousingPresetRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<HousingPresetRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct HousingPresetRow {
columns: Vec<ColumnData>,
}
impl HousingPresetRow {
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
pub fn ExteriorRoof(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ExteriorWall(&self) -> &ColumnData {
&self.columns[9]
}
pub fn ExteriorWindow(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ExteriorDoor(&self) -> &ColumnData {
&self.columns[11]
}
pub fn InteriorWall(&self) -> &ColumnData {
&self.columns[12]
}
pub fn InteriorFlooring(&self) -> &ColumnData {
&self.columns[13]
}
pub fn InteriorLighting(&self) -> &ColumnData {
&self.columns[14]
}
pub fn OtherFloorWall(&self) -> &ColumnData {
&self.columns[15]
}
pub fn OtherFloorFlooring(&self) -> &ColumnData {
&self.columns[16]
}
pub fn OtherFloorLighting(&self) -> &ColumnData {
&self.columns[17]
}
pub fn BasementWall(&self) -> &ColumnData {
&self.columns[18]
}
pub fn BasementFlooring(&self) -> &ColumnData {
&self.columns[19]
}
pub fn BasementLighting(&self) -> &ColumnData {
&self.columns[20]
}
pub fn MansionLighting(&self) -> &ColumnData {
&self.columns[21]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[22]
}
pub fn HousingSize(&self) -> &ColumnData {
&self.columns[23]
}
}
