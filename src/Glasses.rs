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
pub struct GlassesSheet {
    sheet: Sheet,
}
impl GlassesSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Glasses")?;
        let sheet = resolver.read_excel_sheet(&exh, "Glasses", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GlassesRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GlassesRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for GlassesSheet {
    type Row = GlassesRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a GlassesSheet {
    type Item = (u32, Vec<(u16, GlassesRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, GlassesSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GlassesSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GlassesRow<'a> {
    row: &'a Row,
}
impl<'a> GlassesRow<'a> {
    pub fn Singular(&'a self) -> &'a str {
        self.row.columns[4].into_string().unwrap()
    }
    pub fn Plural(&'a self) -> &'a str {
        self.row.columns[6].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[12].into_string().unwrap()
    }
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[13].into_string().unwrap()
    }
    pub fn Unknown_70_1(&'a self) -> i8 {
        self.row.columns[5].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_2(&'a self) -> i8 {
        self.row.columns[7].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_3(&'a self) -> i8 {
        self.row.columns[8].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_4(&'a self) -> i8 {
        self.row.columns[9].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_5(&'a self) -> i8 {
        self.row.columns[10].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_6(&'a self) -> i8 {
        self.row.columns[11].into_i8().copied().unwrap()
    }
    pub fn Unknown_70_7(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
    pub fn Icon(&'a self) -> i32 {
        self.row.columns[2].into_i32().copied().unwrap()
    }
    pub fn Unknown_70_8(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn Style(&'a self) -> i16 {
        self.row.columns[1].into_i16().copied().unwrap()
    }
}
