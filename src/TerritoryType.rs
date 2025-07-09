#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct TerritoryTypeSheet {
exd: EXD,
exh: EXH,
}
impl TerritoryTypeSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "TerritoryType")?;let exd = read_excel_sheet(resource, "TerritoryType", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<TerritoryTypeRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(TerritoryTypeRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<TerritoryTypeRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<TerritoryTypeRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct TerritoryTypeRow {
columns: Vec<ColumnData>,
}
impl TerritoryTypeRow {
pub fn Name(&self) -> &ColumnData {
&self.columns[0]
}
pub fn Bg(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ArrayEventHandler(&self) -> &ColumnData {
&self.columns[2]
}
pub fn PlaceNameRegionIcon(&self) -> &ColumnData {
&self.columns[3]
}
pub fn PlaceNameIcon(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Aetheryte(&self) -> &ColumnData {
&self.columns[5]
}
pub fn FixedTime(&self) -> &ColumnData {
&self.columns[6]
}
pub fn PlaceNameRegion(&self) -> &ColumnData {
&self.columns[7]
}
pub fn PlaceNameZone(&self) -> &ColumnData {
&self.columns[8]
}
pub fn PlaceName(&self) -> &ColumnData {
&self.columns[9]
}
pub fn Map(&self) -> &ColumnData {
&self.columns[10]
}
pub fn ContentFinderCondition(&self) -> &ColumnData {
&self.columns[11]
}
pub fn BGM(&self) -> &ColumnData {
&self.columns[12]
}
pub fn QuestBattle(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Resident(&self) -> &ColumnData {
&self.columns[14]
}
pub fn NotoriousMonsterTerritory(&self) -> &ColumnData {
&self.columns[15]
}
pub fn BattalionMode(&self) -> &ColumnData {
&self.columns[16]
}
pub fn LoadingImage(&self) -> &ColumnData {
&self.columns[17]
}
pub fn ExclusiveType(&self) -> &ColumnData {
&self.columns[18]
}
pub fn TerritoryIntendedUse(&self) -> &ColumnData {
&self.columns[19]
}
pub fn WeatherRate(&self) -> &ColumnData {
&self.columns[20]
}
pub fn Unknown1(&self) -> &ColumnData {
&self.columns[21]
}
pub fn ExVersion(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown2(&self) -> &ColumnData {
&self.columns[23]
}
pub fn ZoneSharedGroup(&self) -> &ColumnData {
&self.columns[24]
}
pub fn AetherCurrentCompFlgSet(&self) -> &ColumnData {
&self.columns[25]
}
pub fn MountSpeed(&self) -> &ColumnData {
&self.columns[26]
}
pub fn IndividualWeather(&self) -> &ColumnData {
&self.columns[27]
}
pub fn AchievementIndex(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[30]
}
pub fn PCSearch(&self) -> &ColumnData {
&self.columns[31]
}
pub fn Stealth(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Mount(&self) -> &ColumnData {
&self.columns[33]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[34]
}
pub fn IsPvpZone(&self) -> &ColumnData {
&self.columns[35]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[36]
}
pub fn Unknown10(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Unknown11(&self) -> &ColumnData {
&self.columns[38]
}
pub fn Unknown12(&self) -> &ColumnData {
&self.columns[39]
}
pub fn Unknown13(&self) -> &ColumnData {
&self.columns[40]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[41]
}
pub fn Unknown15(&self) -> &ColumnData {
&self.columns[42]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[43]
}
}
