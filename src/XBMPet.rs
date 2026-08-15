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
pub struct XBMPetSheet {
    sheet: Sheet,
}
impl XBMPetSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("XBMPet")?;
        let sheet = resolver.read_excel_sheet(&exh, "XBMPet", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<XBMPetRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<XBMPetRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for XBMPetSheet {
    type Row = XBMPetRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[8]
                .into_string()
                .cloned()
                .expect("Expected column 8 to be a string!"),
            Unknown1: row
                .columns[9]
                .into_string()
                .cloned()
                .expect("Expected column 9 to be a string!"),
            Unknown2: row
                .columns[10]
                .into_string()
                .cloned()
                .expect("Expected column 10 to be a string!"),
            Unknown3: row
                .columns[4]
                .into_u32()
                .copied()
                .expect("Expected column 4 to be a uint32!"),
            Pet: row
                .columns[0]
                .into_i32()
                .copied()
                .expect("Expected column 0 to be a int32!"),
            Action: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            Location: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Unknown7: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown8: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown9: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            LocationKey: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            Unknown11: [
                row
                    .columns[11]
                    .into_bool()
                    .copied()
                    .expect("Expected column 11 to be a bool!"),
                row
                    .columns[12]
                    .into_bool()
                    .copied()
                    .expect("Expected column 12 to be a bool!"),
                row
                    .columns[13]
                    .into_bool()
                    .copied()
                    .expect("Expected column 13 to be a bool!"),
                row
                    .columns[14]
                    .into_bool()
                    .copied()
                    .expect("Expected column 14 to be a bool!"),
                row
                    .columns[15]
                    .into_bool()
                    .copied()
                    .expect("Expected column 15 to be a bool!"),
                row
                    .columns[16]
                    .into_bool()
                    .copied()
                    .expect("Expected column 16 to be a bool!"),
                row
                    .columns[17]
                    .into_bool()
                    .copied()
                    .expect("Expected column 17 to be a bool!"),
                row
                    .columns[18]
                    .into_bool()
                    .copied()
                    .expect("Expected column 18 to be a bool!"),
                row
                    .columns[19]
                    .into_bool()
                    .copied()
                    .expect("Expected column 19 to be a bool!"),
                row
                    .columns[20]
                    .into_bool()
                    .copied()
                    .expect("Expected column 20 to be a bool!"),
                row
                    .columns[21]
                    .into_bool()
                    .copied()
                    .expect("Expected column 21 to be a bool!"),
            ],
        })
    }
}
impl<'a> IntoIterator for &'a XBMPetSheet {
    type Item = (u32, Vec<(u16, XBMPetRow)>);
    type IntoIter = StructuredSheetIterator<'a, XBMPetSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, XBMPetSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct XBMPetRow {
    ///""
    pub Unknown0: String,
    ///""
    pub Unknown1: String,
    ///""
    pub Unknown2: String,
    ///""
    pub Unknown3: u32,
    ///""
    pub Pet: i32,
    ///""
    pub Action: u16,
    ///""
    pub Location: u16,
    ///""
    pub Unknown7: u8,
    ///""
    pub Unknown8: u8,
    ///""
    pub Unknown9: u8,
    ///""
    pub LocationKey: u8,
    ///""
    pub Unknown11: [bool; 11],
}
