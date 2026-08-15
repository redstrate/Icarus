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
pub struct BNpcResistSheet {
    sheet: Sheet,
}
impl BNpcResistSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BNpcResist")?;
        let sheet = resolver.read_excel_sheet(&exh, "BNpcResist", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BNpcResistRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BNpcResistRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BNpcResistSheet {
    type Row = BNpcResistRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: [
                row
                    .columns[0]
                    .into_bool()
                    .copied()
                    .expect("Expected column 0 to be a bool!"),
                row
                    .columns[1]
                    .into_bool()
                    .copied()
                    .expect("Expected column 1 to be a bool!"),
                row
                    .columns[2]
                    .into_bool()
                    .copied()
                    .expect("Expected column 2 to be a bool!"),
                row
                    .columns[3]
                    .into_bool()
                    .copied()
                    .expect("Expected column 3 to be a bool!"),
                row
                    .columns[4]
                    .into_bool()
                    .copied()
                    .expect("Expected column 4 to be a bool!"),
                row
                    .columns[5]
                    .into_bool()
                    .copied()
                    .expect("Expected column 5 to be a bool!"),
                row
                    .columns[6]
                    .into_bool()
                    .copied()
                    .expect("Expected column 6 to be a bool!"),
                row
                    .columns[7]
                    .into_bool()
                    .copied()
                    .expect("Expected column 7 to be a bool!"),
                row
                    .columns[8]
                    .into_bool()
                    .copied()
                    .expect("Expected column 8 to be a bool!"),
                row
                    .columns[9]
                    .into_bool()
                    .copied()
                    .expect("Expected column 9 to be a bool!"),
                row
                    .columns[10]
                    .into_bool()
                    .copied()
                    .expect("Expected column 10 to be a bool!"),
            ],
        })
    }
}
impl<'a> IntoIterator for &'a BNpcResistSheet {
    type Item = (u32, Vec<(u16, BNpcResistRow)>);
    type IntoIter = StructuredSheetIterator<'a, BNpcResistSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BNpcResistSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BNpcResistRow {
    ///""
    pub Unknown0: [bool; 11],
}
