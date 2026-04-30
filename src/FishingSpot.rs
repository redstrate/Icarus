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
pub struct FishingSpotSheet {
    sheet: Sheet,
}
impl FishingSpotSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("FishingSpot")?;
        let sheet = resolver.read_excel_sheet(&exh, "FishingSpot", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<FishingSpotRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<FishingSpotRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for FishingSpotSheet {
    type Row = FishingSpotRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            BigFishOnReach: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            BigFishOnEnd: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            BigFishOnRefresh: row
                .columns[3]
                .into_string()
                .cloned()
                .expect("Expected column 3 to be a string!"),
            Item: [
                row
                    .columns[13]
                    .into_u32()
                    .copied()
                    .expect("Expected column 13 to be a uint32!"),
                row
                    .columns[14]
                    .into_u32()
                    .copied()
                    .expect("Expected column 14 to be a uint32!"),
                row
                    .columns[15]
                    .into_u32()
                    .copied()
                    .expect("Expected column 15 to be a uint32!"),
                row
                    .columns[16]
                    .into_u32()
                    .copied()
                    .expect("Expected column 16 to be a uint32!"),
                row
                    .columns[17]
                    .into_u32()
                    .copied()
                    .expect("Expected column 17 to be a uint32!"),
                row
                    .columns[18]
                    .into_u32()
                    .copied()
                    .expect("Expected column 18 to be a uint32!"),
                row
                    .columns[19]
                    .into_u32()
                    .copied()
                    .expect("Expected column 19 to be a uint32!"),
                row
                    .columns[20]
                    .into_u32()
                    .copied()
                    .expect("Expected column 20 to be a uint32!"),
                row
                    .columns[21]
                    .into_u32()
                    .copied()
                    .expect("Expected column 21 to be a uint32!"),
                row
                    .columns[22]
                    .into_u32()
                    .copied()
                    .expect("Expected column 22 to be a uint32!"),
            ],
            TerritoryType: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            PlaceNameMain: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            PlaceNameSub: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Radius: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            PlaceName: row
                .columns[23]
                .into_u16()
                .copied()
                .expect("Expected column 23 to be a uint16!"),
            Order: row
                .columns[24]
                .into_u16()
                .copied()
                .expect("Expected column 24 to be a uint16!"),
            X: row
                .columns[9]
                .into_i16()
                .copied()
                .expect("Expected column 9 to be a int16!"),
            Z: row
                .columns[10]
                .into_i16()
                .copied()
                .expect("Expected column 10 to be a int16!"),
            GatheringLevel: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            FishingSpotCategory: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Unknown0: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            Rare: row
                .columns[5]
                .into_bool()
                .copied()
                .expect("Expected column 5 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a FishingSpotSheet {
    type Item = (u32, Vec<(u16, FishingSpotRow)>);
    type IntoIter = StructuredSheetIterator<'a, FishingSpotSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, FishingSpotSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FishingSpotRow {
    ///""
    pub BigFishOnReach: String,
    ///""
    pub BigFishOnEnd: String,
    ///""
    pub BigFishOnRefresh: String,
    ///""
    pub Item: [u32; 10],
    ///""
    pub TerritoryType: u16,
    ///""
    pub PlaceNameMain: u16,
    ///""
    pub PlaceNameSub: u16,
    ///""
    pub Radius: u16,
    ///""
    pub PlaceName: u16,
    ///""
    pub Order: u16,
    ///""
    pub X: i16,
    ///""
    pub Z: i16,
    ///""
    pub GatheringLevel: u8,
    ///""
    pub FishingSpotCategory: u8,
    ///""
    pub Unknown0: u8,
    ///""
    pub Rare: bool,
}
