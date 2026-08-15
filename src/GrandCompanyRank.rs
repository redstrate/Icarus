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
pub struct GrandCompanyRankSheet {
    sheet: Sheet,
}
impl GrandCompanyRankSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GrandCompanyRank")?;
        let sheet = resolver.read_excel_sheet(&exh, "GrandCompanyRank", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GrandCompanyRankRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GrandCompanyRankRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GrandCompanyRankSheet {
    type Row = GrandCompanyRankRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            MaxSeals: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            RequiredSeals: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            IconMaelstrom: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            IconSerpents: row
                .columns[5]
                .into_i32()
                .copied()
                .expect("Expected column 5 to be a int32!"),
            IconFlames: row
                .columns[6]
                .into_i32()
                .copied()
                .expect("Expected column 6 to be a int32!"),
            QuestMaelstrom: row
                .columns[7]
                .into_i32()
                .copied()
                .expect("Expected column 7 to be a int32!"),
            QuestSerpents: row
                .columns[8]
                .into_i32()
                .copied()
                .expect("Expected column 8 to be a int32!"),
            QuestFlames: row
                .columns[9]
                .into_i32()
                .copied()
                .expect("Expected column 9 to be a int32!"),
            Tier: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            Order: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            RequiredHuntingLogRank: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a GrandCompanyRankSheet {
    type Item = (u32, Vec<(u16, GrandCompanyRankRow)>);
    type IntoIter = StructuredSheetIterator<'a, GrandCompanyRankSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GrandCompanyRankSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GrandCompanyRankRow {
    ///""
    pub MaxSeals: u32,
    ///""
    pub RequiredSeals: u32,
    ///""
    pub IconMaelstrom: i32,
    ///""
    pub IconSerpents: i32,
    ///""
    pub IconFlames: i32,
    ///""
    pub QuestMaelstrom: i32,
    ///""
    pub QuestSerpents: i32,
    ///""
    pub QuestFlames: i32,
    ///""
    pub Tier: u8,
    ///""
    pub Order: u8,
    ///""
    pub RequiredHuntingLogRank: u8,
}
