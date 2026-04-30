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
pub struct MountSheet {
    sheet: Sheet,
}
impl MountSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Mount")?;
        let sheet = resolver.read_excel_sheet(&exh, "Mount", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MountRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MountRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MountSheet {
    type Row = MountRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Singular: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Plural: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            Adjective: row
                .columns[1]
                .into_i8()
                .copied()
                .expect("Expected column 1 to be a int8!"),
            PossessivePronoun: row
                .columns[3]
                .into_i8()
                .copied()
                .expect("Expected column 3 to be a int8!"),
            StartsWithVowel: row
                .columns[4]
                .into_i8()
                .copied()
                .expect("Expected column 4 to be a int8!"),
            Unknown0: row
                .columns[5]
                .into_i8()
                .copied()
                .expect("Expected column 5 to be a int8!"),
            Pronoun: row
                .columns[6]
                .into_i8()
                .copied()
                .expect("Expected column 6 to be a int8!"),
            Article: row
                .columns[7]
                .into_i8()
                .copied()
                .expect("Expected column 7 to be a int8!"),
            Unknown1: row
                .columns[18]
                .into_string()
                .cloned()
                .expect("Expected column 18 to be a string!"),
            Unknown2: row
                .columns[19]
                .into_string()
                .cloned()
                .expect("Expected column 19 to be a string!"),
            Unknown3: row
                .columns[20]
                .into_string()
                .cloned()
                .expect("Expected column 20 to be a string!"),
            ModelChara: row
                .columns[8]
                .into_i32()
                .copied()
                .expect("Expected column 8 to be a int32!"),
            EquipHead: row
                .columns[25]
                .into_i32()
                .copied()
                .expect("Expected column 25 to be a int32!"),
            EquipBody: row
                .columns[26]
                .into_i32()
                .copied()
                .expect("Expected column 26 to be a int32!"),
            EquipLeg: row
                .columns[27]
                .into_i32()
                .copied()
                .expect("Expected column 27 to be a int32!"),
            EquipFoot: row
                .columns[28]
                .into_i32()
                .copied()
                .expect("Expected column 28 to be a int32!"),
            MoveControl: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            RideBGM: row
                .columns[17]
                .into_u16()
                .copied()
                .expect("Expected column 17 to be a uint16!"),
            Icon: row
                .columns[30]
                .into_u16()
                .copied()
                .expect("Expected column 30 to be a uint16!"),
            UIPriority: row
                .columns[31]
                .into_u16()
                .copied()
                .expect("Expected column 31 to be a uint16!"),
            MountAction: row
                .columns[38]
                .into_u16()
                .copied()
                .expect("Expected column 38 to be a uint16!"),
            Unknown_70_1: row
                .columns[48]
                .into_u16()
                .copied()
                .expect("Expected column 48 to be a uint16!"),
            Unknown_70_2: row
                .columns[49]
                .into_u16()
                .copied()
                .expect("Expected column 49 to be a uint16!"),
            Unknown16: row
                .columns[50]
                .into_u16()
                .copied()
                .expect("Expected column 50 to be a uint16!"),
            Unknown17: row
                .columns[52]
                .into_u16()
                .copied()
                .expect("Expected column 52 to be a uint16!"),
            Order: row
                .columns[29]
                .into_i16()
                .copied()
                .expect("Expected column 29 to be a int16!"),
            FlyingCondition: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            Unknown5: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            Unknown6: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            Unknown7: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            IsFlying: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
            Unknown8: row
                .columns[15]
                .into_u8()
                .copied()
                .expect("Expected column 15 to be a uint8!"),
            MountCustomize: row
                .columns[16]
                .into_u8()
                .copied()
                .expect("Expected column 16 to be a uint8!"),
            ExitMoveDist: row
                .columns[21]
                .into_u8()
                .copied()
                .expect("Expected column 21 to be a uint8!"),
            ExitMoveSpeed: row
                .columns[22]
                .into_u8()
                .copied()
                .expect("Expected column 22 to be a uint8!"),
            RadiusRate: row
                .columns[32]
                .into_u8()
                .copied()
                .expect("Expected column 32 to be a uint8!"),
            BaseMotionSpeed_Run: row
                .columns[34]
                .into_u8()
                .copied()
                .expect("Expected column 34 to be a uint8!"),
            BaseMotionSpeed_Walk: row
                .columns[35]
                .into_u8()
                .copied()
                .expect("Expected column 35 to be a uint8!"),
            Unknown9: row
                .columns[36]
                .into_u8()
                .copied()
                .expect("Expected column 36 to be a uint8!"),
            ExtraSeats: row
                .columns[37]
                .into_u8()
                .copied()
                .expect("Expected column 37 to be a uint8!"),
            Unknown10: row
                .columns[44]
                .into_u8()
                .copied()
                .expect("Expected column 44 to be a uint8!"),
            Unknown11: row
                .columns[45]
                .into_u8()
                .copied()
                .expect("Expected column 45 to be a uint8!"),
            Unknown12: row
                .columns[23]
                .into_bool()
                .copied()
                .expect("Expected column 23 to be a bool!"),
            IsEmote: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
            Unknown20: row
                .columns[33]
                .into_bool()
                .copied()
                .expect("Expected column 33 to be a bool!"),
            IsAirborne: row
                .columns[39]
                .into_bool()
                .copied()
                .expect("Expected column 39 to be a bool!"),
            ExHotbarEnableConfig: row
                .columns[40]
                .into_bool()
                .copied()
                .expect("Expected column 40 to be a bool!"),
            UseEP: row
                .columns[41]
                .into_bool()
                .copied()
                .expect("Expected column 41 to be a bool!"),
            Unknown13: row
                .columns[42]
                .into_bool()
                .copied()
                .expect("Expected column 42 to be a bool!"),
            IsImmobile: row
                .columns[43]
                .into_bool()
                .copied()
                .expect("Expected column 43 to be a bool!"),
            Unknown14: row
                .columns[46]
                .into_bool()
                .copied()
                .expect("Expected column 46 to be a bool!"),
            HideHeadgear: row
                .columns[47]
                .into_bool()
                .copied()
                .expect("Expected column 47 to be a bool!"),
            Unknown18: row
                .columns[51]
                .into_bool()
                .copied()
                .expect("Expected column 51 to be a bool!"),
            Unknown19: row
                .columns[53]
                .into_bool()
                .copied()
                .expect("Expected column 53 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a MountSheet {
    type Item = (u32, Vec<(u16, MountRow)>);
    type IntoIter = StructuredSheetIterator<'a, MountSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MountSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MountRow {
    ///""
    pub Singular: String,
    ///""
    pub Plural: String,
    ///""
    pub Adjective: i8,
    ///""
    pub PossessivePronoun: i8,
    ///""
    pub StartsWithVowel: i8,
    ///""
    pub Unknown0: i8,
    ///""
    pub Pronoun: i8,
    ///""
    pub Article: i8,
    ///""
    pub Unknown1: String,
    ///""
    pub Unknown2: String,
    ///""
    pub Unknown3: String,
    ///""
    pub ModelChara: i32,
    ///""
    pub EquipHead: i32,
    ///""
    pub EquipBody: i32,
    ///""
    pub EquipLeg: i32,
    ///""
    pub EquipFoot: i32,
    ///""
    pub MoveControl: u16,
    ///""
    pub RideBGM: u16,
    ///""
    pub Icon: u16,
    ///""
    pub UIPriority: u16,
    ///""
    pub MountAction: u16,
    ///""
    pub Unknown_70_1: u16,
    ///""
    pub Unknown_70_2: u16,
    ///""
    pub Unknown16: u16,
    ///""
    pub Unknown17: u16,
    ///""
    pub Order: i16,
    ///""
    pub FlyingCondition: u8,
    ///""
    pub Unknown5: u8,
    ///""
    pub Unknown6: u8,
    ///""
    pub Unknown7: u8,
    ///""
    pub IsFlying: u8,
    ///""
    pub Unknown8: u8,
    ///""
    pub MountCustomize: u8,
    ///""
    pub ExitMoveDist: u8,
    ///""
    pub ExitMoveSpeed: u8,
    ///""
    pub RadiusRate: u8,
    ///""
    pub BaseMotionSpeed_Run: u8,
    ///""
    pub BaseMotionSpeed_Walk: u8,
    ///""
    pub Unknown9: u8,
    ///""
    pub ExtraSeats: u8,
    ///""
    pub Unknown10: u8,
    ///""
    pub Unknown11: u8,
    ///""
    pub Unknown12: bool,
    ///""
    pub IsEmote: bool,
    ///""
    pub Unknown20: bool,
    ///""
    pub IsAirborne: bool,
    ///""
    pub ExHotbarEnableConfig: bool,
    ///""
    pub UseEP: bool,
    ///""
    pub Unknown13: bool,
    ///""
    pub IsImmobile: bool,
    ///""
    pub Unknown14: bool,
    ///""
    pub HideHeadgear: bool,
    ///""
    pub Unknown18: bool,
    ///""
    pub Unknown19: bool,
}
