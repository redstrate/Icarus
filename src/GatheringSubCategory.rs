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
pub struct GatheringSubCategorySheet {
    sheet: Sheet,
}
impl GatheringSubCategorySheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 2000u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GatheringSubCategory")?;
        let sheet = resolver.read_excel_sheet(&exh, "GatheringSubCategory", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GatheringSubCategoryRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<GatheringSubCategoryRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GatheringSubCategorySheet {
    type Row = GatheringSubCategoryRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            FolkloreBook: row
                .columns[5]
                .into_string()
                .cloned()
                .expect("Expected column 5 to be a string!"),
            Quest: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            Item: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            Division: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            GatheringType: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            ClassJob: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown0: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a GatheringSubCategorySheet {
    type Item = (u32, Vec<(u16, GatheringSubCategoryRow)>);
    type IntoIter = StructuredSheetIterator<'a, GatheringSubCategorySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GatheringSubCategorySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GatheringSubCategoryRow {
    ///""
    pub FolkloreBook: String,
    ///""
    pub Quest: u32,
    ///""
    pub Item: i32,
    ///""
    pub Division: u16,
    ///""
    pub GatheringType: u8,
    ///""
    pub ClassJob: u8,
    ///""
    pub Unknown0: u8,
}
