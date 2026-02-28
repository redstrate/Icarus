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
pub struct ContentFinderConditionSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl ContentFinderConditionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ContentFinderCondition")?;
        let sheet = resolver.read_excel_sheet(&exh, "ContentFinderCondition", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<ContentFinderConditionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<ContentFinderConditionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ContentFinderConditionSheet {
    type Row = ContentFinderConditionRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a ContentFinderConditionSheet {
    type Item = (u32, Vec<(u16, ContentFinderConditionRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ContentFinderConditionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ContentFinderConditionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ContentFinderConditionRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> ContentFinderConditionRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn NameShort(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn LevelingRoulette(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn HighLevelRoulette(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn MSQRoulette(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn GuildHestRoulette(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn ExpertRoulette(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn TrialRoulette(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn DailyFrontlineChallenge(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn LevelCapRoulette(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn MentorRoulette(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn AllianceRoulette(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn FeastTeamRoulette(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn NormalRaidRoulette(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn Unknown10(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn Unknown11(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn Unknown12(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn Unknown13(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn Unknown14(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn Unknown15(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn Unknown16(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn Unknown17(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn Unknown18(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn Unknown19(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn Unknown20(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn Unknown21(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn Unknown22(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn Unknown23(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn Unknown24(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn Unknown25(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn Unknown26(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn Unknown27(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn Unknown28(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn ShortCode(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    /// This would show Addon#102618, if not 0, but the row is empty.
    pub fn Unknown29(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    pub fn ContentCloseCycle(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn UnlockCriteria(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
    pub fn UnlockCriteria2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[47]]
    }
    pub fn JournalGenre(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[48]]
    }
    pub fn Transient(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[49]]
    }
    pub fn Image(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[50]]
    }
    pub fn Icon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[51]]
    }
    pub fn Unknown32(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[52]]
    }
    pub fn Unknown58(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[53]]
    }
    pub fn TerritoryType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[54]]
    }
    pub fn Content(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[55]]
    }
    pub fn ItemLevelRequired(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[56]]
    }
    pub fn ItemLevelSync(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[57]]
    }
    pub fn SortKey(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[58]]
    }
    pub fn ContentLinkType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[59]]
    }
    pub fn RequiredExVersion(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[60]]
    }
    pub fn AcceptClassJobCategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[61]]
    }
    pub fn ContentMemberType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[62]]
    }
    /// Used to control what is displayed as player count (Addon#10806, Addon#10805, etc.). Has some weird logic behind it.
    pub fn Unknown34(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[63]]
    }
    pub fn QueueMaxPlayers(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[64]]
    }
    pub fn UnlockType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[65]]
    }
    pub fn UnlockType2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[66]]
    }
    pub fn ClassJobLevelRequired(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[67]]
    }
    pub fn ClassJobLevelSync(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[68]]
    }
    pub fn LootModeType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[69]]
    }
    pub fn RaidFinderParam(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[70]]
    }
    pub fn ContentType(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[71]]
    }
    pub fn ContentUICategory(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[72]]
    }
    /// Index in PlayerState.PenaltyTimestamps
    pub fn PenaltyTimestampArrayIndex(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[73]]
    }
    pub fn Unknown41(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[74]]
    }
    pub fn PvP(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[75]]
    }
    pub fn Unknown_70_2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[76]]
    }
    /// If true, the players item level is always set to ItemLevelSync (up-sync and down-sync possible).
    pub fn FixedItemLevelSync(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[77]]
    }
    pub fn AllowUndersized(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[78]]
    }
    pub fn NeedsMemberInEveryParty(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[79]]
    }
    pub fn Unknown57(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[80]]
    }
    pub fn AllowReplacement(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[81]]
    }
    pub fn AllowMinimumIL(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[82]]
    }
    pub fn AllowExplorerMode(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[83]]
    }
    pub fn RatedMatch(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[84]]
    }
    pub fn Rated(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[85]]
    }
    pub fn Unknown47(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[86]]
    }
    pub fn IsInDutyFinder(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[87]]
    }
    pub fn HighEndDuty(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[88]]
    }
    pub fn Unknown49(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[89]]
    }
    pub fn HasOnePlayerPerJobDetails(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[90]]
    }
    pub fn Unknown51(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[91]]
    }
    pub fn DutyRecorderAllowed(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[92]]
    }
    pub fn Unknown52(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[93]]
    }
    pub fn IsRegistrationHomeWorldLimited(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[94]]
    }
    pub fn Unknown54(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[95]]
    }
    pub fn Unknown55(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[96]]
    }
    pub fn Unknown56(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[97]]
    }
    pub fn IsRegistrationAllowedFromAnyDataCenter(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[98]]
    }
}
