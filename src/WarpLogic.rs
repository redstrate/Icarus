//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Debug, PartialEq)]
pub struct WarpParamsElement {
    pub Function: String,
    pub Argument: u32,
}
#[derive(Debug, Clone)]
pub struct WarpLogicSheet {
    sheet: Sheet,
}
impl WarpLogicSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WarpLogic")?;
        let sheet = resolver.read_excel_sheet(&exh, "WarpLogic", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WarpLogicRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WarpLogicRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WarpLogicSheet {
    type Row = WarpLogicRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            WarpParams: [
                WarpParamsElement {
                    Function: row
                        .columns[3]
                        .into_string()
                        .cloned()
                        .expect("Expected column 3 to be a string!"),
                    Argument: row
                        .columns[13]
                        .into_u32()
                        .copied()
                        .expect("Expected column 13 to be a uint32!"),
                },
                WarpParamsElement {
                    Function: row
                        .columns[4]
                        .into_string()
                        .cloned()
                        .expect("Expected column 4 to be a string!"),
                    Argument: row
                        .columns[14]
                        .into_u32()
                        .copied()
                        .expect("Expected column 14 to be a uint32!"),
                },
                WarpParamsElement {
                    Function: row
                        .columns[5]
                        .into_string()
                        .cloned()
                        .expect("Expected column 5 to be a string!"),
                    Argument: row
                        .columns[15]
                        .into_u32()
                        .copied()
                        .expect("Expected column 15 to be a uint32!"),
                },
                WarpParamsElement {
                    Function: row
                        .columns[6]
                        .into_string()
                        .cloned()
                        .expect("Expected column 6 to be a string!"),
                    Argument: row
                        .columns[16]
                        .into_u32()
                        .copied()
                        .expect("Expected column 16 to be a uint32!"),
                },
                WarpParamsElement {
                    Function: row
                        .columns[7]
                        .into_string()
                        .cloned()
                        .expect("Expected column 7 to be a string!"),
                    Argument: row
                        .columns[17]
                        .into_u32()
                        .copied()
                        .expect("Expected column 17 to be a uint32!"),
                },
                WarpParamsElement {
                    Function: row
                        .columns[8]
                        .into_string()
                        .cloned()
                        .expect("Expected column 8 to be a string!"),
                    Argument: row
                        .columns[18]
                        .into_u32()
                        .copied()
                        .expect("Expected column 18 to be a uint32!"),
                },
                WarpParamsElement {
                    Function: row
                        .columns[9]
                        .into_string()
                        .cloned()
                        .expect("Expected column 9 to be a string!"),
                    Argument: row
                        .columns[19]
                        .into_u32()
                        .copied()
                        .expect("Expected column 19 to be a uint32!"),
                },
                WarpParamsElement {
                    Function: row
                        .columns[10]
                        .into_string()
                        .cloned()
                        .expect("Expected column 10 to be a string!"),
                    Argument: row
                        .columns[20]
                        .into_u32()
                        .copied()
                        .expect("Expected column 20 to be a uint32!"),
                },
                WarpParamsElement {
                    Function: row
                        .columns[11]
                        .into_string()
                        .cloned()
                        .expect("Expected column 11 to be a string!"),
                    Argument: row
                        .columns[21]
                        .into_u32()
                        .copied()
                        .expect("Expected column 21 to be a uint32!"),
                },
                WarpParamsElement {
                    Function: row
                        .columns[12]
                        .into_string()
                        .cloned()
                        .expect("Expected column 12 to be a string!"),
                    Argument: row
                        .columns[22]
                        .into_u32()
                        .copied()
                        .expect("Expected column 22 to be a uint32!"),
                },
            ],
            Question: row
                .columns[23]
                .into_string()
                .cloned()
                .expect("Expected column 23 to be a string!"),
            ResponseYes: row
                .columns[24]
                .into_string()
                .cloned()
                .expect("Expected column 24 to be a string!"),
            ResponseNo: row
                .columns[25]
                .into_string()
                .cloned()
                .expect("Expected column 25 to be a string!"),
            WarpName: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            Icon: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            CanSkipCutscene: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a WarpLogicSheet {
    type Item = (u32, Vec<(u16, WarpLogicRow)>);
    type IntoIter = StructuredSheetIterator<'a, WarpLogicSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WarpLogicSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WarpLogicRow {
    ///""
    pub WarpParams: [WarpParamsElement; 10],
    ///""
    pub Question: String,
    ///""
    pub ResponseYes: String,
    ///""
    pub ResponseNo: String,
    ///""
    pub WarpName: String,
    ///""
    pub Icon: u32,
    ///""
    pub CanSkipCutscene: bool,
}
