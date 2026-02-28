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
pub struct TerritoryIntendedUseSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl TerritoryIntendedUseSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TerritoryIntendedUse")?;
        let sheet = resolver.read_excel_sheet(&exh, "TerritoryIntendedUse", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<TerritoryIntendedUseRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<TerritoryIntendedUseRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for TerritoryIntendedUseSheet {
    type Row = TerritoryIntendedUseRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a TerritoryIntendedUseSheet {
    type Item = (u32, Vec<(u16, TerritoryIntendedUseRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, TerritoryIntendedUseSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TerritoryIntendedUseSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TerritoryIntendedUseRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> TerritoryIntendedUseRow<'a> {
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    /// Ambient noise of people talking in a crowd. Values: 0 = None, 1 = sound/strm/GAYA_LestArea_01.scd, 2 = sound/strm/GAYA_Village_01.scd
    pub fn GayaSoundId(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn ChatRule(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn EnableCrafting(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn EnableGathering(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn EnableRepairs(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn EnableJobChangeToLimitedJob(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    /// Enables summoning Chocobo Companion
    pub fn EnableCompanion(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    /// Enables summoning Summoner/Scholar pets
    pub fn EnablePets(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn EnableRidePillion(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn DisableLogoutTimer(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn EnableTeleport(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn EnableReturn(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn Unknown16(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn Unknown17(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn EnableRecommendList(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn Unknown19(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn Unknown20(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    /// Related to Map in Island Sanctuary, Cosmic Exploration
    pub fn Unknown21(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn DisableFieldMarkers(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn Unknown23(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn EnableActions(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn EnableConsumableItems(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn Unknown26(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn EnableTripleTriadMatches(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn EnableTripleTriadMatchesAnywhere(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn Unknown29(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn Unknown30(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    /// Related to Idle Cam
    pub fn Unknown31(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn CanPauseTimeWeather(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn Unknown33(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn EnablePvPQuickChat(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn Unknown35(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn Unknown36(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn CanApplyGlamourPlatesAnywhere(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    /// Related to idle timer
    pub fn Unknown38(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn EnableFieldMarkerPresets(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn IgnoreCombatFlagForMarkers(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    /// Related to blocking Say/Yell/Shout of a muted player in Eureka/Bozja/OccultCrescent
    pub fn Unknown41(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn Unknown42(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
}
