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
pub struct BeastTribeSheet {
    sheet: Sheet,
}
impl BeastTribeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BeastTribe")?;
        let sheet = resolver.read_excel_sheet(&exh, "BeastTribe", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BeastTribeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BeastTribeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BeastTribeSheet {
    type Row = BeastTribeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a BeastTribeSheet {
    type Item = (u32, Vec<(u16, BeastTribeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BeastTribeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BeastTribeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BeastTribeRow<'a> {
    row: &'a Row,
}
impl<'a> BeastTribeRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[11].into_string().unwrap()
    }
    pub fn Plural(&'a self) -> &'a str {
        self.row.columns[13].into_string().unwrap()
    }
    pub fn NameRelation(&'a self) -> &'a str {
        self.row.columns[19].into_string().unwrap()
    }
    pub fn Adjective(&'a self) -> i8 {
        self.row.columns[12].into_i8().copied().unwrap()
    }
    pub fn PossessivePronoun(&'a self) -> i8 {
        self.row.columns[14].into_i8().copied().unwrap()
    }
    pub fn StartsWithVowel(&'a self) -> i8 {
        self.row.columns[15].into_i8().copied().unwrap()
    }
    pub fn Pronoun(&'a self) -> i8 {
        self.row.columns[16].into_i8().copied().unwrap()
    }
    pub fn Article(&'a self) -> i8 {
        self.row.columns[17].into_i8().copied().unwrap()
    }
    pub fn DEF(&'a self) -> i8 {
        self.row.columns[18].into_i8().copied().unwrap()
    }
    pub fn IconReputation(&'a self) -> u32 {
        self.row.columns[3].into_u32().copied().unwrap()
    }
    pub fn Icon(&'a self) -> u32 {
        self.row.columns[4].into_u32().copied().unwrap()
    }
    pub fn IntersocietalQuest(&'a self) -> u32 {
        self.row.columns[7].into_u32().copied().unwrap()
    }
    pub fn Level(&'a self) -> u32 {
        self.row.columns[8].into_u32().copied().unwrap()
    }
    pub fn CurrencyItem(&'a self) -> u32 {
        self.row.columns[9].into_u32().copied().unwrap()
    }
    pub fn MinLevel(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn BeastRankBonus(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn MaxRank(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Expansion(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn DisplayOrder(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> bool {
        self.row.columns[0].into_bool().copied().unwrap()
    }
}
