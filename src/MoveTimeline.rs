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
pub struct MoveTimelineSheet {
    sheet: Sheet,
}
impl MoveTimelineSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MoveTimeline")?;
        let sheet = resolver.read_excel_sheet(&exh, "MoveTimeline", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MoveTimelineRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MoveTimelineRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MoveTimelineSheet {
    type Row = MoveTimelineRow;
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
impl<'a> IntoIterator for &'a MoveTimelineSheet {
    type Item = (u32, Vec<(u16, MoveTimelineRow)>);
    type IntoIter = StructuredSheetIterator<'a, MoveTimelineSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MoveTimelineSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MoveTimelineRow {
    columns: Vec<Field>,
}
impl MoveTimelineRow {
    pub fn Idle<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn MoveForward<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn MoveBack<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn MoveLeft<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn MoveRight<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn MoveUp<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn MoveDown<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn MoveTurnLeft<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn MoveTurnRight<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn Extra<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
}
