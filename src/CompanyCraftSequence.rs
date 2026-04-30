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
pub struct CompanyCraftSequenceSheet {
    sheet: Sheet,
}
impl CompanyCraftSequenceSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CompanyCraftSequence")?;
        let sheet = resolver.read_excel_sheet(&exh, "CompanyCraftSequence", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CompanyCraftSequenceRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<CompanyCraftSequenceRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CompanyCraftSequenceSheet {
    type Row = CompanyCraftSequenceRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Order: row
                .columns[13]
                .into_u32()
                .copied()
                .expect("Expected column 13 to be a uint32!"),
            ResultItem: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            Category: row
                .columns[1]
                .into_i32()
                .copied()
                .expect("Expected column 1 to be a int32!"),
            CompanyCraftDraftCategory: row
                .columns[2]
                .into_i32()
                .copied()
                .expect("Expected column 2 to be a int32!"),
            CompanyCraftType: row
                .columns[3]
                .into_i32()
                .copied()
                .expect("Expected column 3 to be a int32!"),
            CompanyCraftDraft: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            CompanyCraftPart: [
                row
                    .columns[5]
                    .into_u16()
                    .copied()
                    .expect("Expected column 5 to be a uint16!"),
                row
                    .columns[6]
                    .into_u16()
                    .copied()
                    .expect("Expected column 6 to be a uint16!"),
                row
                    .columns[7]
                    .into_u16()
                    .copied()
                    .expect("Expected column 7 to be a uint16!"),
                row
                    .columns[8]
                    .into_u16()
                    .copied()
                    .expect("Expected column 8 to be a uint16!"),
                row
                    .columns[9]
                    .into_u16()
                    .copied()
                    .expect("Expected column 9 to be a uint16!"),
                row
                    .columns[10]
                    .into_u16()
                    .copied()
                    .expect("Expected column 10 to be a uint16!"),
                row
                    .columns[11]
                    .into_u16()
                    .copied()
                    .expect("Expected column 11 to be a uint16!"),
                row
                    .columns[12]
                    .into_u16()
                    .copied()
                    .expect("Expected column 12 to be a uint16!"),
            ],
        })
    }
}
impl<'a> IntoIterator for &'a CompanyCraftSequenceSheet {
    type Item = (u32, Vec<(u16, CompanyCraftSequenceRow)>);
    type IntoIter = StructuredSheetIterator<'a, CompanyCraftSequenceSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CompanyCraftSequenceSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CompanyCraftSequenceRow {
    ///""
    pub Order: u32,
    ///""
    pub ResultItem: i32,
    ///""
    pub Category: i32,
    ///""
    pub CompanyCraftDraftCategory: i32,
    ///""
    pub CompanyCraftType: i32,
    ///""
    pub CompanyCraftDraft: i32,
    ///""
    pub CompanyCraftPart: [u16; 8],
}
