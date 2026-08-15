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
pub struct WKSPioneeringTrailSheet {
    sheet: Sheet,
}
impl WKSPioneeringTrailSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSPioneeringTrail")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSPioneeringTrail", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSPioneeringTrailRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSPioneeringTrailRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WKSPioneeringTrailSheet {
    type Row = WKSPioneeringTrailRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Thumbnail: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            ActivationStage: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            Unknown2: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            LogEntry: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
        })
    }
}
impl<'a> IntoIterator for &'a WKSPioneeringTrailSheet {
    type Item = (u32, Vec<(u16, WKSPioneeringTrailRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSPioneeringTrailSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSPioneeringTrailSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WKSPioneeringTrailRow {
    ///"The image used as thumbnail for log entry in the Infrastructure Index."
    pub Thumbnail: u32,
    ///"The stage at which the log entry is added to the Infrastructure Index."
    pub ActivationStage: u16,
    ///""
    pub Unknown2: u16,
    ///""
    pub LogEntry: u16,
}
