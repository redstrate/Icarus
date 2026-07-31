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
pub struct ContentRouletteSheet {
    sheet: Sheet,
}
impl ContentRouletteSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ContentRoulette")?;
        let sheet = resolver.read_excel_sheet(&exh, "ContentRoulette", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ContentRouletteRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ContentRouletteRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ContentRouletteSheet {
    type Row = ContentRouletteRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Category: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            Unknown0: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            Description: row
                .columns[3]
                .into_string()
                .cloned()
                .expect("Expected column 3 to be a string!"),
            DutyType: row
                .columns[4]
                .into_string()
                .cloned()
                .expect("Expected column 4 to be a string!"),
            Unknown1: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            Image: row
                .columns[16]
                .into_u32()
                .copied()
                .expect("Expected column 16 to be a uint32!"),
            RewardPvPExp: row
                .columns[21]
                .into_u32()
                .copied()
                .expect("Expected column 21 to be a uint32!"),
            RewardSeriesExp: row
                .columns[23]
                .into_u32()
                .copied()
                .expect("Expected column 23 to be a uint32!"),
            Region: row
                .columns[47]
                .into_i32()
                .copied()
                .expect("Expected column 47 to be a int32!"),
            ItemLevelRequired: row
                .columns[13]
                .into_u16()
                .copied()
                .expect("Expected column 13 to be a uint16!"),
            ItemLevelSync: row
                .columns[15]
                .into_u16()
                .copied()
                .expect("Expected column 15 to be a uint16!"),
            RewardTomeA: row
                .columns[18]
                .into_u16()
                .copied()
                .expect("Expected column 18 to be a uint16!"),
            RewardTomeB: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            RewardTomeC: row
                .columns[20]
                .into_u16()
                .copied()
                .expect("Expected column 20 to be a uint16!"),
            RewardWolfMarks: row
                .columns[22]
                .into_u16()
                .copied()
                .expect("Expected column 22 to be a uint16!"),
            InstanceContent: row
                .columns[41]
                .into_u16()
                .copied()
                .expect("Expected column 41 to be a uint16!"),
            RequiredExVersion: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            OpenRule: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            RequiredLevel: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            SyncedFromLevel: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            ContentRouletteRoleBonus: row
                .columns[17]
                .into_u8()
                .copied()
                .expect("Expected column 17 to be a uint8!"),
            SortKey: row
                .columns[24]
                .into_u8()
                .copied()
                .expect("Expected column 24 to be a uint8!"),
            ClassJobCategory: row
                .columns[25]
                .into_u8()
                .copied()
                .expect("Expected column 25 to be a uint8!"),
            ContentMemberType: row
                .columns[26]
                .into_u8()
                .copied()
                .expect("Expected column 26 to be a uint8!"),
            Unknown9: row
                .columns[27]
                .into_u8()
                .copied()
                .expect("Expected column 27 to be a uint8!"),
            QueueMaxPlayers: row
                .columns[28]
                .into_u8()
                .copied()
                .expect("Expected column 28 to be a uint8!"),
            ContentType: row
                .columns[31]
                .into_u8()
                .copied()
                .expect("Expected column 31 to be a uint8!"),
            UICategory: row
                .columns[32]
                .into_u8()
                .copied()
                .expect("Expected column 32 to be a uint8!"),
            TimeLimit: row
                .columns[33]
                .into_u8()
                .copied()
                .expect("Expected column 33 to be a uint8!"),
            TimeLimitMax: row
                .columns[34]
                .into_u8()
                .copied()
                .expect("Expected column 34 to be a uint8!"),
            LootModeType: row
                .columns[40]
                .into_u8()
                .copied()
                .expect("Expected column 40 to be a uint8!"),
            Unknown15: row
                .columns[43]
                .into_u8()
                .copied()
                .expect("Expected column 43 to be a uint8!"),
            PenaltyTimestampArrayIndex: row
                .columns[48]
                .into_u8()
                .copied()
                .expect("Expected column 48 to be a uint8!"),
            CompletionArrayIndex: row
                .columns[29]
                .into_i8()
                .copied()
                .expect("Expected column 29 to be a int8!"),
            IsGoldSaucer: row
                .columns[7]
                .into_bool()
                .copied()
                .expect("Expected column 7 to be a bool!"),
            IsInDutyFinder: row
                .columns[8]
                .into_bool()
                .copied()
                .expect("Expected column 8 to be a bool!"),
            IsPvP: row
                .columns[10]
                .into_bool()
                .copied()
                .expect("Expected column 10 to be a bool!"),
            AppliesHighestAverageDutyItemLevel: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
            Unknown18: row
                .columns[30]
                .into_bool()
                .copied()
                .expect("Expected column 30 to be a bool!"),
            AllowConsumableItems: row
                .columns[35]
                .into_bool()
                .copied()
                .expect("Expected column 35 to be a bool!"),
            AllowPhoenixDown: row
                .columns[36]
                .into_bool()
                .copied()
                .expect("Expected column 36 to be a bool!"),
            AllowReplacement: row
                .columns[37]
                .into_bool()
                .copied()
                .expect("Expected column 37 to be a bool!"),
            RatedMatch: row
                .columns[38]
                .into_bool()
                .copied()
                .expect("Expected column 38 to be a bool!"),
            Rated: row
                .columns[39]
                .into_bool()
                .copied()
                .expect("Expected column 39 to be a bool!"),
            Unknown22: row
                .columns[42]
                .into_bool()
                .copied()
                .expect("Expected column 42 to be a bool!"),
            Unknown23: row
                .columns[44]
                .into_bool()
                .copied()
                .expect("Expected column 44 to be a bool!"),
            Unknown24: row
                .columns[45]
                .into_bool()
                .copied()
                .expect("Expected column 45 to be a bool!"),
            IsRegistrationAllowedFromAnyDataCenter: row
                .columns[46]
                .into_bool()
                .copied()
                .expect("Expected column 46 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ContentRouletteSheet {
    type Item = (u32, Vec<(u16, ContentRouletteRow)>);
    type IntoIter = StructuredSheetIterator<'a, ContentRouletteSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ContentRouletteSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ContentRouletteRow {
    ///""
    pub Name: String,
    ///""
    pub Category: String,
    ///""
    pub Unknown0: String,
    ///""
    pub Description: String,
    ///""
    pub DutyType: String,
    ///"This would show Addon#102618, but the row is empty."
    pub Unknown1: u32,
    ///""
    pub Image: u32,
    ///""
    pub RewardPvPExp: u32,
    ///""
    pub RewardSeriesExp: u32,
    ///""
    pub Region: i32,
    ///""
    pub ItemLevelRequired: u16,
    ///""
    pub ItemLevelSync: u16,
    ///""
    pub RewardTomeA: u16,
    ///""
    pub RewardTomeB: u16,
    ///""
    pub RewardTomeC: u16,
    ///""
    pub RewardWolfMarks: u16,
    ///""
    pub InstanceContent: u16,
    ///""
    pub RequiredExVersion: u8,
    ///""
    pub OpenRule: u8,
    ///""
    pub RequiredLevel: u8,
    ///""
    pub SyncedFromLevel: u8,
    ///""
    pub ContentRouletteRoleBonus: u8,
    ///""
    pub SortKey: u8,
    ///""
    pub ClassJobCategory: u8,
    ///""
    pub ContentMemberType: u8,
    ///""
    pub Unknown9: u8,
    ///""
    pub QueueMaxPlayers: u8,
    ///""
    pub ContentType: u8,
    ///""
    pub UICategory: u8,
    ///"In minutes."
    pub TimeLimit: u8,
    ///"In minutes. If 0, only TimeLimit is displayed."
    pub TimeLimitMax: u8,
    ///""
    pub LootModeType: u8,
    ///""
    pub Unknown15: u8,
    ///"Index in PlayerState.PenaltyTimestamps"
    pub PenaltyTimestampArrayIndex: u8,
    ///"Index in PlayerState.ContentRouletteCompletion"
    pub CompletionArrayIndex: i8,
    ///""
    pub IsGoldSaucer: bool,
    ///""
    pub IsInDutyFinder: bool,
    ///""
    pub IsPvP: bool,
    ///"Displays Addon#2828."
    pub AppliesHighestAverageDutyItemLevel: bool,
    ///""
    pub Unknown18: bool,
    ///""
    pub AllowConsumableItems: bool,
    ///""
    pub AllowPhoenixDown: bool,
    ///""
    pub AllowReplacement: bool,
    ///""
    pub RatedMatch: bool,
    ///""
    pub Rated: bool,
    ///"This would show Addon#10833, but the row does not exist."
    pub Unknown22: bool,
    ///""
    pub Unknown23: bool,
    ///""
    pub Unknown24: bool,
    ///""
    pub IsRegistrationAllowedFromAnyDataCenter: bool,
}
