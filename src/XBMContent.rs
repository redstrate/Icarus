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
pub struct XBMContentSheet {
    sheet: Sheet,
}
impl XBMContentSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("XBMContent")?;
        let sheet = resolver.read_excel_sheet(&exh, "XBMContent", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<XBMContentRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<XBMContentRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for XBMContentSheet {
    type Row = XBMContentRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            Unknown1: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            Unknown2: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            Unknown3: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Unknown4: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Unknown5: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            Unknown6: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            Unknown7: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            Unknown8: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            Unknown9: row
                .columns[13]
                .into_u16()
                .copied()
                .expect("Expected column 13 to be a uint16!"),
            Unknown10: row
                .columns[14]
                .into_u16()
                .copied()
                .expect("Expected column 14 to be a uint16!"),
            Unknown11: row
                .columns[15]
                .into_u16()
                .copied()
                .expect("Expected column 15 to be a uint16!"),
            Unknown12: row
                .columns[16]
                .into_u16()
                .copied()
                .expect("Expected column 16 to be a uint16!"),
            Unknown13: row
                .columns[17]
                .into_u16()
                .copied()
                .expect("Expected column 17 to be a uint16!"),
            Unknown14: row
                .columns[18]
                .into_u16()
                .copied()
                .expect("Expected column 18 to be a uint16!"),
            Unknown15: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            Unknown16: row
                .columns[20]
                .into_u16()
                .copied()
                .expect("Expected column 20 to be a uint16!"),
            Unknown17: row
                .columns[21]
                .into_u16()
                .copied()
                .expect("Expected column 21 to be a uint16!"),
            Unknown18: row
                .columns[22]
                .into_u16()
                .copied()
                .expect("Expected column 22 to be a uint16!"),
            Unknown19: row
                .columns[23]
                .into_u16()
                .copied()
                .expect("Expected column 23 to be a uint16!"),
            Unknown20: row
                .columns[24]
                .into_u16()
                .copied()
                .expect("Expected column 24 to be a uint16!"),
            Unknown21: row
                .columns[25]
                .into_u16()
                .copied()
                .expect("Expected column 25 to be a uint16!"),
            Unknown22: row
                .columns[26]
                .into_u16()
                .copied()
                .expect("Expected column 26 to be a uint16!"),
            Unknown23: row
                .columns[27]
                .into_u16()
                .copied()
                .expect("Expected column 27 to be a uint16!"),
            Unknown24: row
                .columns[28]
                .into_u16()
                .copied()
                .expect("Expected column 28 to be a uint16!"),
            Unknown25: row
                .columns[29]
                .into_u16()
                .copied()
                .expect("Expected column 29 to be a uint16!"),
            Unknown26: row
                .columns[30]
                .into_u16()
                .copied()
                .expect("Expected column 30 to be a uint16!"),
            Unknown27: row
                .columns[31]
                .into_u16()
                .copied()
                .expect("Expected column 31 to be a uint16!"),
            Unknown28: row
                .columns[32]
                .into_u16()
                .copied()
                .expect("Expected column 32 to be a uint16!"),
            Unknown29: row
                .columns[33]
                .into_u16()
                .copied()
                .expect("Expected column 33 to be a uint16!"),
            Unknown30: row
                .columns[34]
                .into_u16()
                .copied()
                .expect("Expected column 34 to be a uint16!"),
            Unknown31: row
                .columns[35]
                .into_u16()
                .copied()
                .expect("Expected column 35 to be a uint16!"),
            Unknown32: row
                .columns[36]
                .into_u16()
                .copied()
                .expect("Expected column 36 to be a uint16!"),
            Unknown33: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            Unknown34: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown35: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown36: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a XBMContentSheet {
    type Item = (u32, Vec<(u16, XBMContentRow)>);
    type IntoIter = StructuredSheetIterator<'a, XBMContentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, XBMContentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct XBMContentRow {
    ///""
    pub Unknown0: u16,
    ///""
    pub Unknown1: u16,
    ///""
    pub Unknown2: u16,
    ///""
    pub Unknown3: u16,
    ///""
    pub Unknown4: u16,
    ///""
    pub Unknown5: u16,
    ///""
    pub Unknown6: u16,
    ///""
    pub Unknown7: u16,
    ///""
    pub Unknown8: u16,
    ///""
    pub Unknown9: u16,
    ///""
    pub Unknown10: u16,
    ///""
    pub Unknown11: u16,
    ///""
    pub Unknown12: u16,
    ///""
    pub Unknown13: u16,
    ///""
    pub Unknown14: u16,
    ///""
    pub Unknown15: u16,
    ///""
    pub Unknown16: u16,
    ///""
    pub Unknown17: u16,
    ///""
    pub Unknown18: u16,
    ///""
    pub Unknown19: u16,
    ///""
    pub Unknown20: u16,
    ///""
    pub Unknown21: u16,
    ///""
    pub Unknown22: u16,
    ///""
    pub Unknown23: u16,
    ///""
    pub Unknown24: u16,
    ///""
    pub Unknown25: u16,
    ///""
    pub Unknown26: u16,
    ///""
    pub Unknown27: u16,
    ///""
    pub Unknown28: u16,
    ///""
    pub Unknown29: u16,
    ///""
    pub Unknown30: u16,
    ///""
    pub Unknown31: u16,
    ///""
    pub Unknown32: u16,
    ///""
    pub Unknown33: u16,
    ///""
    pub Unknown34: u8,
    ///""
    pub Unknown35: u8,
    ///""
    pub Unknown36: u8,
}
