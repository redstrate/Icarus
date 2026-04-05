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
pub struct BNpcCustomizeSheet {
    sheet: Sheet,
}
impl BNpcCustomizeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BNpcCustomize")?;
        let sheet = resolver.read_excel_sheet(&exh, "BNpcCustomize", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BNpcCustomizeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BNpcCustomizeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BNpcCustomizeSheet {
    type Row = BNpcCustomizeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BNpcCustomizeSheet {
    type Item = (u32, Vec<(u16, BNpcCustomizeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BNpcCustomizeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BNpcCustomizeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BNpcCustomizeRow<'a> {
    row: &'a Row,
}
impl<'a> BNpcCustomizeRow<'a> {
    pub fn Race(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn Gender(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn BodyType(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn Height(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn Tribe(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    pub fn Face(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn HairStyle(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn HairHighlight(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn SkinColor(&'a self) -> u8 {
        self.row.columns[8].into_u8().copied().unwrap()
    }
    pub fn EyeHeterochromia(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn HairColor(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
    pub fn HairHighlightColor(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
    pub fn FacialFeature(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn FacialFeatureColor(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn Eyebrows(&'a self) -> u8 {
        self.row.columns[14].into_u8().copied().unwrap()
    }
    pub fn EyeColor(&'a self) -> u8 {
        self.row.columns[15].into_u8().copied().unwrap()
    }
    pub fn EyeShape(&'a self) -> u8 {
        self.row.columns[16].into_u8().copied().unwrap()
    }
    pub fn Nose(&'a self) -> u8 {
        self.row.columns[17].into_u8().copied().unwrap()
    }
    pub fn Jaw(&'a self) -> u8 {
        self.row.columns[18].into_u8().copied().unwrap()
    }
    pub fn Mouth(&'a self) -> u8 {
        self.row.columns[19].into_u8().copied().unwrap()
    }
    pub fn LipColor(&'a self) -> u8 {
        self.row.columns[20].into_u8().copied().unwrap()
    }
    pub fn BustOrTone1(&'a self) -> u8 {
        self.row.columns[21].into_u8().copied().unwrap()
    }
    pub fn ExtraFeature1(&'a self) -> u8 {
        self.row.columns[22].into_u8().copied().unwrap()
    }
    pub fn ExtraFeature2OrBust(&'a self) -> u8 {
        self.row.columns[23].into_u8().copied().unwrap()
    }
    pub fn FacePaint(&'a self) -> u8 {
        self.row.columns[24].into_u8().copied().unwrap()
    }
    pub fn FacePaintColor(&'a self) -> u8 {
        self.row.columns[25].into_u8().copied().unwrap()
    }
}
