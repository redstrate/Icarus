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
pub struct OnlineStatusSheet {
    sheet: Sheet,
}
impl OnlineStatusSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("OnlineStatus")?;
        let sheet = resolver.read_excel_sheet(&exh, "OnlineStatus", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<OnlineStatusRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<OnlineStatusRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for OnlineStatusSheet {
    type Row = OnlineStatusRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a OnlineStatusSheet {
    type Item = (u32, Vec<(u16, OnlineStatusRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, OnlineStatusSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, OnlineStatusSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct OnlineStatusRow<'a> {
    row: &'a Row,
}
impl<'a> OnlineStatusRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[6].into_string().unwrap()
    }
    pub fn Icon(&'a self) -> u32 {
        self.row.columns[4].into_u32().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> i32 {
        self.row.columns[5].into_i32().copied().unwrap()
    }
    pub fn Priority(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> bool {
        self.row.columns[0].into_bool().copied().unwrap()
    }
    pub fn List(&'a self) -> bool {
        self.row.columns[1].into_bool().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
}
