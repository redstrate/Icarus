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
pub struct AOZArrangementSheet {
    sheet: Sheet,
}
impl AOZArrangementSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AOZArrangement")?;
        let sheet = resolver.read_excel_sheet(&exh, "AOZArrangement", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AOZArrangementRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<AOZArrangementRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for AOZArrangementSheet {
    type Row = AOZArrangementRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            AOZContentBriefingBNpc: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            Position: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
        })
    }
}
impl<'a> IntoIterator for &'a AOZArrangementSheet {
    type Item = (u32, Vec<(u16, AOZArrangementRow)>);
    type IntoIter = StructuredSheetIterator<'a, AOZArrangementSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AOZArrangementSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AOZArrangementRow {
    ///""
    pub AOZContentBriefingBNpc: u16,
    ///""
    pub Position: u16,
}
