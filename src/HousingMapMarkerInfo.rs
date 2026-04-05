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
pub struct HousingMapMarkerInfoSheet {
    sheet: Sheet,
}
impl HousingMapMarkerInfoSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HousingMapMarkerInfo")?;
        let sheet = resolver.read_excel_sheet(&exh, "HousingMapMarkerInfo", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<HousingMapMarkerInfoRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<HousingMapMarkerInfoRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for HousingMapMarkerInfoSheet {
    type Row = HousingMapMarkerInfoRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a HousingMapMarkerInfoSheet {
    type Item = (u32, Vec<(u16, HousingMapMarkerInfoRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, HousingMapMarkerInfoSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HousingMapMarkerInfoSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HousingMapMarkerInfoRow<'a> {
    row: &'a Row,
}
impl<'a> HousingMapMarkerInfoRow<'a> {
    pub fn X(&'a self) -> f32 {
        self.row.columns[0].into_f32().copied().unwrap()
    }
    pub fn Y(&'a self) -> f32 {
        self.row.columns[1].into_f32().copied().unwrap()
    }
    pub fn Z(&'a self) -> f32 {
        self.row.columns[2].into_f32().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> f32 {
        self.row.columns[3].into_f32().copied().unwrap()
    }
    pub fn Map(&'a self) -> u16 {
        self.row.columns[4].into_u16().copied().unwrap()
    }
}
