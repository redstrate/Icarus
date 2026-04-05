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
pub struct RecastNavimeshSheet {
    sheet: Sheet,
}
impl RecastNavimeshSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("RecastNavimesh")?;
        let sheet = resolver.read_excel_sheet(&exh, "RecastNavimesh", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<RecastNavimeshRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<RecastNavimeshRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for RecastNavimeshSheet {
    type Row = RecastNavimeshRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a RecastNavimeshSheet {
    type Item = (u32, Vec<(u16, RecastNavimeshRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, RecastNavimeshSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, RecastNavimeshSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RecastNavimeshRow<'a> {
    row: &'a Row,
}
impl<'a> RecastNavimeshRow<'a> {
    pub fn Unknown0(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn TileSize(&'a self) -> f32 {
        self.row.columns[1].into_f32().copied().unwrap()
    }
    pub fn CellSize(&'a self) -> f32 {
        self.row.columns[2].into_f32().copied().unwrap()
    }
    pub fn CellHeight(&'a self) -> f32 {
        self.row.columns[3].into_f32().copied().unwrap()
    }
    pub fn AgentHeight(&'a self) -> f32 {
        self.row.columns[4].into_f32().copied().unwrap()
    }
    pub fn AgentRadius(&'a self) -> f32 {
        self.row.columns[5].into_f32().copied().unwrap()
    }
    pub fn AgentMaxClimb(&'a self) -> f32 {
        self.row.columns[6].into_f32().copied().unwrap()
    }
    pub fn AgentMaxSlope(&'a self) -> f32 {
        self.row.columns[7].into_f32().copied().unwrap()
    }
    pub fn RegionMinSize(&'a self) -> f32 {
        self.row.columns[9].into_f32().copied().unwrap()
    }
    pub fn RegionMergedSize(&'a self) -> f32 {
        self.row.columns[10].into_f32().copied().unwrap()
    }
    pub fn MaxEdgeLength(&'a self) -> f32 {
        self.row.columns[12].into_f32().copied().unwrap()
    }
    pub fn MaxEdgeError(&'a self) -> f32 {
        self.row.columns[13].into_f32().copied().unwrap()
    }
    pub fn VertsPerPoly(&'a self) -> f32 {
        self.row.columns[14].into_f32().copied().unwrap()
    }
    pub fn DetailMeshSampleDistance(&'a self) -> f32 {
        self.row.columns[15].into_f32().copied().unwrap()
    }
    pub fn DetailMeshMaxSampleError(&'a self) -> f32 {
        self.row.columns[16].into_f32().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> f32 {
        self.row.columns[17].into_f32().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> f32 {
        self.row.columns[18].into_f32().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> f32 {
        self.row.columns[19].into_f32().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> f32 {
        self.row.columns[20].into_f32().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> f32 {
        self.row.columns[21].into_f32().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> f32 {
        self.row.columns[22].into_f32().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> f32 {
        self.row.columns[23].into_f32().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> f32 {
        self.row.columns[24].into_f32().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> f32 {
        self.row.columns[25].into_f32().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> f32 {
        self.row.columns[26].into_f32().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> f32 {
        self.row.columns[27].into_f32().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> f32 {
        self.row.columns[28].into_f32().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> f32 {
        self.row.columns[29].into_f32().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> f32 {
        self.row.columns[31].into_f32().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> f32 {
        self.row.columns[32].into_f32().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> f32 {
        self.row.columns[33].into_f32().copied().unwrap()
    }
    pub fn Unknown17(&'a self) -> bool {
        self.row.columns[8].into_bool().copied().unwrap()
    }
    pub fn Unknown18(&'a self) -> bool {
        self.row.columns[11].into_bool().copied().unwrap()
    }
    pub fn Unknown19(&'a self) -> bool {
        self.row.columns[30].into_bool().copied().unwrap()
    }
    pub fn Unknown20(&'a self) -> bool {
        self.row.columns[34].into_bool().copied().unwrap()
    }
}
