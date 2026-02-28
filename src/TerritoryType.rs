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
pub struct TerritoryTypeSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl TerritoryTypeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TerritoryType")?;
        let sheet = resolver.read_excel_sheet(&exh, "TerritoryType", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<TerritoryTypeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<TerritoryTypeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for TerritoryTypeSheet {
    type Row = TerritoryTypeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a TerritoryTypeSheet {
    type Item = (u32, Vec<(u16, TerritoryTypeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, TerritoryTypeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TerritoryTypeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TerritoryTypeRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> TerritoryTypeRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Bg(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn ArrayEventHandler(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn PlaceNameRegionIcon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn PlaceNameIcon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn Aetheryte(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn FixedTime(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn PlaceNameRegion(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn PlaceNameZone(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn PlaceName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn Map(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn ContentFinderCondition(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn BGM(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn QuestBattle(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn Resident(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn NotoriousMonsterTerritory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn BattalionMode(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn LoadingImage(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn ExclusiveType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn TerritoryIntendedUse(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn WeatherRate(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn ExVersion(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn ZoneSharedGroup(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn AetherCurrentCompFlgSet(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn MountSpeed(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn IndividualWeather(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn AchievementIndex(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn PCSearch(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn Stealth(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn Mount(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn IsPvpZone(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn IsInUse(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn Unknown11(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn Unknown12(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn Unknown13(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn Unknown14(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn Unknown15(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn Unknown16(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
}
