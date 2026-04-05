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
pub struct BeastRankBonusSheet {
    sheet: Sheet,
}
impl BeastRankBonusSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BeastRankBonus")?;
        let sheet = resolver.read_excel_sheet(&exh, "BeastRankBonus", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BeastRankBonusRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BeastRankBonusRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BeastRankBonusSheet {
    type Row = BeastRankBonusRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BeastRankBonusSheet {
    type Item = (u32, Vec<(u16, BeastRankBonusRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BeastRankBonusSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BeastRankBonusSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BeastRankBonusRow<'a> {
    row: &'a Row,
}
impl<'a> BeastRankBonusRow<'a> {
    pub fn Item(&'a self) -> u32 {
        self.row.columns[8].into_u32().copied().unwrap()
    }
    pub fn Neutral(&'a self) -> u16 {
        self.row.columns[0].into_u16().copied().unwrap()
    }
    pub fn Recognized(&'a self) -> u16 {
        self.row.columns[1].into_u16().copied().unwrap()
    }
    pub fn Friendly(&'a self) -> u16 {
        self.row.columns[2].into_u16().copied().unwrap()
    }
    pub fn Trusted(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn Respected(&'a self) -> u16 {
        self.row.columns[4].into_u16().copied().unwrap()
    }
    pub fn Honored(&'a self) -> u16 {
        self.row.columns[5].into_u16().copied().unwrap()
    }
    pub fn Sworn(&'a self) -> u16 {
        self.row.columns[6].into_u16().copied().unwrap()
    }
    pub fn AlliedBloodsworn(&'a self) -> u16 {
        self.row.columns[7].into_u16().copied().unwrap()
    }
    pub fn ItemQuantity(&'a self) -> [u8; 8] {
        [
            self.row.columns[9].into_u8().copied().unwrap(),
            self.row.columns[10].into_u8().copied().unwrap(),
            self.row.columns[11].into_u8().copied().unwrap(),
            self.row.columns[12].into_u8().copied().unwrap(),
            self.row.columns[13].into_u8().copied().unwrap(),
            self.row.columns[14].into_u8().copied().unwrap(),
            self.row.columns[15].into_u8().copied().unwrap(),
            self.row.columns[16].into_u8().copied().unwrap(),
        ]
    }
}
