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
pub struct TransformationSheet {
    sheet: Sheet,
}
impl TransformationSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Transformation")?;
        let sheet = resolver.read_excel_sheet(&exh, "Transformation", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<TransformationRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<TransformationRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for TransformationSheet {
    type Row = TransformationRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Speed: row
                .columns[24]
                .into_f32()
                .copied()
                .expect("Expected column 24 to be a float32!"),
            Scale: row
                .columns[25]
                .into_f32()
                .copied()
                .expect("Expected column 25 to be a float32!"),
            Action6: row
                .columns[33]
                .into_u32()
                .copied()
                .expect("Expected column 33 to be a uint32!"),
            BNpcCustomize: row
                .columns[3]
                .into_i32()
                .copied()
                .expect("Expected column 3 to be a int32!"),
            NpcEquip: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            BNpcName: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            Action: [
                row
                    .columns[6]
                    .into_u16()
                    .copied()
                    .expect("Expected column 6 to be a uint16!"),
                row
                    .columns[8]
                    .into_u16()
                    .copied()
                    .expect("Expected column 8 to be a uint16!"),
                row
                    .columns[10]
                    .into_u16()
                    .copied()
                    .expect("Expected column 10 to be a uint16!"),
                row
                    .columns[12]
                    .into_u16()
                    .copied()
                    .expect("Expected column 12 to be a uint16!"),
                row
                    .columns[14]
                    .into_u16()
                    .copied()
                    .expect("Expected column 14 to be a uint16!"),
                row
                    .columns[16]
                    .into_u16()
                    .copied()
                    .expect("Expected column 16 to be a uint16!"),
                row
                    .columns[18]
                    .into_u16()
                    .copied()
                    .expect("Expected column 18 to be a uint16!"),
            ],
            RemoveAction: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            StartVFX: row
                .columns[31]
                .into_u16()
                .copied()
                .expect("Expected column 31 to be a uint16!"),
            EndVFX: row
                .columns[32]
                .into_u16()
                .copied()
                .expect("Expected column 32 to be a uint16!"),
            Action7: row
                .columns[36]
                .into_u16()
                .copied()
                .expect("Expected column 36 to be a uint16!"),
            Model: row
                .columns[1]
                .into_i16()
                .copied()
                .expect("Expected column 1 to be a int16!"),
            Unknown0: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            Unknown1: row
                .columns[22]
                .into_u8()
                .copied()
                .expect("Expected column 22 to be a uint8!"),
            RPParameter: row
                .columns[37]
                .into_u8()
                .copied()
                .expect("Expected column 37 to be a uint8!"),
            Unknown3: row
                .columns[34]
                .into_i8()
                .copied()
                .expect("Expected column 34 to be a int8!"),
            Unknown4: row
                .columns[35]
                .into_i8()
                .copied()
                .expect("Expected column 35 to be a int8!"),
            ExHotbarEnableConfig: [
                row
                    .columns[5]
                    .into_bool()
                    .copied()
                    .expect("Expected column 5 to be a bool!"),
                row
                    .columns[7]
                    .into_bool()
                    .copied()
                    .expect("Expected column 7 to be a bool!"),
                row
                    .columns[9]
                    .into_bool()
                    .copied()
                    .expect("Expected column 9 to be a bool!"),
                row
                    .columns[11]
                    .into_bool()
                    .copied()
                    .expect("Expected column 11 to be a bool!"),
                row
                    .columns[13]
                    .into_bool()
                    .copied()
                    .expect("Expected column 13 to be a bool!"),
                row
                    .columns[15]
                    .into_bool()
                    .copied()
                    .expect("Expected column 15 to be a bool!"),
                row
                    .columns[17]
                    .into_bool()
                    .copied()
                    .expect("Expected column 17 to be a bool!"),
            ],
            Unknown11: row
                .columns[20]
                .into_bool()
                .copied()
                .expect("Expected column 20 to be a bool!"),
            Unknown12: row
                .columns[21]
                .into_bool()
                .copied()
                .expect("Expected column 21 to be a bool!"),
            Unknown13: row
                .columns[23]
                .into_bool()
                .copied()
                .expect("Expected column 23 to be a bool!"),
            IsPvP: row
                .columns[26]
                .into_bool()
                .copied()
                .expect("Expected column 26 to be a bool!"),
            IsEvent: row
                .columns[27]
                .into_bool()
                .copied()
                .expect("Expected column 27 to be a bool!"),
            PlayerCamera: row
                .columns[28]
                .into_bool()
                .copied()
                .expect("Expected column 28 to be a bool!"),
            Unknown14: row
                .columns[29]
                .into_bool()
                .copied()
                .expect("Expected column 29 to be a bool!"),
            Unknown15: row
                .columns[30]
                .into_bool()
                .copied()
                .expect("Expected column 30 to be a bool!"),
            Unknown16: row
                .columns[38]
                .into_bool()
                .copied()
                .expect("Expected column 38 to be a bool!"),
            Unknown17: row
                .columns[39]
                .into_bool()
                .copied()
                .expect("Expected column 39 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a TransformationSheet {
    type Item = (u32, Vec<(u16, TransformationRow)>);
    type IntoIter = StructuredSheetIterator<'a, TransformationSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TransformationSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TransformationRow {
    ///""
    pub Speed: f32,
    ///""
    pub Scale: f32,
    ///""
    pub Action6: u32,
    ///""
    pub BNpcCustomize: i32,
    ///""
    pub NpcEquip: i32,
    ///""
    pub BNpcName: u16,
    ///""
    pub Action: [u16; 7],
    ///""
    pub RemoveAction: u16,
    ///""
    pub StartVFX: u16,
    ///""
    pub EndVFX: u16,
    ///""
    pub Action7: u16,
    ///""
    pub Model: i16,
    ///""
    pub Unknown0: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub RPParameter: u8,
    ///""
    pub Unknown3: i8,
    ///""
    pub Unknown4: i8,
    ///""
    pub ExHotbarEnableConfig: [bool; 7],
    ///""
    pub Unknown11: bool,
    ///""
    pub Unknown12: bool,
    ///""
    pub Unknown13: bool,
    ///""
    pub IsPvP: bool,
    ///""
    pub IsEvent: bool,
    ///""
    pub PlayerCamera: bool,
    ///""
    pub Unknown14: bool,
    ///""
    pub Unknown15: bool,
    ///""
    pub Unknown16: bool,
    ///""
    pub Unknown17: bool,
}
