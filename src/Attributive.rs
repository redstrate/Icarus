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
pub struct AttributiveSheet {
    sheet: Sheet,
}
impl AttributiveSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 1u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Attributive")?;
        let sheet = resolver.read_excel_sheet(&exh, "Attributive", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<AttributiveRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<AttributiveRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for AttributiveSheet {
    type Row = AttributiveRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            JapaneseDemonstrative: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            JapaneseDemonstrativePlural: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            EnglishConsonantSingular: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            EnglishConsonantGeneric: row
                .columns[3]
                .into_string()
                .cloned()
                .expect("Expected column 3 to be a string!"),
            EnglishConsonantPlural: row
                .columns[4]
                .into_string()
                .cloned()
                .expect("Expected column 4 to be a string!"),
            EnglishVowelSingular: row
                .columns[5]
                .into_string()
                .cloned()
                .expect("Expected column 5 to be a string!"),
            EnglishVowelGeneric: row
                .columns[6]
                .into_string()
                .cloned()
                .expect("Expected column 6 to be a string!"),
            EnglishVowelPlural: row
                .columns[7]
                .into_string()
                .cloned()
                .expect("Expected column 7 to be a string!"),
            GermanNominativeMasculine: row
                .columns[8]
                .into_string()
                .cloned()
                .expect("Expected column 8 to be a string!"),
            GermanNominativeFeminine: row
                .columns[9]
                .into_string()
                .cloned()
                .expect("Expected column 9 to be a string!"),
            GermanNominativeNeutral: row
                .columns[10]
                .into_string()
                .cloned()
                .expect("Expected column 10 to be a string!"),
            GermanNominativePlural: row
                .columns[11]
                .into_string()
                .cloned()
                .expect("Expected column 11 to be a string!"),
            GermanGenitiveMasculine: row
                .columns[12]
                .into_string()
                .cloned()
                .expect("Expected column 12 to be a string!"),
            GermanGenitiveFeminine: row
                .columns[13]
                .into_string()
                .cloned()
                .expect("Expected column 13 to be a string!"),
            GermanGenitiveNeutral: row
                .columns[14]
                .into_string()
                .cloned()
                .expect("Expected column 14 to be a string!"),
            GermanGenitivePlural: row
                .columns[15]
                .into_string()
                .cloned()
                .expect("Expected column 15 to be a string!"),
            GermanDativeMasculine: row
                .columns[16]
                .into_string()
                .cloned()
                .expect("Expected column 16 to be a string!"),
            GermanDativeFeminine: row
                .columns[17]
                .into_string()
                .cloned()
                .expect("Expected column 17 to be a string!"),
            GermanDativeNeutral: row
                .columns[18]
                .into_string()
                .cloned()
                .expect("Expected column 18 to be a string!"),
            GermanDativePlural: row
                .columns[19]
                .into_string()
                .cloned()
                .expect("Expected column 19 to be a string!"),
            GermanAccusativeMasculine: row
                .columns[20]
                .into_string()
                .cloned()
                .expect("Expected column 20 to be a string!"),
            GermanAccusativeFeminine: row
                .columns[21]
                .into_string()
                .cloned()
                .expect("Expected column 21 to be a string!"),
            GermanAccusativeNeutral: row
                .columns[22]
                .into_string()
                .cloned()
                .expect("Expected column 22 to be a string!"),
            GermanAccusativePlural: row
                .columns[23]
                .into_string()
                .cloned()
                .expect("Expected column 23 to be a string!"),
            FrenchMasculineConsonantBase: row
                .columns[24]
                .into_string()
                .cloned()
                .expect("Expected column 24 to be a string!"),
            FrenchMasculineConsonantSingular: row
                .columns[25]
                .into_string()
                .cloned()
                .expect("Expected column 25 to be a string!"),
            FrenchMasculineConsonantPlural: row
                .columns[26]
                .into_string()
                .cloned()
                .expect("Expected column 26 to be a string!"),
            FrenchMasculineConsonantMass: row
                .columns[27]
                .into_string()
                .cloned()
                .expect("Expected column 27 to be a string!"),
            FrenchMasculineVowelBase: row
                .columns[28]
                .into_string()
                .cloned()
                .expect("Expected column 28 to be a string!"),
            FrenchMasculineVowelSingular: row
                .columns[29]
                .into_string()
                .cloned()
                .expect("Expected column 29 to be a string!"),
            FrenchMasculineVowelPlural: row
                .columns[30]
                .into_string()
                .cloned()
                .expect("Expected column 30 to be a string!"),
            FrenchMasculineVowelMass: row
                .columns[31]
                .into_string()
                .cloned()
                .expect("Expected column 31 to be a string!"),
            FrenchFeminineConsonantBase: row
                .columns[32]
                .into_string()
                .cloned()
                .expect("Expected column 32 to be a string!"),
            FrenchFeminineConsonantSingular: row
                .columns[33]
                .into_string()
                .cloned()
                .expect("Expected column 33 to be a string!"),
            FrenchFeminineConsonantPlural: row
                .columns[34]
                .into_string()
                .cloned()
                .expect("Expected column 34 to be a string!"),
            FrenchFeminineConsonantMass: row
                .columns[35]
                .into_string()
                .cloned()
                .expect("Expected column 35 to be a string!"),
            FrenchFeminineVowelBase: row
                .columns[36]
                .into_string()
                .cloned()
                .expect("Expected column 36 to be a string!"),
            FrenchFeminineVowelSingular: row
                .columns[37]
                .into_string()
                .cloned()
                .expect("Expected column 37 to be a string!"),
            FrenchFeminineVowelPlural: row
                .columns[38]
                .into_string()
                .cloned()
                .expect("Expected column 38 to be a string!"),
            FrenchFeminineVowelMass: row
                .columns[39]
                .into_string()
                .cloned()
                .expect("Expected column 39 to be a string!"),
            Unknown40: row
                .columns[40]
                .into_string()
                .cloned()
                .expect("Expected column 40 to be a string!"),
        })
    }
}
impl<'a> IntoIterator for &'a AttributiveSheet {
    type Item = (u32, Vec<(u16, AttributiveRow)>);
    type IntoIter = StructuredSheetIterator<'a, AttributiveSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AttributiveSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct AttributiveRow {
    ///""
    pub JapaneseDemonstrative: String,
    ///""
    pub JapaneseDemonstrativePlural: String,
    ///""
    pub EnglishConsonantSingular: String,
    ///""
    pub EnglishConsonantGeneric: String,
    ///""
    pub EnglishConsonantPlural: String,
    ///""
    pub EnglishVowelSingular: String,
    ///""
    pub EnglishVowelGeneric: String,
    ///""
    pub EnglishVowelPlural: String,
    ///""
    pub GermanNominativeMasculine: String,
    ///""
    pub GermanNominativeFeminine: String,
    ///""
    pub GermanNominativeNeutral: String,
    ///""
    pub GermanNominativePlural: String,
    ///""
    pub GermanGenitiveMasculine: String,
    ///""
    pub GermanGenitiveFeminine: String,
    ///""
    pub GermanGenitiveNeutral: String,
    ///""
    pub GermanGenitivePlural: String,
    ///""
    pub GermanDativeMasculine: String,
    ///""
    pub GermanDativeFeminine: String,
    ///""
    pub GermanDativeNeutral: String,
    ///""
    pub GermanDativePlural: String,
    ///""
    pub GermanAccusativeMasculine: String,
    ///""
    pub GermanAccusativeFeminine: String,
    ///""
    pub GermanAccusativeNeutral: String,
    ///""
    pub GermanAccusativePlural: String,
    ///""
    pub FrenchMasculineConsonantBase: String,
    ///""
    pub FrenchMasculineConsonantSingular: String,
    ///""
    pub FrenchMasculineConsonantPlural: String,
    ///""
    pub FrenchMasculineConsonantMass: String,
    ///""
    pub FrenchMasculineVowelBase: String,
    ///""
    pub FrenchMasculineVowelSingular: String,
    ///""
    pub FrenchMasculineVowelPlural: String,
    ///""
    pub FrenchMasculineVowelMass: String,
    ///""
    pub FrenchFeminineConsonantBase: String,
    ///""
    pub FrenchFeminineConsonantSingular: String,
    ///""
    pub FrenchFeminineConsonantPlural: String,
    ///""
    pub FrenchFeminineConsonantMass: String,
    ///""
    pub FrenchFeminineVowelBase: String,
    ///""
    pub FrenchFeminineVowelSingular: String,
    ///""
    pub FrenchFeminineVowelPlural: String,
    ///""
    pub FrenchFeminineVowelMass: String,
    ///""
    pub Unknown40: String,
}
