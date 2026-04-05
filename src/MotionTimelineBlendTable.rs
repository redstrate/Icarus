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
pub struct MotionTimelineBlendTableSheet {
    sheet: Sheet,
}
impl MotionTimelineBlendTableSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MotionTimelineBlendTable")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "MotionTimelineBlendTable", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MotionTimelineBlendTableRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<MotionTimelineBlendTableRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for MotionTimelineBlendTableSheet {
    type Row = MotionTimelineBlendTableRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a MotionTimelineBlendTableSheet {
    type Item = (u32, Vec<(u16, MotionTimelineBlendTableRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MotionTimelineBlendTableSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MotionTimelineBlendTableSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MotionTimelineBlendTableRow<'a> {
    row: &'a Row,
}
impl<'a> MotionTimelineBlendTableRow<'a> {
    pub fn DestBlendGroup(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn SrcBlendGroup(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn BlendFrame_PC(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn BlendFram_TypeA(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn BlendFram_TypeB(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    pub fn BlendFram_TypeC(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
}
