#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct ActionTimelineSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl ActionTimelineSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "ActionTimeline")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "ActionTimeline", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<ActionTimelineRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(ActionTimelineRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<ActionTimelineRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<ActionTimelineRow> {
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
pub struct ActionTimelineRow {
columns: Vec<ColumnData>,
}
impl ActionTimelineRow {
pub fn Key(&self) -> &ColumnData {
&self.columns[0]
}
pub fn WeaponTimeline(&self) -> &ColumnData {
&self.columns[1]
}
pub fn KillUpper(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown_70(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Type(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Priority(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Stance(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Slot(&self) -> &ColumnData {
&self.columns[7]
}
pub fn LookAtMode(&self) -> &ColumnData {
&self.columns[8]
}
pub fn ActionTimelineIDMode(&self) -> &ColumnData {
&self.columns[9]
}
pub fn LoadType(&self) -> &ColumnData {
&self.columns[10]
}
pub fn StartAttach(&self) -> &ColumnData {
&self.columns[11]
}
pub fn ResidentPap(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[14]
}
pub fn Pause(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Resident(&self) -> &ColumnData {
&self.columns[16]
}
pub fn IsMotionCanceledByMoving(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[19]
}
pub fn IsLoop(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[21]
}
}
