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
pub struct CSBonusSeasonSheet {
    sheet: Sheet,
}
impl CSBonusSeasonSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CSBonusSeason")?;
        let sheet = resolver.read_excel_sheet(&exh, "CSBonusSeason", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CSBonusSeasonRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CSBonusSeasonRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CSBonusSeasonSheet {
    type Row = CSBonusSeasonRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Item: row
                .columns[5]
                .into_u32()
                .copied()
                .expect("Expected column 5 to be a uint32!"),
            Unknown13: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            Unknown1: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            Unknown2: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            Category0: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Category1: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            Category2: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            Category3: row
                .columns[13]
                .into_u16()
                .copied()
                .expect("Expected column 13 to be a uint16!"),
            Unknown14: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            Unknown15: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            Unknown16: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            Unknown17: row
                .columns[14]
                .into_u16()
                .copied()
                .expect("Expected column 14 to be a uint16!"),
            Text0: row
                .columns[15]
                .into_u8()
                .copied()
                .expect("Expected column 15 to be a uint8!"),
            Text1: row
                .columns[16]
                .into_u8()
                .copied()
                .expect("Expected column 16 to be a uint8!"),
            Unknown12: row
                .columns[17]
                .into_u8()
                .copied()
                .expect("Expected column 17 to be a uint8!"),
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
            Unknown0: row
                .columns[0]
                .into_bool()
                .copied()
                .expect("Expected column 0 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a CSBonusSeasonSheet {
    type Item = (u32, Vec<(u16, CSBonusSeasonRow)>);
    type IntoIter = StructuredSheetIterator<'a, CSBonusSeasonSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CSBonusSeasonSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct CSBonusSeasonRow {
    ///""
    pub Item: u32,
    ///""
    pub Unknown13: u32,
    ///""
    pub Unknown1: u16,
    ///""
    pub Unknown2: u16,
    ///""
    pub Category0: u16,
    ///""
    pub Category1: u16,
    ///""
    pub Category2: u16,
    ///""
    pub Category3: u16,
    ///""
    pub Unknown14: u16,
    ///""
    pub Unknown15: u16,
    ///""
    pub Unknown16: u16,
    ///""
    pub Unknown17: u16,
    ///""
    pub Text0: u8,
    ///""
    pub Text1: u8,
    ///""
    pub Unknown12: u8,
    ///""
    pub Unknown3: bool,
    ///""
    pub Unknown4: bool,
    ///""
    pub Unknown0: bool,
}
