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
pub struct HudTransientSheet {
    sheet: Sheet,
}
impl HudTransientSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HudTransient")?;
        let sheet = resolver.read_excel_sheet(&exh, "HudTransient", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<HudTransientRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<HudTransientRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for HudTransientSheet {
    type Row = HudTransientRow;
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
        })
    }
}
impl<'a> IntoIterator for &'a HudTransientSheet {
    type Item = (u32, Vec<(u16, HudTransientRow)>);
    type IntoIter = StructuredSheetIterator<'a, HudTransientSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HudTransientSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct HudTransientRow {
    ///""
    pub Unknown0: i8,
    ///""
    pub Unknown1: i8,
    ///""
    pub Unknown2: i8,
    ///""
    pub Unknown3: i8,
}
