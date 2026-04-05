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
pub struct AdventureSheet {
    sheet: Sheet,
}
impl AdventureSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Adventure")?;
        let sheet = resolver.read_excel_sheet(&exh, "Adventure", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AdventureRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<AdventureRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for AdventureSheet {
    type Row = AdventureRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a AdventureSheet {
    type Item = (u32, Vec<(u16, AdventureRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AdventureSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AdventureSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AdventureRow<'a> {
    row: &'a Row,
}
impl<'a> AdventureRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[11].into_string().unwrap()
    }
    pub fn Impression(&'a self) -> &'a str {
        self.row.columns[12].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[13].into_string().unwrap()
    }
    pub fn Level(&'a self) -> i32 {
        self.row.columns[0].into_i32().copied().unwrap()
    }
    pub fn MinLevel(&'a self) -> i32 {
        self.row.columns[1].into_i32().copied().unwrap()
    }
    pub fn PlaceName(&'a self) -> i32 {
        self.row.columns[6].into_i32().copied().unwrap()
    }
    pub fn IconList(&'a self) -> i32 {
        self.row.columns[7].into_i32().copied().unwrap()
    }
    pub fn IconDiscovered(&'a self) -> i32 {
        self.row.columns[8].into_i32().copied().unwrap()
    }
    pub fn IconUndiscovered(&'a self) -> i32 {
        self.row.columns[9].into_i32().copied().unwrap()
    }
    pub fn Emote(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn MinTime(&'a self) -> u16 {
        self.row.columns[4].into_u16().copied().unwrap()
    }
    pub fn MaxTime(&'a self) -> u16 {
        self.row.columns[5].into_u16().copied().unwrap()
    }
    pub fn MaxLevel(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn IsInitial(&'a self) -> bool {
        self.row.columns[10].into_bool().copied().unwrap()
    }
}
