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
pub struct MYCWarResultNotebookSheet {
    sheet: Sheet,
}
impl MYCWarResultNotebookSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MYCWarResultNotebook")?;
        let sheet = resolver.read_excel_sheet(&exh, "MYCWarResultNotebook", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MYCWarResultNotebookRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<MYCWarResultNotebookRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for MYCWarResultNotebookSheet {
    type Row = MYCWarResultNotebookRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a MYCWarResultNotebookSheet {
    type Item = (u32, Vec<(u16, MYCWarResultNotebookRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MYCWarResultNotebookSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MYCWarResultNotebookSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MYCWarResultNotebookRow<'a> {
    row: &'a Row,
}
impl<'a> MYCWarResultNotebookRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[9].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[10].into_string().unwrap()
    }
    pub fn NameJP(&'a self) -> &'a str {
        self.row.columns[8].into_string().unwrap()
    }
    pub fn Quest(&'a self) -> i32 {
        self.row.columns[3].into_i32().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> i32 {
        self.row.columns[4].into_i32().copied().unwrap()
    }
    pub fn Icon(&'a self) -> i32 {
        self.row.columns[5].into_i32().copied().unwrap()
    }
    pub fn Image(&'a self) -> i32 {
        self.row.columns[6].into_i32().copied().unwrap()
    }
    pub fn Number(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn Link(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn Rarity(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
}
