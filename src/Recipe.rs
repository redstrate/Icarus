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
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
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
impl StructuredSheet for RecipeSheet {
    type Row = RecipeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            RequiredQuality: row
                .columns[29]
                .into_u32()
                .copied()
                .expect("Expected column 29 to be a uint32!"),
            Quest: row
                .columns[35]
                .into_u32()
                .copied()
                .expect("Expected column 35 to be a uint32!"),
            Number: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            CraftType: row
                .columns[1]
                .into_i32()
                .copied()
                .expect("Expected column 1 to be a int32!"),
            ItemResult: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            Ingredient: [
                row
                    .columns[6]
                    .into_i32()
                    .copied()
                    .expect("Expected column 6 to be a int32!"),
                row
                    .columns[8]
                    .into_i32()
                    .copied()
                    .expect("Expected column 8 to be a int32!"),
                row
                    .columns[10]
                    .into_i32()
                    .copied()
                    .expect("Expected column 10 to be a int32!"),
                row
                    .columns[12]
                    .into_i32()
                    .copied()
                    .expect("Expected column 12 to be a int32!"),
                row
                    .columns[14]
                    .into_i32()
                    .copied()
                    .expect("Expected column 14 to be a int32!"),
                row
                    .columns[16]
                    .into_i32()
                    .copied()
                    .expect("Expected column 16 to be a int32!"),
                row
                    .columns[18]
                    .into_i32()
                    .copied()
                    .expect("Expected column 18 to be a int32!"),
                row
                    .columns[20]
                    .into_i32()
                    .copied()
                    .expect("Expected column 20 to be a int32!"),
            ],
            StatusRequired: row
                .columns[40]
                .into_i32()
                .copied()
                .expect("Expected column 40 to be a int32!"),
            ItemRequired: row
                .columns[41]
                .into_i32()
                .copied()
                .expect("Expected column 41 to be a int32!"),
            RecipeLevelTable: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            MaxAdjustableJobLevel: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            RecipeNotebookList: row
                .columns[22]
                .into_u16()
                .copied()
                .expect("Expected column 22 to be a uint16!"),
            DisplayPriority: row
                .columns[23]
                .into_u16()
                .copied()
                .expect("Expected column 23 to be a uint16!"),
            DifficultyFactor: row
                .columns[26]
                .into_u16()
                .copied()
                .expect("Expected column 26 to be a uint16!"),
            QualityFactor: row
                .columns[27]
                .into_u16()
                .copied()
                .expect("Expected column 27 to be a uint16!"),
            DurabilityFactor: row
                .columns[28]
                .into_u16()
                .copied()
                .expect("Expected column 28 to be a uint16!"),
            RequiredCraftsmanship: row
                .columns[30]
                .into_u16()
                .copied()
                .expect("Expected column 30 to be a uint16!"),
            RequiredControl: row
                .columns[31]
                .into_u16()
                .copied()
                .expect("Expected column 31 to be a uint16!"),
            QuickSynthCraftsmanship: row
                .columns[32]
                .into_u16()
                .copied()
                .expect("Expected column 32 to be a uint16!"),
            QuickSynthControl: row
                .columns[33]
                .into_u16()
                .copied()
                .expect("Expected column 33 to be a uint16!"),
            SecretRecipeBook: row
                .columns[34]
                .into_u16()
                .copied()
                .expect("Expected column 34 to be a uint16!"),
            CollectableMetadata: row
                .columns[45]
                .into_u16()
                .copied()
                .expect("Expected column 45 to be a uint16!"),
            PatchNumber: row
                .columns[46]
                .into_u16()
                .copied()
                .expect("Expected column 46 to be a uint16!"),
            AmountResult: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            AmountIngredient: [
                row
                    .columns[7]
                    .into_u8()
                    .copied()
                    .expect("Expected column 7 to be a uint8!"),
                row
                    .columns[9]
                    .into_u8()
                    .copied()
                    .expect("Expected column 9 to be a uint8!"),
                row
                    .columns[11]
                    .into_u8()
                    .copied()
                    .expect("Expected column 11 to be a uint8!"),
                row
                    .columns[13]
                    .into_u8()
                    .copied()
                    .expect("Expected column 13 to be a uint8!"),
                row
                    .columns[15]
                    .into_u8()
                    .copied()
                    .expect("Expected column 15 to be a uint8!"),
                row
                    .columns[17]
                    .into_u8()
                    .copied()
                    .expect("Expected column 17 to be a uint8!"),
                row
                    .columns[19]
                    .into_u8()
                    .copied()
                    .expect("Expected column 19 to be a uint8!"),
                row
                    .columns[21]
                    .into_u8()
                    .copied()
                    .expect("Expected column 21 to be a uint8!"),
            ],
            MaterialQualityFactor: row
                .columns[25]
                .into_u8()
                .copied()
                .expect("Expected column 25 to be a uint8!"),
            CollectableMetadataKey: row
                .columns[44]
                .into_u8()
                .copied()
                .expect("Expected column 44 to be a uint8!"),
            IsSecondary: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
            CanQuickSynth: row
                .columns[36]
                .into_bool()
                .copied()
                .expect("Expected column 36 to be a bool!"),
            CanHq: row
                .columns[37]
                .into_bool()
                .copied()
                .expect("Expected column 37 to be a bool!"),
            ExpRewarded: row
                .columns[38]
                .into_bool()
                .copied()
                .expect("Expected column 38 to be a bool!"),
            Unknown1: row
                .columns[39]
                .into_bool()
                .copied()
                .expect("Expected column 39 to be a bool!"),
            IsSpecializationRequired: row
                .columns[42]
                .into_bool()
                .copied()
                .expect("Expected column 42 to be a bool!"),
            IsExpert: row
                .columns[43]
                .into_bool()
                .copied()
                .expect("Expected column 43 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a RecipeSheet {
    type Item = (u32, Vec<(u16, RecipeRow)>);
    type IntoIter = StructuredSheetIterator<'a, RecipeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, RecipeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct RecipeRow {
    ///""
    pub RequiredQuality: u32,
    ///""
    pub Quest: u32,
    ///""
    pub Number: i32,
    ///""
    pub CraftType: i32,
    ///""
    pub ItemResult: i32,
    ///""
    pub Ingredient: [i32; 8],
    ///""
    pub StatusRequired: i32,
    ///""
    pub ItemRequired: i32,
    ///""
    pub RecipeLevelTable: u16,
    ///""
    pub MaxAdjustableJobLevel: u16,
    ///""
    pub RecipeNotebookList: u16,
    ///""
    pub DisplayPriority: u16,
    ///""
    pub DifficultyFactor: u16,
    ///""
    pub QualityFactor: u16,
    ///""
    pub DurabilityFactor: u16,
    ///""
    pub RequiredCraftsmanship: u16,
    ///""
    pub RequiredControl: u16,
    ///""
    pub QuickSynthCraftsmanship: u16,
    ///""
    pub QuickSynthControl: u16,
    ///""
    pub SecretRecipeBook: u16,
    ///""
    pub CollectableMetadata: u16,
    ///""
    pub PatchNumber: u16,
    ///""
    pub AmountResult: u8,
    ///""
    pub AmountIngredient: [u8; 8],
    ///""
    pub MaterialQualityFactor: u8,
    ///""
    pub CollectableMetadataKey: u8,
    ///""
    pub IsSecondary: bool,
    ///""
    pub CanQuickSynth: bool,
    ///""
    pub CanHq: bool,
    ///""
    pub ExpRewarded: bool,
    ///""
    pub Unknown1: bool,
    ///""
    pub IsSpecializationRequired: bool,
    ///""
    pub IsExpert: bool,
}
