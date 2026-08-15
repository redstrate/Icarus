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
pub struct MJIGatheringItemSheet {
    sheet: Sheet,
}
impl MJIGatheringItemSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MJIGatheringItem")?;
        let sheet = resolver.read_excel_sheet(&exh, "MJIGatheringItem", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MJIGatheringItemRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MJIGatheringItemRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MJIGatheringItemSheet {
    type Row = MJIGatheringItemRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Radius: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            X: row
                .columns[3]
                .into_i16()
                .copied()
                .expect("Expected column 3 to be a int16!"),
            Y: row
                .columns[4]
                .into_i16()
                .copied()
                .expect("Expected column 4 to be a int16!"),
            Unknown0: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown1: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            Item: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            Sort: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a MJIGatheringItemSheet {
    type Item = (u32, Vec<(u16, MJIGatheringItemRow)>);
    type IntoIter = StructuredSheetIterator<'a, MJIGatheringItemSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MJIGatheringItemSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MJIGatheringItemRow {
    ///""
    pub Radius: u16,
    ///""
    pub X: i16,
    ///""
    pub Y: i16,
    ///""
    pub Unknown0: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub Item: u32,
    ///""
    pub Sort: u8,
}
