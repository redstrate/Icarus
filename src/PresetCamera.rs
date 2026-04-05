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
pub struct PresetCameraSheet {
    sheet: Sheet,
}
impl PresetCameraSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PresetCamera")?;
        let sheet = resolver.read_excel_sheet(&exh, "PresetCamera", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PresetCameraRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PresetCameraRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for PresetCameraSheet {
    type Row = PresetCameraRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a PresetCameraSheet {
    type Item = (u32, Vec<(u16, PresetCameraRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, PresetCameraSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PresetCameraSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PresetCameraRow<'a> {
    row: &'a Row,
}
impl<'a> PresetCameraRow<'a> {
    pub fn PosX(&'a self) -> f32 {
        self.row.columns[1].into_f32().copied().unwrap()
    }
    pub fn PosY(&'a self) -> f32 {
        self.row.columns[2].into_f32().copied().unwrap()
    }
    pub fn PosZ(&'a self) -> f32 {
        self.row.columns[3].into_f32().copied().unwrap()
    }
    pub fn Elezen(&'a self) -> f32 {
        self.row.columns[4].into_f32().copied().unwrap()
    }
    pub fn Lalafell(&'a self) -> f32 {
        self.row.columns[5].into_f32().copied().unwrap()
    }
    pub fn Miqote(&'a self) -> f32 {
        self.row.columns[6].into_f32().copied().unwrap()
    }
    pub fn Roe(&'a self) -> f32 {
        self.row.columns[7].into_f32().copied().unwrap()
    }
    pub fn Hrothgar(&'a self) -> f32 {
        self.row.columns[8].into_f32().copied().unwrap()
    }
    pub fn Viera(&'a self) -> f32 {
        self.row.columns[9].into_f32().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> f32 {
        self.row.columns[10].into_f32().copied().unwrap()
    }
    pub fn Hyur_F(&'a self) -> f32 {
        self.row.columns[11].into_f32().copied().unwrap()
    }
    pub fn Elezen_F(&'a self) -> f32 {
        self.row.columns[12].into_f32().copied().unwrap()
    }
    pub fn Lalafell_F(&'a self) -> f32 {
        self.row.columns[13].into_f32().copied().unwrap()
    }
    pub fn Miqote_F(&'a self) -> f32 {
        self.row.columns[14].into_f32().copied().unwrap()
    }
    pub fn Roe_F(&'a self) -> f32 {
        self.row.columns[15].into_f32().copied().unwrap()
    }
    pub fn Hrothgar_F(&'a self) -> f32 {
        self.row.columns[16].into_f32().copied().unwrap()
    }
    pub fn Viera_F(&'a self) -> f32 {
        self.row.columns[17].into_f32().copied().unwrap()
    }
    pub fn Unknown_70(&'a self) -> f32 {
        self.row.columns[18].into_f32().copied().unwrap()
    }
    pub fn EID(&'a self) -> u16 {
        self.row.columns[0].into_u16().copied().unwrap()
    }
}
