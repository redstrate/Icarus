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
impl StructuredSheet for BeastTribeSheet {
    type Row = BeastTribeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[11]
                .into_string()
                .cloned()
                .expect("Expected column 11 to be a string!"),
            Plural: row
                .columns[13]
                .into_string()
                .cloned()
                .expect("Expected column 13 to be a string!"),
            NameRelation: row
                .columns[19]
                .into_string()
                .cloned()
                .expect("Expected column 19 to be a string!"),
            Adjective: row
                .columns[12]
                .into_i8()
                .copied()
                .expect("Expected column 12 to be a int8!"),
            PossessivePronoun: row
                .columns[14]
                .into_i8()
                .copied()
                .expect("Expected column 14 to be a int8!"),
            StartsWithVowel: row
                .columns[15]
                .into_i8()
                .copied()
                .expect("Expected column 15 to be a int8!"),
            Pronoun: row
                .columns[16]
                .into_i8()
                .copied()
                .expect("Expected column 16 to be a int8!"),
            Article: row
                .columns[17]
                .into_i8()
                .copied()
                .expect("Expected column 17 to be a int8!"),
            DEF: row
                .columns[18]
                .into_i8()
                .copied()
                .expect("Expected column 18 to be a int8!"),
            IconReputation: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            Icon: row
                .columns[4]
                .into_u32()
                .copied()
                .expect("Expected column 4 to be a uint32!"),
            IntersocietalQuest: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
            Level: row
                .columns[8]
                .into_u32()
                .copied()
                .expect("Expected column 8 to be a uint32!"),
            CurrencyItem: row
                .columns[9]
                .into_u32()
                .copied()
                .expect("Expected column 9 to be a uint32!"),
            MinLevel: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            BeastRankBonus: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            MaxRank: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Expansion: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            DisplayOrder: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            Unknown0: row
                .columns[0]
                .into_bool()
                .copied()
                .expect("Expected column 0 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a BeastTribeSheet {
    type Item = (u32, Vec<(u16, BeastTribeRow)>);
    type IntoIter = StructuredSheetIterator<'a, BeastTribeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BeastTribeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct BeastTribeRow {
    ///""
    pub Name: String,
    ///""
    pub Plural: String,
    ///""
    pub NameRelation: String,
    ///""
    pub Adjective: i8,
    ///""
    pub PossessivePronoun: i8,
    ///""
    pub StartsWithVowel: i8,
    ///""
    pub Pronoun: i8,
    ///""
    pub Article: i8,
    ///""
    pub DEF: i8,
    ///""
    pub IconReputation: u32,
    ///""
    pub Icon: u32,
    ///""
    pub IntersocietalQuest: u32,
    ///""
    pub Level: u32,
    ///""
    pub CurrencyItem: u32,
    ///""
    pub MinLevel: u8,
    ///""
    pub BeastRankBonus: u8,
    ///""
    pub MaxRank: u8,
    ///""
    pub Expansion: u8,
    ///""
    pub DisplayOrder: u8,
    ///""
    pub Unknown0: bool,
}
