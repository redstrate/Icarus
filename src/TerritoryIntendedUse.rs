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
pub struct TerritoryIntendedUseSheet {
    sheet: Sheet,
}
impl TerritoryIntendedUseSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TerritoryIntendedUse")?;
        let sheet = resolver.read_excel_sheet(&exh, "TerritoryIntendedUse", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<TerritoryIntendedUseRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<TerritoryIntendedUseRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for TerritoryIntendedUseSheet {
    type Row = TerritoryIntendedUseRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            GayaSoundId: row
                .columns[20]
                .into_u8()
                .copied()
                .expect("Expected column 20 to be a uint8!"),
            ChatRule: row
                .columns[33]
                .into_u8()
                .copied()
                .expect("Expected column 33 to be a uint8!"),
            Unknown3: row
                .columns[36]
                .into_u8()
                .copied()
                .expect("Expected column 36 to be a uint8!"),
            EnableCrafting: row
                .columns[0]
                .into_bool()
                .copied()
                .expect("Expected column 0 to be a bool!"),
            EnableGathering: row
                .columns[1]
                .into_bool()
                .copied()
                .expect("Expected column 1 to be a bool!"),
            EnableRepairs: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
            EnableClassJobChange: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
            EnableClassJobChangeCombatJobCheck: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
            EnableJobChangeToLimitedJob: row
                .columns[5]
                .into_bool()
                .copied()
                .expect("Expected column 5 to be a bool!"),
            EnableCompanion: row
                .columns[6]
                .into_bool()
                .copied()
                .expect("Expected column 6 to be a bool!"),
            EnablePets: row
                .columns[7]
                .into_bool()
                .copied()
                .expect("Expected column 7 to be a bool!"),
            EnableRidePillion: row
                .columns[8]
                .into_bool()
                .copied()
                .expect("Expected column 8 to be a bool!"),
            DisableLogoutTimer: row
                .columns[9]
                .into_bool()
                .copied()
                .expect("Expected column 9 to be a bool!"),
            EnableTeleport: row
                .columns[10]
                .into_bool()
                .copied()
                .expect("Expected column 10 to be a bool!"),
            EnableReturn: row
                .columns[11]
                .into_bool()
                .copied()
                .expect("Expected column 11 to be a bool!"),
            Unknown16: row
                .columns[12]
                .into_bool()
                .copied()
                .expect("Expected column 12 to be a bool!"),
            Unknown17: row
                .columns[13]
                .into_bool()
                .copied()
                .expect("Expected column 13 to be a bool!"),
            EnableRecommendList: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
            Unknown19: row
                .columns[15]
                .into_bool()
                .copied()
                .expect("Expected column 15 to be a bool!"),
            Unknown20: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            DisableMapDiscovery: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            DisableFieldMarkers: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            Unknown23: row
                .columns[21]
                .into_bool()
                .copied()
                .expect("Expected column 21 to be a bool!"),
            EnableActions: row
                .columns[22]
                .into_bool()
                .copied()
                .expect("Expected column 22 to be a bool!"),
            EnableConsumableItems: row
                .columns[23]
                .into_bool()
                .copied()
                .expect("Expected column 23 to be a bool!"),
            Unknown26: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
            EnableTripleTriadMatches: row
                .columns[25]
                .into_bool()
                .copied()
                .expect("Expected column 25 to be a bool!"),
            EnableTripleTriadMatchesAnywhere: row
                .columns[26]
                .into_bool()
                .copied()
                .expect("Expected column 26 to be a bool!"),
            Unknown29: row
                .columns[27]
                .into_bool()
                .copied()
                .expect("Expected column 27 to be a bool!"),
            Unknown30: row
                .columns[28]
                .into_bool()
                .copied()
                .expect("Expected column 28 to be a bool!"),
            Unknown31: row
                .columns[29]
                .into_bool()
                .copied()
                .expect("Expected column 29 to be a bool!"),
            CanPauseTimeWeather: row
                .columns[30]
                .into_bool()
                .copied()
                .expect("Expected column 30 to be a bool!"),
            Unknown33: row
                .columns[31]
                .into_bool()
                .copied()
                .expect("Expected column 31 to be a bool!"),
            EnablePvPQuickChat: row
                .columns[32]
                .into_bool()
                .copied()
                .expect("Expected column 32 to be a bool!"),
            Unknown35: row
                .columns[34]
                .into_bool()
                .copied()
                .expect("Expected column 34 to be a bool!"),
            Unknown36: row
                .columns[35]
                .into_bool()
                .copied()
                .expect("Expected column 35 to be a bool!"),
            CanApplyGlamourPlatesAnywhere: row
                .columns[37]
                .into_bool()
                .copied()
                .expect("Expected column 37 to be a bool!"),
            Unknown38: row
                .columns[38]
                .into_bool()
                .copied()
                .expect("Expected column 38 to be a bool!"),
            EnableFieldMarkerPresets: row
                .columns[39]
                .into_bool()
                .copied()
                .expect("Expected column 39 to be a bool!"),
            IgnoreCombatFlagForMarkers: row
                .columns[40]
                .into_bool()
                .copied()
                .expect("Expected column 40 to be a bool!"),
            Unknown41: row
                .columns[41]
                .into_bool()
                .copied()
                .expect("Expected column 41 to be a bool!"),
            Unknown42: row
                .columns[42]
                .into_bool()
                .copied()
                .expect("Expected column 42 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a TerritoryIntendedUseSheet {
    type Item = (u32, Vec<(u16, TerritoryIntendedUseRow)>);
    type IntoIter = StructuredSheetIterator<'a, TerritoryIntendedUseSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TerritoryIntendedUseSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TerritoryIntendedUseRow {
    ///""
    pub Unknown0: u16,
    ///"Ambient noise of people talking in a crowd. Values: 0 = None, 1 = sound/strm/GAYA_LestArea_01.scd, 2 = sound/strm/GAYA_Village_01.scd"
    pub GayaSoundId: u8,
    ///""
    pub ChatRule: u8,
    ///""
    pub Unknown3: u8,
    ///""
    pub EnableCrafting: bool,
    ///""
    pub EnableGathering: bool,
    ///""
    pub EnableRepairs: bool,
    ///""
    pub EnableClassJobChange: bool,
    ///"When true, requires that the target ClassJob is a combat job"
    pub EnableClassJobChangeCombatJobCheck: bool,
    ///""
    pub EnableJobChangeToLimitedJob: bool,
    ///"Enables summoning Chocobo Companion"
    pub EnableCompanion: bool,
    ///"Enables summoning Summoner/Scholar pets"
    pub EnablePets: bool,
    ///""
    pub EnableRidePillion: bool,
    ///""
    pub DisableLogoutTimer: bool,
    ///""
    pub EnableTeleport: bool,
    ///""
    pub EnableReturn: bool,
    ///""
    pub Unknown16: bool,
    ///""
    pub Unknown17: bool,
    ///""
    pub EnableRecommendList: bool,
    ///""
    pub Unknown19: bool,
    ///""
    pub Unknown20: bool,
    ///""
    pub DisableMapDiscovery: bool,
    ///""
    pub DisableFieldMarkers: bool,
    ///""
    pub Unknown23: bool,
    ///""
    pub EnableActions: bool,
    ///""
    pub EnableConsumableItems: bool,
    ///""
    pub Unknown26: bool,
    ///""
    pub EnableTripleTriadMatches: bool,
    ///""
    pub EnableTripleTriadMatchesAnywhere: bool,
    ///""
    pub Unknown29: bool,
    ///""
    pub Unknown30: bool,
    ///"Related to Idle Cam"
    pub Unknown31: bool,
    ///""
    pub CanPauseTimeWeather: bool,
    ///""
    pub Unknown33: bool,
    ///""
    pub EnablePvPQuickChat: bool,
    ///""
    pub Unknown35: bool,
    ///""
    pub Unknown36: bool,
    ///""
    pub CanApplyGlamourPlatesAnywhere: bool,
    ///"Related to idle timer"
    pub Unknown38: bool,
    ///""
    pub EnableFieldMarkerPresets: bool,
    ///""
    pub IgnoreCombatFlagForMarkers: bool,
    ///"Related to blocking Say/Yell/Shout of a muted player in Eureka/Bozja/OccultCrescent"
    pub Unknown41: bool,
    ///""
    pub Unknown42: bool,
}
