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
pub struct CreditSheet {
    sheet: Sheet,
}
impl CreditSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Credit")?;
        let sheet = resolver.read_excel_sheet(&exh, "Credit", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CreditRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CreditRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CreditSheet {
    type Row = CreditRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Roles1: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            JapaneseCast1: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            EnglishCast1: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            FrenchCast1: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            GermanCast1: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            Roles2: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            JapaneseCast2: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            EnglishCast2: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            FrenchCast2: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            GermanCast2: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            Unknown0: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a CreditSheet {
    type Item = (u32, Vec<(u16, CreditRow)>);
    type IntoIter = StructuredSheetIterator<'a, CreditSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CreditSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreditRow {
    ///""
    pub Roles1: u16,
    ///""
    pub JapaneseCast1: u16,
    ///""
    pub EnglishCast1: u16,
    ///""
    pub FrenchCast1: u16,
    ///""
    pub GermanCast1: u16,
    ///""
    pub Roles2: u16,
    ///""
    pub JapaneseCast2: u16,
    ///""
    pub EnglishCast2: u16,
    ///""
    pub FrenchCast2: u16,
    ///""
    pub GermanCast2: u16,
    ///""
    pub Unknown0: u8,
}
