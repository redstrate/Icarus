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
pub struct ContentFinderConditionSheet {
    sheet: Sheet,
}
impl ContentFinderConditionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ContentFinderCondition")?;
        let sheet = resolver.read_excel_sheet(&exh, "ContentFinderCondition", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for ContentFinderConditionSheet {
    type Row = ContentFinderConditionRow;
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
impl<'a> IntoIterator for &'a ContentFinderConditionSheet {
    type Item = (u32, Vec<(u16, ContentFinderConditionRow)>);
    type IntoIter = StructuredSheetIterator<'a, ContentFinderConditionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ContentFinderConditionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ContentFinderConditionRow {
    columns: Vec<Field>,
}
impl ContentFinderConditionRow {
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn NameShort<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn LevelingRoulette<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn HighLevelRoulette<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn MSQRoulette<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn GuildHestRoulette<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn ExpertRoulette<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn TrialRoulette<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn DailyFrontlineChallenge<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn LevelCapRoulette<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn MentorRoulette<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn AllianceRoulette<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn FeastTeamRoulette<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn NormalRaidRoulette<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a Field {
        &self.columns[26]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a Field {
        &self.columns[27]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a Field {
        &self.columns[28]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a Field {
        &self.columns[29]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a Field {
        &self.columns[30]
    }
    pub fn Unknown17<'a>(&'a self) -> &'a Field {
        &self.columns[31]
    }
    pub fn Unknown18<'a>(&'a self) -> &'a Field {
        &self.columns[32]
    }
    pub fn Unknown19<'a>(&'a self) -> &'a Field {
        &self.columns[33]
    }
    pub fn Unknown20<'a>(&'a self) -> &'a Field {
        &self.columns[34]
    }
    pub fn Unknown21<'a>(&'a self) -> &'a Field {
        &self.columns[35]
    }
    pub fn Unknown22<'a>(&'a self) -> &'a Field {
        &self.columns[36]
    }
    pub fn Unknown23<'a>(&'a self) -> &'a Field {
        &self.columns[37]
    }
    pub fn Unknown24<'a>(&'a self) -> &'a Field {
        &self.columns[38]
    }
    pub fn Unknown25<'a>(&'a self) -> &'a Field {
        &self.columns[39]
    }
    pub fn Unknown26<'a>(&'a self) -> &'a Field {
        &self.columns[40]
    }
    pub fn Unknown27<'a>(&'a self) -> &'a Field {
        &self.columns[41]
    }
    pub fn Unknown28<'a>(&'a self) -> &'a Field {
        &self.columns[42]
    }
    pub fn ShortCode<'a>(&'a self) -> &'a Field {
        &self.columns[43]
    }
    /// This would show Addon#102618, if not 0, but the row is empty.
    pub fn Unknown29<'a>(&'a self) -> &'a Field {
        &self.columns[44]
    }
    pub fn ContentCloseCycle<'a>(&'a self) -> &'a Field {
        &self.columns[45]
    }
    pub fn UnlockCriteria<'a>(&'a self) -> &'a Field {
        &self.columns[46]
    }
    pub fn UnlockCriteria2<'a>(&'a self) -> &'a Field {
        &self.columns[47]
    }
    pub fn JournalGenre<'a>(&'a self) -> &'a Field {
        &self.columns[48]
    }
    pub fn Transient<'a>(&'a self) -> &'a Field {
        &self.columns[49]
    }
    pub fn Image<'a>(&'a self) -> &'a Field {
        &self.columns[50]
    }
    pub fn Icon<'a>(&'a self) -> &'a Field {
        &self.columns[51]
    }
    pub fn Unknown32<'a>(&'a self) -> &'a Field {
        &self.columns[52]
    }
    pub fn Unknown58<'a>(&'a self) -> &'a Field {
        &self.columns[53]
    }
    pub fn TerritoryType<'a>(&'a self) -> &'a Field {
        &self.columns[54]
    }
    pub fn Content<'a>(&'a self) -> &'a Field {
        &self.columns[55]
    }
    pub fn ItemLevelRequired<'a>(&'a self) -> &'a Field {
        &self.columns[56]
    }
    pub fn ItemLevelSync<'a>(&'a self) -> &'a Field {
        &self.columns[57]
    }
    pub fn SortKey<'a>(&'a self) -> &'a Field {
        &self.columns[58]
    }
    pub fn ContentLinkType<'a>(&'a self) -> &'a Field {
        &self.columns[59]
    }
    pub fn RequiredExVersion<'a>(&'a self) -> &'a Field {
        &self.columns[60]
    }
    pub fn AcceptClassJobCategory<'a>(&'a self) -> &'a Field {
        &self.columns[61]
    }
    pub fn ContentMemberType<'a>(&'a self) -> &'a Field {
        &self.columns[62]
    }
    /// Used to control what is displayed as player count (Addon#10806, Addon#10805, etc.). Has some weird logic behind it.
    pub fn Unknown34<'a>(&'a self) -> &'a Field {
        &self.columns[63]
    }
    pub fn QueueMaxPlayers<'a>(&'a self) -> &'a Field {
        &self.columns[64]
    }
    pub fn UnlockType<'a>(&'a self) -> &'a Field {
        &self.columns[65]
    }
    pub fn UnlockType2<'a>(&'a self) -> &'a Field {
        &self.columns[66]
    }
    pub fn ClassJobLevelRequired<'a>(&'a self) -> &'a Field {
        &self.columns[67]
    }
    pub fn ClassJobLevelSync<'a>(&'a self) -> &'a Field {
        &self.columns[68]
    }
    pub fn LootModeType<'a>(&'a self) -> &'a Field {
        &self.columns[69]
    }
    pub fn RaidFinderParam<'a>(&'a self) -> &'a Field {
        &self.columns[70]
    }
    pub fn ContentType<'a>(&'a self) -> &'a Field {
        &self.columns[71]
    }
    pub fn ContentUICategory<'a>(&'a self) -> &'a Field {
        &self.columns[72]
    }
    /// Index in PlayerState.PenaltyTimestamps
    pub fn PenaltyTimestampArrayIndex<'a>(&'a self) -> &'a Field {
        &self.columns[73]
    }
    pub fn Unknown41<'a>(&'a self) -> &'a Field {
        &self.columns[74]
    }
    pub fn PvP<'a>(&'a self) -> &'a Field {
        &self.columns[75]
    }
    pub fn Unknown_70_2<'a>(&'a self) -> &'a Field {
        &self.columns[76]
    }
    /// If true, the players item level is always set to ItemLevelSync (up-sync and down-sync possible).
    pub fn FixedItemLevelSync<'a>(&'a self) -> &'a Field {
        &self.columns[77]
    }
    pub fn AllowUndersized<'a>(&'a self) -> &'a Field {
        &self.columns[78]
    }
    pub fn NeedsMemberInEveryParty<'a>(&'a self) -> &'a Field {
        &self.columns[79]
    }
    pub fn Unknown57<'a>(&'a self) -> &'a Field {
        &self.columns[80]
    }
    pub fn AllowReplacement<'a>(&'a self) -> &'a Field {
        &self.columns[81]
    }
    pub fn AllowMinimumIL<'a>(&'a self) -> &'a Field {
        &self.columns[82]
    }
    pub fn AllowExplorerMode<'a>(&'a self) -> &'a Field {
        &self.columns[83]
    }
    pub fn RatedMatch<'a>(&'a self) -> &'a Field {
        &self.columns[84]
    }
    pub fn Rated<'a>(&'a self) -> &'a Field {
        &self.columns[85]
    }
    pub fn Unknown47<'a>(&'a self) -> &'a Field {
        &self.columns[86]
    }
    pub fn IsInDutyFinder<'a>(&'a self) -> &'a Field {
        &self.columns[87]
    }
    pub fn HighEndDuty<'a>(&'a self) -> &'a Field {
        &self.columns[88]
    }
    pub fn Unknown49<'a>(&'a self) -> &'a Field {
        &self.columns[89]
    }
    pub fn HasOnePlayerPerJobDetails<'a>(&'a self) -> &'a Field {
        &self.columns[90]
    }
    pub fn Unknown51<'a>(&'a self) -> &'a Field {
        &self.columns[91]
    }
    pub fn DutyRecorderAllowed<'a>(&'a self) -> &'a Field {
        &self.columns[92]
    }
    pub fn Unknown52<'a>(&'a self) -> &'a Field {
        &self.columns[93]
    }
    pub fn IsRegistrationHomeWorldLimited<'a>(&'a self) -> &'a Field {
        &self.columns[94]
    }
    pub fn Unknown54<'a>(&'a self) -> &'a Field {
        &self.columns[95]
    }
    pub fn Unknown55<'a>(&'a self) -> &'a Field {
        &self.columns[96]
    }
    pub fn Unknown56<'a>(&'a self) -> &'a Field {
        &self.columns[97]
    }
    pub fn IsRegistrationAllowedFromAnyDataCenter<'a>(&'a self) -> &'a Field {
        &self.columns[98]
    }
}
