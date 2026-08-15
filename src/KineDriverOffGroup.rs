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
pub struct KineDriverOffGroupSheet {
    sheet: Sheet,
}
impl KineDriverOffGroupSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("KineDriverOffGroup")?;
        let sheet = resolver.read_excel_sheet(&exh, "KineDriverOffGroup", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<KineDriverOffGroupRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<KineDriverOffGroupRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for KineDriverOffGroupSheet {
    type Row = KineDriverOffGroupRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_i8()
                .copied()
                .expect("Expected column 0 to be a int8!"),
            Unknown1: row
                .columns[1]
                .into_i8()
                .copied()
                .expect("Expected column 1 to be a int8!"),
            Unknown2: row
                .columns[2]
                .into_i8()
                .copied()
                .expect("Expected column 2 to be a int8!"),
            Unknown3: row
                .columns[3]
                .into_i8()
                .copied()
                .expect("Expected column 3 to be a int8!"),
            Unknown4: row
                .columns[4]
                .into_i8()
                .copied()
                .expect("Expected column 4 to be a int8!"),
        })
    }
}
impl<'a> IntoIterator for &'a KineDriverOffGroupSheet {
    type Item = (u32, Vec<(u16, KineDriverOffGroupRow)>);
    type IntoIter = StructuredSheetIterator<'a, KineDriverOffGroupSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, KineDriverOffGroupSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct KineDriverOffGroupRow {
    ///""
    pub Unknown0: i8,
    ///""
    pub Unknown1: i8,
    ///""
    pub Unknown2: i8,
    ///""
    pub Unknown3: i8,
    ///""
    pub Unknown4: i8,
}
