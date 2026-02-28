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
    index_mapping: Vec<usize>,
}
impl RecipeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Recipe")?;
        let sheet = resolver.read_excel_sheet(&exh, "Recipe", language)?;
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
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
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
    index_mapping: Vec<usize>,
}
impl<'a> RecipeRow<'a> {
    pub fn RequiredQuality(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Quest(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Number(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn CraftType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn ItemResult(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn Ingredient(&'a self) -> [&'a Field; 8] {
        [
            &self.row.columns[self.index_mapping[5]],
            &self.row.columns[self.index_mapping[6]],
            &self.row.columns[self.index_mapping[7]],
            &self.row.columns[self.index_mapping[8]],
            &self.row.columns[self.index_mapping[9]],
            &self.row.columns[self.index_mapping[10]],
            &self.row.columns[self.index_mapping[11]],
            &self.row.columns[self.index_mapping[12]],
        ]
    }
    pub fn StatusRequired(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn ItemRequired(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn RecipeLevelTable(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn MaxAdjustableJobLevel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn RecipeNotebookList(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn DisplayPriority(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn DifficultyFactor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn QualityFactor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn DurabilityFactor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn RequiredCraftsmanship(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn RequiredControl(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn QuickSynthCraftsmanship(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn QuickSynthControl(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn SecretRecipeBook(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn CollectableMetadata(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn PatchNumber(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn AmountResult(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn AmountIngredient(&'a self) -> [&'a Field; 8] {
        [
            &self.row.columns[self.index_mapping[30]],
            &self.row.columns[self.index_mapping[31]],
            &self.row.columns[self.index_mapping[32]],
            &self.row.columns[self.index_mapping[33]],
            &self.row.columns[self.index_mapping[34]],
            &self.row.columns[self.index_mapping[35]],
            &self.row.columns[self.index_mapping[36]],
            &self.row.columns[self.index_mapping[37]],
        ]
    }
    pub fn MaterialQualityFactor(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn CollectableMetadataKey(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn IsSecondary(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn CanQuickSynth(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn CanHq(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn ExpRewarded(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    pub fn IsSpecializationRequired(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn IsExpert(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
}
