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
pub struct SkyIslandMapMarkerSheet {
    sheet: Sheet,
}
impl SkyIslandMapMarkerSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SkyIslandMapMarker")?;
        let sheet = resolver.read_excel_sheet(&exh, "SkyIslandMapMarker", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SkyIslandMapMarkerRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SkyIslandMapMarkerRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for SkyIslandMapMarkerSheet {
    type Row = SkyIslandMapMarkerRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[0]
                .into_i16()
                .copied()
                .expect("Expected column 0 to be a int16!"),
            Unknown1: row
                .columns[1]
                .into_i16()
                .copied()
                .expect("Expected column 1 to be a int16!"),
            Unknown2: row
                .columns[3]
                .into_i16()
                .copied()
                .expect("Expected column 3 to be a int16!"),
            Unknown3: row
                .columns[6]
                .into_i16()
                .copied()
                .expect("Expected column 6 to be a int16!"),
            Unknown4: row
                .columns[9]
                .into_i16()
                .copied()
                .expect("Expected column 9 to be a int16!"),
            Unknown5: row
                .columns[12]
                .into_i16()
                .copied()
                .expect("Expected column 12 to be a int16!"),
            Unknown6: row
                .columns[15]
                .into_i16()
                .copied()
                .expect("Expected column 15 to be a int16!"),
            Unknown7: row
                .columns[4]
                .into_i16()
                .copied()
                .expect("Expected column 4 to be a int16!"),
            Unknown8: row
                .columns[7]
                .into_i16()
                .copied()
                .expect("Expected column 7 to be a int16!"),
            Unknown9: row
                .columns[10]
                .into_i16()
                .copied()
                .expect("Expected column 10 to be a int16!"),
            Unknown10: row
                .columns[13]
                .into_i16()
                .copied()
                .expect("Expected column 13 to be a int16!"),
            Unknown11: row
                .columns[16]
                .into_i16()
                .copied()
                .expect("Expected column 16 to be a int16!"),
            Unknown12: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown13: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Unknown14: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            Unknown15: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            Unknown16: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a SkyIslandMapMarkerSheet {
    type Item = (u32, Vec<(u16, SkyIslandMapMarkerRow)>);
    type IntoIter = StructuredSheetIterator<'a, SkyIslandMapMarkerSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SkyIslandMapMarkerSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct SkyIslandMapMarkerRow {
    ///""
    pub Unknown0: i16,
    ///""
    pub Unknown1: i16,
    ///""
    pub Unknown2: i16,
    ///""
    pub Unknown3: i16,
    ///""
    pub Unknown4: i16,
    ///""
    pub Unknown5: i16,
    ///""
    pub Unknown6: i16,
    ///""
    pub Unknown7: i16,
    ///""
    pub Unknown8: i16,
    ///""
    pub Unknown9: i16,
    ///""
    pub Unknown10: i16,
    ///""
    pub Unknown11: i16,
    ///""
    pub Unknown12: u8,
    ///""
    pub Unknown13: u8,
    ///""
    pub Unknown14: u8,
    ///""
    pub Unknown15: u8,
    ///""
    pub Unknown16: u8,
}
