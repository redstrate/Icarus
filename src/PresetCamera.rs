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
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
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
impl StructuredSheet for PresetCameraSheet {
    type Row = PresetCameraRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            PosX: row
                .columns[1]
                .into_f32()
                .copied()
                .expect("Expected column 1 to be a float32!"),
            PosY: row
                .columns[2]
                .into_f32()
                .copied()
                .expect("Expected column 2 to be a float32!"),
            PosZ: row
                .columns[3]
                .into_f32()
                .copied()
                .expect("Expected column 3 to be a float32!"),
            Elezen: row
                .columns[4]
                .into_f32()
                .copied()
                .expect("Expected column 4 to be a float32!"),
            Lalafell: row
                .columns[5]
                .into_f32()
                .copied()
                .expect("Expected column 5 to be a float32!"),
            Miqote: row
                .columns[6]
                .into_f32()
                .copied()
                .expect("Expected column 6 to be a float32!"),
            Roe: row
                .columns[7]
                .into_f32()
                .copied()
                .expect("Expected column 7 to be a float32!"),
            Hrothgar: row
                .columns[8]
                .into_f32()
                .copied()
                .expect("Expected column 8 to be a float32!"),
            Viera: row
                .columns[9]
                .into_f32()
                .copied()
                .expect("Expected column 9 to be a float32!"),
            Unknown0: row
                .columns[10]
                .into_f32()
                .copied()
                .expect("Expected column 10 to be a float32!"),
            Hyur_F: row
                .columns[11]
                .into_f32()
                .copied()
                .expect("Expected column 11 to be a float32!"),
            Elezen_F: row
                .columns[12]
                .into_f32()
                .copied()
                .expect("Expected column 12 to be a float32!"),
            Lalafell_F: row
                .columns[13]
                .into_f32()
                .copied()
                .expect("Expected column 13 to be a float32!"),
            Miqote_F: row
                .columns[14]
                .into_f32()
                .copied()
                .expect("Expected column 14 to be a float32!"),
            Roe_F: row
                .columns[15]
                .into_f32()
                .copied()
                .expect("Expected column 15 to be a float32!"),
            Hrothgar_F: row
                .columns[16]
                .into_f32()
                .copied()
                .expect("Expected column 16 to be a float32!"),
            Viera_F: row
                .columns[17]
                .into_f32()
                .copied()
                .expect("Expected column 17 to be a float32!"),
            Unknown_70: row
                .columns[18]
                .into_f32()
                .copied()
                .expect("Expected column 18 to be a float32!"),
            EID: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
        })
    }
}
impl<'a> IntoIterator for &'a PresetCameraSheet {
    type Item = (u32, Vec<(u16, PresetCameraRow)>);
    type IntoIter = StructuredSheetIterator<'a, PresetCameraSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PresetCameraSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PresetCameraRow {
    ///""
    pub PosX: f32,
    ///""
    pub PosY: f32,
    ///""
    pub PosZ: f32,
    ///""
    pub Elezen: f32,
    ///""
    pub Lalafell: f32,
    ///""
    pub Miqote: f32,
    ///""
    pub Roe: f32,
    ///""
    pub Hrothgar: f32,
    ///""
    pub Viera: f32,
    ///""
    pub Unknown0: f32,
    ///""
    pub Hyur_F: f32,
    ///""
    pub Elezen_F: f32,
    ///""
    pub Lalafell_F: f32,
    ///""
    pub Miqote_F: f32,
    ///""
    pub Roe_F: f32,
    ///""
    pub Hrothgar_F: f32,
    ///""
    pub Viera_F: f32,
    ///""
    pub Unknown_70: f32,
    ///""
    pub EID: u16,
}
