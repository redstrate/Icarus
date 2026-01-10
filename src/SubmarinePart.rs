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
pub struct SubmarinePartSheet {
    sheet: Sheet,
}
impl SubmarinePartSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SubmarinePart")?;
        let sheet = resolver.read_excel_sheet(&exh, "SubmarinePart", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SubmarinePartRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SubmarinePartRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for SubmarinePartSheet {
    type Row = SubmarinePartRow;
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
impl<'a> IntoIterator for &'a SubmarinePartSheet {
    type Item = (u32, Vec<(u16, SubmarinePartRow)>);
    type IntoIter = StructuredSheetIterator<'a, SubmarinePartSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SubmarinePartSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubmarinePartRow {
    columns: Vec<Field>,
}
impl SubmarinePartRow {
    pub fn Class<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Surveillance<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn Retrieval<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Speed<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Range<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn Favor<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn Slot<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Rank<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn Components<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn RepairMaterials<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
}
