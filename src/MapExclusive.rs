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
pub struct MapExclusiveSheet {
    sheet: Sheet,
}
impl MapExclusiveSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MapExclusive")?;
        let sheet = resolver.read_excel_sheet(&exh, "MapExclusive", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MapExclusiveRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MapExclusiveRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MapExclusiveSheet {
    type Row = MapExclusiveRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            Unknown1: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a MapExclusiveSheet {
    type Item = (u32, Vec<(u16, MapExclusiveRow)>);
    type IntoIter = StructuredSheetIterator<'a, MapExclusiveSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MapExclusiveSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MapExclusiveRow {
    ///""
    pub Unknown0: u16,
    ///""
    pub Unknown1: u8,
}
