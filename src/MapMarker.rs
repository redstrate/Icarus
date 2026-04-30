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
pub struct MapMarkerSheet {
    sheet: Sheet,
}
impl MapMarkerSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MapMarker")?;
        let sheet = resolver.read_excel_sheet(&exh, "MapMarker", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MapMarkerRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MapMarkerRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MapMarkerSheet {
    type Row = MapMarkerRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Icon: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            PlaceNameSubtext: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            DataKey: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            MapMarkerUnlockedBit: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            X: row
                .columns[0]
                .into_i16()
                .copied()
                .expect("Expected column 0 to be a int16!"),
            Y: row
                .columns[1]
                .into_i16()
                .copied()
                .expect("Expected column 1 to be a int16!"),
            SubtextOrientation: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            MapMarkerRegion: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Type: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            DataType: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            Unknown0: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a MapMarkerSheet {
    type Item = (u32, Vec<(u16, MapMarkerRow)>);
    type IntoIter = StructuredSheetIterator<'a, MapMarkerSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MapMarkerSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MapMarkerRow {
    ///""
    pub Icon: u16,
    ///""
    pub PlaceNameSubtext: u16,
    ///""
    pub DataKey: u16,
    ///""
    pub MapMarkerUnlockedBit: u16,
    ///""
    pub X: i16,
    ///""
    pub Y: i16,
    ///""
    pub SubtextOrientation: u8,
    ///""
    pub MapMarkerRegion: u8,
    ///""
    pub Type: u8,
    ///""
    pub DataType: u8,
    ///""
    pub Unknown0: u8,
}
