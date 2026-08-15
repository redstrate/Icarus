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
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 1u32;
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
impl StructuredSheet for TerritoryTypeSheet {
    type Row = TerritoryTypeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Bg: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            ArrayEventHandler: row
                .columns[22]
                .into_u32()
                .copied()
                .expect("Expected column 22 to be a uint32!"),
            PlaceNameRegionIcon: row
                .columns[20]
                .into_i32()
                .copied()
                .expect("Expected column 20 to be a int32!"),
            PlaceNameIcon: row
                .columns[21]
                .into_i32()
                .copied()
                .expect("Expected column 21 to be a int32!"),
            Aetheryte: row
                .columns[24]
                .into_i32()
                .copied()
                .expect("Expected column 24 to be a int32!"),
            FixedTime: row
                .columns[25]
                .into_i32()
                .copied()
                .expect("Expected column 25 to be a int32!"),
            PlaceNameRegion: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            PlaceNameZone: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            PlaceName: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            Map: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            ContentFinderCondition: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            BGM: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            QuestBattle: row
                .columns[23]
                .into_u16()
                .copied()
                .expect("Expected column 23 to be a uint16!"),
            Resident: row
                .columns[26]
                .into_u16()
                .copied()
                .expect("Expected column 26 to be a uint16!"),
            NotoriousMonsterTerritory: row
                .columns[43]
                .into_u16()
                .copied()
                .expect("Expected column 43 to be a uint16!"),
            BattalionMode: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            LoadingImage: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            ExclusiveType: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            TerritoryIntendedUse: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            WeatherRate: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            Unknown1: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
            ExVersion: row
                .columns[29]
                .into_u8()
                .copied()
                .expect("Expected column 29 to be a uint8!"),
            Unknown2: row
                .columns[30]
                .into_u8()
                .copied()
                .expect("Expected column 30 to be a uint8!"),
            ZoneSharedGroup: row
                .columns[31]
                .into_u8()
                .copied()
                .expect("Expected column 31 to be a uint8!"),
            AetherCurrentCompFlgSet: row
                .columns[32]
                .into_u8()
                .copied()
                .expect("Expected column 32 to be a uint8!"),
            MountSpeed: row
                .columns[33]
                .into_u8()
                .copied()
                .expect("Expected column 33 to be a uint8!"),
            IndividualWeather: row
                .columns[36]
                .into_u8()
                .copied()
                .expect("Expected column 36 to be a uint8!"),
            AchievementIndex: row
                .columns[27]
                .into_i8()
                .copied()
                .expect("Expected column 27 to be a int8!"),
            Unknown6: row
                .columns[11]
                .into_bool()
                .copied()
                .expect("Expected column 11 to be a bool!"),
            Unknown7: row
                .columns[13]
                .into_bool()
                .copied()
                .expect("Expected column 13 to be a bool!"),
            PCSearch: row
                .columns[15]
                .into_bool()
                .copied()
                .expect("Expected column 15 to be a bool!"),
            Stealth: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            Mount: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            Unknown8: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            IsPvpZone: row
                .columns[28]
                .into_bool()
                .copied()
                .expect("Expected column 28 to be a bool!"),
            Unknown9: row
                .columns[34]
                .into_bool()
                .copied()
                .expect("Expected column 34 to be a bool!"),
            IsInUse: row
                .columns[35]
                .into_bool()
                .copied()
                .expect("Expected column 35 to be a bool!"),
            Unknown11: row
                .columns[37]
                .into_bool()
                .copied()
                .expect("Expected column 37 to be a bool!"),
            Unknown12: row
                .columns[38]
                .into_bool()
                .copied()
                .expect("Expected column 38 to be a bool!"),
            Unknown13: row
                .columns[39]
                .into_bool()
                .copied()
                .expect("Expected column 39 to be a bool!"),
            Unknown14: row
                .columns[40]
                .into_bool()
                .copied()
                .expect("Expected column 40 to be a bool!"),
            Unknown15: row
                .columns[41]
                .into_bool()
                .copied()
                .expect("Expected column 41 to be a bool!"),
            Unknown16: row
                .columns[42]
                .into_bool()
                .copied()
                .expect("Expected column 42 to be a bool!"),
            Unknown17: row
                .columns[44]
                .into_bool()
                .copied()
                .expect("Expected column 44 to be a bool!"),
            Unknown18: row
                .columns[45]
                .into_bool()
                .copied()
                .expect("Expected column 45 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a TerritoryTypeSheet {
    type Item = (u32, Vec<(u16, TerritoryTypeRow)>);
    type IntoIter = StructuredSheetIterator<'a, TerritoryTypeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TerritoryTypeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TerritoryTypeRow {
    ///""
    pub Name: String,
    ///""
    pub Bg: String,
    ///""
    pub ArrayEventHandler: u32,
    ///""
    pub PlaceNameRegionIcon: i32,
    ///""
    pub PlaceNameIcon: i32,
    ///""
    pub Aetheryte: i32,
    ///""
    pub FixedTime: i32,
    ///""
    pub PlaceNameRegion: u16,
    ///""
    pub PlaceNameZone: u16,
    ///""
    pub PlaceName: u16,
    ///""
    pub Map: u16,
    ///""
    pub ContentFinderCondition: u16,
    ///""
    pub BGM: u16,
    ///""
    pub QuestBattle: u16,
    ///""
    pub Resident: u16,
    ///""
    pub NotoriousMonsterTerritory: u16,
    ///""
    pub BattalionMode: u8,
    ///""
    pub LoadingImage: u8,
    ///""
    pub ExclusiveType: u8,
    ///""
    pub TerritoryIntendedUse: u8,
    ///""
    pub WeatherRate: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub ExVersion: u8,
    ///""
    pub Unknown2: u8,
    ///""
    pub ZoneSharedGroup: u8,
    ///""
    pub AetherCurrentCompFlgSet: u8,
    ///""
    pub MountSpeed: u8,
    ///""
    pub IndividualWeather: u8,
    ///""
    pub AchievementIndex: i8,
    ///""
    pub Unknown6: bool,
    ///""
    pub Unknown7: bool,
    ///""
    pub PCSearch: bool,
    ///""
    pub Stealth: bool,
    ///""
    pub Mount: bool,
    ///""
    pub Unknown8: bool,
    ///""
    pub IsPvpZone: bool,
    ///""
    pub Unknown9: bool,
    ///""
    pub IsInUse: bool,
    ///""
    pub Unknown11: bool,
    ///""
    pub Unknown12: bool,
    ///""
    pub Unknown13: bool,
    ///""
    pub Unknown14: bool,
    ///""
    pub Unknown15: bool,
    ///""
    pub Unknown16: bool,
    ///""
    pub Unknown17: bool,
    ///""
    pub Unknown18: bool,
}
