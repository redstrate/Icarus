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
pub struct BGMSheet {
    sheet: Sheet,
}
impl BGMSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BGM")?;
        let sheet = resolver.read_excel_sheet(&exh, "BGM", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BGMRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BGMRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BGMSheet {
    type Row = BGMRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BGMSheet {
    type Item = (u32, Vec<(u16, BGMRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BGMSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BGMSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BGMRow<'a> {
    row: &'a Row,
}
impl<'a> BGMRow<'a> {
    pub fn File(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn DisableRestartResetTime(&'a self) -> f32 {
        self.row.columns[5].into_f32().copied().unwrap()
    }
    pub fn Priority(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn SpecialMode(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn DisableRestartTimeOut(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
    pub fn DisableRestart(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    pub fn PassEnd(&'a self) -> bool {
        self.row.columns[4].into_bool().copied().unwrap()
    }
}
