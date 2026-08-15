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
pub struct WKSFortunePatternUIPlaceSheet {
    sheet: Sheet,
}
impl WKSFortunePatternUIPlaceSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSFortunePatternUIPlace")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "WKSFortunePatternUIPlace", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSFortunePatternUIPlaceRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<WKSFortunePatternUIPlaceRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WKSFortunePatternUIPlaceSheet {
    type Row = WKSFortunePatternUIPlaceRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            Unknown1: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            Unknown2: row
                .columns[4]
                .into_u32()
                .copied()
                .expect("Expected column 4 to be a uint32!"),
            Unknown3: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            Unknown4: row
                .columns[8]
                .into_u32()
                .copied()
                .expect("Expected column 8 to be a uint32!"),
            Unknown5: row
                .columns[10]
                .into_u32()
                .copied()
                .expect("Expected column 10 to be a uint32!"),
            Unknown6: row
                .columns[12]
                .into_u32()
                .copied()
                .expect("Expected column 12 to be a uint32!"),
            Unknown7: row
                .columns[14]
                .into_u32()
                .copied()
                .expect("Expected column 14 to be a uint32!"),
            Unknown8: row
                .columns[16]
                .into_u32()
                .copied()
                .expect("Expected column 16 to be a uint32!"),
            Unknown9: row
                .columns[18]
                .into_u32()
                .copied()
                .expect("Expected column 18 to be a uint32!"),
            Unknown10: row
                .columns[20]
                .into_u32()
                .copied()
                .expect("Expected column 20 to be a uint32!"),
            Unknown11: row
                .columns[22]
                .into_u32()
                .copied()
                .expect("Expected column 22 to be a uint32!"),
            Unknown12: row
                .columns[24]
                .into_u32()
                .copied()
                .expect("Expected column 24 to be a uint32!"),
            Unknown13: row
                .columns[26]
                .into_u32()
                .copied()
                .expect("Expected column 26 to be a uint32!"),
            Unknown14: row
                .columns[28]
                .into_u32()
                .copied()
                .expect("Expected column 28 to be a uint32!"),
            Unknown15: row
                .columns[30]
                .into_u32()
                .copied()
                .expect("Expected column 30 to be a uint32!"),
            Unknown16: row
                .columns[32]
                .into_u32()
                .copied()
                .expect("Expected column 32 to be a uint32!"),
            Unknown17: row
                .columns[34]
                .into_u32()
                .copied()
                .expect("Expected column 34 to be a uint32!"),
            Unknown18: row
                .columns[36]
                .into_u32()
                .copied()
                .expect("Expected column 36 to be a uint32!"),
            Unknown19: row
                .columns[38]
                .into_u32()
                .copied()
                .expect("Expected column 38 to be a uint32!"),
            Unknown20: row
                .columns[1]
                .into_u32()
                .copied()
                .expect("Expected column 1 to be a uint32!"),
            Unknown21: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            Unknown22: row
                .columns[5]
                .into_u32()
                .copied()
                .expect("Expected column 5 to be a uint32!"),
            Unknown23: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
            Unknown24: row
                .columns[9]
                .into_u32()
                .copied()
                .expect("Expected column 9 to be a uint32!"),
            Unknown25: row
                .columns[11]
                .into_u32()
                .copied()
                .expect("Expected column 11 to be a uint32!"),
            Unknown26: row
                .columns[13]
                .into_u32()
                .copied()
                .expect("Expected column 13 to be a uint32!"),
            Unknown27: row
                .columns[15]
                .into_u32()
                .copied()
                .expect("Expected column 15 to be a uint32!"),
            Unknown28: row
                .columns[17]
                .into_u32()
                .copied()
                .expect("Expected column 17 to be a uint32!"),
            Unknown29: row
                .columns[19]
                .into_u32()
                .copied()
                .expect("Expected column 19 to be a uint32!"),
            Unknown30: row
                .columns[21]
                .into_u32()
                .copied()
                .expect("Expected column 21 to be a uint32!"),
            Unknown31: row
                .columns[23]
                .into_u32()
                .copied()
                .expect("Expected column 23 to be a uint32!"),
            Unknown32: row
                .columns[25]
                .into_u32()
                .copied()
                .expect("Expected column 25 to be a uint32!"),
            Unknown33: row
                .columns[27]
                .into_u32()
                .copied()
                .expect("Expected column 27 to be a uint32!"),
            Unknown34: row
                .columns[29]
                .into_u32()
                .copied()
                .expect("Expected column 29 to be a uint32!"),
            Unknown35: row
                .columns[31]
                .into_u32()
                .copied()
                .expect("Expected column 31 to be a uint32!"),
            Unknown36: row
                .columns[33]
                .into_u32()
                .copied()
                .expect("Expected column 33 to be a uint32!"),
            Unknown37: row
                .columns[35]
                .into_u32()
                .copied()
                .expect("Expected column 35 to be a uint32!"),
            Unknown38: row
                .columns[37]
                .into_u32()
                .copied()
                .expect("Expected column 37 to be a uint32!"),
            Unknown39: row
                .columns[39]
                .into_u32()
                .copied()
                .expect("Expected column 39 to be a uint32!"),
        })
    }
}
impl<'a> IntoIterator for &'a WKSFortunePatternUIPlaceSheet {
    type Item = (u32, Vec<(u16, WKSFortunePatternUIPlaceRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSFortunePatternUIPlaceSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSFortunePatternUIPlaceSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WKSFortunePatternUIPlaceRow {
    ///""
    pub Unknown0: u32,
    ///""
    pub Unknown1: u32,
    ///""
    pub Unknown2: u32,
    ///""
    pub Unknown3: u32,
    ///""
    pub Unknown4: u32,
    ///""
    pub Unknown5: u32,
    ///""
    pub Unknown6: u32,
    ///""
    pub Unknown7: u32,
    ///""
    pub Unknown8: u32,
    ///""
    pub Unknown9: u32,
    ///""
    pub Unknown10: u32,
    ///""
    pub Unknown11: u32,
    ///""
    pub Unknown12: u32,
    ///""
    pub Unknown13: u32,
    ///""
    pub Unknown14: u32,
    ///""
    pub Unknown15: u32,
    ///""
    pub Unknown16: u32,
    ///""
    pub Unknown17: u32,
    ///""
    pub Unknown18: u32,
    ///""
    pub Unknown19: u32,
    ///""
    pub Unknown20: u32,
    ///""
    pub Unknown21: u32,
    ///""
    pub Unknown22: u32,
    ///""
    pub Unknown23: u32,
    ///""
    pub Unknown24: u32,
    ///""
    pub Unknown25: u32,
    ///""
    pub Unknown26: u32,
    ///""
    pub Unknown27: u32,
    ///""
    pub Unknown28: u32,
    ///""
    pub Unknown29: u32,
    ///""
    pub Unknown30: u32,
    ///""
    pub Unknown31: u32,
    ///""
    pub Unknown32: u32,
    ///""
    pub Unknown33: u32,
    ///""
    pub Unknown34: u32,
    ///""
    pub Unknown35: u32,
    ///""
    pub Unknown36: u32,
    ///""
    pub Unknown37: u32,
    ///""
    pub Unknown38: u32,
    ///""
    pub Unknown39: u32,
}
