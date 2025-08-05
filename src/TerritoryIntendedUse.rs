#![allow(warnings)]
/// This file is auto-generated! It is generated from schema from https://github.com/xivdev/EXDSchema.
use physis::{resource::{Resource, read_excel_sheet_header, read_excel_sheet}, exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow}, exh::{EXH, ExcelColumnDefinition}, common::Language};
pub struct TerritoryIntendedUseSheet {
exd: EXD,
exh: EXH,
}
impl TerritoryIntendedUseSheet {
pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
let exh = read_excel_sheet_header(resource, "TerritoryIntendedUse")?;let exd = read_excel_sheet(resource, "TerritoryIntendedUse", &exh, language, 0)?;Some(Self {
exh,
exd,
})
}
fn read_row(&self, row: &ExcelSingleRow) -> Option<TerritoryIntendedUseRow> {
let column_defs = &self.exh.column_definitions;
let mut zipped: Vec<_> = row.columns.clone().into_iter().zip(column_defs).collect();
zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition> ) = zipped.into_iter().unzip();
Some(TerritoryIntendedUseRow { columns })
}
/// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
pub fn get_row(&self, row_id: u32) -> Option<TerritoryIntendedUseRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => row,
ExcelRowKind::SubRows(rows) => &rows.first()?.1,
};
self.read_row(row)
}
/// Fetches the specified subrow from the sheet.
pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<TerritoryIntendedUseRow> {
let Some(row) = &self.exd.get_row(row_id) else { return None; };
let row = match row {
ExcelRowKind::SingleRow(row) => return None,
ExcelRowKind::SubRows(subrows) => &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1,
};
self.read_row(row)
}
}
pub struct TerritoryIntendedUseRow {
columns: Vec<ColumnData>,
}
impl TerritoryIntendedUseRow {
pub fn Unknown0(&self) -> &ColumnData {
&self.columns[0]
}
/// Ambient noise of people talking in a crowd. Values: 0 = None, 1 = sound/strm/GAYA_LestArea_01.scd, 2 = sound/strm/GAYA_Village_01.scd
pub fn GayaSoundId(&self) -> &ColumnData {
&self.columns[1]
}
pub fn ChatRule(&self) -> &ColumnData {
&self.columns[2]
}
pub fn Unknown3(&self) -> &ColumnData {
&self.columns[3]
}
pub fn Unknown4(&self) -> &ColumnData {
&self.columns[4]
}
pub fn Unknown5(&self) -> &ColumnData {
&self.columns[5]
}
pub fn Unknown6(&self) -> &ColumnData {
&self.columns[6]
}
pub fn Unknown7(&self) -> &ColumnData {
&self.columns[7]
}
pub fn Unknown8(&self) -> &ColumnData {
&self.columns[8]
}
pub fn Unknown9(&self) -> &ColumnData {
&self.columns[9]
}
/// Enables summoning Chocobo Companion
pub fn EnableCompanion(&self) -> &ColumnData {
&self.columns[10]
}
/// Enables summoning Summoner/Scholar pets
pub fn EnablePets(&self) -> &ColumnData {
&self.columns[11]
}
pub fn EnableRidePillion(&self) -> &ColumnData {
&self.columns[12]
}
pub fn DisableLogoutTimer(&self) -> &ColumnData {
&self.columns[13]
}
pub fn Unknown14(&self) -> &ColumnData {
&self.columns[14]
}
pub fn EnableReturn(&self) -> &ColumnData {
&self.columns[15]
}
pub fn Unknown16(&self) -> &ColumnData {
&self.columns[16]
}
pub fn Unknown17(&self) -> &ColumnData {
&self.columns[17]
}
pub fn EnableRecommendList(&self) -> &ColumnData {
&self.columns[18]
}
pub fn Unknown19(&self) -> &ColumnData {
&self.columns[19]
}
pub fn Unknown20(&self) -> &ColumnData {
&self.columns[20]
}
/// Related to Map in Island Sanctuary, Cosmic Exploration
pub fn Unknown21(&self) -> &ColumnData {
&self.columns[21]
}
pub fn DisableFieldMarkers(&self) -> &ColumnData {
&self.columns[22]
}
pub fn Unknown23(&self) -> &ColumnData {
&self.columns[23]
}
pub fn EnableActions(&self) -> &ColumnData {
&self.columns[24]
}
pub fn Unknown25(&self) -> &ColumnData {
&self.columns[25]
}
pub fn Unknown26(&self) -> &ColumnData {
&self.columns[26]
}
pub fn EnableTripleTriadMatches(&self) -> &ColumnData {
&self.columns[27]
}
pub fn EnableTripleTriadMatchesAnywhere(&self) -> &ColumnData {
&self.columns[28]
}
pub fn Unknown29(&self) -> &ColumnData {
&self.columns[29]
}
pub fn Unknown30(&self) -> &ColumnData {
&self.columns[30]
}
/// Related to Idle Cam filtering EventNpcs/EventObjs??
pub fn Unknown31(&self) -> &ColumnData {
&self.columns[31]
}
pub fn CanPauseTimeWeather(&self) -> &ColumnData {
&self.columns[32]
}
pub fn Unknown33(&self) -> &ColumnData {
&self.columns[33]
}
pub fn EnablePvPQuickChat(&self) -> &ColumnData {
&self.columns[34]
}
pub fn Unknown35(&self) -> &ColumnData {
&self.columns[35]
}
pub fn Unknown36(&self) -> &ColumnData {
&self.columns[36]
}
pub fn CanApplyGlamourPlatesAnywhere(&self) -> &ColumnData {
&self.columns[37]
}
pub fn Unknown38(&self) -> &ColumnData {
&self.columns[38]
}
pub fn EnableFieldMarkerPresets(&self) -> &ColumnData {
&self.columns[39]
}
pub fn Unknown40(&self) -> &ColumnData {
&self.columns[40]
}
/// Related to blocking Say/Yell/Shout in Eureka/Bozja/Unknown61??
pub fn Unknown41(&self) -> &ColumnData {
&self.columns[41]
}
pub fn Unknown42(&self) -> &ColumnData {
&self.columns[42]
}
}
