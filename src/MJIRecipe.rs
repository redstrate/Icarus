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
pub struct MJIRecipeSheet {
    sheet: Sheet,
}
impl MJIRecipeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MJIRecipe")?;
        let sheet = resolver.read_excel_sheet(&exh, "MJIRecipe", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MJIRecipeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MJIRecipeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MJIRecipeSheet {
    type Row = MJIRecipeRow;
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
impl<'a> IntoIterator for &'a MJIRecipeSheet {
    type Item = (u32, Vec<(u16, MJIRecipeRow)>);
    type IntoIter = StructuredSheetIterator<'a, MJIRecipeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MJIRecipeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MJIRecipeRow {
    columns: Vec<Field>,
}
impl MJIRecipeRow {
    pub fn LogMessage<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn KeyItem<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn ItemPouch<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Yield<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Material<'a>(&'a self) -> [&'a Field; 5] {
        [
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
        ]
    }
    pub fn Amount<'a>(&'a self) -> [&'a Field; 5] {
        [
            &self.columns[9],
            &self.columns[10],
            &self.columns[11],
            &self.columns[12],
            &self.columns[13],
        ]
    }
    pub fn Order<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
}
