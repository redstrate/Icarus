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
pub struct InstanceContentSheet {
    sheet: Sheet,
}
impl InstanceContentSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("InstanceContent")?;
        let sheet = resolver.read_excel_sheet(&exh, "InstanceContent", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<InstanceContentRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<InstanceContentRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for InstanceContentSheet {
    type Row = InstanceContentRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            NewPlayerBonusGil: row
                .columns[16]
                .into_u32()
                .copied()
                .expect("Expected column 16 to be a uint32!"),
            NewPlayerBonusExp: row
                .columns[17]
                .into_u32()
                .copied()
                .expect("Expected column 17 to be a uint32!"),
            FinalBossExp: row
                .columns[20]
                .into_u32()
                .copied()
                .expect("Expected column 20 to be a uint32!"),
            Unknown0: row
                .columns[22]
                .into_u32()
                .copied()
                .expect("Expected column 22 to be a uint32!"),
            BossExp: [
                row
                    .columns[26]
                    .into_u32()
                    .copied()
                    .expect("Expected column 26 to be a uint32!"),
                row
                    .columns[27]
                    .into_u32()
                    .copied()
                    .expect("Expected column 27 to be a uint32!"),
                row
                    .columns[28]
                    .into_u32()
                    .copied()
                    .expect("Expected column 28 to be a uint32!"),
                row
                    .columns[29]
                    .into_u32()
                    .copied()
                    .expect("Expected column 29 to be a uint32!"),
                row
                    .columns[30]
                    .into_u32()
                    .copied()
                    .expect("Expected column 30 to be a uint32!"),
            ],
            InstanceClearExp: row
                .columns[46]
                .into_u32()
                .copied()
                .expect("Expected column 46 to be a uint32!"),
            InstanceClearGil: row
                .columns[47]
                .into_u32()
                .copied()
                .expect("Expected column 47 to be a uint32!"),
            InstanceContentRewardItem: row
                .columns[48]
                .into_u32()
                .copied()
                .expect("Expected column 48 to be a uint32!"),
            NewPlayerBonusA: row
                .columns[18]
                .into_u16()
                .copied()
                .expect("Expected column 18 to be a uint16!"),
            NewPlayerBonusB: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            FinalBossCurrencyA: row
                .columns[23]
                .into_u16()
                .copied()
                .expect("Expected column 23 to be a uint16!"),
            FinalBossCurrencyB: row
                .columns[24]
                .into_u16()
                .copied()
                .expect("Expected column 24 to be a uint16!"),
            FinalBossCurrencyC: row
                .columns[25]
                .into_u16()
                .copied()
                .expect("Expected column 25 to be a uint16!"),
            BossCurrencyA: [
                row
                    .columns[31]
                    .into_u16()
                    .copied()
                    .expect("Expected column 31 to be a uint16!"),
                row
                    .columns[32]
                    .into_u16()
                    .copied()
                    .expect("Expected column 32 to be a uint16!"),
                row
                    .columns[33]
                    .into_u16()
                    .copied()
                    .expect("Expected column 33 to be a uint16!"),
                row
                    .columns[34]
                    .into_u16()
                    .copied()
                    .expect("Expected column 34 to be a uint16!"),
                row
                    .columns[35]
                    .into_u16()
                    .copied()
                    .expect("Expected column 35 to be a uint16!"),
            ],
            BossCurrencyB: [
                row
                    .columns[36]
                    .into_u16()
                    .copied()
                    .expect("Expected column 36 to be a uint16!"),
                row
                    .columns[37]
                    .into_u16()
                    .copied()
                    .expect("Expected column 37 to be a uint16!"),
                row
                    .columns[38]
                    .into_u16()
                    .copied()
                    .expect("Expected column 38 to be a uint16!"),
                row
                    .columns[39]
                    .into_u16()
                    .copied()
                    .expect("Expected column 39 to be a uint16!"),
                row
                    .columns[40]
                    .into_u16()
                    .copied()
                    .expect("Expected column 40 to be a uint16!"),
            ],
            BossCurrencyC: [
                row
                    .columns[41]
                    .into_u16()
                    .copied()
                    .expect("Expected column 41 to be a uint16!"),
                row
                    .columns[42]
                    .into_u16()
                    .copied()
                    .expect("Expected column 42 to be a uint16!"),
                row
                    .columns[43]
                    .into_u16()
                    .copied()
                    .expect("Expected column 43 to be a uint16!"),
                row
                    .columns[44]
                    .into_u16()
                    .copied()
                    .expect("Expected column 44 to be a uint16!"),
                row
                    .columns[45]
                    .into_u16()
                    .copied()
                    .expect("Expected column 45 to be a uint16!"),
            ],
            Unknown1: row
                .columns[49]
                .into_u16()
                .copied()
                .expect("Expected column 49 to be a uint16!"),
            Unknown8: row
                .columns[21]
                .into_u8()
                .copied()
                .expect("Expected column 21 to be a uint8!"),
            LimitedTimeBonus: row
                .columns[50]
                .into_bool()
                .copied()
                .expect("Expected column 50 to be a bool!"),
            Cutscene: row
                .columns[6]
                .into_u32()
                .copied()
                .expect("Expected column 6 to be a uint32!"),
            LGBEventRange: row
                .columns[7]
                .into_u32()
                .copied()
                .expect("Expected column 7 to be a uint32!"),
            InstanceContentTextDataBossStart: row
                .columns[10]
                .into_u32()
                .copied()
                .expect("Expected column 10 to be a uint32!"),
            InstanceContentTextDataBossEnd: row
                .columns[11]
                .into_u32()
                .copied()
                .expect("Expected column 11 to be a uint32!"),
            BNpcBaseBoss: row
                .columns[12]
                .into_u32()
                .copied()
                .expect("Expected column 12 to be a uint32!"),
            InstanceContentTextDataObjectiveStart: row
                .columns[13]
                .into_u32()
                .copied()
                .expect("Expected column 13 to be a uint32!"),
            InstanceContentTextDataObjectiveEnd: row
                .columns[14]
                .into_u32()
                .copied()
                .expect("Expected column 14 to be a uint32!"),
            Unknown2: row
                .columns[51]
                .into_u32()
                .copied()
                .expect("Expected column 51 to be a uint32!"),
            ReqInstance: row
                .columns[54]
                .into_u32()
                .copied()
                .expect("Expected column 54 to be a uint32!"),
            InstanceContentBuff: row
                .columns[52]
                .into_i32()
                .copied()
                .expect("Expected column 52 to be a int32!"),
            TimeLimitmin: row
                .columns[2]
                .into_u16()
                .copied()
                .expect("Expected column 2 to be a uint16!"),
            BGM: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            WinBGM: row
                .columns[5]
                .into_u16()
                .copied()
                .expect("Expected column 5 to be a uint16!"),
            ContentFinderCondition: row
                .columns[8]
                .into_u16()
                .copied()
                .expect("Expected column 8 to be a uint16!"),
            SortKey: row
                .columns[15]
                .into_u16()
                .copied()
                .expect("Expected column 15 to be a uint16!"),
            ContentRoute: row
                .columns[62]
                .into_u16()
                .copied()
                .expect("Expected column 62 to be a uint16!"),
            ContentDirectorManagedSG: row
                .columns[63]
                .into_u16()
                .copied()
                .expect("Expected column 63 to be a uint16!"),
            ContentTodo: row
                .columns[64]
                .into_u16()
                .copied()
                .expect("Expected column 64 to be a uint16!"),
            Unknown6: row
                .columns[66]
                .into_u16()
                .copied()
                .expect("Expected column 66 to be a uint16!"),
            LookupIndex: row
                .columns[67]
                .into_u16()
                .copied()
                .expect("Expected column 67 to be a uint16!"),
            ContentEventItem: row
                .columns[69]
                .into_u16()
                .copied()
                .expect("Expected column 69 to be a uint16!"),
            ContentDirectorBattleTalk: row
                .columns[70]
                .into_u16()
                .copied()
                .expect("Expected column 70 to be a uint16!"),
            PartyCondition: row
                .columns[55]
                .into_i16()
                .copied()
                .expect("Expected column 55 to be a int16!"),
            InstanceContentType: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            WeekRestriction: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown9: row
                .columns[56]
                .into_u8()
                .copied()
                .expect("Expected column 56 to be a uint8!"),
            QTE1: row
                .columns[57]
                .into_u8()
                .copied()
                .expect("Expected column 57 to be a uint8!"),
            QTE2: row
                .columns[58]
                .into_u8()
                .copied()
                .expect("Expected column 58 to be a uint8!"),
            Unknown12: row
                .columns[59]
                .into_u8()
                .copied()
                .expect("Expected column 59 to be a uint8!"),
            ContentAttributeRect: row
                .columns[60]
                .into_u8()
                .copied()
                .expect("Expected column 60 to be a uint8!"),
            Unknown13: row
                .columns[65]
                .into_u8()
                .copied()
                .expect("Expected column 65 to be a uint8!"),
            Unknown14: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
            Unknown15: row
                .columns[9]
                .into_bool()
                .copied()
                .expect("Expected column 9 to be a bool!"),
            Unknown16: row
                .columns[53]
                .into_bool()
                .copied()
                .expect("Expected column 53 to be a bool!"),
            Unknown17: row
                .columns[61]
                .into_bool()
                .copied()
                .expect("Expected column 61 to be a bool!"),
            Unknown18: row
                .columns[68]
                .into_bool()
                .copied()
                .expect("Expected column 68 to be a bool!"),
            AllowPhoenixDown: row
                .columns[71]
                .into_bool()
                .copied()
                .expect("Expected column 71 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a InstanceContentSheet {
    type Item = (u32, Vec<(u16, InstanceContentRow)>);
    type IntoIter = StructuredSheetIterator<'a, InstanceContentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, InstanceContentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct InstanceContentRow {
    ///""
    pub NewPlayerBonusGil: u32,
    ///""
    pub NewPlayerBonusExp: u32,
    ///""
    pub FinalBossExp: u32,
    ///""
    pub Unknown0: u32,
    ///""
    pub BossExp: [u32; 5],
    ///""
    pub InstanceClearExp: u32,
    ///""
    pub InstanceClearGil: u32,
    ///""
    pub InstanceContentRewardItem: u32,
    ///""
    pub NewPlayerBonusA: u16,
    ///""
    pub NewPlayerBonusB: u16,
    ///""
    pub FinalBossCurrencyA: u16,
    ///""
    pub FinalBossCurrencyB: u16,
    ///""
    pub FinalBossCurrencyC: u16,
    ///""
    pub BossCurrencyA: [u16; 5],
    ///""
    pub BossCurrencyB: [u16; 5],
    ///""
    pub BossCurrencyC: [u16; 5],
    ///""
    pub Unknown1: u16,
    ///""
    pub Unknown8: u8,
    ///""
    pub LimitedTimeBonus: bool,
    ///""
    pub Cutscene: u32,
    ///"Points to the game object ID of the exit range."
    pub LGBEventRange: u32,
    ///""
    pub InstanceContentTextDataBossStart: u32,
    ///""
    pub InstanceContentTextDataBossEnd: u32,
    ///""
    pub BNpcBaseBoss: u32,
    ///""
    pub InstanceContentTextDataObjectiveStart: u32,
    ///""
    pub InstanceContentTextDataObjectiveEnd: u32,
    ///""
    pub Unknown2: u32,
    ///""
    pub ReqInstance: u32,
    ///""
    pub InstanceContentBuff: i32,
    ///""
    pub TimeLimitmin: u16,
    ///""
    pub BGM: u16,
    ///""
    pub WinBGM: u16,
    ///""
    pub ContentFinderCondition: u16,
    ///""
    pub SortKey: u16,
    ///""
    pub ContentRoute: u16,
    ///""
    pub ContentDirectorManagedSG: u16,
    ///""
    pub ContentTodo: u16,
    ///""
    pub Unknown6: u16,
    ///"Used for certain duties. If not 0, then the client reads from a special unlock/completed array in PlayerState."
    pub LookupIndex: u16,
    ///""
    pub ContentEventItem: u16,
    ///""
    pub ContentDirectorBattleTalk: u16,
    ///""
    pub PartyCondition: i16,
    ///""
    pub InstanceContentType: u8,
    ///""
    pub WeekRestriction: u8,
    ///""
    pub Unknown9: u8,
    ///""
    pub QTE1: u8,
    ///""
    pub QTE2: u8,
    ///""
    pub Unknown12: u8,
    ///""
    pub ContentAttributeRect: u8,
    ///""
    pub Unknown13: u8,
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
    pub AllowPhoenixDown: bool,
}
