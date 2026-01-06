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
pub struct GatheringLeveSheet {
    sheet: Sheet,
}
impl GatheringLeveSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GatheringLeve")?;
        let sheet = resolver.read_excel_sheet(&exh, "GatheringLeve", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GatheringLeveRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GatheringLeveRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GatheringLeveSheet {
    type Row = GatheringLeveRow;
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
impl<'a> IntoIterator for &'a GatheringLeveSheet {
    type Item = (u32, Vec<(u16, GatheringLeveRow)>);
    type IntoIter = StructuredSheetIterator<'a, GatheringLeveSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GatheringLeveSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GatheringLeveRow {
    columns: Vec<Field>,
}
impl GatheringLeveRow {
    pub fn Route<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[0], &self.columns[1], &self.columns[2], &self.columns[3]]
    }
    pub fn RequiredItem<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[4], &self.columns[5], &self.columns[6], &self.columns[7]]
    }
    pub fn Rule<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn BNpcEntry<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn Objective<'a>(&'a self) -> [&'a Field; 2] {
        [&self.columns[10], &self.columns[11]]
    }
    pub fn RequiredItemQuantity<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[12], &self.columns[13], &self.columns[14], &self.columns[15]]
    }
    pub fn ItemNumber<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn Varient<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn UseSecondaryTool<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
}
