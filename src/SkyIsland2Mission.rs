//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct SkyIsland2MissionSheet {
    sheet: Sheet,
}
impl SkyIsland2MissionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SkyIsland2Mission")?;
        let sheet = resolver.read_excel_sheet(&exh, "SkyIsland2Mission", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<SkyIsland2MissionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SkyIsland2MissionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for SkyIsland2MissionSheet {
    type Row = SkyIsland2MissionRow;
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
impl<'a> IntoIterator for &'a SkyIsland2MissionSheet {
    type Item = (u32, Vec<(u16, SkyIsland2MissionRow)>);
    type IntoIter = StructuredSheetIterator<'a, SkyIsland2MissionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SkyIsland2MissionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SkyIsland2MissionRow {
    columns: Vec<Field>,
}
impl SkyIsland2MissionRow {
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn Item1<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn Item2<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn PopRange0<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn PopRange1<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn PopRange2<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn Image<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn PlaceName<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn Objective1<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn Objective2<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn Objective3<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn RequiredAmount1<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn RequiredAmount2<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
}
