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
pub struct MapSheet {
    sheet: Sheet,
}
impl MapSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Map")?;
        let sheet = resolver.read_excel_sheet(&exh, "Map", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MapRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<MapRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for MapSheet {
    type Row = MapRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Id: row
                .columns[6]
                .into_string()
                .cloned()
                .expect("Expected column 6 to be a string!"),
            DiscoveryFlag: row
                .columns[15]
                .into_u32()
                .copied()
                .expect("Expected column 15 to be a uint32!"),
            MapMarkerRange: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            SizeFactor: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            PlaceNameRegion: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            PlaceName: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            PlaceNameSub: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            TerritoryType: row
                .columns[16]
                .into_u16()
                .copied()
                .expect("Expected column 16 to be a uint16!"),
            OffsetX: row
                .columns[8]
                .into_i16()
                .copied()
                .expect("Expected column 8 to be a int16!"),
            OffsetY: row
                .columns[9]
                .into_i16()
                .copied()
                .expect("Expected column 9 to be a int16!"),
            DiscoveryIndex: row
                .columns[14]
                .into_i16()
                .copied()
                .expect("Expected column 14 to be a int16!"),
            MapCondition: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            PriorityCategoryUI: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            PriorityUI: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            MapType: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Unknown2: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            MapReplace: row
                .columns[20]
                .into_u8()
                .copied()
                .expect("Expected column 20 to be a uint8!"),
            MapIndex: row
                .columns[3]
                .into_i8()
                .copied()
                .expect("Expected column 3 to be a int8!"),
            DiscoveryArrayByte: row
                .columns[17]
                .into_bool()
                .copied()
                .expect("Expected column 17 to be a bool!"),
            IsEvent: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            Unknown1: row
                .columns[19]
                .into_bool()
                .copied()
                .expect("Expected column 19 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a MapSheet {
    type Item = (u32, Vec<(u16, MapRow)>);
    type IntoIter = StructuredSheetIterator<'a, MapSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MapSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MapRow {
    ///""
    pub Id: String,
    ///""
    pub DiscoveryFlag: u32,
    ///""
    pub MapMarkerRange: u16,
    ///""
    pub SizeFactor: u16,
    ///""
    pub PlaceNameRegion: u16,
    ///""
    pub PlaceName: u16,
    ///""
    pub PlaceNameSub: u16,
    ///""
    pub TerritoryType: u16,
    ///""
    pub OffsetX: i16,
    ///""
    pub OffsetY: i16,
    ///""
    pub DiscoveryIndex: i16,
    ///""
    pub MapCondition: u8,
    ///""
    pub PriorityCategoryUI: u8,
    ///""
    pub PriorityUI: u8,
    ///""
    pub MapType: u8,
    ///""
    pub Unknown2: u8,
    ///""
    pub MapReplace: u8,
    ///""
    pub MapIndex: i8,
    ///""
    pub DiscoveryArrayByte: bool,
    ///""
    pub IsEvent: bool,
    ///""
    pub Unknown1: bool,
}
