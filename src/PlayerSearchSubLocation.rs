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
pub struct PlayerSearchSubLocationSheet {
    sheet: Sheet,
}
impl PlayerSearchSubLocationSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PlayerSearchSubLocation")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "PlayerSearchSubLocation", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PlayerSearchSubLocationRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<PlayerSearchSubLocationRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for PlayerSearchSubLocationSheet {
    type Row = PlayerSearchSubLocationRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a PlayerSearchSubLocationSheet {
    type Item = (u32, Vec<(u16, PlayerSearchSubLocationRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, PlayerSearchSubLocationSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PlayerSearchSubLocationSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PlayerSearchSubLocationRow<'a> {
    row: &'a Row,
}
impl<'a> PlayerSearchSubLocationRow<'a> {
    pub fn Name0(&'a self) -> &'a str {
        self.row.columns[3].into_string().unwrap()
    }
    pub fn Name1(&'a self) -> &'a str {
        self.row.columns[4].into_string().unwrap()
    }
    pub fn Name2(&'a self) -> &'a str {
        self.row.columns[5].into_string().unwrap()
    }
    pub fn PlaceName(&'a self) -> i32 {
        self.row.columns[1].into_i32().copied().unwrap()
    }
    /// The UI category this appears in.
    pub fn Location(&'a self) -> i32 {
        self.row.columns[2].into_i32().copied().unwrap()
    }
    pub fn SortKey(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
}
