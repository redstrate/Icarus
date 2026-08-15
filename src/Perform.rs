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
pub struct PerformSheet {
    sheet: Sheet,
}
impl PerformSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Perform")?;
        let sheet = resolver.read_excel_sheet(&exh, "Perform", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PerformRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PerformRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for PerformSheet {
    type Row = PerformRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            AnimationPlay01: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            AnimationPlay02: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Instrument: row
                .columns[9]
                .into_string()
                .cloned()
                .expect("Expected column 9 to be a string!"),
            ModelKey: row
                .columns[2]
                .into_u64()
                .copied()
                .expect("Expected column 2 to be a uint64!"),
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            UnlockLink: row
                .columns[8]
                .into_i32()
                .copied()
                .expect("Expected column 8 to be a int32!"),
            Icon: row
                .columns[10]
                .into_i32()
                .copied()
                .expect("Expected column 10 to be a int32!"),
            AnimationStart: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            AnimationEnd: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            AnimationIdle: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            Transient: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            PerformGroup: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            Unknown1: row
                .columns[1]
                .into_bool()
                .copied()
                .expect("Expected column 1 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a PerformSheet {
    type Item = (u32, Vec<(u16, PerformRow)>);
    type IntoIter = StructuredSheetIterator<'a, PerformSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PerformSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PerformRow {
    ///""
    pub AnimationPlay01: u16,
    ///""
    pub AnimationPlay02: u16,
    ///""
    pub Instrument: String,
    ///""
    pub ModelKey: u64,
    ///""
    pub Name: String,
    ///""
    pub UnlockLink: i32,
    ///""
    pub Icon: i32,
    ///""
    pub AnimationStart: u16,
    ///""
    pub AnimationEnd: u16,
    ///""
    pub AnimationIdle: u16,
    ///""
    pub Transient: u8,
    ///""
    pub PerformGroup: u8,
    ///""
    pub Unknown1: bool,
}
