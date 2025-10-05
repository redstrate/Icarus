#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct CutsceneMotionSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl CutsceneMotionSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "CutsceneMotion")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "CutsceneMotion", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<CutsceneMotionRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(CutsceneMotionRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<CutsceneMotionRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<CutsceneMotionRow> {
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
pub struct CutsceneMotionRow {
columns: Vec<ColumnData>,
}
impl CutsceneMotionRow {
pub fn WALK_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[0]
}
pub fn RUN_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[1]
}
pub fn SLOWWALK_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[2]
}
pub fn SLOWRUN_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[3]
}
pub fn BATTLEWALK_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[4]
}
pub fn BATTLERUN_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[5]
}
pub fn DASH_LOOP_SPEED(&self) -> &ColumnData {
&self.columns[6]
}
pub fn TURN_CW90_FRAME(&self) -> &ColumnData {
&self.columns[7]
}
pub fn TURN_CCW90_FRAME(&self) -> &ColumnData {
&self.columns[8]
}
pub fn TURN_CW180_FRAME(&self) -> &ColumnData {
&self.columns[9]
}
pub fn TURN_CCW180_FRAME(&self) -> &ColumnData {
&self.columns[10]
}
}
