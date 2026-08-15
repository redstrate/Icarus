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
pub struct WKSDevGradeBGSetSheet {
    sheet: Sheet,
}
impl WKSDevGradeBGSetSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSDevGradeBGSet")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSDevGradeBGSet", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSDevGradeBGSetRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSDevGradeBGSetRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WKSDevGradeBGSetSheet {
    type Row = WKSDevGradeBGSetRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            Unknown1: row
                .columns[1]
                .into_bool()
                .copied()
                .expect("Expected column 1 to be a bool!"),
            Unknown2: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
            Unknown3: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
            Unknown4: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
            Unknown5: row
                .columns[5]
                .into_bool()
                .copied()
                .expect("Expected column 5 to be a bool!"),
            Unknown6: row
                .columns[6]
                .into_bool()
                .copied()
                .expect("Expected column 6 to be a bool!"),
            Unknown7: row
                .columns[7]
                .into_bool()
                .copied()
                .expect("Expected column 7 to be a bool!"),
            Unknown8: row
                .columns[8]
                .into_bool()
                .copied()
                .expect("Expected column 8 to be a bool!"),
            Unknown9: row
                .columns[9]
                .into_bool()
                .copied()
                .expect("Expected column 9 to be a bool!"),
            Unknown10: row
                .columns[10]
                .into_bool()
                .copied()
                .expect("Expected column 10 to be a bool!"),
            Unknown11: row
                .columns[11]
                .into_bool()
                .copied()
                .expect("Expected column 11 to be a bool!"),
            Unknown12: row
                .columns[12]
                .into_bool()
                .copied()
                .expect("Expected column 12 to be a bool!"),
            Unknown13: row
                .columns[13]
                .into_bool()
                .copied()
                .expect("Expected column 13 to be a bool!"),
            Unknown14: row
                .columns[14]
                .into_bool()
                .copied()
                .expect("Expected column 14 to be a bool!"),
            Unknown15: row
                .columns[15]
                .into_bool()
                .copied()
                .expect("Expected column 15 to be a bool!"),
            Unknown16: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            Unknown17: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            Unknown18: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            Unknown19: row
                .columns[19]
                .into_bool()
                .copied()
                .expect("Expected column 19 to be a bool!"),
            Unknown20: row
                .columns[20]
                .into_bool()
                .copied()
                .expect("Expected column 20 to be a bool!"),
            Unknown21: row
                .columns[21]
                .into_bool()
                .copied()
                .expect("Expected column 21 to be a bool!"),
            Unknown22: row
                .columns[22]
                .into_bool()
                .copied()
                .expect("Expected column 22 to be a bool!"),
            Unknown23: row
                .columns[23]
                .into_bool()
                .copied()
                .expect("Expected column 23 to be a bool!"),
            Unknown24: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
            Unknown25: row
                .columns[25]
                .into_bool()
                .copied()
                .expect("Expected column 25 to be a bool!"),
            Unknown26: row
                .columns[26]
                .into_bool()
                .copied()
                .expect("Expected column 26 to be a bool!"),
            Unknown27: row
                .columns[27]
                .into_bool()
                .copied()
                .expect("Expected column 27 to be a bool!"),
            Unknown28: row
                .columns[28]
                .into_bool()
                .copied()
                .expect("Expected column 28 to be a bool!"),
            Unknown29: row
                .columns[29]
                .into_bool()
                .copied()
                .expect("Expected column 29 to be a bool!"),
            Unknown30: row
                .columns[30]
                .into_bool()
                .copied()
                .expect("Expected column 30 to be a bool!"),
            Unknown31: row
                .columns[31]
                .into_bool()
                .copied()
                .expect("Expected column 31 to be a bool!"),
            Unknown32: row
                .columns[32]
                .into_bool()
                .copied()
                .expect("Expected column 32 to be a bool!"),
            Unknown33: row
                .columns[33]
                .into_bool()
                .copied()
                .expect("Expected column 33 to be a bool!"),
            Unknown34: row
                .columns[34]
                .into_bool()
                .copied()
                .expect("Expected column 34 to be a bool!"),
            Unknown35: row
                .columns[35]
                .into_bool()
                .copied()
                .expect("Expected column 35 to be a bool!"),
            Unknown36: row
                .columns[36]
                .into_bool()
                .copied()
                .expect("Expected column 36 to be a bool!"),
            Unknown37: row
                .columns[37]
                .into_bool()
                .copied()
                .expect("Expected column 37 to be a bool!"),
            Unknown38: row
                .columns[38]
                .into_bool()
                .copied()
                .expect("Expected column 38 to be a bool!"),
            Unknown39: row
                .columns[39]
                .into_bool()
                .copied()
                .expect("Expected column 39 to be a bool!"),
            Unknown40: row
                .columns[40]
                .into_bool()
                .copied()
                .expect("Expected column 40 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a WKSDevGradeBGSetSheet {
    type Item = (u32, Vec<(u16, WKSDevGradeBGSetRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSDevGradeBGSetSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSDevGradeBGSetSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WKSDevGradeBGSetRow {
    ///""
    pub Unknown0: u16,
    ///""
    pub Unknown1: bool,
    ///""
    pub Unknown2: bool,
    ///""
    pub Unknown3: bool,
    ///""
    pub Unknown4: bool,
    ///""
    pub Unknown5: bool,
    ///""
    pub Unknown6: bool,
    ///""
    pub Unknown7: bool,
    ///""
    pub Unknown8: bool,
    ///""
    pub Unknown9: bool,
    ///""
    pub Unknown10: bool,
    ///""
    pub Unknown11: bool,
    ///""
    pub Unknown12: bool,
    ///""
    pub Unknown13: bool,
    ///""
    pub Unknown14: bool,
    ///""
    pub Unknown15: bool,
    ///""
    pub Unknown16: bool,
    ///""
    pub Unknown17: bool,
    ///""
    pub Unknown18: bool,
    ///""
    pub Unknown19: bool,
    ///""
    pub Unknown20: bool,
    ///""
    pub Unknown21: bool,
    ///""
    pub Unknown22: bool,
    ///""
    pub Unknown23: bool,
    ///""
    pub Unknown24: bool,
    ///""
    pub Unknown25: bool,
    ///""
    pub Unknown26: bool,
    ///""
    pub Unknown27: bool,
    ///""
    pub Unknown28: bool,
    ///""
    pub Unknown29: bool,
    ///""
    pub Unknown30: bool,
    ///""
    pub Unknown31: bool,
    ///""
    pub Unknown32: bool,
    ///""
    pub Unknown33: bool,
    ///""
    pub Unknown34: bool,
    ///""
    pub Unknown35: bool,
    ///""
    pub Unknown36: bool,
    ///""
    pub Unknown37: bool,
    ///""
    pub Unknown38: bool,
    ///""
    pub Unknown39: bool,
    ///""
    pub Unknown40: bool,
}
