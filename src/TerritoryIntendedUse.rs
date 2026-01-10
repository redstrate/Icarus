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
}
impl TerritoryIntendedUseSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TerritoryIntendedUse")?;
        let sheet = resolver.read_excel_sheet(&exh, "TerritoryIntendedUse", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for TerritoryIntendedUseSheet {
    type Row = TerritoryIntendedUseRow;
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
impl<'a> IntoIterator for &'a TerritoryIntendedUseSheet {
    type Item = (u32, Vec<(u16, TerritoryIntendedUseRow)>);
    type IntoIter = StructuredSheetIterator<'a, TerritoryIntendedUseSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TerritoryIntendedUseSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TerritoryIntendedUseRow {
    columns: Vec<Field>,
}
impl TerritoryIntendedUseRow {
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    /// Ambient noise of people talking in a crowd. Values: 0 = None, 1 = sound/strm/GAYA_LestArea_01.scd, 2 = sound/strm/GAYA_Village_01.scd
    pub fn GayaSoundId<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn ChatRule<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn EnableCrafting<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn EnableGathering<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn EnableRepairs<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn EnableJobChangeToLimitedJob<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    /// Enables summoning Chocobo Companion
    pub fn EnableCompanion<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    /// Enables summoning Summoner/Scholar pets
    pub fn EnablePets<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn EnableRidePillion<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn DisableLogoutTimer<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn EnableTeleport<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn EnableReturn<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn Unknown17<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn EnableRecommendList<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn Unknown19<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn Unknown20<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    /// Related to Map in Island Sanctuary, Cosmic Exploration
    pub fn Unknown21<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn DisableFieldMarkers<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn Unknown23<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn EnableActions<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn EnableConsumableItems<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
    pub fn Unknown26<'a>(&'a self) -> &'a Field {
        &self.columns[26]
    }
    pub fn EnableTripleTriadMatches<'a>(&'a self) -> &'a Field {
        &self.columns[27]
    }
    pub fn EnableTripleTriadMatchesAnywhere<'a>(&'a self) -> &'a Field {
        &self.columns[28]
    }
    pub fn Unknown29<'a>(&'a self) -> &'a Field {
        &self.columns[29]
    }
    pub fn Unknown30<'a>(&'a self) -> &'a Field {
        &self.columns[30]
    }
    /// Related to Idle Cam
    pub fn Unknown31<'a>(&'a self) -> &'a Field {
        &self.columns[31]
    }
    pub fn CanPauseTimeWeather<'a>(&'a self) -> &'a Field {
        &self.columns[32]
    }
    pub fn Unknown33<'a>(&'a self) -> &'a Field {
        &self.columns[33]
    }
    pub fn EnablePvPQuickChat<'a>(&'a self) -> &'a Field {
        &self.columns[34]
    }
    pub fn Unknown35<'a>(&'a self) -> &'a Field {
        &self.columns[35]
    }
    pub fn Unknown36<'a>(&'a self) -> &'a Field {
        &self.columns[36]
    }
    pub fn CanApplyGlamourPlatesAnywhere<'a>(&'a self) -> &'a Field {
        &self.columns[37]
    }
    /// Related to idle timer
    pub fn Unknown38<'a>(&'a self) -> &'a Field {
        &self.columns[38]
    }
    pub fn EnableFieldMarkerPresets<'a>(&'a self) -> &'a Field {
        &self.columns[39]
    }
    pub fn IgnoreCombatFlagForMarkers<'a>(&'a self) -> &'a Field {
        &self.columns[40]
    }
    /// Related to blocking Say/Yell/Shout of a muted player in Eureka/Bozja/OccultCrescent
    pub fn Unknown41<'a>(&'a self) -> &'a Field {
        &self.columns[41]
    }
    pub fn Unknown42<'a>(&'a self) -> &'a Field {
        &self.columns[42]
    }
}
