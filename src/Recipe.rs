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
impl StructuredSheet for RecipeSheet {
    type Row = RecipeRow;
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
#[derive(Debug, Clone)]
pub struct RecipeRow {
    columns: Vec<Field>,
}
impl RecipeRow {
    pub fn RequiredQuality<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Quest<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn Number<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn CraftType<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn ItemResult<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn Ingredient<'a>(&'a self) -> [&'a Field; 8] {
        [
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
            &self.columns[9],
            &self.columns[10],
            &self.columns[11],
            &self.columns[12],
        ]
    }
    pub fn StatusRequired<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn ItemRequired<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn RecipeLevelTable<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn MaxAdjustableJobLevel<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn RecipeNotebookList<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn DisplayPriority<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn DifficultyFactor<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn QualityFactor<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn DurabilityFactor<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn RequiredCraftsmanship<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn RequiredControl<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn QuickSynthCraftsmanship<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn QuickSynthControl<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
    pub fn SecretRecipeBook<'a>(&'a self) -> &'a Field {
        &self.columns[26]
    }
    pub fn CollectableMetadata<'a>(&'a self) -> &'a Field {
        &self.columns[27]
    }
    pub fn PatchNumber<'a>(&'a self) -> &'a Field {
        &self.columns[28]
    }
    pub fn AmountResult<'a>(&'a self) -> &'a Field {
        &self.columns[29]
    }
    pub fn AmountIngredient<'a>(&'a self) -> [&'a Field; 8] {
        [
            &self.columns[30],
            &self.columns[31],
            &self.columns[32],
            &self.columns[33],
            &self.columns[34],
            &self.columns[35],
            &self.columns[36],
            &self.columns[37],
        ]
    }
    pub fn MaterialQualityFactor<'a>(&'a self) -> &'a Field {
        &self.columns[38]
    }
    pub fn CollectableMetadataKey<'a>(&'a self) -> &'a Field {
        &self.columns[39]
    }
    pub fn IsSecondary<'a>(&'a self) -> &'a Field {
        &self.columns[40]
    }
    pub fn CanQuickSynth<'a>(&'a self) -> &'a Field {
        &self.columns[41]
    }
    pub fn CanHq<'a>(&'a self) -> &'a Field {
        &self.columns[42]
    }
    pub fn ExpRewarded<'a>(&'a self) -> &'a Field {
        &self.columns[43]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[44]
    }
    pub fn IsSpecializationRequired<'a>(&'a self) -> &'a Field {
        &self.columns[45]
    }
    pub fn IsExpert<'a>(&'a self) -> &'a Field {
        &self.columns[46]
    }
}
