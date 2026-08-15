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
pub struct WorldSheet {
    sheet: Sheet,
}
impl WorldSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("World")?;
        let sheet = resolver.read_excel_sheet(&exh, "World", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WorldRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WorldRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WorldSheet {
    type Row = WorldRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            InternalName: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Name: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            DataCenter: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown0: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            UserType: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Unknown1: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            Region: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            Unknown2: row
                .columns[5]
                .into_bool()
                .copied()
                .expect("Expected column 5 to be a bool!"),
            IsPublic: row
                .columns[8]
                .into_bool()
                .copied()
                .expect("Expected column 8 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a WorldSheet {
    type Item = (u32, Vec<(u16, WorldRow)>);
    type IntoIter = StructuredSheetIterator<'a, WorldSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WorldSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WorldRow {
    ///""
    pub InternalName: String,
    ///""
    pub Name: String,
    ///""
    pub DataCenter: u8,
    ///""
    pub Unknown0: u8,
    ///""
    pub UserType: u8,
    ///""
    pub Unknown1: u8,
    ///""
    pub Region: u8,
    ///""
    pub Unknown2: bool,
    ///""
    pub IsPublic: bool,
}
