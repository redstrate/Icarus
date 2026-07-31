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
pub struct OmikujiSheet {
    sheet: Sheet,
}
impl OmikujiSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Omikuji")?;
        let sheet = resolver.read_excel_sheet(&exh, "Omikuji", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<OmikujiRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<OmikujiRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for OmikujiSheet {
    type Row = OmikujiRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Fortune: row
                .columns[5]
                .into_string()
                .cloned()
                .expect("Expected column 5 to be a string!"),
            Description: row
                .columns[6]
                .into_string()
                .cloned()
                .expect("Expected column 6 to be a string!"),
            GuidanceAdventure: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            GuidanceRelationships: row
                .columns[1]
                .into_i32()
                .copied()
                .expect("Expected column 1 to be a int32!"),
            GuidanceProsperity: row
                .columns[2]
                .into_i32()
                .copied()
                .expect("Expected column 2 to be a int32!"),
            GuidanceConflict: row
                .columns[3]
                .into_i32()
                .copied()
                .expect("Expected column 3 to be a int32!"),
            GuidanceFavor: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
        })
    }
}
impl<'a> IntoIterator for &'a OmikujiSheet {
    type Item = (u32, Vec<(u16, OmikujiRow)>);
    type IntoIter = StructuredSheetIterator<'a, OmikujiSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, OmikujiSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OmikujiRow {
    ///""
    pub Fortune: String,
    ///""
    pub Description: String,
    ///""
    pub GuidanceAdventure: i32,
    ///""
    pub GuidanceRelationships: i32,
    ///""
    pub GuidanceProsperity: i32,
    ///""
    pub GuidanceConflict: i32,
    ///""
    pub GuidanceFavor: i32,
}
