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
pub struct StatusSheet {
    sheet: Sheet,
}
impl StatusSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Status")?;
        let sheet = resolver.read_excel_sheet(&exh, "Status", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<StatusRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<StatusRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for StatusSheet {
    type Row = StatusRow;
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
            Icon: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            ParamModifier: row
                .columns[23]
                .into_i32()
                .copied()
                .expect("Expected column 23 to be a int32!"),
            VFX: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Log: row
                .columns[26]
                .into_u16()
                .copied()
                .expect("Expected column 26 to be a uint16!"),
            ExclusionGroup: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            MaxStacks: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            ClassJobCategory: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            StatusCategory: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            HitEffect: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            PartyListPriority: row
                .columns[19]
                .into_u8()
                .copied()
                .expect("Expected column 19 to be a uint8!"),
            CanIncreaseRewards: row
                .columns[20]
                .into_u8()
                .copied()
                .expect("Expected column 20 to be a uint8!"),
            ParamEffect: row
                .columns[24]
                .into_u8()
                .copied()
                .expect("Expected column 24 to be a uint8!"),
            TargetType: row
                .columns[30]
                .into_u8()
                .copied()
                .expect("Expected column 30 to be a uint8!"),
            Flags: row
                .columns[31]
                .into_u8()
                .copied()
                .expect("Expected column 31 to be a uint8!"),
            Flag2: row
                .columns[32]
                .into_u8()
                .copied()
                .expect("Expected column 32 to be a uint8!"),
            Unknown_70_1: row
                .columns[36]
                .into_u8()
                .copied()
                .expect("Expected column 36 to be a uint8!"),
            Unknown2: row
                .columns[28]
                .into_i8()
                .copied()
                .expect("Expected column 28 to be a int8!"),
            LockMovement: row
                .columns[9]
                .into_bool()
                .copied()
                .expect("Expected column 9 to be a bool!"),
            Unknown3: row
                .columns[10]
                .into_bool()
                .copied()
                .expect("Expected column 10 to be a bool!"),
            LockActions: row
                .columns[11]
                .into_bool()
                .copied()
                .expect("Expected column 11 to be a bool!"),
            LockControl: row
                .columns[12]
                .into_bool()
                .copied()
                .expect("Expected column 12 to be a bool!"),
            Transfiguration: row
                .columns[13]
                .into_bool()
                .copied()
                .expect("Expected column 13 to be a bool!"),
            IsGaze: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
            CanDispel: row
                .columns[15]
                .into_bool()
                .copied()
                .expect("Expected column 15 to be a bool!"),
            Unknown8: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            InflictedByActor: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            IsPermanent: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            NoLogVfx: row
                .columns[21]
                .into_bool()
                .copied()
                .expect("Expected column 21 to be a bool!"),
            Unknown5: row
                .columns[22]
                .into_bool()
                .copied()
                .expect("Expected column 22 to be a bool!"),
            CanStatusOff: row
                .columns[25]
                .into_bool()
                .copied()
                .expect("Expected column 25 to be a bool!"),
            IsFcBuff: row
                .columns[27]
                .into_bool()
                .copied()
                .expect("Expected column 27 to be a bool!"),
            Invisibility: row
                .columns[29]
                .into_bool()
                .copied()
                .expect("Expected column 29 to be a bool!"),
            Unknown6: row
                .columns[33]
                .into_bool()
                .copied()
                .expect("Expected column 33 to be a bool!"),
            Unknown_70_2: row
                .columns[34]
                .into_bool()
                .copied()
                .expect("Expected column 34 to be a bool!"),
            Unknown7: row
                .columns[35]
                .into_bool()
                .copied()
                .expect("Expected column 35 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a StatusSheet {
    type Item = (u32, Vec<(u16, StatusRow)>);
    type IntoIter = StructuredSheetIterator<'a, StatusSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, StatusSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct StatusRow {
    ///""
    pub Name: String,
    ///""
    pub Description: String,
    ///""
    pub Icon: u32,
    ///""
    pub ParamModifier: i32,
    ///""
    pub VFX: u16,
    ///""
    pub Log: u16,
    ///""
    pub ExclusionGroup: u8,
    ///""
    pub MaxStacks: u8,
    ///""
    pub ClassJobCategory: u8,
    ///""
    pub StatusCategory: u8,
    ///""
    pub HitEffect: u8,
    ///""
    pub PartyListPriority: u8,
    ///""
    pub CanIncreaseRewards: u8,
    ///""
    pub ParamEffect: u8,
    ///""
    pub TargetType: u8,
    ///"actually an index of the flag"
    pub Flags: u8,
    ///""
    pub Flag2: u8,
    ///""
    pub Unknown_70_1: u8,
    ///""
    pub Unknown2: i8,
    ///""
    pub LockMovement: bool,
    ///""
    pub Unknown3: bool,
    ///""
    pub LockActions: bool,
    ///""
    pub LockControl: bool,
    ///""
    pub Transfiguration: bool,
    ///""
    pub IsGaze: bool,
    ///""
    pub CanDispel: bool,
    ///""
    pub Unknown8: bool,
    ///""
    pub InflictedByActor: bool,
    ///""
    pub IsPermanent: bool,
    ///""
    pub NoLogVfx: bool,
    ///""
    pub Unknown5: bool,
    ///""
    pub CanStatusOff: bool,
    ///""
    pub IsFcBuff: bool,
    ///""
    pub Invisibility: bool,
    ///""
    pub Unknown6: bool,
    ///""
    pub Unknown_70_2: bool,
    ///""
    pub Unknown7: bool,
}
