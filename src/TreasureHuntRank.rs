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
pub struct TreasureHuntRankSheet {
    sheet: Sheet,
}
impl TreasureHuntRankSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TreasureHuntRank")?;
        let sheet = resolver.read_excel_sheet(&exh, "TreasureHuntRank", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<TreasureHuntRankRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<TreasureHuntRankRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for TreasureHuntRankSheet {
    type Row = TreasureHuntRankRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Icon: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            ItemName: row
                .columns[2]
                .into_i32()
                .copied()
                .expect("Expected column 2 to be a int32!"),
            KeyItemName: row
                .columns[3]
                .into_i32()
                .copied()
                .expect("Expected column 3 to be a int32!"),
            InstanceMap: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            Unknown0: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Unknown1: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            MaxPartySize: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            TreasureHuntTexture: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            Unknown2: row
                .columns[8]
                .into_bool()
                .copied()
                .expect("Expected column 8 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a TreasureHuntRankSheet {
    type Item = (u32, Vec<(u16, TreasureHuntRankRow)>);
    type IntoIter = StructuredSheetIterator<'a, TreasureHuntRankSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TreasureHuntRankSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TreasureHuntRankRow {
    ///""
    pub Icon: u32,
    ///""
    pub ItemName: i32,
    ///""
    pub KeyItemName: i32,
    ///""
    pub InstanceMap: i32,
    ///""
    pub Unknown0: u16,
    ///""
    pub Unknown1: u8,
    ///""
    pub MaxPartySize: u8,
    ///""
    pub TreasureHuntTexture: u8,
    ///""
    pub Unknown2: bool,
}
