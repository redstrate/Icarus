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
pub struct WeeklyBingoMultipleOrderSheet {
    sheet: Sheet,
}
impl WeeklyBingoMultipleOrderSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WeeklyBingoMultipleOrder")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "WeeklyBingoMultipleOrder", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WeeklyBingoMultipleOrderRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<WeeklyBingoMultipleOrderRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WeeklyBingoMultipleOrderSheet {
    type Row = WeeklyBingoMultipleOrderRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Content: [
                row
                    .columns[0]
                    .into_u32()
                    .copied()
                    .expect("Expected column 0 to be a uint32!"),
                row
                    .columns[1]
                    .into_u32()
                    .copied()
                    .expect("Expected column 1 to be a uint32!"),
                row
                    .columns[2]
                    .into_u32()
                    .copied()
                    .expect("Expected column 2 to be a uint32!"),
                row
                    .columns[3]
                    .into_u32()
                    .copied()
                    .expect("Expected column 3 to be a uint32!"),
                row
                    .columns[4]
                    .into_u32()
                    .copied()
                    .expect("Expected column 4 to be a uint32!"),
            ],
            Unknown5: row
                .columns[5]
                .into_u32()
                .copied()
                .expect("Expected column 5 to be a uint32!"),
            Unknown6: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            Unknown7: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
            Unknown8: row
                .columns[8]
                .into_u32()
                .copied()
                .expect("Expected column 8 to be a uint32!"),
            Unknown9: row
                .columns[9]
                .into_u32()
                .copied()
                .expect("Expected column 9 to be a uint32!"),
            Unknown10: row
                .columns[10]
                .into_u32()
                .copied()
                .expect("Expected column 10 to be a uint32!"),
            Unknown11: row
                .columns[11]
                .into_u32()
                .copied()
                .expect("Expected column 11 to be a uint32!"),
            Unknown12: row
                .columns[12]
                .into_u32()
                .copied()
                .expect("Expected column 12 to be a uint32!"),
            Unknown13: row
                .columns[13]
                .into_u32()
                .copied()
                .expect("Expected column 13 to be a uint32!"),
            Unknown14: row
                .columns[14]
                .into_u32()
                .copied()
                .expect("Expected column 14 to be a uint32!"),
            Unknown15: row
                .columns[15]
                .into_u32()
                .copied()
                .expect("Expected column 15 to be a uint32!"),
            Unknown16: row
                .columns[16]
                .into_u32()
                .copied()
                .expect("Expected column 16 to be a uint32!"),
            Unknown17: row
                .columns[17]
                .into_u32()
                .copied()
                .expect("Expected column 17 to be a uint32!"),
            Unknown18: row
                .columns[18]
                .into_u32()
                .copied()
                .expect("Expected column 18 to be a uint32!"),
            Unknown19: row
                .columns[19]
                .into_u32()
                .copied()
                .expect("Expected column 19 to be a uint32!"),
        })
    }
}
impl<'a> IntoIterator for &'a WeeklyBingoMultipleOrderSheet {
    type Item = (u32, Vec<(u16, WeeklyBingoMultipleOrderRow)>);
    type IntoIter = StructuredSheetIterator<'a, WeeklyBingoMultipleOrderSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WeeklyBingoMultipleOrderSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WeeklyBingoMultipleOrderRow {
    ///"I *think* this is correct? You're welcome MidoriKami :3"
    pub Content: [u32; 5],
    ///""
    pub Unknown5: u32,
    ///""
    pub Unknown6: u32,
    ///""
    pub Unknown7: u32,
    ///""
    pub Unknown8: u32,
    ///""
    pub Unknown9: u32,
    ///""
    pub Unknown10: u32,
    ///""
    pub Unknown11: u32,
    ///""
    pub Unknown12: u32,
    ///""
    pub Unknown13: u32,
    ///""
    pub Unknown14: u32,
    ///""
    pub Unknown15: u32,
    ///""
    pub Unknown16: u32,
    ///""
    pub Unknown17: u32,
    ///""
    pub Unknown18: u32,
    ///""
    pub Unknown19: u32,
}
