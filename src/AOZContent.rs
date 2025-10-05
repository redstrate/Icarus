#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct AOZContentSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl AOZContentSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "AOZContent")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "AOZContent", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<AOZContentRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(AOZContentRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<AOZContentRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<AOZContentRow> {
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
pub struct AOZContentRow {
columns: Vec<ColumnData>,
}
impl AOZContentRow {
pub fn GilReward(&self) -> &ColumnData {
&self.columns[0]
}
pub fn AlliedSealsReward(&self) -> &ColumnData {
&self.columns[1]
}
pub fn TomestonesReward(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ContentEntry(&self) -> &ColumnData {
&self.columns[3]
}
pub fn StandardFinishTime(&self) -> &ColumnData {
&self.columns[4]
}
pub fn IdealFinishTime(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Act1(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Act2(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Act3(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[11]
}
pub fn Act1FightType(&self) -> &ColumnData {
&self.columns[12]
}
pub fn Act2FightType(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Act3FightType(&self) -> &ColumnData {
&self.columns[14]
}
pub fn ArenaType1(&self) -> &ColumnData {
&self.columns[15]
}
pub fn ArenaType2(&self) -> &ColumnData {
&self.columns[16]
}
pub fn ArenaType3(&self) -> &ColumnData {
&self.columns[17]
}
pub fn Order(&self) -> &ColumnData {
&self.columns[18]
}
}
