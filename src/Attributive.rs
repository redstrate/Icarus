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
impl<'a> StructuredSheet<'a> for AttributiveSheet {
    type Row = AttributiveRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a AttributiveSheet {
    type Item = (u32, Vec<(u16, AttributiveRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, AttributiveSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, AttributiveSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AttributiveRow<'a> {
    row: &'a Row,
}
impl<'a> AttributiveRow<'a> {
    pub fn JapaneseSingularDemonstrative(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn JapanesePluralDemonstrative(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn EnglishArticleSingularConsonant(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn EnglishArticleGenericConsonant(&'a self) -> &'a str {
        self.row.columns[3].into_string().unwrap()
    }
    pub fn EnglishArticlePluralConsonant(&'a self) -> &'a str {
        self.row.columns[4].into_string().unwrap()
    }
    pub fn EnglishArticleSingularVowel(&'a self) -> &'a str {
        self.row.columns[5].into_string().unwrap()
    }
    pub fn EnglishArticleGenericVowel(&'a self) -> &'a str {
        self.row.columns[6].into_string().unwrap()
    }
    pub fn EnglishArticlePluralVowel(&'a self) -> &'a str {
        self.row.columns[7].into_string().unwrap()
    }
    pub fn GermanNominativeMasculine(&'a self) -> &'a str {
        self.row.columns[8].into_string().unwrap()
    }
    pub fn GermanNominativeFeminine(&'a self) -> &'a str {
        self.row.columns[9].into_string().unwrap()
    }
    pub fn GermanNominativeNeutral(&'a self) -> &'a str {
        self.row.columns[10].into_string().unwrap()
    }
    pub fn GermanNominativePlural(&'a self) -> &'a str {
        self.row.columns[11].into_string().unwrap()
    }
    pub fn GermanGenitiveMasculine(&'a self) -> &'a str {
        self.row.columns[12].into_string().unwrap()
    }
    pub fn GermanGenitiveFeminine(&'a self) -> &'a str {
        self.row.columns[13].into_string().unwrap()
    }
    pub fn GermanGenitiveNeutral(&'a self) -> &'a str {
        self.row.columns[14].into_string().unwrap()
    }
    pub fn GermanGenitivePlural(&'a self) -> &'a str {
        self.row.columns[15].into_string().unwrap()
    }
    pub fn GermanDativeMasculine(&'a self) -> &'a str {
        self.row.columns[16].into_string().unwrap()
    }
    pub fn GermanDativeFeminine(&'a self) -> &'a str {
        self.row.columns[17].into_string().unwrap()
    }
    pub fn GermanDativeNeutral(&'a self) -> &'a str {
        self.row.columns[18].into_string().unwrap()
    }
    pub fn GermanDativePlural(&'a self) -> &'a str {
        self.row.columns[19].into_string().unwrap()
    }
    pub fn GermanAccusativeMasculine(&'a self) -> &'a str {
        self.row.columns[20].into_string().unwrap()
    }
    pub fn GermanAccusativeFeminine(&'a self) -> &'a str {
        self.row.columns[21].into_string().unwrap()
    }
    pub fn GermanAccusativeNeutral(&'a self) -> &'a str {
        self.row.columns[22].into_string().unwrap()
    }
    pub fn GermanAccusativePlural(&'a self) -> &'a str {
        self.row.columns[23].into_string().unwrap()
    }
    pub fn FrenchArticleSingular(&'a self) -> &'a str {
        self.row.columns[24].into_string().unwrap()
    }
    pub fn FrenchArticleSingularMasculine(&'a self) -> &'a str {
        self.row.columns[25].into_string().unwrap()
    }
    pub fn FrenchArticlePluralMasculine(&'a self) -> &'a str {
        self.row.columns[26].into_string().unwrap()
    }
    pub fn FrenchArticleSingularMasculineElided(&'a self) -> &'a str {
        self.row.columns[27].into_string().unwrap()
    }
    pub fn FrenchArticlePluralMasculineElided(&'a self) -> &'a str {
        self.row.columns[28].into_string().unwrap()
    }
    pub fn FrenchArticleSingularElided(&'a self) -> &'a str {
        self.row.columns[29].into_string().unwrap()
    }
    pub fn FrenchArticlePluralElided(&'a self) -> &'a str {
        self.row.columns[30].into_string().unwrap()
    }
    pub fn FrenchArticleSingularMasculineContracted(&'a self) -> &'a str {
        self.row.columns[31].into_string().unwrap()
    }
    pub fn FrenchArticlePluralMasculineContracted(&'a self) -> &'a str {
        self.row.columns[32].into_string().unwrap()
    }
    pub fn FrenchArticleSingularFeminine(&'a self) -> &'a str {
        self.row.columns[33].into_string().unwrap()
    }
    pub fn FrenchArticlePluralFeminine(&'a self) -> &'a str {
        self.row.columns[34].into_string().unwrap()
    }
    pub fn FrenchArticleSingularFeminineElided(&'a self) -> &'a str {
        self.row.columns[35].into_string().unwrap()
    }
    pub fn FrenchArticlePluralFeminineElided(&'a self) -> &'a str {
        self.row.columns[36].into_string().unwrap()
    }
    pub fn FrenchArticleSingularElidedAlt(&'a self) -> &'a str {
        self.row.columns[37].into_string().unwrap()
    }
    pub fn FrenchArticlePluralElidedAlt(&'a self) -> &'a str {
        self.row.columns[38].into_string().unwrap()
    }
    pub fn FrenchArticleSingularNeutral(&'a self) -> &'a str {
        self.row.columns[39].into_string().unwrap()
    }
    pub fn FrenchArticlePluralNeutral(&'a self) -> &'a str {
        self.row.columns[40].into_string().unwrap()
    }
}
