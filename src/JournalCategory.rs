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
pub struct JournalCategorySheet {
    sheet: Sheet,
}
impl JournalCategorySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("JournalCategory")?;
        let sheet = resolver.read_excel_sheet(&exh, "JournalCategory", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<JournalCategoryRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<JournalCategoryRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for JournalCategorySheet {
    type Row = JournalCategoryRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a JournalCategorySheet {
    type Item = (u32, Vec<(u16, JournalCategoryRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, JournalCategorySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, JournalCategorySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct JournalCategoryRow<'a> {
    row: &'a Row,
}
impl<'a> JournalCategoryRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn SeparateType(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn DataType(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn JournalSection(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn MapCondition(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
}
