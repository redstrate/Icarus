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
pub struct MotionTimelineAdvanceBlendSheet {
    sheet: Sheet,
}
impl MotionTimelineAdvanceBlendSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MotionTimelineAdvanceBlend")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "MotionTimelineAdvanceBlend", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MotionTimelineAdvanceBlendRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<MotionTimelineAdvanceBlendRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for MotionTimelineAdvanceBlendSheet {
    type Row = MotionTimelineAdvanceBlendRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a MotionTimelineAdvanceBlendSheet {
    type Item = (u32, Vec<(u16, MotionTimelineAdvanceBlendRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MotionTimelineAdvanceBlendSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MotionTimelineAdvanceBlendSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MotionTimelineAdvanceBlendRow<'a> {
    row: &'a Row,
}
impl<'a> MotionTimelineAdvanceBlendRow<'a> {
    pub fn Unknown0(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Unknown1(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
}
