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
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
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
impl StructuredSheet for BNpcCustomizeSheet {
    type Row = BNpcCustomizeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Race: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            Gender: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            BodyType: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Height: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Tribe: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Face: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            HairStyle: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            HairHighlight: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            SkinColor: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            EyeHeterochromia: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            HairColor: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            HairHighlightColor: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            FacialFeature: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            FacialFeatureColor: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            Eyebrows: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
            EyeColor: row
                .columns[15]
                .into_u8()
                .copied()
                .expect("Expected column 15 to be a uint8!"),
            EyeShape: row
                .columns[16]
                .into_u8()
                .copied()
                .expect("Expected column 16 to be a uint8!"),
            Nose: row
                .columns[17]
                .into_u8()
                .copied()
                .expect("Expected column 17 to be a uint8!"),
            Jaw: row
                .columns[18]
                .into_u8()
                .copied()
                .expect("Expected column 18 to be a uint8!"),
            Mouth: row
                .columns[19]
                .into_u8()
                .copied()
                .expect("Expected column 19 to be a uint8!"),
            LipColor: row
                .columns[20]
                .into_u8()
                .copied()
                .expect("Expected column 20 to be a uint8!"),
            BustOrTone1: row
                .columns[21]
                .into_u8()
                .copied()
                .expect("Expected column 21 to be a uint8!"),
            ExtraFeature1: row
                .columns[22]
                .into_u8()
                .copied()
                .expect("Expected column 22 to be a uint8!"),
            ExtraFeature2OrBust: row
                .columns[23]
                .into_u8()
                .copied()
                .expect("Expected column 23 to be a uint8!"),
            FacePaint: row
                .columns[24]
                .into_u8()
                .copied()
                .expect("Expected column 24 to be a uint8!"),
            FacePaintColor: row
                .columns[25]
                .into_u8()
                .copied()
                .expect("Expected column 25 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a BNpcCustomizeSheet {
    type Item = (u32, Vec<(u16, BNpcCustomizeRow)>);
    type IntoIter = StructuredSheetIterator<'a, BNpcCustomizeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BNpcCustomizeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BNpcCustomizeRow {
    ///""
    pub Race: u8,
    ///""
    pub Gender: u8,
    ///""
    pub BodyType: u8,
    ///""
    pub Height: u8,
    ///""
    pub Tribe: u8,
    ///""
    pub Face: u8,
    ///""
    pub HairStyle: u8,
    ///""
    pub HairHighlight: u8,
    ///""
    pub SkinColor: u8,
    ///""
    pub EyeHeterochromia: u8,
    ///""
    pub HairColor: u8,
    ///""
    pub HairHighlightColor: u8,
    ///""
    pub FacialFeature: u8,
    ///""
    pub FacialFeatureColor: u8,
    ///""
    pub Eyebrows: u8,
    ///""
    pub EyeColor: u8,
    ///""
    pub EyeShape: u8,
    ///""
    pub Nose: u8,
    ///""
    pub Jaw: u8,
    ///""
    pub Mouth: u8,
    ///""
    pub LipColor: u8,
    ///""
    pub BustOrTone1: u8,
    ///""
    pub ExtraFeature1: u8,
    ///""
    pub ExtraFeature2OrBust: u8,
    ///""
    pub FacePaint: u8,
    ///""
    pub FacePaintColor: u8,
}
