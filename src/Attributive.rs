//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct AttributiveSheet {
    sheet: ExcelSheet,
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
    fn read_row(&self, row: &ExcelSingleRow) -> Option<AttributiveRow> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(AttributiveRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<AttributiveRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<AttributiveRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => return None,
            ExcelRowKind::SubRows(subrows) => {
                &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
            }
        };
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
pub struct AttributiveRow {
    columns: Vec<ColumnData>,
}
impl AttributiveRow {
    pub fn JapaneseSingularDemonstrative<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn JapanesePluralDemonstrative<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn EnglishArticleSingularConsonant<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn EnglishArticleGenericConsonant<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn EnglishArticlePluralConsonant<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn EnglishArticleSingularVowel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn EnglishArticleGenericVowel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn EnglishArticlePluralVowel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn GermanNominativeMasculine<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn GermanNominativeFeminine<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn GermanNominativeNeutral<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn GermanNominativePlural<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn GermanGenitiveMasculine<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn GermanGenitiveFeminine<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn GermanGenitiveNeutral<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn GermanGenitivePlural<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn GermanDativeMasculine<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn GermanDativeFeminine<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn GermanDativeNeutral<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn GermanDativePlural<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn GermanAccusativeMasculine<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn GermanAccusativeFeminine<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn GermanAccusativeNeutral<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn GermanAccusativePlural<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn FrenchArticleSingular<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn FrenchArticleSingularMasculine<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn FrenchArticlePluralMasculine<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn FrenchArticleSingularMasculineElided<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn FrenchArticlePluralMasculineElided<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn FrenchArticleSingularElided<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn FrenchArticlePluralElided<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn FrenchArticleSingularMasculineContracted<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn FrenchArticlePluralMasculineContracted<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn FrenchArticleSingularFeminine<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn FrenchArticlePluralFeminine<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn FrenchArticleSingularFeminineElided<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn FrenchArticlePluralFeminineElided<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn FrenchArticleSingularElidedAlt<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn FrenchArticlePluralElidedAlt<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn FrenchArticleSingularNeutral<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn FrenchArticlePluralNeutral<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
}
