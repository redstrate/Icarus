#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct LeveSheet {
pages: Vec<EXD>,
exh: EXH,
row_count: u32,
}
impl LeveSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "Leve")?;
let mut pages = Vec::new();
for (i, _) in exh.pages.iter().enumerate() {
pages.push(read_excel_sheet(resource, "Leve", &exh, language, i)?);
}let row_count = exh.header.row_count;
Some(Self {
exh,
pages,
row_count,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<LeveRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(LeveRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<LeveRow> {
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
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<LeveRow> {
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
pub struct LeveRow {
columns: Vec<ColumnData>,
}
impl LeveRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Description(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ExpFactor(&self) -> &ColumnData {
&self.columns[2]
}
pub fn ExpReward(&self) -> &ColumnData {
&self.columns[3]
}
pub fn GilReward(&self) -> &ColumnData {
&self.columns[4]
}
pub fn LeveRewardItem(&self) -> &ColumnData {
&self.columns[5]
}
pub fn JournalGenre(&self) -> &ColumnData {
&self.columns[6]
}
pub fn LevelLevemete(&self) -> &ColumnData {
&self.columns[7]
}
pub fn LevelStart(&self) -> &ColumnData {
&self.columns[8]
}
pub fn LeveClient(&self) -> &ColumnData {
&self.columns[9]
}
pub fn LeveAssignmentType(&self) -> &ColumnData {
&self.columns[10]
}
pub fn Town(&self) -> &ColumnData {
&self.columns[11]
}
pub fn PlaceNameStart(&self) -> &ColumnData {
&self.columns[12]
}
pub fn PlaceNameIssued(&self) -> &ColumnData {
&self.columns[13]
}
pub fn PlaceNameStartZone(&self) -> &ColumnData {
&self.columns[14]
}
pub fn IconCityState(&self) -> &ColumnData {
&self.columns[15]
}
pub fn DataId(&self) -> &ColumnData {
&self.columns[16]
}
pub fn IconIssuer(&self) -> &ColumnData {
&self.columns[17]
}
pub fn ClassJobLevel(&self) -> &ColumnData {
&self.columns[18]
}
pub fn FishingSpot(&self) -> &ColumnData {
&self.columns[19]
}
pub fn BGM(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[21]
}
pub fn TimeLimit(&self) -> &ColumnData {
&self.columns[22]
}
pub fn AllowanceCost(&self) -> &ColumnData {
&self.columns[23]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[24]
}
pub fn ClassJobCategory(&self) -> &ColumnData {
&self.columns[25]
}
pub fn MaxDifficulty(&self) -> &ColumnData {
&self.columns[26]
}
pub fn LeveVfx(&self) -> &ColumnData {
&self.columns[27]
}
pub fn LeveVfxFrame(&self) -> &ColumnData {
&self.columns[28]
}
pub fn CanCancel(&self) -> &ColumnData {
&self.columns[29]
}
pub fn LockedLeve(&self) -> &ColumnData {
&self.columns[30]
}
}
