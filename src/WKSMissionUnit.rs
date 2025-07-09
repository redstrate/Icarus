#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct WKSMissionUnitSheet {
exd: EXD,
exh: EXH,
}
impl WKSMissionUnitSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "WKSMissionUnit")?;let exd = read_excel_sheet(resource, "WKSMissionUnit", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<WKSMissionUnitRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(WKSMissionUnitRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<WKSMissionUnitRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSMissionUnitRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct WKSMissionUnitRow {
columns: Vec<ColumnData>,
}
impl WKSMissionUnitRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn ClassJobCategory(&self) -> [&ColumnData; 2] {
[&self.columns[1],&self.columns[2],]
}
pub fn MissionTime(&self) -> &ColumnData {
&self.columns[3]
}
pub fn MissionReward(&self) -> &ColumnData {
&self.columns[4]
}
pub fn SilverStarRequirement(&self) -> &ColumnData {
&self.columns[5]
}
pub fn GoldStarRequirement(&self) -> &ColumnData {
&self.columns[6]
}
pub fn MissionToDo(&self) -> [&ColumnData; 3] {
[&self.columns[7],&self.columns[8],&self.columns[9],]
}
pub fn LockedBehind(&self) -> &ColumnData {
&self.columns[10]
}
pub fn WKSMissionSupplyItem(&self) -> &ColumnData {
&self.columns[11]
}
pub fn WKSMissionRecipe(&self) -> &ColumnData {
&self.columns[12]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[13]
}
pub fn SortKey(&self) -> &ColumnData {
&self.columns[14]
}
pub fn WKSMissionText(&self) -> &ColumnData {
&self.columns[15]
}
pub fn WKSFunction(&self) -> &ColumnData {
&self.columns[16]
}
pub fn LevelGroup(&self) -> &ColumnData {
&self.columns[17]
}
pub fn WKSMissionLotterySpecialCond(&self) -> &ColumnData {
&self.columns[18]
}
pub fn IsSynced(&self) -> &ColumnData {
&self.columns[19]
}
pub fn IsSpecialQuest(&self) -> &ColumnData {
&self.columns[20]
}
}
