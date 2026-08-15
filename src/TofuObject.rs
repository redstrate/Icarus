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
pub struct TofuObjectSheet {
    sheet: Sheet,
}
impl TofuObjectSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TofuObject")?;
        let sheet = resolver.read_excel_sheet(&exh, "TofuObject", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<TofuObjectRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<TofuObjectRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for TofuObjectSheet {
    type Row = TofuObjectRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[8]
                .into_string()
                .cloned()
                .expect("Expected column 8 to be a string!"),
            Icon: row
                .columns[4]
                .into_u32()
                .copied()
                .expect("Expected column 4 to be a uint32!"),
            AltIcon: row
                .columns[5]
                .into_u32()
                .copied()
                .expect("Expected column 5 to be a uint32!"),
            Unknown3: row
                .columns[2]
                .into_i32()
                .copied()
                .expect("Expected column 2 to be a int32!"),
            Unknown4: row
                .columns[9]
                .into_i32()
                .copied()
                .expect("Expected column 9 to be a int32!"),
            Unknown5: row
                .columns[10]
                .into_i32()
                .copied()
                .expect("Expected column 10 to be a int32!"),
            Unknown6: row
                .columns[11]
                .into_i32()
                .copied()
                .expect("Expected column 11 to be a int32!"),
            Unknown7: row
                .columns[12]
                .into_i32()
                .copied()
                .expect("Expected column 12 to be a int32!"),
            Unknown8: row
                .columns[13]
                .into_i32()
                .copied()
                .expect("Expected column 13 to be a int32!"),
            Unknown9: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            Unknown16: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            Unknown17: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Unknown18: row
                .columns[14]
                .into_i16()
                .copied()
                .expect("Expected column 14 to be a int16!"),
            Unknown19: row
                .columns[15]
                .into_i16()
                .copied()
                .expect("Expected column 15 to be a int16!"),
            Unknown20: row
                .columns[16]
                .into_i16()
                .copied()
                .expect("Expected column 16 to be a int16!"),
            Unknown21: row
                .columns[17]
                .into_i16()
                .copied()
                .expect("Expected column 17 to be a int16!"),
            Unknown10: row
                .columns[18]
                .into_i16()
                .copied()
                .expect("Expected column 18 to be a int16!"),
            Category: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown12: row
                .columns[21]
                .into_i8()
                .copied()
                .expect("Expected column 21 to be a int8!"),
            Unknown13: row
                .columns[0]
                .into_bool()
                .copied()
                .expect("Expected column 0 to be a bool!"),
            Unknown14: row
                .columns[19]
                .into_bool()
                .copied()
                .expect("Expected column 19 to be a bool!"),
            Unknown15: row
                .columns[20]
                .into_bool()
                .copied()
                .expect("Expected column 20 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a TofuObjectSheet {
    type Item = (u32, Vec<(u16, TofuObjectRow)>);
    type IntoIter = StructuredSheetIterator<'a, TofuObjectSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TofuObjectSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TofuObjectRow {
    ///""
    pub Name: String,
    ///""
    pub Icon: u32,
    ///""
    pub AltIcon: u32,
    ///""
    pub Unknown3: i32,
    ///""
    pub Unknown4: i32,
    ///""
    pub Unknown5: i32,
    ///""
    pub Unknown6: i32,
    ///""
    pub Unknown7: i32,
    ///""
    pub Unknown8: i32,
    ///""
    pub Unknown9: u16,
    ///""
    pub Unknown16: u16,
    ///""
    pub Unknown17: u16,
    ///""
    pub Unknown18: i16,
    ///""
    pub Unknown19: i16,
    ///""
    pub Unknown20: i16,
    ///""
    pub Unknown21: i16,
    ///""
    pub Unknown10: i16,
    ///""
    pub Category: u8,
    ///""
    pub Unknown12: i8,
    ///""
    pub Unknown13: bool,
    ///""
    pub Unknown14: bool,
    ///""
    pub Unknown15: bool,
}
