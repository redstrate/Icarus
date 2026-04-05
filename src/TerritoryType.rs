//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Debug, Clone)]
pub struct TerritoryTypeSheet {
    sheet: Sheet,
}
impl TerritoryTypeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TerritoryType")?;
        let sheet = resolver.read_excel_sheet(&exh, "TerritoryType", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<TerritoryTypeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<TerritoryTypeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for TerritoryTypeSheet {
    type Row = TerritoryTypeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a TerritoryTypeSheet {
    type Item = (u32, Vec<(u16, TerritoryTypeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, TerritoryTypeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TerritoryTypeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TerritoryTypeRow<'a> {
    row: &'a Row,
}
impl<'a> TerritoryTypeRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Bg(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn ArrayEventHandler(&'a self) -> u32 {
        self.row.columns[22].into_u32().copied().unwrap()
    }
    pub fn PlaceNameRegionIcon(&'a self) -> i32 {
        self.row.columns[20].into_i32().copied().unwrap()
    }
    pub fn PlaceNameIcon(&'a self) -> i32 {
        self.row.columns[21].into_i32().copied().unwrap()
    }
    pub fn Aetheryte(&'a self) -> i32 {
        self.row.columns[24].into_i32().copied().unwrap()
    }
    pub fn FixedTime(&'a self) -> i32 {
        self.row.columns[25].into_i32().copied().unwrap()
    }
    pub fn PlaceNameRegion(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn PlaceNameZone(&'a self) -> u16 {
        self.row.columns[4].into_u16().copied().unwrap()
    }
    pub fn PlaceName(&'a self) -> u16 {
        self.row.columns[5].into_u16().copied().unwrap()
    }
    pub fn Map(&'a self) -> u16 {
        self.row.columns[6].into_u16().copied().unwrap()
    }
    pub fn ContentFinderCondition(&'a self) -> u16 {
        self.row.columns[10].into_u16().copied().unwrap()
    }
    pub fn BGM(&'a self) -> u16 {
        self.row.columns[19].into_u16().copied().unwrap()
    }
    pub fn QuestBattle(&'a self) -> u16 {
        self.row.columns[23].into_u16().copied().unwrap()
    }
    pub fn Resident(&'a self) -> u16 {
        self.row.columns[26].into_u16().copied().unwrap()
    }
    pub fn NotoriousMonsterTerritory(&'a self) -> u16 {
        self.row.columns[43].into_u16().copied().unwrap()
    }
    pub fn BattalionMode(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn LoadingImage(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn ExclusiveType(&'a self) -> u8 {
        self.row.columns[8].into_u8().copied().unwrap()
    }
    pub fn TerritoryIntendedUse(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn WeatherRate(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u8 {
        self.row.columns[14].into_u8().copied().unwrap()
    }
    pub fn ExVersion(&'a self) -> u8 {
        self.row.columns[29].into_u8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u8 {
        self.row.columns[30].into_u8().copied().unwrap()
    }
    pub fn ZoneSharedGroup(&'a self) -> u8 {
        self.row.columns[31].into_u8().copied().unwrap()
    }
    pub fn AetherCurrentCompFlgSet(&'a self) -> u8 {
        self.row.columns[32].into_u8().copied().unwrap()
    }
    pub fn MountSpeed(&'a self) -> u8 {
        self.row.columns[33].into_u8().copied().unwrap()
    }
    pub fn IndividualWeather(&'a self) -> u8 {
        self.row.columns[36].into_u8().copied().unwrap()
    }
    pub fn AchievementIndex(&'a self) -> i8 {
        self.row.columns[27].into_i8().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> bool {
        self.row.columns[11].into_bool().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> bool {
        self.row.columns[13].into_bool().copied().unwrap()
    }
    pub fn PCSearch(&'a self) -> bool {
        self.row.columns[15].into_bool().copied().unwrap()
    }
    pub fn Stealth(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
    pub fn Mount(&'a self) -> bool {
        self.row.columns[17].into_bool().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> bool {
        self.row.columns[18].into_bool().copied().unwrap()
    }
    pub fn IsPvpZone(&'a self) -> bool {
        self.row.columns[28].into_bool().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> bool {
        self.row.columns[34].into_bool().copied().unwrap()
    }
    pub fn IsInUse(&'a self) -> bool {
        self.row.columns[35].into_bool().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> bool {
        self.row.columns[37].into_bool().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> bool {
        self.row.columns[38].into_bool().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> bool {
        self.row.columns[39].into_bool().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> bool {
        self.row.columns[40].into_bool().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> bool {
        self.row.columns[41].into_bool().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> bool {
        self.row.columns[42].into_bool().copied().unwrap()
    }
}
