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
pub struct GrandCompanyRankSheet {
    sheet: Sheet,
}
impl GrandCompanyRankSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GrandCompanyRank")?;
        let sheet = resolver.read_excel_sheet(&exh, "GrandCompanyRank", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GrandCompanyRankRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GrandCompanyRankRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for GrandCompanyRankSheet {
    type Row = GrandCompanyRankRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a GrandCompanyRankSheet {
    type Item = (u32, Vec<(u16, GrandCompanyRankRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, GrandCompanyRankSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GrandCompanyRankSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GrandCompanyRankRow<'a> {
    row: &'a Row,
}
impl<'a> GrandCompanyRankRow<'a> {
    pub fn MaxSeals(&'a self) -> u32 {
        self.row.columns[2].into_u32().copied().unwrap()
    }
    pub fn RequiredSeals(&'a self) -> u32 {
        self.row.columns[3].into_u32().copied().unwrap()
    }
    pub fn IconMaelstrom(&'a self) -> i32 {
        self.row.columns[4].into_i32().copied().unwrap()
    }
    pub fn IconSerpents(&'a self) -> i32 {
        self.row.columns[5].into_i32().copied().unwrap()
    }
    pub fn IconFlames(&'a self) -> i32 {
        self.row.columns[6].into_i32().copied().unwrap()
    }
    pub fn QuestMaelstrom(&'a self) -> i32 {
        self.row.columns[7].into_i32().copied().unwrap()
    }
    pub fn QuestSerpents(&'a self) -> i32 {
        self.row.columns[8].into_i32().copied().unwrap()
    }
    pub fn QuestFlames(&'a self) -> i32 {
        self.row.columns[9].into_i32().copied().unwrap()
    }
    pub fn Tier(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn Order(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
}
