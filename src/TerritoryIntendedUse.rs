//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct TerritoryIntendedUseSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl TerritoryIntendedUseSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "TerritoryIntendedUse")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(
                    read_excel_sheet(
                        resource,
                        "TerritoryIntendedUse",
                        &exh,
                        language,
                        i,
                    )?,
                );
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<TerritoryIntendedUseRow> {
        let column_defs = &self.exh.column_definitions;
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
        Some(TerritoryIntendedUseRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<TerritoryIntendedUseRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => row,
                ExcelRowKind::SubRows(rows) => &rows.first()?.1,
            };
            return self.read_row(row);
        }
        None
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<TerritoryIntendedUseRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => return None,
                ExcelRowKind::SubRows(subrows) => {
                    &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
                }
            };
            return self.read_row(row);
        }
        None
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.row_count
    }
}
pub struct TerritoryIntendedUseRow {
    columns: Vec<ColumnData>,
}
impl TerritoryIntendedUseRow {
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    /// Ambient noise of people talking in a crowd. Values: 0 = None, 1 = sound/strm/GAYA_LestArea_01.scd, 2 = sound/strm/GAYA_Village_01.scd
    pub fn GayaSoundId<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn ChatRule<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[3]
    }
    pub fn EnableCrafting<'a>(&'a self) -> &'a ColumnData {
        &self.columns[4]
    }
    pub fn EnableGathering<'a>(&'a self) -> &'a ColumnData {
        &self.columns[5]
    }
    pub fn EnableRepairs<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn EnableJobChangeToLimitedJob<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    /// Enables summoning Chocobo Companion
    pub fn EnableCompanion<'a>(&'a self) -> &'a ColumnData {
        &self.columns[10]
    }
    /// Enables summoning Summoner/Scholar pets
    pub fn EnablePets<'a>(&'a self) -> &'a ColumnData {
        &self.columns[11]
    }
    pub fn EnableRidePillion<'a>(&'a self) -> &'a ColumnData {
        &self.columns[12]
    }
    pub fn DisableLogoutTimer<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn EnableTeleport<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn EnableReturn<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a ColumnData {
        &self.columns[16]
    }
    pub fn Unknown17<'a>(&'a self) -> &'a ColumnData {
        &self.columns[17]
    }
    pub fn EnableRecommendList<'a>(&'a self) -> &'a ColumnData {
        &self.columns[18]
    }
    pub fn Unknown19<'a>(&'a self) -> &'a ColumnData {
        &self.columns[19]
    }
    pub fn Unknown20<'a>(&'a self) -> &'a ColumnData {
        &self.columns[20]
    }
    /// Related to Map in Island Sanctuary, Cosmic Exploration
    pub fn Unknown21<'a>(&'a self) -> &'a ColumnData {
        &self.columns[21]
    }
    pub fn DisableFieldMarkers<'a>(&'a self) -> &'a ColumnData {
        &self.columns[22]
    }
    pub fn Unknown23<'a>(&'a self) -> &'a ColumnData {
        &self.columns[23]
    }
    pub fn EnableActions<'a>(&'a self) -> &'a ColumnData {
        &self.columns[24]
    }
    pub fn EnableConsumableItems<'a>(&'a self) -> &'a ColumnData {
        &self.columns[25]
    }
    pub fn Unknown26<'a>(&'a self) -> &'a ColumnData {
        &self.columns[26]
    }
    pub fn EnableTripleTriadMatches<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
    pub fn EnableTripleTriadMatchesAnywhere<'a>(&'a self) -> &'a ColumnData {
        &self.columns[28]
    }
    pub fn Unknown29<'a>(&'a self) -> &'a ColumnData {
        &self.columns[29]
    }
    pub fn Unknown30<'a>(&'a self) -> &'a ColumnData {
        &self.columns[30]
    }
    /// Related to Idle Cam
    pub fn Unknown31<'a>(&'a self) -> &'a ColumnData {
        &self.columns[31]
    }
    pub fn CanPauseTimeWeather<'a>(&'a self) -> &'a ColumnData {
        &self.columns[32]
    }
    pub fn Unknown33<'a>(&'a self) -> &'a ColumnData {
        &self.columns[33]
    }
    pub fn EnablePvPQuickChat<'a>(&'a self) -> &'a ColumnData {
        &self.columns[34]
    }
    pub fn Unknown35<'a>(&'a self) -> &'a ColumnData {
        &self.columns[35]
    }
    pub fn Unknown36<'a>(&'a self) -> &'a ColumnData {
        &self.columns[36]
    }
    pub fn CanApplyGlamourPlatesAnywhere<'a>(&'a self) -> &'a ColumnData {
        &self.columns[37]
    }
    /// Related to idle timer
    pub fn Unknown38<'a>(&'a self) -> &'a ColumnData {
        &self.columns[38]
    }
    pub fn EnableFieldMarkerPresets<'a>(&'a self) -> &'a ColumnData {
        &self.columns[39]
    }
    pub fn IgnoreCombatFlagForMarkers<'a>(&'a self) -> &'a ColumnData {
        &self.columns[40]
    }
    /// Related to blocking Say/Yell/Shout of a muted player in Eureka/Bozja/OccultCrescent
    pub fn Unknown41<'a>(&'a self) -> &'a ColumnData {
        &self.columns[41]
    }
    pub fn Unknown42<'a>(&'a self) -> &'a ColumnData {
        &self.columns[42]
    }
}
