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
        Some(Self::Row {
            Name: row
                .columns[43]
                .into_string()
                .cloned()
                .expect("Expected column 43 to be a string!"),
            NameShort: row
                .columns[44]
                .into_string()
                .cloned()
                .expect("Expected column 44 to be a string!"),
            LevelingRoulette: row
                .columns[58]
                .into_bool()
                .copied()
                .expect("Expected column 58 to be a bool!"),
            HighLevelRoulette: row
                .columns[59]
                .into_bool()
                .copied()
                .expect("Expected column 59 to be a bool!"),
            MSQRoulette: row
                .columns[60]
                .into_bool()
                .copied()
                .expect("Expected column 60 to be a bool!"),
            GuildHestRoulette: row
                .columns[61]
                .into_bool()
                .copied()
                .expect("Expected column 61 to be a bool!"),
            ExpertRoulette: row
                .columns[62]
                .into_bool()
                .copied()
                .expect("Expected column 62 to be a bool!"),
            TrialRoulette: row
                .columns[63]
                .into_bool()
                .copied()
                .expect("Expected column 63 to be a bool!"),
            DailyFrontlineChallenge: row
                .columns[64]
                .into_bool()
                .copied()
                .expect("Expected column 64 to be a bool!"),
            LevelCapRoulette: row
                .columns[65]
                .into_bool()
                .copied()
                .expect("Expected column 65 to be a bool!"),
            MentorRoulette: row
                .columns[66]
                .into_bool()
                .copied()
                .expect("Expected column 66 to be a bool!"),
            Unknown0: row
                .columns[67]
                .into_bool()
                .copied()
                .expect("Expected column 67 to be a bool!"),
            Unknown1: row
                .columns[68]
                .into_bool()
                .copied()
                .expect("Expected column 68 to be a bool!"),
            Unknown2: row
                .columns[69]
                .into_bool()
                .copied()
                .expect("Expected column 69 to be a bool!"),
            Unknown3: row
                .columns[70]
                .into_bool()
                .copied()
                .expect("Expected column 70 to be a bool!"),
            Unknown4: row
                .columns[71]
                .into_bool()
                .copied()
                .expect("Expected column 71 to be a bool!"),
            AllianceRoulette: row
                .columns[72]
                .into_bool()
                .copied()
                .expect("Expected column 72 to be a bool!"),
            FeastTeamRoulette: row
                .columns[73]
                .into_bool()
                .copied()
                .expect("Expected column 73 to be a bool!"),
            NormalRaidRoulette: row
                .columns[74]
                .into_bool()
                .copied()
                .expect("Expected column 74 to be a bool!"),
            Unknown5: row
                .columns[75]
                .into_bool()
                .copied()
                .expect("Expected column 75 to be a bool!"),
            Unknown6: row
                .columns[76]
                .into_bool()
                .copied()
                .expect("Expected column 76 to be a bool!"),
            Unknown7: row
                .columns[77]
                .into_bool()
                .copied()
                .expect("Expected column 77 to be a bool!"),
            Unknown8: row
                .columns[78]
                .into_bool()
                .copied()
                .expect("Expected column 78 to be a bool!"),
            Unknown9: row
                .columns[79]
                .into_bool()
                .copied()
                .expect("Expected column 79 to be a bool!"),
            Unknown10: row
                .columns[80]
                .into_bool()
                .copied()
                .expect("Expected column 80 to be a bool!"),
            Unknown11: row
                .columns[81]
                .into_bool()
                .copied()
                .expect("Expected column 81 to be a bool!"),
            Unknown12: row
                .columns[82]
                .into_bool()
                .copied()
                .expect("Expected column 82 to be a bool!"),
            Unknown13: row
                .columns[83]
                .into_bool()
                .copied()
                .expect("Expected column 83 to be a bool!"),
            Unknown14: row
                .columns[84]
                .into_bool()
                .copied()
                .expect("Expected column 84 to be a bool!"),
            Unknown15: row
                .columns[85]
                .into_bool()
                .copied()
                .expect("Expected column 85 to be a bool!"),
            Unknown16: row
                .columns[86]
                .into_bool()
                .copied()
                .expect("Expected column 86 to be a bool!"),
            Unknown17: row
                .columns[87]
                .into_bool()
                .copied()
                .expect("Expected column 87 to be a bool!"),
            Unknown18: row
                .columns[88]
                .into_bool()
                .copied()
                .expect("Expected column 88 to be a bool!"),
            Unknown19: row
                .columns[89]
                .into_bool()
                .copied()
                .expect("Expected column 89 to be a bool!"),
            Unknown20: row
                .columns[90]
                .into_bool()
                .copied()
                .expect("Expected column 90 to be a bool!"),
            Unknown21: row
                .columns[91]
                .into_bool()
                .copied()
                .expect("Expected column 91 to be a bool!"),
            Unknown22: row
                .columns[92]
                .into_bool()
                .copied()
                .expect("Expected column 92 to be a bool!"),
            Unknown23: row
                .columns[93]
                .into_bool()
                .copied()
                .expect("Expected column 93 to be a bool!"),
            Unknown24: row
                .columns[94]
                .into_bool()
                .copied()
                .expect("Expected column 94 to be a bool!"),
            Unknown25: row
                .columns[95]
                .into_bool()
                .copied()
                .expect("Expected column 95 to be a bool!"),
            Unknown26: row
                .columns[96]
                .into_bool()
                .copied()
                .expect("Expected column 96 to be a bool!"),
            CrystallineConflictCasualRoulette: row
                .columns[97]
                .into_bool()
                .copied()
                .expect("Expected column 97 to be a bool!"),
            CrystallineConflictRankedRoulette: row
                .columns[98]
                .into_bool()
                .copied()
                .expect("Expected column 98 to be a bool!"),
            ShortCode: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Unknown29: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
            ContentCloseCycle: row
                .columns[8]
                .into_u32()
                .copied()
                .expect("Expected column 8 to be a uint32!"),
            UnlockCriteria: row
                .columns[14]
                .into_u32()
                .copied()
                .expect("Expected column 14 to be a uint32!"),
            UnlockCriteria2: row
                .columns[16]
                .into_u32()
                .copied()
                .expect("Expected column 16 to be a uint32!"),
            JournalGenre: row
                .columns[47]
                .into_u32()
                .copied()
                .expect("Expected column 47 to be a uint32!"),
            Transient: row
                .columns[48]
                .into_u32()
                .copied()
                .expect("Expected column 48 to be a uint32!"),
            Image: row
                .columns[50]
                .into_u32()
                .copied()
                .expect("Expected column 50 to be a uint32!"),
            Icon: row
                .columns[51]
                .into_u32()
                .copied()
                .expect("Expected column 51 to be a uint32!"),
            Unknown32: row
                .columns[53]
                .into_i32()
                .copied()
                .expect("Expected column 53 to be a int32!"),
            Unknown58: row
                .columns[56]
                .into_i32()
                .copied()
                .expect("Expected column 56 to be a int32!"),
            TerritoryType: row
                .columns[1]
                .into_u16()
                .copied()
                .expect("Expected column 1 to be a uint16!"),
            Content: row
                .columns[3]
                .into_u16()
                .copied()
                .expect("Expected column 3 to be a uint16!"),
            ItemLevelRequired: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            ItemLevelSync: row
                .columns[20]
                .into_u16()
                .copied()
                .expect("Expected column 20 to be a uint16!"),
            SortKey: row
                .columns[49]
                .into_u16()
                .copied()
                .expect("Expected column 49 to be a uint16!"),
            ContentLinkType: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            RequiredExVersion: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            AcceptClassJobCategory: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            ContentMemberType: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            Unknown34: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            QueueMaxPlayers: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            UnlockType: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            UnlockType2: row
                .columns[15]
                .into_u8()
                .copied()
                .expect("Expected column 15 to be a uint8!"),
            ClassJobLevelRequired: row
                .columns[17]
                .into_u8()
                .copied()
                .expect("Expected column 17 to be a uint8!"),
            ClassJobLevelSync: row
                .columns[18]
                .into_u8()
                .copied()
                .expect("Expected column 18 to be a uint8!"),
            LootModeType: row
                .columns[30]
                .into_u8()
                .copied()
                .expect("Expected column 30 to be a uint8!"),
            RaidFinderParam: row
                .columns[35]
                .into_u8()
                .copied()
                .expect("Expected column 35 to be a uint8!"),
            ContentType: row
                .columns[45]
                .into_u8()
                .copied()
                .expect("Expected column 45 to be a uint8!"),
            ContentUICategory: row
                .columns[46]
                .into_u8()
                .copied()
                .expect("Expected column 46 to be a uint8!"),
            PenaltyTimestampArrayIndex: row
                .columns[57]
                .into_u8()
                .copied()
                .expect("Expected column 57 to be a uint8!"),
            Unknown41: row
                .columns[52]
                .into_i8()
                .copied()
                .expect("Expected column 52 to be a int8!"),
            PvP: row
                .columns[4]
                .into_bool()
                .copied()
                .expect("Expected column 4 to be a bool!"),
            Unknown_70_2: row
                .columns[5]
                .into_bool()
                .copied()
                .expect("Expected column 5 to be a bool!"),
            FixedItemLevelSync: row
                .columns[21]
                .into_bool()
                .copied()
                .expect("Expected column 21 to be a bool!"),
            AllowUndersized: row
                .columns[22]
                .into_bool()
                .copied()
                .expect("Expected column 22 to be a bool!"),
            NeedsMemberInEveryParty: row
                .columns[23]
                .into_bool()
                .copied()
                .expect("Expected column 23 to be a bool!"),
            Unknown57: row
                .columns[24]
                .into_bool()
                .copied()
                .expect("Expected column 24 to be a bool!"),
            AllowReplacement: row
                .columns[25]
                .into_bool()
                .copied()
                .expect("Expected column 25 to be a bool!"),
            AllowMinimumIL: row
                .columns[26]
                .into_bool()
                .copied()
                .expect("Expected column 26 to be a bool!"),
            AllowExplorerMode: row
                .columns[27]
                .into_bool()
                .copied()
                .expect("Expected column 27 to be a bool!"),
            RatedMatch: row
                .columns[28]
                .into_bool()
                .copied()
                .expect("Expected column 28 to be a bool!"),
            Rated: row
                .columns[29]
                .into_bool()
                .copied()
                .expect("Expected column 29 to be a bool!"),
            Unknown47: row
                .columns[31]
                .into_bool()
                .copied()
                .expect("Expected column 31 to be a bool!"),
            IsInDutyFinder: row
                .columns[32]
                .into_bool()
                .copied()
                .expect("Expected column 32 to be a bool!"),
            HighEndDuty: row
                .columns[33]
                .into_bool()
                .copied()
                .expect("Expected column 33 to be a bool!"),
            Unknown49: row
                .columns[34]
                .into_bool()
                .copied()
                .expect("Expected column 34 to be a bool!"),
            HasOnePlayerPerJobDetails: row
                .columns[36]
                .into_bool()
                .copied()
                .expect("Expected column 36 to be a bool!"),
            Unknown51: row
                .columns[37]
                .into_bool()
                .copied()
                .expect("Expected column 37 to be a bool!"),
            DutyRecorderAllowed: row
                .columns[38]
                .into_bool()
                .copied()
                .expect("Expected column 38 to be a bool!"),
            Unknown52: row
                .columns[39]
                .into_bool()
                .copied()
                .expect("Expected column 39 to be a bool!"),
            IsRegistrationHomeWorldLimited: row
                .columns[40]
                .into_bool()
                .copied()
                .expect("Expected column 40 to be a bool!"),
            Unknown54: row
                .columns[41]
                .into_bool()
                .copied()
                .expect("Expected column 41 to be a bool!"),
            Unknown55: row
                .columns[42]
                .into_bool()
                .copied()
                .expect("Expected column 42 to be a bool!"),
            Unknown56: row
                .columns[54]
                .into_bool()
                .copied()
                .expect("Expected column 54 to be a bool!"),
            IsRegistrationAllowedFromAnyDataCenter: row
                .columns[55]
                .into_bool()
                .copied()
                .expect("Expected column 55 to be a bool!"),
        })
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
#[derive(Clone, Debug, PartialEq)]
pub struct ContentFinderConditionRow {
    ///""
    pub Name: String,
    ///""
    pub NameShort: String,
    ///""
    pub LevelingRoulette: bool,
    ///""
    pub HighLevelRoulette: bool,
    ///""
    pub MSQRoulette: bool,
    ///""
    pub GuildHestRoulette: bool,
    ///""
    pub ExpertRoulette: bool,
    ///""
    pub TrialRoulette: bool,
    ///""
    pub DailyFrontlineChallenge: bool,
    ///""
    pub LevelCapRoulette: bool,
    ///""
    pub MentorRoulette: bool,
    ///""
    pub Unknown0: bool,
    ///""
    pub Unknown1: bool,
    ///""
    pub Unknown2: bool,
    ///""
    pub Unknown3: bool,
    ///""
    pub Unknown4: bool,
    ///""
    pub AllianceRoulette: bool,
    ///""
    pub FeastTeamRoulette: bool,
    ///""
    pub NormalRaidRoulette: bool,
    ///""
    pub Unknown5: bool,
    ///""
    pub Unknown6: bool,
    ///""
    pub Unknown7: bool,
    ///""
    pub Unknown8: bool,
    ///""
    pub Unknown9: bool,
    ///""
    pub Unknown10: bool,
    ///""
    pub Unknown11: bool,
    ///""
    pub Unknown12: bool,
    ///""
    pub Unknown13: bool,
    ///""
    pub Unknown14: bool,
    ///""
    pub Unknown15: bool,
    ///""
    pub Unknown16: bool,
    ///""
    pub Unknown17: bool,
    ///""
    pub Unknown18: bool,
    ///""
    pub Unknown19: bool,
    ///""
    pub Unknown20: bool,
    ///""
    pub Unknown21: bool,
    ///""
    pub Unknown22: bool,
    ///""
    pub Unknown23: bool,
    ///""
    pub Unknown24: bool,
    ///""
    pub Unknown25: bool,
    ///""
    pub Unknown26: bool,
    ///""
    pub CrystallineConflictCasualRoulette: bool,
    ///""
    pub CrystallineConflictRankedRoulette: bool,
    ///""
    pub ShortCode: String,
    ///"This would show Addon#102618, if not 0, but the row is empty."
    pub Unknown29: u32,
    ///""
    pub ContentCloseCycle: u32,
    ///""
    pub UnlockCriteria: u32,
    ///""
    pub UnlockCriteria2: u32,
    ///""
    pub JournalGenre: u32,
    ///""
    pub Transient: u32,
    ///""
    pub Image: u32,
    ///""
    pub Icon: u32,
    ///""
    pub Unknown32: i32,
    ///""
    pub Unknown58: i32,
    ///""
    pub TerritoryType: u16,
    ///""
    pub Content: u16,
    ///""
    pub ItemLevelRequired: u16,
    ///""
    pub ItemLevelSync: u16,
    ///""
    pub SortKey: u16,
    ///""
    pub ContentLinkType: u8,
    ///""
    pub RequiredExVersion: u8,
    ///""
    pub AcceptClassJobCategory: u8,
    ///""
    pub ContentMemberType: u8,
    ///"Used to control what is displayed as player count (Addon#10806, Addon#10805, etc.). Has some weird logic behind it."
    pub Unknown34: u8,
    ///""
    pub QueueMaxPlayers: u8,
    ///""
    pub UnlockType: u8,
    ///""
    pub UnlockType2: u8,
    ///""
    pub ClassJobLevelRequired: u8,
    ///""
    pub ClassJobLevelSync: u8,
    ///""
    pub LootModeType: u8,
    ///""
    pub RaidFinderParam: u8,
    ///""
    pub ContentType: u8,
    ///""
    pub ContentUICategory: u8,
    ///"Index in PlayerState.PenaltyTimestamps"
    pub PenaltyTimestampArrayIndex: u8,
    ///""
    pub Unknown41: i8,
    ///""
    pub PvP: bool,
    ///""
    pub Unknown_70_2: bool,
    ///"If true, the players item level is always set to ItemLevelSync (up-sync and down-sync possible)."
    pub FixedItemLevelSync: bool,
    ///""
    pub AllowUndersized: bool,
    ///""
    pub NeedsMemberInEveryParty: bool,
    ///""
    pub Unknown57: bool,
    ///""
    pub AllowReplacement: bool,
    ///""
    pub AllowMinimumIL: bool,
    ///""
    pub AllowExplorerMode: bool,
    ///""
    pub RatedMatch: bool,
    ///""
    pub Rated: bool,
    ///""
    pub Unknown47: bool,
    ///""
    pub IsInDutyFinder: bool,
    ///""
    pub HighEndDuty: bool,
    ///""
    pub Unknown49: bool,
    ///""
    pub HasOnePlayerPerJobDetails: bool,
    ///""
    pub Unknown51: bool,
    ///""
    pub DutyRecorderAllowed: bool,
    ///""
    pub Unknown52: bool,
    ///""
    pub IsRegistrationHomeWorldLimited: bool,
    ///""
    pub Unknown54: bool,
    ///""
    pub Unknown55: bool,
    ///""
    pub Unknown56: bool,
    ///""
    pub IsRegistrationAllowedFromAnyDataCenter: bool,
}
