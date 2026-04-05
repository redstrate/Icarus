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
pub struct SkyIsland2MissionTypeSheet {
    sheet: Sheet,
}
impl SkyIsland2MissionTypeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SkyIsland2MissionType")?;
        let sheet = resolver.read_excel_sheet(&exh, "SkyIsland2MissionType", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SkyIsland2MissionTypeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<SkyIsland2MissionTypeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for SkyIsland2MissionTypeSheet {
    type Row = SkyIsland2MissionTypeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a SkyIsland2MissionTypeSheet {
    type Item = (u32, Vec<(u16, SkyIsland2MissionTypeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, SkyIsland2MissionTypeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SkyIsland2MissionTypeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SkyIsland2MissionTypeRow<'a> {
    row: &'a Row,
}
impl<'a> SkyIsland2MissionTypeRow<'a> {
    pub fn Type(&'a self) -> bool {
        self.row.columns[0].into_bool().copied().unwrap()
    }
}
