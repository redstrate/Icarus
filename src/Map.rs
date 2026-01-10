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
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
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
#[derive(Debug, Clone)]
pub struct MapRow {
    columns: Vec<Field>,
}
impl MapRow {
    pub fn Id<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn DiscoveryFlag<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn MapMarkerRange<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn SizeFactor<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn PlaceNameRegion<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn PlaceName<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn PlaceNameSub<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn TerritoryType<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn OffsetX<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn OffsetY<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn DiscoveryIndex<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn MapCondition<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn PriorityCategoryUI<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn PriorityUI<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn MapType<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn MapReplace<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn MapIndex<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn DiscoveryArrayByte<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn IsEvent<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
}
