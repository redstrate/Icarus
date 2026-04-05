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
pub struct HousingPresetSheet {
    sheet: Sheet,
}
impl HousingPresetSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HousingPreset")?;
        let sheet = resolver.read_excel_sheet(&exh, "HousingPreset", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<HousingPresetRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<HousingPresetRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for HousingPresetSheet {
    type Row = HousingPresetRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a HousingPresetSheet {
    type Item = (u32, Vec<(u16, HousingPresetRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, HousingPresetSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HousingPresetSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HousingPresetRow<'a> {
    row: &'a Row,
}
impl<'a> HousingPresetRow<'a> {
    pub fn Singular(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Plural(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn Adjective(&'a self) -> i8 {
        self.row.columns[1].into_i8().copied().unwrap()
    }
    pub fn PossessivePronoun(&'a self) -> i8 {
        self.row.columns[3].into_i8().copied().unwrap()
    }
    pub fn StartsWithVowel(&'a self) -> i8 {
        self.row.columns[4].into_i8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> i8 {
        self.row.columns[5].into_i8().copied().unwrap()
    }
    pub fn Pronoun(&'a self) -> i8 {
        self.row.columns[6].into_i8().copied().unwrap()
    }
    pub fn Article(&'a self) -> i8 {
        self.row.columns[7].into_i8().copied().unwrap()
    }
    pub fn ExteriorRoof(&'a self) -> i32 {
        self.row.columns[10].into_i32().copied().unwrap()
    }
    pub fn ExteriorWall(&'a self) -> i32 {
        self.row.columns[11].into_i32().copied().unwrap()
    }
    pub fn ExteriorWindow(&'a self) -> i32 {
        self.row.columns[12].into_i32().copied().unwrap()
    }
    pub fn ExteriorDoor(&'a self) -> i32 {
        self.row.columns[13].into_i32().copied().unwrap()
    }
    pub fn InteriorWall(&'a self) -> i32 {
        self.row.columns[14].into_i32().copied().unwrap()
    }
    pub fn InteriorFlooring(&'a self) -> i32 {
        self.row.columns[15].into_i32().copied().unwrap()
    }
    pub fn InteriorLighting(&'a self) -> i32 {
        self.row.columns[16].into_i32().copied().unwrap()
    }
    pub fn OtherFloorWall(&'a self) -> i32 {
        self.row.columns[17].into_i32().copied().unwrap()
    }
    pub fn OtherFloorFlooring(&'a self) -> i32 {
        self.row.columns[18].into_i32().copied().unwrap()
    }
    pub fn OtherFloorLighting(&'a self) -> i32 {
        self.row.columns[19].into_i32().copied().unwrap()
    }
    pub fn BasementWall(&'a self) -> i32 {
        self.row.columns[20].into_i32().copied().unwrap()
    }
    pub fn BasementFlooring(&'a self) -> i32 {
        self.row.columns[21].into_i32().copied().unwrap()
    }
    pub fn BasementLighting(&'a self) -> i32 {
        self.row.columns[22].into_i32().copied().unwrap()
    }
    pub fn MansionLighting(&'a self) -> i32 {
        self.row.columns[23].into_i32().copied().unwrap()
    }
    pub fn PlaceName(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn HousingSize(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
}
