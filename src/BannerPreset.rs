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
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
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
impl StructuredSheet for BannerPresetSheet {
    type Row = BannerPresetRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            CameraPositionX: row
                .columns[0]
                .into_f32()
                .copied()
                .expect("Expected column 0 to be a float32!"),
            CameraPositionY: row
                .columns[1]
                .into_f32()
                .copied()
                .expect("Expected column 1 to be a float32!"),
            CameraPositionZ: row
                .columns[2]
                .into_f32()
                .copied()
                .expect("Expected column 2 to be a float32!"),
            CameraTargetX: row
                .columns[3]
                .into_f32()
                .copied()
                .expect("Expected column 3 to be a float32!"),
            CameraTargetY: row
                .columns[4]
                .into_f32()
                .copied()
                .expect("Expected column 4 to be a float32!"),
            CameraTargetZ: row
                .columns[5]
                .into_f32()
                .copied()
                .expect("Expected column 5 to be a float32!"),
            AnimationProgress: row
                .columns[9]
                .into_f32()
                .copied()
                .expect("Expected column 9 to be a float32!"),
            HeadDirectionX: row
                .columns[11]
                .into_f32()
                .copied()
                .expect("Expected column 11 to be a float32!"),
            HeadDirectionY: row
                .columns[12]
                .into_f32()
                .copied()
                .expect("Expected column 12 to be a float32!"),
            EyeDirectionX: row
                .columns[13]
                .into_f32()
                .copied()
                .expect("Expected column 13 to be a float32!"),
            EyeDirectionY: row
                .columns[14]
                .into_f32()
                .copied()
                .expect("Expected column 14 to be a float32!"),
            Expression: row
                .columns[10]
                .into_i32()
                .copied()
                .expect("Expected column 10 to be a int32!"),
            BannerTimeline: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            ImageRotation: row
                .columns[6]
                .into_i16()
                .copied()
                .expect("Expected column 6 to be a int16!"),
            DirectionalLightingVerticalAngle: row
                .columns[20]
                .into_i16()
                .copied()
                .expect("Expected column 20 to be a int16!"),
            DirectionalLightingHorizontalAngle: row
                .columns[21]
                .into_i16()
                .copied()
                .expect("Expected column 21 to be a int16!"),
            CameraZoom: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            BannerDesignPreset: row
                .columns[15]
                .into_u8()
                .copied()
                .expect("Expected column 15 to be a uint8!"),
            DirectionalLightingColorRed: row
                .columns[16]
                .into_u8()
                .copied()
                .expect("Expected column 16 to be a uint8!"),
            DirectionalLightingColorGreen: row
                .columns[17]
                .into_u8()
                .copied()
                .expect("Expected column 17 to be a uint8!"),
            DirectionalLightingColorBlue: row
                .columns[18]
                .into_u8()
                .copied()
                .expect("Expected column 18 to be a uint8!"),
            DirectionalLightingBrightness: row
                .columns[19]
                .into_u8()
                .copied()
                .expect("Expected column 19 to be a uint8!"),
            AmbientLightingColorRed: row
                .columns[22]
                .into_u8()
                .copied()
                .expect("Expected column 22 to be a uint8!"),
            AmbientLightingColorGreen: row
                .columns[23]
                .into_u8()
                .copied()
                .expect("Expected column 23 to be a uint8!"),
            AmbientLightingColorBlue: row
                .columns[24]
                .into_u8()
                .copied()
                .expect("Expected column 24 to be a uint8!"),
            AmbientLightingBrightness: row
                .columns[25]
                .into_u8()
                .copied()
                .expect("Expected column 25 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a BannerPresetSheet {
    type Item = (u32, Vec<(u16, BannerPresetRow)>);
    type IntoIter = StructuredSheetIterator<'a, BannerPresetSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BannerPresetSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BannerPresetRow {
    ///""
    pub CameraPositionX: f32,
    ///""
    pub CameraPositionY: f32,
    ///""
    pub CameraPositionZ: f32,
    ///""
    pub CameraTargetX: f32,
    ///""
    pub CameraTargetY: f32,
    ///""
    pub CameraTargetZ: f32,
    ///""
    pub AnimationProgress: f32,
    ///""
    pub HeadDirectionX: f32,
    ///""
    pub HeadDirectionY: f32,
    ///""
    pub EyeDirectionX: f32,
    ///""
    pub EyeDirectionY: f32,
    ///""
    pub Expression: i32,
    ///""
    pub BannerTimeline: u16,
    ///""
    pub ImageRotation: i16,
    ///""
    pub DirectionalLightingVerticalAngle: i16,
    ///""
    pub DirectionalLightingHorizontalAngle: i16,
    ///""
    pub CameraZoom: u8,
    ///""
    pub BannerDesignPreset: u8,
    ///""
    pub DirectionalLightingColorRed: u8,
    ///""
    pub DirectionalLightingColorGreen: u8,
    ///""
    pub DirectionalLightingColorBlue: u8,
    ///""
    pub DirectionalLightingBrightness: u8,
    ///""
    pub AmbientLightingColorRed: u8,
    ///""
    pub AmbientLightingColorGreen: u8,
    ///""
    pub AmbientLightingColorBlue: u8,
    ///""
    pub AmbientLightingBrightness: u8,
}
