#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct RecipeLevelTableSheet {
exd: EXD,
exh: EXH,
}
impl RecipeLevelTableSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "RecipeLevelTable")?;let exd = read_excel_sheet(resource, "RecipeLevelTable", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<RecipeLevelTableRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(RecipeLevelTableRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<RecipeLevelTableRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<RecipeLevelTableRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct RecipeLevelTableRow {
columns: Vec<ColumnData>,
}
impl RecipeLevelTableRow {
pub fn Quality(&self) -> &ColumnData {
&self.columns[0]
}
pub fn SuggestedCraftsmanship(&self) -> &ColumnData {
&self.columns[1]
}
pub fn Difficulty(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Durability(&self) -> &ColumnData {
&self.columns[3]
}
pub fn ConditionsFlag(&self) -> &ColumnData {
&self.columns[4]
}
pub fn ClassJobLevel(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Stars(&self) -> &ColumnData {
&self.columns[6]
}
pub fn ProgressDivider(&self) -> &ColumnData {
&self.columns[7]
}
pub fn QualityDivider(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ProgressModifier(&self) -> &ColumnData {
&self.columns[9]
}
pub fn QualityModifier(&self) -> &ColumnData {
&self.columns[10]
}
}
