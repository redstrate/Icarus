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
pub struct MJILandmarkPlaceSheet {
    sheet: Sheet,
}
impl MJILandmarkPlaceSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MJILandmarkPlace")?;
        let sheet = resolver.read_excel_sheet(&exh, "MJILandmarkPlace", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MJILandmarkPlaceRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MJILandmarkPlaceRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MJILandmarkPlaceSheet {
    type Row = MJILandmarkPlaceRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            Name: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            SGB: [
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
            ],
            Unknown1: row
                .columns[5]
                .into_i16()
                .copied()
                .expect("Expected column 5 to be a int16!"),
            Unknown2: row
                .columns[6]
                .into_i16()
                .copied()
                .expect("Expected column 6 to be a int16!"),
            Unknown3: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a MJILandmarkPlaceSheet {
    type Item = (u32, Vec<(u16, MJILandmarkPlaceRow)>);
    type IntoIter = StructuredSheetIterator<'a, MJILandmarkPlaceSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MJILandmarkPlaceSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MJILandmarkPlaceRow {
    ///""
    pub Unknown0: u32,
    ///""
    pub Name: u32,
    ///""
    pub SGB: [u32; 2],
    ///""
    pub Unknown1: i16,
    ///""
    pub Unknown2: i16,
    ///""
    pub Unknown3: u8,
}
