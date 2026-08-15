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
pub struct ContentsNoteRewardEurekaEXPSheet {
    sheet: Sheet,
}
impl ContentsNoteRewardEurekaEXPSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ContentsNoteRewardEurekaEXP")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "ContentsNoteRewardEurekaEXP", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ContentsNoteRewardEurekaEXPRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<ContentsNoteRewardEurekaEXPRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ContentsNoteRewardEurekaEXPSheet {
    type Row = ContentsNoteRewardEurekaEXPRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
        })
    }
}
impl<'a> IntoIterator for &'a ContentsNoteRewardEurekaEXPSheet {
    type Item = (u32, Vec<(u16, ContentsNoteRewardEurekaEXPRow)>);
    type IntoIter = StructuredSheetIterator<'a, ContentsNoteRewardEurekaEXPSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ContentsNoteRewardEurekaEXPSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ContentsNoteRewardEurekaEXPRow {
    ///""
    pub Unknown0: u32,
}
