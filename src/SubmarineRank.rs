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
pub struct SubmarineRankSheet {
    sheet: Sheet,
}
impl SubmarineRankSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SubmarineRank")?;
        let sheet = resolver.read_excel_sheet(&exh, "SubmarineRank", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SubmarineRankRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SubmarineRankRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for SubmarineRankSheet {
    type Row = SubmarineRankRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            ExpToNext: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            Capacity: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            SurveillanceBonus: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            RetrievalBonus: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            SpeedBonus: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            RangeBonus: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            FavorBonus: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a SubmarineRankSheet {
    type Item = (u32, Vec<(u16, SubmarineRankRow)>);
    type IntoIter = StructuredSheetIterator<'a, SubmarineRankSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SubmarineRankSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SubmarineRankRow {
    ///""
    pub ExpToNext: u32,
    ///""
    pub Capacity: u16,
    ///""
    pub SurveillanceBonus: u8,
    ///""
    pub RetrievalBonus: u8,
    ///""
    pub SpeedBonus: u8,
    ///""
    pub RangeBonus: u8,
    ///""
    pub FavorBonus: u8,
}
