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
pub struct BenchmarkOverrideEquipmentSheet {
    sheet: Sheet,
}
impl BenchmarkOverrideEquipmentSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BenchmarkOverrideEquipment")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "BenchmarkOverrideEquipment", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BenchmarkOverrideEquipmentRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<BenchmarkOverrideEquipmentRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BenchmarkOverrideEquipmentSheet {
    type Row = BenchmarkOverrideEquipmentRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            ModelMainHand: row
                .columns[4]
                .into_u64()
                .copied()
                .expect("Expected column 4 to be a uint64!"),
            ModelOffHand: row
                .columns[6]
                .into_u64()
                .copied()
                .expect("Expected column 6 to be a uint64!"),
            Unknown0: row
                .columns[8]
                .into_u64()
                .copied()
                .expect("Expected column 8 to be a uint64!"),
            Unknown1: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            Unknown2: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            ModelHead: row
                .columns[10]
                .into_u32()
                .copied()
                .expect("Expected column 10 to be a uint32!"),
            ModelBody: row
                .columns[12]
                .into_u32()
                .copied()
                .expect("Expected column 12 to be a uint32!"),
            ModelHands: row
                .columns[14]
                .into_u32()
                .copied()
                .expect("Expected column 14 to be a uint32!"),
            ModelLegs: row
                .columns[16]
                .into_u32()
                .copied()
                .expect("Expected column 16 to be a uint32!"),
            ModelFeet: row
                .columns[18]
                .into_u32()
                .copied()
                .expect("Expected column 18 to be a uint32!"),
            ModelEars: row
                .columns[20]
                .into_u32()
                .copied()
                .expect("Expected column 20 to be a uint32!"),
            ModelNeck: row
                .columns[22]
                .into_u32()
                .copied()
                .expect("Expected column 22 to be a uint32!"),
            ModelWrists: row
                .columns[24]
                .into_u32()
                .copied()
                .expect("Expected column 24 to be a uint32!"),
            ModelLeftRing: row
                .columns[26]
                .into_u32()
                .copied()
                .expect("Expected column 26 to be a uint32!"),
            ModelRightRing: row
                .columns[28]
                .into_u32()
                .copied()
                .expect("Expected column 28 to be a uint32!"),
            Unknown3: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            DyeMainHand: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            DyeOffHand: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            Unknown4: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            DyeHead: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            DyeBody: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            DyeHands: row
                .columns[15]
                .into_u8()
                .copied()
                .expect("Expected column 15 to be a uint8!"),
            DyeLegs: row
                .columns[17]
                .into_u8()
                .copied()
                .expect("Expected column 17 to be a uint8!"),
            DyeFeet: row
                .columns[19]
                .into_u8()
                .copied()
                .expect("Expected column 19 to be a uint8!"),
            DyeEars: row
                .columns[21]
                .into_u8()
                .copied()
                .expect("Expected column 21 to be a uint8!"),
            DyeNeck: row
                .columns[23]
                .into_u8()
                .copied()
                .expect("Expected column 23 to be a uint8!"),
            DyeWrists: row
                .columns[25]
                .into_u8()
                .copied()
                .expect("Expected column 25 to be a uint8!"),
            DyeLeftRing: row
                .columns[27]
                .into_u8()
                .copied()
                .expect("Expected column 27 to be a uint8!"),
            DyeRightRing: row
                .columns[29]
                .into_u8()
                .copied()
                .expect("Expected column 29 to be a uint8!"),
            Unknown5: row
                .columns[3]
                .into_i8()
                .copied()
                .expect("Expected column 3 to be a int8!"),
        })
    }
}
impl<'a> IntoIterator for &'a BenchmarkOverrideEquipmentSheet {
    type Item = (u32, Vec<(u16, BenchmarkOverrideEquipmentRow)>);
    type IntoIter = StructuredSheetIterator<'a, BenchmarkOverrideEquipmentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BenchmarkOverrideEquipmentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BenchmarkOverrideEquipmentRow {
    ///""
    pub ModelMainHand: u64,
    ///""
    pub ModelOffHand: u64,
    ///""
    pub Unknown0: u64,
    ///""
    pub Unknown1: u32,
    ///""
    pub Unknown2: u32,
    ///""
    pub ModelHead: u32,
    ///""
    pub ModelBody: u32,
    ///""
    pub ModelHands: u32,
    ///""
    pub ModelLegs: u32,
    ///""
    pub ModelFeet: u32,
    ///""
    pub ModelEars: u32,
    ///""
    pub ModelNeck: u32,
    ///""
    pub ModelWrists: u32,
    ///""
    pub ModelLeftRing: u32,
    ///""
    pub ModelRightRing: u32,
    ///""
    pub Unknown3: u8,
    ///""
    pub DyeMainHand: u8,
    ///""
    pub DyeOffHand: u8,
    ///""
    pub Unknown4: u8,
    ///""
    pub DyeHead: u8,
    ///""
    pub DyeBody: u8,
    ///""
    pub DyeHands: u8,
    ///""
    pub DyeLegs: u8,
    ///""
    pub DyeFeet: u8,
    ///""
    pub DyeEars: u8,
    ///""
    pub DyeNeck: u8,
    ///""
    pub DyeWrists: u8,
    ///""
    pub DyeLeftRing: u8,
    ///""
    pub DyeRightRing: u8,
    ///""
    pub Unknown5: i8,
}
