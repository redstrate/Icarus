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
pub struct RecipeSheet {
    sheet: Sheet,
}
impl RecipeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Recipe")?;
        let sheet = resolver.read_excel_sheet(&exh, "Recipe", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<RecipeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<RecipeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for RecipeSheet {
    type Row = RecipeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a RecipeSheet {
    type Item = (u32, Vec<(u16, RecipeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, RecipeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, RecipeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RecipeRow<'a> {
    row: &'a Row,
}
impl<'a> RecipeRow<'a> {
    pub fn RequiredQuality(&'a self) -> u32 {
        self.row.columns[29].into_u32().copied().unwrap()
    }
    pub fn Quest(&'a self) -> u32 {
        self.row.columns[35].into_u32().copied().unwrap()
    }
    pub fn Number(&'a self) -> i32 {
        self.row.columns[0].into_i32().copied().unwrap()
    }
    pub fn CraftType(&'a self) -> i32 {
        self.row.columns[1].into_i32().copied().unwrap()
    }
    pub fn ItemResult(&'a self) -> i32 {
        self.row.columns[4].into_i32().copied().unwrap()
    }
    pub fn Ingredient(&'a self) -> [i32; 8] {
        [
            self.row.columns[6].into_i32().copied().unwrap(),
            self.row.columns[8].into_i32().copied().unwrap(),
            self.row.columns[10].into_i32().copied().unwrap(),
            self.row.columns[12].into_i32().copied().unwrap(),
            self.row.columns[14].into_i32().copied().unwrap(),
            self.row.columns[16].into_i32().copied().unwrap(),
            self.row.columns[18].into_i32().copied().unwrap(),
            self.row.columns[20].into_i32().copied().unwrap(),
        ]
    }
    pub fn StatusRequired(&'a self) -> i32 {
        self.row.columns[40].into_i32().copied().unwrap()
    }
    pub fn ItemRequired(&'a self) -> i32 {
        self.row.columns[41].into_i32().copied().unwrap()
    }
    pub fn RecipeLevelTable(&'a self) -> u16 {
        self.row.columns[2].into_u16().copied().unwrap()
    }
    pub fn MaxAdjustableJobLevel(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn RecipeNotebookList(&'a self) -> u16 {
        self.row.columns[22].into_u16().copied().unwrap()
    }
    pub fn DisplayPriority(&'a self) -> u16 {
        self.row.columns[23].into_u16().copied().unwrap()
    }
    pub fn DifficultyFactor(&'a self) -> u16 {
        self.row.columns[26].into_u16().copied().unwrap()
    }
    pub fn QualityFactor(&'a self) -> u16 {
        self.row.columns[27].into_u16().copied().unwrap()
    }
    pub fn DurabilityFactor(&'a self) -> u16 {
        self.row.columns[28].into_u16().copied().unwrap()
    }
    pub fn RequiredCraftsmanship(&'a self) -> u16 {
        self.row.columns[30].into_u16().copied().unwrap()
    }
    pub fn RequiredControl(&'a self) -> u16 {
        self.row.columns[31].into_u16().copied().unwrap()
    }
    pub fn QuickSynthCraftsmanship(&'a self) -> u16 {
        self.row.columns[32].into_u16().copied().unwrap()
    }
    pub fn QuickSynthControl(&'a self) -> u16 {
        self.row.columns[33].into_u16().copied().unwrap()
    }
    pub fn SecretRecipeBook(&'a self) -> u16 {
        self.row.columns[34].into_u16().copied().unwrap()
    }
    pub fn CollectableMetadata(&'a self) -> u16 {
        self.row.columns[45].into_u16().copied().unwrap()
    }
    pub fn PatchNumber(&'a self) -> u16 {
        self.row.columns[46].into_u16().copied().unwrap()
    }
    pub fn AmountResult(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn AmountIngredient(&'a self) -> [u8; 8] {
        [
            self.row.columns[7].into_u8().copied().unwrap(),
            self.row.columns[9].into_u8().copied().unwrap(),
            self.row.columns[11].into_u8().copied().unwrap(),
            self.row.columns[13].into_u8().copied().unwrap(),
            self.row.columns[15].into_u8().copied().unwrap(),
            self.row.columns[17].into_u8().copied().unwrap(),
            self.row.columns[19].into_u8().copied().unwrap(),
            self.row.columns[21].into_u8().copied().unwrap(),
        ]
    }
    pub fn MaterialQualityFactor(&'a self) -> u8 {
        self.row.columns[25].into_u8().copied().unwrap()
    }
    pub fn CollectableMetadataKey(&'a self) -> u8 {
        self.row.columns[44].into_u8().copied().unwrap()
    }
    pub fn IsSecondary(&'a self) -> bool {
        self.row.columns[24].into_bool().copied().unwrap()
    }
    pub fn CanQuickSynth(&'a self) -> bool {
        self.row.columns[36].into_bool().copied().unwrap()
    }
    pub fn CanHq(&'a self) -> bool {
        self.row.columns[37].into_bool().copied().unwrap()
    }
    pub fn ExpRewarded(&'a self) -> bool {
        self.row.columns[38].into_bool().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> bool {
        self.row.columns[39].into_bool().copied().unwrap()
    }
    pub fn IsSpecializationRequired(&'a self) -> bool {
        self.row.columns[42].into_bool().copied().unwrap()
    }
    pub fn IsExpert(&'a self) -> bool {
        self.row.columns[43].into_bool().copied().unwrap()
    }
}
