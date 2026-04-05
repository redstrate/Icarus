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
impl<'a> StructuredSheet<'a> for MJILandmarkPlaceSheet {
    type Row = MJILandmarkPlaceRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a MJILandmarkPlaceSheet {
    type Item = (u32, Vec<(u16, MJILandmarkPlaceRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MJILandmarkPlaceSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MJILandmarkPlaceSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MJILandmarkPlaceRow<'a> {
    row: &'a Row,
}
impl<'a> MJILandmarkPlaceRow<'a> {
    pub fn Unknown0(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
    pub fn Name(&'a self) -> u32 {
        self.row.columns[1].into_u32().copied().unwrap()
    }
    pub fn SGB(&'a self) -> [u32; 2] {
        [
            self.row.columns[2].into_u32().copied().unwrap(),
            self.row.columns[3].into_u32().copied().unwrap(),
        ]
    }
    pub fn Unknown1(&'a self) -> i16 {
        self.row.columns[5].into_i16().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> i16 {
        self.row.columns[6].into_i16().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
}
