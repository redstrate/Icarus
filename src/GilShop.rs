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
pub struct GilShopSheet {
    sheet: Sheet,
}
impl GilShopSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 262144u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GilShop")?;
        let sheet = resolver.read_excel_sheet(&exh, "GilShop", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GilShopRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GilShopRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GilShopSheet {
    type Row = GilShopRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Icon: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            Quest: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            AcceptTalk: row
                .columns[3]
                .into_i32()
                .copied()
                .expect("Expected column 3 to be a int32!"),
            FailTalk: row
                .columns[4]
                .into_i32()
                .copied()
                .expect("Expected column 4 to be a int32!"),
            FestivalId: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            FestivalPhase: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Unknown2: row
                .columns[5]
                .into_bool()
                .copied()
                .expect("Expected column 5 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a GilShopSheet {
    type Item = (u32, Vec<(u16, GilShopRow)>);
    type IntoIter = StructuredSheetIterator<'a, GilShopSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GilShopSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GilShopRow {
    ///""
    pub Name: String,
    ///""
    pub Icon: u32,
    ///""
    pub Quest: u32,
    ///""
    pub AcceptTalk: i32,
    ///""
    pub FailTalk: i32,
    ///""
    pub FestivalId: u16,
    ///""
    pub FestivalPhase: u16,
    ///""
    pub Unknown2: bool,
}
