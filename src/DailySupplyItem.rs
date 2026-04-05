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
pub struct DailySupplyItemSheet {
    sheet: Sheet,
}
impl DailySupplyItemSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("DailySupplyItem")?;
        let sheet = resolver.read_excel_sheet(&exh, "DailySupplyItem", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<DailySupplyItemRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<DailySupplyItemRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for DailySupplyItemSheet {
    type Row = DailySupplyItemRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a DailySupplyItemSheet {
    type Item = (u32, Vec<(u16, DailySupplyItemRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, DailySupplyItemSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, DailySupplyItemSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct DailySupplyItemRow<'a> {
    row: &'a Row,
}
impl<'a> DailySupplyItemRow<'a> {
    pub fn Item(&'a self) -> [i32; 8] {
        [
            self.row.columns[0].into_i32().copied().unwrap(),
            self.row.columns[3].into_i32().copied().unwrap(),
            self.row.columns[6].into_i32().copied().unwrap(),
            self.row.columns[9].into_i32().copied().unwrap(),
            self.row.columns[12].into_i32().copied().unwrap(),
            self.row.columns[15].into_i32().copied().unwrap(),
            self.row.columns[18].into_i32().copied().unwrap(),
            self.row.columns[21].into_i32().copied().unwrap(),
        ]
    }
    pub fn Quantity(&'a self) -> [u8; 8] {
        [
            self.row.columns[1].into_u8().copied().unwrap(),
            self.row.columns[4].into_u8().copied().unwrap(),
            self.row.columns[7].into_u8().copied().unwrap(),
            self.row.columns[10].into_u8().copied().unwrap(),
            self.row.columns[13].into_u8().copied().unwrap(),
            self.row.columns[16].into_u8().copied().unwrap(),
            self.row.columns[19].into_u8().copied().unwrap(),
            self.row.columns[22].into_u8().copied().unwrap(),
        ]
    }
    pub fn RecipeLevel(&'a self) -> [u8; 8] {
        [
            self.row.columns[2].into_u8().copied().unwrap(),
            self.row.columns[5].into_u8().copied().unwrap(),
            self.row.columns[8].into_u8().copied().unwrap(),
            self.row.columns[11].into_u8().copied().unwrap(),
            self.row.columns[14].into_u8().copied().unwrap(),
            self.row.columns[17].into_u8().copied().unwrap(),
            self.row.columns[20].into_u8().copied().unwrap(),
            self.row.columns[23].into_u8().copied().unwrap(),
        ]
    }
}
