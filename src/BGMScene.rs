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
pub struct BGMSceneSheet {
    sheet: Sheet,
}
impl BGMSceneSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BGMScene")?;
        let sheet = resolver.read_excel_sheet(&exh, "BGMScene", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BGMSceneRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BGMSceneRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BGMSceneSheet {
    type Row = BGMSceneRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BGMSceneSheet {
    type Item = (u32, Vec<(u16, BGMSceneRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BGMSceneSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BGMSceneSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BGMSceneRow<'a> {
    row: &'a Row,
}
impl<'a> BGMSceneRow<'a> {
    pub fn EnableDisableRestart(&'a self) -> bool {
        self.row.columns[0].into_bool().copied().unwrap()
    }
    pub fn Resume(&'a self) -> bool {
        self.row.columns[1].into_bool().copied().unwrap()
    }
    pub fn EnablePassEnd(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
    pub fn ForceAutoReset(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    pub fn IgnoreBattle(&'a self) -> bool {
        self.row.columns[4].into_bool().copied().unwrap()
    }
}
