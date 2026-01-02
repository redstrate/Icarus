//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct TerritoryTypeSheet {
    sheet: ExcelSheet,
}
impl TerritoryTypeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TerritoryType")?;
        let sheet = resolver.read_excel_sheet(exh, "TerritoryType", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<TerritoryTypeRow> {
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
        Some(TerritoryTypeRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<TerritoryTypeRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<TerritoryTypeRow> {
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
pub struct TerritoryTypeRow {
    columns: Vec<ColumnData>,
}
impl TerritoryTypeRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Bg<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn ArrayEventHandler<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn PlaceNameRegionIcon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn PlaceNameIcon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn Aetheryte<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn FixedTime<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn PlaceNameRegion<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn PlaceNameZone<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn PlaceName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn Map<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    pub fn ContentFinderCondition<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn BGM<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn QuestBattle<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn Resident<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn NotoriousMonsterTerritory<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn BattalionMode<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn LoadingImage<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn ExclusiveType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn TerritoryIntendedUse<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn WeatherRate<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn ExVersion<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn ZoneSharedGroup<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn AetherCurrentCompFlgSet<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn MountSpeed<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn IndividualWeather<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn AchievementIndex<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    pub fn PCSearch<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn Stealth<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn Mount<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn IsPvpZone<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn IsInUse<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a ColumnData {
        &self.columns[43]
    }
}
