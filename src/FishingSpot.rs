//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
#[derive(Debug, Clone)]
pub struct FishingSpotSheet {
    sheet: ExcelSheet,
}
impl FishingSpotSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("FishingSpot")?;
        let sheet = resolver.read_excel_sheet(exh, "FishingSpot", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<FishingSpotRow> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(FishingSpotRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<FishingSpotRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<FishingSpotRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => return None,
            ExcelRowKind::SubRows(subrows) => {
                &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
            }
        };
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
pub struct FishingSpotRow {
    columns: Vec<ColumnData>,
}
impl FishingSpotRow {
    pub fn BigFishOnReach<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn BigFishOnEnd<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn BigFishOnRefresh<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Item<'a>(&'a self) -> [&'a ColumnData; 10] {
        [
            &self.columns[3],
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
            &self.columns[9],
            &self.columns[10],
            &self.columns[11],
            &self.columns[12],
        ]
    }
    pub fn TerritoryType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn PlaceNameMain<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn PlaceNameSub<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn Radius<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn PlaceName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn Order<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn X<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn Z<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn GatheringLevel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn FishingSpotCategory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn Rare<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
}
