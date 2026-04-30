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
pub struct BattalionSheet {
    sheet: Sheet,
}
impl BattalionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Battalion")?;
        let sheet = resolver.read_excel_sheet(&exh, "Battalion", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BattalionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BattalionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BattalionSheet {
    type Row = BattalionRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            IsEnemyTo: [
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
                row
                    .columns[11]
                    .into_bool()
                    .copied()
                    .expect("Expected column 11 to be a bool!"),
                row
                    .columns[12]
                    .into_bool()
                    .copied()
                    .expect("Expected column 12 to be a bool!"),
                row
                    .columns[13]
                    .into_bool()
                    .copied()
                    .expect("Expected column 13 to be a bool!"),
                row
                    .columns[14]
                    .into_bool()
                    .copied()
                    .expect("Expected column 14 to be a bool!"),
            ],
        })
    }
}
impl<'a> IntoIterator for &'a BattalionSheet {
    type Item = (u32, Vec<(u16, BattalionRow)>);
    type IntoIter = StructuredSheetIterator<'a, BattalionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BattalionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BattalionRow {
    ///"Index is the other battalion."
    pub IsEnemyTo: [bool; 15],
}
