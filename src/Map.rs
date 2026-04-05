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
impl<'a> StructuredSheet<'a> for MapSheet {
    type Row = MapRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a MapSheet {
    type Item = (u32, Vec<(u16, MapRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MapSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MapSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MapRow<'a> {
    row: &'a Row,
}
impl<'a> MapRow<'a> {
    pub fn Id(&'a self) -> &'a str {
        self.row.columns[6].into_string().unwrap()
    }
    pub fn DiscoveryFlag(&'a self) -> u32 {
        self.row.columns[15].into_u32().copied().unwrap()
    }
    pub fn MapMarkerRange(&'a self) -> u16 {
        self.row.columns[5].into_u16().copied().unwrap()
    }
    pub fn SizeFactor(&'a self) -> u16 {
        self.row.columns[7].into_u16().copied().unwrap()
    }
    pub fn PlaceNameRegion(&'a self) -> u16 {
        self.row.columns[10].into_u16().copied().unwrap()
    }
    pub fn PlaceName(&'a self) -> u16 {
        self.row.columns[11].into_u16().copied().unwrap()
    }
    pub fn PlaceNameSub(&'a self) -> u16 {
        self.row.columns[12].into_u16().copied().unwrap()
    }
    pub fn TerritoryType(&'a self) -> u16 {
        self.row.columns[16].into_u16().copied().unwrap()
    }
    pub fn OffsetX(&'a self) -> i16 {
        self.row.columns[8].into_i16().copied().unwrap()
    }
    pub fn OffsetY(&'a self) -> i16 {
        self.row.columns[9].into_i16().copied().unwrap()
    }
    pub fn DiscoveryIndex(&'a self) -> i16 {
        self.row.columns[14].into_i16().copied().unwrap()
    }
    pub fn MapCondition(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn PriorityCategoryUI(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn PriorityUI(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn MapType(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn MapReplace(&'a self) -> u8 {
        self.row.columns[20].into_u8().copied().unwrap()
    }
    pub fn MapIndex(&'a self) -> i8 {
        self.row.columns[3].into_i8().copied().unwrap()
    }
    pub fn DiscoveryArrayByte(&'a self) -> bool {
        self.row.columns[17].into_bool().copied().unwrap()
    }
    pub fn IsEvent(&'a self) -> bool {
        self.row.columns[18].into_bool().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> bool {
        self.row.columns[19].into_bool().copied().unwrap()
    }
}
