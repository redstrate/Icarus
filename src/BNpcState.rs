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
pub struct BNpcStateSheet {
    sheet: Sheet,
}
impl BNpcStateSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BNpcState")?;
        let sheet = resolver.read_excel_sheet(&exh, "BNpcState", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BNpcStateRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BNpcStateRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BNpcStateSheet {
    type Row = BNpcStateRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Scale: row
                .columns[11]
                .into_f32()
                .copied()
                .expect("Expected column 11 to be a float32!"),
            LoopTimeline: row
                .columns[13]
                .into_i32()
                .copied()
                .expect("Expected column 13 to be a int32!"),
            Idle: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            Slot: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            ModelState: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Attribute0: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Attribute1: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            Attribute2: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            ModelScale: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            AnimationStates: [
                row
                    .columns[1]
                    .into_i8()
                    .copied()
                    .expect("Expected column 1 to be a int8!"),
                row
                    .columns[2]
                    .into_i8()
                    .copied()
                    .expect("Expected column 2 to be a int8!"),
            ],
            AttributeFlag0: row
                .columns[6]
                .into_bool()
                .copied()
                .expect("Expected column 6 to be a bool!"),
            AttributeFlag1: row
                .columns[8]
                .into_bool()
                .copied()
                .expect("Expected column 8 to be a bool!"),
            AttributeFlag2: row
                .columns[10]
                .into_bool()
                .copied()
                .expect("Expected column 10 to be a bool!"),
            Unknown3: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a BNpcStateSheet {
    type Item = (u32, Vec<(u16, BNpcStateRow)>);
    type IntoIter = StructuredSheetIterator<'a, BNpcStateSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BNpcStateSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BNpcStateRow {
    ///""
    pub Scale: f32,
    ///""
    pub LoopTimeline: i32,
    ///""
    pub Idle: u16,
    ///""
    pub Slot: u8,
    ///""
    pub ModelState: u8,
    ///""
    pub Attribute0: u8,
    ///""
    pub Attribute1: u8,
    ///""
    pub Attribute2: u8,
    ///""
    pub ModelScale: u8,
    ///""
    pub AnimationStates: [i8; 2],
    ///""
    pub AttributeFlag0: bool,
    ///""
    pub AttributeFlag1: bool,
    ///""
    pub AttributeFlag2: bool,
    ///""
    pub Unknown3: bool,
}
