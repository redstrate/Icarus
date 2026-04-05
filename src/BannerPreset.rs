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
pub struct BannerPresetSheet {
    sheet: Sheet,
}
impl BannerPresetSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BannerPreset")?;
        let sheet = resolver.read_excel_sheet(&exh, "BannerPreset", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BannerPresetRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BannerPresetRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BannerPresetSheet {
    type Row = BannerPresetRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BannerPresetSheet {
    type Item = (u32, Vec<(u16, BannerPresetRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BannerPresetSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BannerPresetSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BannerPresetRow<'a> {
    row: &'a Row,
}
impl<'a> BannerPresetRow<'a> {
    pub fn CameraPositionX(&'a self) -> f32 {
        self.row.columns[0].into_f32().copied().unwrap()
    }
    pub fn CameraPositionY(&'a self) -> f32 {
        self.row.columns[1].into_f32().copied().unwrap()
    }
    pub fn CameraPositionZ(&'a self) -> f32 {
        self.row.columns[2].into_f32().copied().unwrap()
    }
    pub fn CameraTargetX(&'a self) -> f32 {
        self.row.columns[3].into_f32().copied().unwrap()
    }
    pub fn CameraTargetY(&'a self) -> f32 {
        self.row.columns[4].into_f32().copied().unwrap()
    }
    pub fn CameraTargetZ(&'a self) -> f32 {
        self.row.columns[5].into_f32().copied().unwrap()
    }
    pub fn AnimationProgress(&'a self) -> f32 {
        self.row.columns[9].into_f32().copied().unwrap()
    }
    pub fn HeadDirectionX(&'a self) -> f32 {
        self.row.columns[11].into_f32().copied().unwrap()
    }
    pub fn HeadDirectionY(&'a self) -> f32 {
        self.row.columns[12].into_f32().copied().unwrap()
    }
    pub fn EyeDirectionX(&'a self) -> f32 {
        self.row.columns[13].into_f32().copied().unwrap()
    }
    pub fn EyeDirectionY(&'a self) -> f32 {
        self.row.columns[14].into_f32().copied().unwrap()
    }
    pub fn Expression(&'a self) -> i32 {
        self.row.columns[10].into_i32().copied().unwrap()
    }
    pub fn BannerTimeline(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn ImageRotation(&'a self) -> i16 {
        self.row.columns[6].into_i16().copied().unwrap()
    }
    pub fn DirectionalLightingVerticalAngle(&'a self) -> i16 {
        self.row.columns[20].into_i16().copied().unwrap()
    }
    pub fn DirectionalLightingHorizontalAngle(&'a self) -> i16 {
        self.row.columns[21].into_i16().copied().unwrap()
    }
    pub fn CameraZoom(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn BannerDesignPreset(&'a self) -> u8 {
        self.row.columns[15].into_u8().copied().unwrap()
    }
    pub fn DirectionalLightingColorRed(&'a self) -> u8 {
        self.row.columns[16].into_u8().copied().unwrap()
    }
    pub fn DirectionalLightingColorGreen(&'a self) -> u8 {
        self.row.columns[17].into_u8().copied().unwrap()
    }
    pub fn DirectionalLightingColorBlue(&'a self) -> u8 {
        self.row.columns[18].into_u8().copied().unwrap()
    }
    pub fn DirectionalLightingBrightness(&'a self) -> u8 {
        self.row.columns[19].into_u8().copied().unwrap()
    }
    pub fn AmbientLightingColorRed(&'a self) -> u8 {
        self.row.columns[22].into_u8().copied().unwrap()
    }
    pub fn AmbientLightingColorGreen(&'a self) -> u8 {
        self.row.columns[23].into_u8().copied().unwrap()
    }
    pub fn AmbientLightingColorBlue(&'a self) -> u8 {
        self.row.columns[24].into_u8().copied().unwrap()
    }
    pub fn AmbientLightingBrightness(&'a self) -> u8 {
        self.row.columns[25].into_u8().copied().unwrap()
    }
}
