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
pub struct WarpSheet {
    sheet: Sheet,
}
impl WarpSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 131072u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Warp")?;
        let sheet = resolver.read_excel_sheet(&exh, "Warp", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WarpRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WarpRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WarpSheet {
    type Row = WarpRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[10]
                .into_string()
                .cloned()
                .expect("Expected column 10 to be a string!"),
            Question: row
                .columns[11]
                .into_string()
                .cloned()
                .expect("Expected column 11 to be a string!"),
            PopRange: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            ConditionSuccessEvent: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            ConditionFailEvent: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            ConfirmEvent: row
                .columns[4]
                .into_u32()
                .copied()
                .expect("Expected column 4 to be a uint32!"),
            TerritoryType: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            WarpCondition: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            WarpLogic: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            StartCutscene: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            EndCutscene: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            CanSkipCutscene: row
                .columns[9]
                .into_bool()
                .copied()
                .expect("Expected column 9 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a WarpSheet {
    type Item = (u32, Vec<(u16, WarpRow)>);
    type IntoIter = StructuredSheetIterator<'a, WarpSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WarpSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WarpRow {
    ///""
    pub Name: String,
    ///""
    pub Question: String,
    ///""
    pub PopRange: u32,
    ///""
    pub ConditionSuccessEvent: u32,
    ///""
    pub ConditionFailEvent: u32,
    ///""
    pub ConfirmEvent: u32,
    ///""
    pub TerritoryType: u16,
    ///""
    pub WarpCondition: u16,
    ///""
    pub WarpLogic: u16,
    ///""
    pub StartCutscene: u16,
    ///""
    pub EndCutscene: u16,
    ///""
    pub CanSkipCutscene: bool,
}
