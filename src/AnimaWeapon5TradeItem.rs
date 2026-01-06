//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct AnimaWeapon5TradeItemSheet {
    sheet: Sheet,
}
impl AnimaWeapon5TradeItemSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AnimaWeapon5TradeItem")?;
        let sheet = resolver.read_excel_sheet(&exh, "AnimaWeapon5TradeItem", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AnimaWeapon5TradeItemRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<AnimaWeapon5TradeItemRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for AnimaWeapon5TradeItemSheet {
    type Row = AnimaWeapon5TradeItemRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
    }
}
impl<'a> IntoIterator for &'a AnimaWeapon5TradeItemSheet {
    type Item = (u32, Vec<(u16, AnimaWeapon5TradeItemRow)>);
    type IntoIter = StructuredSheetIterator<'a, AnimaWeapon5TradeItemSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AnimaWeapon5TradeItemSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnimaWeapon5TradeItemRow {
    columns: Vec<Field>,
}
impl AnimaWeapon5TradeItemRow {
    pub fn CrystalSand<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Item<'a>(&'a self) -> [&'a Field; 8] {
        [
            &self.columns[1],
            &self.columns[2],
            &self.columns[3],
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
        ]
    }
    pub fn Order<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn ReceiveQuantity<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn Quantity<'a>(&'a self) -> [&'a Field; 8] {
        [
            &self.columns[11],
            &self.columns[12],
            &self.columns[13],
            &self.columns[14],
            &self.columns[15],
            &self.columns[16],
            &self.columns[17],
            &self.columns[18],
        ]
    }
    pub fn Category<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn IsHQ<'a>(&'a self) -> [&'a Field; 8] {
        [
            &self.columns[20],
            &self.columns[21],
            &self.columns[22],
            &self.columns[23],
            &self.columns[24],
            &self.columns[25],
            &self.columns[26],
            &self.columns[27],
        ]
    }
}
