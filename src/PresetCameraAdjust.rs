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
pub struct PresetCameraAdjustSheet {
    sheet: Sheet,
}
impl PresetCameraAdjustSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PresetCameraAdjust")?;
        let sheet = resolver.read_excel_sheet(&exh, "PresetCameraAdjust", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PresetCameraAdjustRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PresetCameraAdjustRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for PresetCameraAdjustSheet {
    type Row = PresetCameraAdjustRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a PresetCameraAdjustSheet {
    type Item = (u32, Vec<(u16, PresetCameraAdjustRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, PresetCameraAdjustSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PresetCameraAdjustSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PresetCameraAdjustRow<'a> {
    row: &'a Row,
}
impl<'a> PresetCameraAdjustRow<'a> {
    pub fn Hyur_M(&'a self) -> f32 {
        self.row.columns[0].into_f32().copied().unwrap()
    }
    pub fn Hyur_F(&'a self) -> f32 {
        self.row.columns[1].into_f32().copied().unwrap()
    }
    pub fn Elezen_M(&'a self) -> f32 {
        self.row.columns[2].into_f32().copied().unwrap()
    }
    pub fn Elezen_F(&'a self) -> f32 {
        self.row.columns[3].into_f32().copied().unwrap()
    }
    pub fn Lalafell_M(&'a self) -> f32 {
        self.row.columns[4].into_f32().copied().unwrap()
    }
    pub fn Lalafell_F(&'a self) -> f32 {
        self.row.columns[5].into_f32().copied().unwrap()
    }
    pub fn Miqote_M(&'a self) -> f32 {
        self.row.columns[6].into_f32().copied().unwrap()
    }
    pub fn Miqote_F(&'a self) -> f32 {
        self.row.columns[7].into_f32().copied().unwrap()
    }
    pub fn Roe_M(&'a self) -> f32 {
        self.row.columns[8].into_f32().copied().unwrap()
    }
    pub fn Roe_F(&'a self) -> f32 {
        self.row.columns[9].into_f32().copied().unwrap()
    }
    pub fn Hrothgar_M(&'a self) -> f32 {
        self.row.columns[10].into_f32().copied().unwrap()
    }
    pub fn Hrothgar_F(&'a self) -> f32 {
        self.row.columns[11].into_f32().copied().unwrap()
    }
    pub fn Viera_M(&'a self) -> f32 {
        self.row.columns[12].into_f32().copied().unwrap()
    }
    pub fn Viera_F(&'a self) -> f32 {
        self.row.columns[13].into_f32().copied().unwrap()
    }
    pub fn Unknown_70(&'a self) -> f32 {
        self.row.columns[14].into_f32().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> f32 {
        self.row.columns[15].into_f32().copied().unwrap()
    }
}
