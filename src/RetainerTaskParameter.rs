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
pub struct RetainerTaskParameterSheet {
    sheet: Sheet,
}
impl RetainerTaskParameterSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("RetainerTaskParameter")?;
        let sheet = resolver.read_excel_sheet(&exh, "RetainerTaskParameter", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<RetainerTaskParameterRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<RetainerTaskParameterRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for RetainerTaskParameterSheet {
    type Row = RetainerTaskParameterRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            ItemLevelDoW: [
                row
                    .columns[0]
                    .into_i16()
                    .copied()
                    .expect("Expected column 0 to be a int16!"),
                row
                    .columns[1]
                    .into_i16()
                    .copied()
                    .expect("Expected column 1 to be a int16!"),
                row
                    .columns[2]
                    .into_i16()
                    .copied()
                    .expect("Expected column 2 to be a int16!"),
                row
                    .columns[3]
                    .into_i16()
                    .copied()
                    .expect("Expected column 3 to be a int16!"),
            ],
            PerceptionDoL: [
                row
                    .columns[4]
                    .into_i16()
                    .copied()
                    .expect("Expected column 4 to be a int16!"),
                row
                    .columns[5]
                    .into_i16()
                    .copied()
                    .expect("Expected column 5 to be a int16!"),
                row
                    .columns[6]
                    .into_i16()
                    .copied()
                    .expect("Expected column 6 to be a int16!"),
                row
                    .columns[7]
                    .into_i16()
                    .copied()
                    .expect("Expected column 7 to be a int16!"),
            ],
            PerceptionFSH: [
                row
                    .columns[8]
                    .into_i16()
                    .copied()
                    .expect("Expected column 8 to be a int16!"),
                row
                    .columns[9]
                    .into_i16()
                    .copied()
                    .expect("Expected column 9 to be a int16!"),
                row
                    .columns[10]
                    .into_i16()
                    .copied()
                    .expect("Expected column 10 to be a int16!"),
                row
                    .columns[11]
                    .into_i16()
                    .copied()
                    .expect("Expected column 11 to be a int16!"),
            ],
        })
    }
}
impl<'a> IntoIterator for &'a RetainerTaskParameterSheet {
    type Item = (u32, Vec<(u16, RetainerTaskParameterRow)>);
    type IntoIter = StructuredSheetIterator<'a, RetainerTaskParameterSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, RetainerTaskParameterSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RetainerTaskParameterRow {
    ///""
    pub ItemLevelDoW: [i16; 4],
    ///""
    pub PerceptionDoL: [i16; 4],
    ///""
    pub PerceptionFSH: [i16; 4],
}
