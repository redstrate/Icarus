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
pub struct DeepDungeonDemicloneSheet {
    sheet: Sheet,
}
impl DeepDungeonDemicloneSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("DeepDungeonDemiclone")?;
        let sheet = resolver.read_excel_sheet(&exh, "DeepDungeonDemiclone", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<DeepDungeonDemicloneRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<DeepDungeonDemicloneRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for DeepDungeonDemicloneSheet {
    type Row = DeepDungeonDemicloneRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a DeepDungeonDemicloneSheet {
    type Item = (u32, Vec<(u16, DeepDungeonDemicloneRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, DeepDungeonDemicloneSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, DeepDungeonDemicloneSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct DeepDungeonDemicloneRow<'a> {
    row: &'a Row,
}
impl<'a> DeepDungeonDemicloneRow<'a> {
    pub fn Singular(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn Plural(&'a self) -> &'a str {
        self.row.columns[3].into_string().unwrap()
    }
    pub fn TitleCase(&'a self) -> &'a str {
        self.row.columns[9].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[10].into_string().unwrap()
    }
    pub fn Unknown4(&'a self) -> i8 {
        self.row.columns[2].into_i8().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> i8 {
        self.row.columns[4].into_i8().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> i8 {
        self.row.columns[5].into_i8().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> i8 {
        self.row.columns[6].into_i8().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> i8 {
        self.row.columns[7].into_i8().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> i8 {
        self.row.columns[8].into_i8().copied().unwrap()
    }
    pub fn Icon(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
}
