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
pub struct ClassJobSheet {
    sheet: Sheet,
}
impl ClassJobSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ClassJob")?;
        let sheet = resolver.read_excel_sheet(&exh, "ClassJob", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ClassJobRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ClassJobRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for ClassJobSheet {
    type Row = ClassJobRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Name: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Abbreviation: row
                .columns[1]
                .into_string()
                .cloned()
                .expect("Expected column 1 to be a string!"),
            NameFemale: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            CanQueueForDuty: row
                .columns[50]
                .into_bool()
                .copied()
                .expect("Expected column 50 to be a bool!"),
            NameEnglish: row
                .columns[30]
                .into_string()
                .cloned()
                .expect("Expected column 30 to be a string!"),
            ItemSoulCrystal: row
                .columns[41]
                .into_u32()
                .copied()
                .expect("Expected column 41 to be a uint32!"),
            UnlockQuest: row
                .columns[42]
                .into_u32()
                .copied()
                .expect("Expected column 42 to be a uint32!"),
            RelicQuest: row
                .columns[43]
                .into_u32()
                .copied()
                .expect("Expected column 43 to be a uint32!"),
            Prerequisite: row
                .columns[44]
                .into_u32()
                .copied()
                .expect("Expected column 44 to be a uint32!"),
            Unknown_70_1: row
                .columns[23]
                .into_i32()
                .copied()
                .expect("Expected column 23 to be a int32!"),
            Unknown_70_2: row
                .columns[24]
                .into_i32()
                .copied()
                .expect("Expected column 24 to be a int32!"),
            Unknown9: row
                .columns[25]
                .into_i32()
                .copied()
                .expect("Expected column 25 to be a int32!"),
            ItemStartingWeaponMainHand: row
                .columns[31]
                .into_i32()
                .copied()
                .expect("Expected column 31 to be a int32!"),
            ItemStartingWeaponOffHand: row
                .columns[32]
                .into_i32()
                .copied()
                .expect("Expected column 32 to be a int32!"),
            ModifierHitPoints: row
                .columns[9]
                .into_u16()
                .copied()
                .expect("Expected column 9 to be a uint16!"),
            ModifierManaPoints: row
                .columns[10]
                .into_u16()
                .copied()
                .expect("Expected column 10 to be a uint16!"),
            ModifierStrength: row
                .columns[11]
                .into_u16()
                .copied()
                .expect("Expected column 11 to be a uint16!"),
            ModifierVitality: row
                .columns[12]
                .into_u16()
                .copied()
                .expect("Expected column 12 to be a uint16!"),
            ModifierDexterity: row
                .columns[13]
                .into_u16()
                .copied()
                .expect("Expected column 13 to be a uint16!"),
            ModifierIntelligence: row
                .columns[14]
                .into_u16()
                .copied()
                .expect("Expected column 14 to be a uint16!"),
            ModifierMind: row
                .columns[15]
                .into_u16()
                .copied()
                .expect("Expected column 15 to be a uint16!"),
            ModifierPiety: row
                .columns[16]
                .into_u16()
                .copied()
                .expect("Expected column 16 to be a uint16!"),
            Unknown2: row
                .columns[17]
                .into_u16()
                .copied()
                .expect("Expected column 17 to be a uint16!"),
            Unknown3: row
                .columns[18]
                .into_u16()
                .copied()
                .expect("Expected column 18 to be a uint16!"),
            Unknown4: row
                .columns[19]
                .into_u16()
                .copied()
                .expect("Expected column 19 to be a uint16!"),
            Unknown5: row
                .columns[20]
                .into_u16()
                .copied()
                .expect("Expected column 20 to be a uint16!"),
            Unknown6: row
                .columns[21]
                .into_u16()
                .copied()
                .expect("Expected column 21 to be a uint16!"),
            Unknown7: row
                .columns[22]
                .into_u16()
                .copied()
                .expect("Expected column 22 to be a uint16!"),
            LimitBreak1: row
                .columns[37]
                .into_u16()
                .copied()
                .expect("Expected column 37 to be a uint16!"),
            LimitBreak2: row
                .columns[38]
                .into_u16()
                .copied()
                .expect("Expected column 38 to be a uint16!"),
            LimitBreak3: row
                .columns[39]
                .into_u16()
                .copied()
                .expect("Expected column 39 to be a uint16!"),
            ClassJobCategory: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
            Unknown8: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            JobIndex: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            PvPBaseParamValue: row
                .columns[26]
                .into_u8()
                .copied()
                .expect("Expected column 26 to be a uint8!"),
            PvPActionSortRow: row
                .columns[27]
                .into_u8()
                .copied()
                .expect("Expected column 27 to be a uint8!"),
            PvPInitialSelectActionTrait: row
                .columns[28]
                .into_u8()
                .copied()
                .expect("Expected column 28 to be a uint8!"),
            ClassJobParent: row
                .columns[29]
                .into_u8()
                .copied()
                .expect("Expected column 29 to be a uint8!"),
            Role: row
                .columns[33]
                .into_u8()
                .copied()
                .expect("Expected column 33 to be a uint8!"),
            StartingTown: row
                .columns[34]
                .into_u8()
                .copied()
                .expect("Expected column 34 to be a uint8!"),
            PrimaryStat: row
                .columns[36]
                .into_u8()
                .copied()
                .expect("Expected column 36 to be a uint8!"),
            UIPriority: row
                .columns[40]
                .into_u8()
                .copied()
                .expect("Expected column 40 to be a uint8!"),
            StartingLevel: row
                .columns[45]
                .into_u8()
                .copied()
                .expect("Expected column 45 to be a uint8!"),
            PartyBonus: row
                .columns[46]
                .into_u8()
                .copied()
                .expect("Expected column 46 to be a uint8!"),
            JobType: row
                .columns[47]
                .into_u8()
                .copied()
                .expect("Expected column 47 to be a uint8!"),
            ExpArrayIndex: row
                .columns[4]
                .into_i8()
                .copied()
                .expect("Expected column 4 to be a int8!"),
            BattleClassIndex: row
                .columns[5]
                .into_i8()
                .copied()
                .expect("Expected column 5 to be a int8!"),
            DohDolJobIndex: row
                .columns[8]
                .into_i8()
                .copied()
                .expect("Expected column 8 to be a int8!"),
            MonsterNote: row
                .columns[35]
                .into_i8()
                .copied()
                .expect("Expected column 35 to be a int8!"),
            IsLimitedJob: row
                .columns[48]
                .into_bool()
                .copied()
                .expect("Expected column 48 to be a bool!"),
            Unknown10: row
                .columns[49]
                .into_bool()
                .copied()
                .expect("Expected column 49 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ClassJobSheet {
    type Item = (u32, Vec<(u16, ClassJobRow)>);
    type IntoIter = StructuredSheetIterator<'a, ClassJobSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ClassJobSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClassJobRow {
    ///""
    pub Name: String,
    ///""
    pub Abbreviation: String,
    ///""
    pub NameFemale: String,
    ///""
    pub CanQueueForDuty: bool,
    ///""
    pub NameEnglish: String,
    ///""
    pub ItemSoulCrystal: u32,
    ///""
    pub UnlockQuest: u32,
    ///""
    pub RelicQuest: u32,
    ///""
    pub Prerequisite: u32,
    ///""
    pub Unknown_70_1: i32,
    ///""
    pub Unknown_70_2: i32,
    ///""
    pub Unknown9: i32,
    ///""
    pub ItemStartingWeaponMainHand: i32,
    ///""
    pub ItemStartingWeaponOffHand: i32,
    ///""
    pub ModifierHitPoints: u16,
    ///""
    pub ModifierManaPoints: u16,
    ///""
    pub ModifierStrength: u16,
    ///""
    pub ModifierVitality: u16,
    ///""
    pub ModifierDexterity: u16,
    ///""
    pub ModifierIntelligence: u16,
    ///""
    pub ModifierMind: u16,
    ///""
    pub ModifierPiety: u16,
    ///""
    pub Unknown2: u16,
    ///""
    pub Unknown3: u16,
    ///""
    pub Unknown4: u16,
    ///""
    pub Unknown5: u16,
    ///""
    pub Unknown6: u16,
    ///""
    pub Unknown7: u16,
    ///""
    pub LimitBreak1: u16,
    ///""
    pub LimitBreak2: u16,
    ///""
    pub LimitBreak3: u16,
    ///""
    pub ClassJobCategory: u8,
    ///""
    pub Unknown8: u8,
    ///""
    pub JobIndex: u8,
    ///""
    pub PvPBaseParamValue: u8,
    ///""
    pub PvPActionSortRow: u8,
    ///""
    pub PvPInitialSelectActionTrait: u8,
    ///""
    pub ClassJobParent: u8,
    ///""
    pub Role: u8,
    ///""
    pub StartingTown: u8,
    ///""
    pub PrimaryStat: u8,
    ///""
    pub UIPriority: u8,
    ///""
    pub StartingLevel: u8,
    ///""
    pub PartyBonus: u8,
    ///"1 = Tank\n /// 2 = Pure Healer\n /// 3 = Melee\n /// 4 = Physical Ranged\n /// 5 = Magical Ranged\n /// 6 = Barrier Healer\n /// "
    pub JobType: u8,
    ///""
    pub ExpArrayIndex: i8,
    ///""
    pub BattleClassIndex: i8,
    ///""
    pub DohDolJobIndex: i8,
    ///""
    pub MonsterNote: i8,
    ///""
    pub IsLimitedJob: bool,
    ///""
    pub Unknown10: bool,
}
