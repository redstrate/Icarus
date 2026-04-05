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
pub struct RetainerTaskParameterSheet {
    sheet: Sheet,
}
impl RetainerTaskParameterSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("RetainerTaskParameter")?;
        let sheet = resolver.read_excel_sheet(&exh, "RetainerTaskParameter", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<RetainerTaskParameterRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<RetainerTaskParameterRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for RetainerTaskParameterSheet {
    type Row = RetainerTaskParameterRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a RetainerTaskParameterSheet {
    type Item = (u32, Vec<(u16, RetainerTaskParameterRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, RetainerTaskParameterSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, RetainerTaskParameterSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RetainerTaskParameterRow<'a> {
    row: &'a Row,
}
impl<'a> RetainerTaskParameterRow<'a> {
    pub fn ItemLevelDoW(&'a self) -> [i16; 4] {
        [
            self.row.columns[0].into_i16().copied().unwrap(),
            self.row.columns[1].into_i16().copied().unwrap(),
            self.row.columns[2].into_i16().copied().unwrap(),
            self.row.columns[3].into_i16().copied().unwrap(),
        ]
    }
    pub fn PerceptionDoL(&'a self) -> [i16; 4] {
        [
            self.row.columns[4].into_i16().copied().unwrap(),
            self.row.columns[5].into_i16().copied().unwrap(),
            self.row.columns[6].into_i16().copied().unwrap(),
            self.row.columns[7].into_i16().copied().unwrap(),
        ]
    }
    pub fn PerceptionFSH(&'a self) -> [i16; 4] {
        [
            self.row.columns[8].into_i16().copied().unwrap(),
            self.row.columns[9].into_i16().copied().unwrap(),
            self.row.columns[10].into_i16().copied().unwrap(),
            self.row.columns[11].into_i16().copied().unwrap(),
        ]
    }
}
