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
pub struct PhysicsWindSheet {
    sheet: Sheet,
}
impl PhysicsWindSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PhysicsWind")?;
        let sheet = resolver.read_excel_sheet(&exh, "PhysicsWind", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PhysicsWindRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PhysicsWindRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for PhysicsWindSheet {
    type Row = PhysicsWindRow;
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
impl<'a> IntoIterator for &'a PhysicsWindSheet {
    type Item = (u32, Vec<(u16, PhysicsWindRow)>);
    type IntoIter = StructuredSheetIterator<'a, PhysicsWindSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PhysicsWindSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PhysicsWindRow {
    columns: Vec<Field>,
}
impl PhysicsWindRow {
    pub fn Threshold<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Amplitude<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn AmplitudeFrequency<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn PowerMin<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn PowerMax<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn PowerFrequency<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
}
