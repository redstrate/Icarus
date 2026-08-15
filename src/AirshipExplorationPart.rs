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
pub struct AirshipExplorationPartSheet {
    sheet: Sheet,
}
impl AirshipExplorationPartSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AirshipExplorationPart")?;
        let sheet = resolver.read_excel_sheet(&exh, "AirshipExplorationPart", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AirshipExplorationPartRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<AirshipExplorationPartRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for AirshipExplorationPartSheet {
    type Row = AirshipExplorationPartRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Class: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Surveillance: row
                .columns[3]
                .into_i16()
                .copied()
                .expect("Expected column 3 to be a int16!"),
            Retrieval: row
                .columns[4]
                .into_i16()
                .copied()
                .expect("Expected column 4 to be a int16!"),
            Speed: row
                .columns[5]
                .into_i16()
                .copied()
                .expect("Expected column 5 to be a int16!"),
            Range: row
                .columns[6]
                .into_i16()
                .copied()
                .expect("Expected column 6 to be a int16!"),
            Favor: row
                .columns[7]
                .into_i16()
                .copied()
                .expect("Expected column 7 to be a int16!"),
            Slot: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            Rank: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Components: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            RepairMaterials: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a AirshipExplorationPartSheet {
    type Item = (u32, Vec<(u16, AirshipExplorationPartRow)>);
    type IntoIter = StructuredSheetIterator<'a, AirshipExplorationPartSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AirshipExplorationPartSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AirshipExplorationPartRow {
    ///""
    pub Class: u16,
    ///""
    pub Surveillance: i16,
    ///""
    pub Retrieval: i16,
    ///""
    pub Speed: i16,
    ///""
    pub Range: i16,
    ///""
    pub Favor: i16,
    ///""
    pub Slot: u8,
    ///""
    pub Rank: u8,
    ///""
    pub Components: u8,
    ///""
    pub RepairMaterials: u8,
}
