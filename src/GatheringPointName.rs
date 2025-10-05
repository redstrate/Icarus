#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct GatheringPointNameSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl GatheringPointNameSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "GatheringPointName")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "GatheringPointName", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<GatheringPointNameRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(GatheringPointNameRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<GatheringPointNameRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<GatheringPointNameRow> {
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
pub struct GatheringPointNameRow {
columns: Vec<ColumnData>,
}
impl GatheringPointNameRow {
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
}
