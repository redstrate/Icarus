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
pub struct ContentRouletteSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl ContentRouletteSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ContentRoulette")?;
        let sheet = resolver.read_excel_sheet(&exh, "ContentRoulette", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<ContentRouletteRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ContentRouletteRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ContentRouletteSheet {
    type Row = ContentRouletteRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a ContentRouletteSheet {
    type Item = (u32, Vec<(u16, ContentRouletteRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ContentRouletteSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ContentRouletteSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ContentRouletteRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> ContentRouletteRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Category(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn Description(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn DutyType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    /// This would show Addon#102618, but the row is empty.
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn Image(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn ItemLevelRequired(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn ItemLevelSync(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn RewardTomeA(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn RewardTomeB(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn RewardTomeC(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn InstanceContent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn RequiredExVersion(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn OpenRule(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn RequiredLevel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn SyncedFromLevel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn ContentRouletteRoleBonus(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn SortKey(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn ClassJobCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn ContentMemberType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn QueueMaxPlayers(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn ContentType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn Unknown12(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    /// In minutes.
    pub fn TimeLimit(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    /// In minutes. If 0, only TimeLimit is displayed.
    pub fn TimeLimitMax(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn LootModeType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn Unknown15(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    /// Index in PlayerState.PenaltyTimestamps
    pub fn PenaltyTimestampArrayIndex(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    /// Index in PlayerState.ContentRouletteCompletion
    pub fn CompletionArrayIndex(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn IsGoldSaucer(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn IsInDutyFinder(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn IsPvP(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    /// Displays Addon#2828.
    pub fn AppliesHighestAverageDutyItemLevel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn Unknown18(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn AllowConsumableItems(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn AllowPhoenixDown(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn AllowReplacement(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn RatedMatch(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    pub fn Rated(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    /// This would show Addon#10833, but the row does not exist.
    pub fn Unknown22(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn Unknown23(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
    pub fn Unknown24(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[47]]
    }
    pub fn IsRegistrationAllowedFromAnyDataCenter(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[48]]
    }
}
