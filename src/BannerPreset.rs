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
    index_mapping: Vec<usize>,
}
impl BannerPresetSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BannerPreset")?;
        let sheet = resolver.read_excel_sheet(&exh, "BannerPreset", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
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
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
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
    index_mapping: Vec<usize>,
}
impl<'a> BannerPresetRow<'a> {
    pub fn CameraPositionX(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn CameraPositionY(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn CameraPositionZ(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn CameraTargetX(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn CameraTargetY(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn CameraTargetZ(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn AnimationProgress(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn HeadDirectionX(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn HeadDirectionY(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn EyeDirectionX(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn EyeDirectionY(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn Expression(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn BannerTimeline(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn ImageRotation(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn DirectionalLightingVerticalAngle(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn DirectionalLightingHorizontalAngle(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn CameraZoom(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn BannerDesignPreset(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn DirectionalLightingColorRed(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn DirectionalLightingColorGreen(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn DirectionalLightingColorBlue(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn DirectionalLightingBrightness(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn AmbientLightingColorRed(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn AmbientLightingColorGreen(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn AmbientLightingColorBlue(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn AmbientLightingBrightness(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
}
