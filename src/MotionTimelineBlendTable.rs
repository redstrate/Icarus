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
impl StructuredSheet for MotionTimelineBlendTableSheet {
    type Row = MotionTimelineBlendTableRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            DestBlendGroup: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            SrcBlendGroup: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            BlendFrame_PC: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            BlendFram_TypeA: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            BlendFram_TypeB: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            BlendFram_TypeC: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a MotionTimelineBlendTableSheet {
    type Item = (u32, Vec<(u16, MotionTimelineBlendTableRow)>);
    type IntoIter = StructuredSheetIterator<'a, MotionTimelineBlendTableSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MotionTimelineBlendTableSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MotionTimelineBlendTableRow {
    ///""
    pub DestBlendGroup: u8,
    ///""
    pub SrcBlendGroup: u8,
    ///""
    pub BlendFrame_PC: u8,
    ///""
    pub BlendFram_TypeA: u8,
    ///""
    pub BlendFram_TypeB: u8,
    ///""
    pub BlendFram_TypeC: u8,
}
