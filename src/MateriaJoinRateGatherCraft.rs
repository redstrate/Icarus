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
pub struct MateriaJoinRateGatherCraftSheet {
    sheet: Sheet,
}
impl MateriaJoinRateGatherCraftSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MateriaJoinRateGatherCraft")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "MateriaJoinRateGatherCraft", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MateriaJoinRateGatherCraftRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<MateriaJoinRateGatherCraftRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MateriaJoinRateGatherCraftSheet {
    type Row = MateriaJoinRateGatherCraftRow;
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
impl<'a> IntoIterator for &'a MateriaJoinRateGatherCraftSheet {
    type Item = (u32, Vec<(u16, MateriaJoinRateGatherCraftRow)>);
    type IntoIter = StructuredSheetIterator<'a, MateriaJoinRateGatherCraftSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MateriaJoinRateGatherCraftSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MateriaJoinRateGatherCraftRow {
    columns: Vec<Field>,
}
impl MateriaJoinRateGatherCraftRow {
    pub fn NQOvermeldPercentSlot<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[0], &self.columns[1], &self.columns[2], &self.columns[3]]
    }
    pub fn HQOvermeldPercentSlot<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[4], &self.columns[5], &self.columns[6], &self.columns[7]]
    }
}
