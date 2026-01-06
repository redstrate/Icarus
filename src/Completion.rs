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
pub struct CompletionSheet {
    sheet: Sheet,
}
impl CompletionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Completion")?;
        let sheet = resolver.read_excel_sheet(&exh, "Completion", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CompletionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CompletionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CompletionSheet {
    type Row = CompletionRow;
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
impl<'a> IntoIterator for &'a CompletionSheet {
    type Item = (u32, Vec<(u16, CompletionRow)>);
    type IntoIter = StructuredSheetIterator<'a, CompletionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CompletionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CompletionRow {
    columns: Vec<Field>,
}
impl CompletionRow {
    pub fn Text<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn GroupTitle<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn LookupTable<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Group<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Key<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
}
