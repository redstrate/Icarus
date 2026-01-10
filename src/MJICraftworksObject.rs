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
pub struct MJICraftworksObjectSheet {
    sheet: Sheet,
}
impl MJICraftworksObjectSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MJICraftworksObject")?;
        let sheet = resolver.read_excel_sheet(&exh, "MJICraftworksObject", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MJICraftworksObjectRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MJICraftworksObjectRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MJICraftworksObjectSheet {
    type Row = MJICraftworksObjectRow;
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
impl<'a> IntoIterator for &'a MJICraftworksObjectSheet {
    type Item = (u32, Vec<(u16, MJICraftworksObjectRow)>);
    type IntoIter = StructuredSheetIterator<'a, MJICraftworksObjectSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MJICraftworksObjectSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MJICraftworksObjectRow {
    columns: Vec<Field>,
}
impl MJICraftworksObjectRow {
    pub fn Item<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Theme<'a>(&'a self) -> [&'a Field; 2] {
        [&self.columns[1], &self.columns[2]]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Material<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[4], &self.columns[5], &self.columns[6], &self.columns[7]]
    }
    pub fn Amount<'a>(&'a self) -> [&'a Field; 4] {
        [&self.columns[8], &self.columns[9], &self.columns[10], &self.columns[11]]
    }
    pub fn LevelReq<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn CraftingTime<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn Value<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
}
