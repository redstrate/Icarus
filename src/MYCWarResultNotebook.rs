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
pub struct MYCWarResultNotebookSheet {
    sheet: Sheet,
}
impl MYCWarResultNotebookSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MYCWarResultNotebook")?;
        let sheet = resolver.read_excel_sheet(&exh, "MYCWarResultNotebook", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MYCWarResultNotebookRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<MYCWarResultNotebookRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MYCWarResultNotebookSheet {
    type Row = MYCWarResultNotebookRow;
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
impl<'a> IntoIterator for &'a MYCWarResultNotebookSheet {
    type Item = (u32, Vec<(u16, MYCWarResultNotebookRow)>);
    type IntoIter = StructuredSheetIterator<'a, MYCWarResultNotebookSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MYCWarResultNotebookSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MYCWarResultNotebookRow {
    columns: Vec<Field>,
}
impl MYCWarResultNotebookRow {
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Description<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn NameJP<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Quest<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn Icon<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn Image<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Number<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn Link<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn Rarity<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
}
