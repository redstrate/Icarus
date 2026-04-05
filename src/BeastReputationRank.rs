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
pub struct BeastReputationRankSheet {
    sheet: Sheet,
}
impl BeastReputationRankSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BeastReputationRank")?;
        let sheet = resolver.read_excel_sheet(&exh, "BeastReputationRank", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BeastReputationRankRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BeastReputationRankRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BeastReputationRankSheet {
    type Row = BeastReputationRankRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BeastReputationRankSheet {
    type Item = (u32, Vec<(u16, BeastReputationRankRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BeastReputationRankSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BeastReputationRankSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BeastReputationRankRow<'a> {
    row: &'a Row,
}
impl<'a> BeastReputationRankRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn AlliedNames(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn Color(&'a self) -> u32 {
        self.row.columns[3].into_u32().copied().unwrap()
    }
    pub fn RequiredReputation(&'a self) -> u16 {
        self.row.columns[0].into_u16().copied().unwrap()
    }
}
