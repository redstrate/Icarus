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
pub struct MJICraftworksObjectSheet {
    sheet: Sheet,
}
impl MJICraftworksObjectSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MJICraftworksObject")?;
        let sheet = resolver.read_excel_sheet(&exh, "MJICraftworksObject", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MJICraftworksObjectRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MJICraftworksObjectRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MJICraftworksObjectSheet {
    type Row = MJICraftworksObjectRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Item: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            Theme: [
                row
                    .columns[1]
                    .into_u16()
                    .copied()
                    .expect("Expected column 1 to be a uint16!"),
                row
                    .columns[2]
                    .into_u16()
                    .copied()
                    .expect("Expected column 2 to be a uint16!"),
            ],
            Unknown0: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            Material: [
                row
                    .columns[4]
                    .into_u16()
                    .copied()
                    .expect("Expected column 4 to be a uint16!"),
                row
                    .columns[6]
                    .into_u16()
                    .copied()
                    .expect("Expected column 6 to be a uint16!"),
                row
                    .columns[8]
                    .into_u16()
                    .copied()
                    .expect("Expected column 8 to be a uint16!"),
                row
                    .columns[10]
                    .into_u16()
                    .copied()
                    .expect("Expected column 10 to be a uint16!"),
            ],
            Amount: [
                row
                    .columns[5]
                    .into_u16()
                    .copied()
                    .expect("Expected column 5 to be a uint16!"),
                row
                    .columns[7]
                    .into_u16()
                    .copied()
                    .expect("Expected column 7 to be a uint16!"),
                row
                    .columns[9]
                    .into_u16()
                    .copied()
                    .expect("Expected column 9 to be a uint16!"),
                row
                    .columns[11]
                    .into_u16()
                    .copied()
                    .expect("Expected column 11 to be a uint16!"),
            ],
            LevelReq: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            CraftingTime: row
                .columns[13]
                .into_u16()
                .copied()
                .expect("Expected column 13 to be a uint16!"),
            Value: row
                .columns[14]
                .into_u16()
                .copied()
                .expect("Expected column 14 to be a uint16!"),
        })
    }
}
impl<'a> IntoIterator for &'a MJICraftworksObjectSheet {
    type Item = (u32, Vec<(u16, MJICraftworksObjectRow)>);
    type IntoIter = StructuredSheetIterator<'a, MJICraftworksObjectSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MJICraftworksObjectSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MJICraftworksObjectRow {
    ///""
    pub Item: u16,
    ///""
    pub Theme: [u16; 2],
    ///""
    pub Unknown0: u16,
    ///""
    pub Material: [u16; 4],
    ///""
    pub Amount: [u16; 4],
    ///""
    pub LevelReq: u16,
    ///""
    pub CraftingTime: u16,
    ///""
    pub Value: u16,
}
