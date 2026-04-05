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
pub struct SkyIsland2MissionDetailSheet {
    sheet: Sheet,
}
impl SkyIsland2MissionDetailSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SkyIsland2MissionDetail")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "SkyIsland2MissionDetail", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SkyIsland2MissionDetailRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<SkyIsland2MissionDetailRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for SkyIsland2MissionDetailSheet {
    type Row = SkyIsland2MissionDetailRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a SkyIsland2MissionDetailSheet {
    type Item = (u32, Vec<(u16, SkyIsland2MissionDetailRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, SkyIsland2MissionDetailSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SkyIsland2MissionDetailSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SkyIsland2MissionDetailRow<'a> {
    row: &'a Row,
}
impl<'a> SkyIsland2MissionDetailRow<'a> {
    pub fn Objective(&'a self) -> &'a str {
        self.row.columns[7].into_string().unwrap()
    }
    pub fn Unknown0(&'a self) -> &'a str {
        self.row.columns[8].into_string().unwrap()
    }
    pub fn Unknown1(&'a self) -> &'a str {
        self.row.columns[9].into_string().unwrap()
    }
    pub fn Unknown2(&'a self) -> &'a str {
        self.row.columns[10].into_string().unwrap()
    }
    pub fn EObj(&'a self) -> u32 {
        self.row.columns[4].into_u32().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u32 {
        self.row.columns[5].into_u32().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> u32 {
        self.row.columns[6].into_u32().copied().unwrap()
    }
    pub fn Type(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn Range(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> i8 {
        self.row.columns[3].into_i8().copied().unwrap()
    }
}
