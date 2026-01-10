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
impl StructuredSheet for BannerPresetSheet {
    type Row = BannerPresetRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
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
#[derive(Debug, Clone)]
pub struct BannerPresetRow {
    columns: Vec<Field>,
}
impl BannerPresetRow {
    pub fn CameraPositionX<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn CameraPositionY<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn CameraPositionZ<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn CameraTargetX<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn CameraTargetY<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn CameraTargetZ<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn AnimationProgress<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn HeadDirectionX<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn HeadDirectionY<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn EyeDirectionX<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn EyeDirectionY<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn Expression<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn BannerTimeline<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn ImageRotation<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn DirectionalLightingVerticalAngle<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn DirectionalLightingHorizontalAngle<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn CameraZoom<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn BannerDesignPreset<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn DirectionalLightingColorRed<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn DirectionalLightingColorGreen<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn DirectionalLightingColorBlue<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn DirectionalLightingBrightness<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn AmbientLightingColorRed<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn AmbientLightingColorGreen<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn AmbientLightingColorBlue<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn AmbientLightingBrightness<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
}
