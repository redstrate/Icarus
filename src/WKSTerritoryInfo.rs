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
pub struct WKSTerritoryInfoSheet {
    sheet: Sheet,
}
impl WKSTerritoryInfoSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSTerritoryInfo")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSTerritoryInfo", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WKSTerritoryInfoRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSTerritoryInfoRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for WKSTerritoryInfoSheet {
    type Row = WKSTerritoryInfoRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown6: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            Unknown7: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
            Unknown8: row
                .columns[8]
                .into_u32()
                .copied()
                .expect("Expected column 8 to be a uint32!"),
            Unknown9: row
                .columns[9]
                .into_u32()
                .copied()
                .expect("Expected column 9 to be a uint32!"),
            Unknown10: row
                .columns[10]
                .into_u32()
                .copied()
                .expect("Expected column 10 to be a uint32!"),
            Unknown11: row
                .columns[11]
                .into_u32()
                .copied()
                .expect("Expected column 11 to be a uint32!"),
            Unknown12: row
                .columns[12]
                .into_u32()
                .copied()
                .expect("Expected column 12 to be a uint32!"),
            TerritoryType: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            MechaEventNpc: row
                .columns[2]
                .into_u32()
                .copied()
                .expect("Expected column 2 to be a uint32!"),
            Item: row
                .columns[3]
                .into_u32()
                .copied()
                .expect("Expected column 3 to be a uint32!"),
            Unknown3: row
                .columns[5]
                .into_u32()
                .copied()
                .expect("Expected column 5 to be a uint32!"),
            Unknown4: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            Unknown5: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a WKSTerritoryInfoSheet {
    type Item = (u32, Vec<(u16, WKSTerritoryInfoRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSTerritoryInfoSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSTerritoryInfoSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct WKSTerritoryInfoRow {
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
    pub TerritoryType: u32,
    ///""
    pub MechaEventNpc: u32,
    ///""
    pub Item: u32,
    ///""
    pub Unknown3: u32,
    ///""
    pub Unknown4: u16,
    ///""
    pub Unknown5: u8,
}
