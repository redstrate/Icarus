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
pub struct LeveSheet {
    sheet: Sheet,
}
impl LeveSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Leve")?;
        let sheet = resolver.read_excel_sheet(&exh, "Leve", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<LeveRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<LeveRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for LeveSheet {
    type Row = LeveRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Description: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            ExpFactor: row
                .columns[20]
                .into_f32()
                .copied()
                .expect("Expected column 20 to be a float32!"),
            ExpReward: row
                .columns[21]
                .into_u32()
                .copied()
                .expect("Expected column 21 to be a uint32!"),
            GilReward: row
                .columns[22]
                .into_u32()
                .copied()
                .expect("Expected column 22 to be a uint32!"),
            LeveRewardItem: row
                .columns[23]
                .into_u16()
                .copied()
                .expect("Expected column 23 to be a uint16!"),
            JournalGenre: row
                .columns[14]
                .into_u32()
                .copied()
                .expect("Expected column 14 to be a uint32!"),
            LevelLevemete: row
                .columns[26]
                .into_u32()
                .copied()
                .expect("Expected column 26 to be a uint32!"),
            LevelStart: row
                .columns[29]
                .into_u32()
                .copied()
                .expect("Expected column 29 to be a uint32!"),
            LeveClient: row
                .columns[2]
                .into_i32()
                .copied()
                .expect("Expected column 2 to be a int32!"),
            LeveAssignmentType: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            Town: row
                .columns[5]
                .into_i32()
                .copied()
                .expect("Expected column 5 to be a int32!"),
            PlaceNameStart: row
                .columns[9]
                .into_i32()
                .copied()
                .expect("Expected column 9 to be a int32!"),
            PlaceNameIssued: row
                .columns[10]
                .into_i32()
                .copied()
                .expect("Expected column 10 to be a int32!"),
            PlaceNameStartZone: row
                .columns[15]
                .into_i32()
                .copied()
                .expect("Expected column 15 to be a int32!"),
            IconCityState: row
                .columns[16]
                .into_i32()
                .copied()
                .expect("Expected column 16 to be a int32!"),
            DataId: row
                .columns[17]
                .into_i32()
                .copied()
                .expect("Expected column 17 to be a int32!"),
            IconIssuer: row
                .columns[27]
                .into_i32()
                .copied()
                .expect("Expected column 27 to be a int32!"),
            ClassJobLevel: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            FishingSpot: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            BGM: row
                .columns[30]
                .into_u16()
                .copied()
                .expect("Expected column 30 to be a uint16!"),
            Unknown1: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            TimeLimit: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            AllowanceCost: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            Unknown2: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            ClassJobCategory: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            MaxDifficulty: row
                .columns[19]
                .into_u8()
                .copied()
                .expect("Expected column 19 to be a uint8!"),
            LeveVfx: row
                .columns[24]
                .into_u8()
                .copied()
                .expect("Expected column 24 to be a uint8!"),
            LeveVfxFrame: row
                .columns[25]
                .into_u8()
                .copied()
                .expect("Expected column 25 to be a uint8!"),
            CanCancel: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            LockedLeve: row
                .columns[28]
                .into_bool()
                .copied()
                .expect("Expected column 28 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a LeveSheet {
    type Item = (u32, Vec<(u16, LeveRow)>);
    type IntoIter = StructuredSheetIterator<'a, LeveSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, LeveSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LeveRow {
    ///""
    pub Name: String,
    ///""
    pub Description: String,
    ///""
    pub ExpFactor: f32,
    ///""
    pub ExpReward: u32,
    ///""
    pub GilReward: u32,
    ///""
    pub LeveRewardItem: u16,
    ///""
    pub JournalGenre: u32,
    ///""
    pub LevelLevemete: u32,
    ///""
    pub LevelStart: u32,
    ///""
    pub LeveClient: i32,
    ///""
    pub LeveAssignmentType: i32,
    ///""
    pub Town: i32,
    ///""
    pub PlaceNameStart: i32,
    ///""
    pub PlaceNameIssued: i32,
    ///""
    pub PlaceNameStartZone: i32,
    ///""
    pub IconCityState: i32,
    ///""
    pub DataId: i32,
    ///""
    pub IconIssuer: i32,
    ///""
    pub ClassJobLevel: u16,
    ///""
    pub FishingSpot: u16,
    ///""
    pub BGM: u16,
    ///""
    pub Unknown1: u8,
    ///""
    pub TimeLimit: u8,
    ///""
    pub AllowanceCost: u8,
    ///""
    pub Unknown2: u8,
    ///""
    pub ClassJobCategory: u8,
    ///""
    pub MaxDifficulty: u8,
    ///""
    pub LeveVfx: u8,
    ///""
    pub LeveVfxFrame: u8,
    ///""
    pub CanCancel: bool,
    ///""
    pub LockedLeve: bool,
}
