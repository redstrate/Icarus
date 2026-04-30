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
pub struct BehaviorSheet {
    sheet: Sheet,
}
impl BehaviorSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Behavior")?;
        let sheet = resolver.read_excel_sheet(&exh, "Behavior", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BehaviorRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BehaviorRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BehaviorSheet {
    type Row = BehaviorRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            ContentArgument0: row
                .columns[11]
                .into_u32()
                .copied()
                .expect("Expected column 11 to be a uint32!"),
            Unknown0: row
                .columns[15]
                .into_u32()
                .copied()
                .expect("Expected column 15 to be a uint32!"),
            Unknown1: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            Unknown2: row
                .columns[6]
                .into_i32()
                .copied()
                .expect("Expected column 6 to be a int32!"),
            Unknown3: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Balloon: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Unknown4: row
                .columns[16]
                .into_u16()
                .copied()
                .expect("Expected column 16 to be a uint16!"),
            Unknown5: row
                .columns[5]
                .into_i16()
                .copied()
                .expect("Expected column 5 to be a int16!"),
            Unknown6: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            Unknown7: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Condition0Target: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Condition0Type: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Condition1Target: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            Condition1Type: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            ContentArgument1: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            Unknown8: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            Unknown9: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a BehaviorSheet {
    type Item = (u32, Vec<(u16, BehaviorRow)>);
    type IntoIter = StructuredSheetIterator<'a, BehaviorSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BehaviorSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BehaviorRow {
    ///""
    pub ContentArgument0: u32,
    ///""
    pub Unknown0: u32,
    ///""
    pub Unknown1: i32,
    ///""
    pub Unknown2: i32,
    ///""
    pub Unknown3: u16,
    ///""
    pub Balloon: u16,
    ///""
    pub Unknown4: u16,
    ///""
    pub Unknown5: i16,
    ///""
    pub Unknown6: u8,
    ///""
    pub Unknown7: u8,
    ///""
    pub Condition0Target: u8,
    ///""
    pub Condition0Type: u8,
    ///""
    pub Condition1Target: u8,
    ///""
    pub Condition1Type: u8,
    ///""
    pub ContentArgument1: u8,
    ///""
    pub Unknown8: u8,
    ///""
    pub Unknown9: u8,
}
