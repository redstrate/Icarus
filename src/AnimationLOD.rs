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
pub struct AnimationLODSheet {
    sheet: Sheet,
}
impl AnimationLODSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("AnimationLOD")?;
        let sheet = resolver.read_excel_sheet(&exh, "AnimationLOD", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AnimationLODRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<AnimationLODRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for AnimationLODSheet {
    type Row = AnimationLODRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a AnimationLODSheet {
    type Item = (u32, Vec<(u16, AnimationLODRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AnimationLODSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AnimationLODSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnimationLODRow<'a> {
    row: &'a Row,
}
impl<'a> AnimationLODRow<'a> {
    pub fn CameraDistance(&'a self) -> f32 {
        self.row.columns[0].into_f32().copied().unwrap()
    }
    pub fn SampleInterval(&'a self) -> f32 {
        self.row.columns[1].into_f32().copied().unwrap()
    }
    pub fn BoneLOD(&'a self) -> i8 {
        self.row.columns[2].into_i8().copied().unwrap()
    }
    pub fn AnimationEnable(&'a self) -> [bool; 8] {
        [
            self.row.columns[3].into_bool().copied().unwrap(),
            self.row.columns[4].into_bool().copied().unwrap(),
            self.row.columns[5].into_bool().copied().unwrap(),
            self.row.columns[6].into_bool().copied().unwrap(),
            self.row.columns[7].into_bool().copied().unwrap(),
            self.row.columns[8].into_bool().copied().unwrap(),
            self.row.columns[9].into_bool().copied().unwrap(),
            self.row.columns[10].into_bool().copied().unwrap(),
        ]
    }
}
