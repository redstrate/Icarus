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
impl<'a> StructuredSheet<'a> for ClassJobSheet {
    type Row = ClassJobRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a ClassJobSheet {
    type Item = (u32, Vec<(u16, ClassJobRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ClassJobSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ClassJobSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClassJobRow<'a> {
    row: &'a Row,
}
impl<'a> ClassJobRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Abbreviation(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn NameFemale(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn CanQueueForDuty(&'a self) -> bool {
        self.row.columns[50].into_bool().copied().unwrap()
    }
    pub fn NameEnglish(&'a self) -> &'a str {
        self.row.columns[30].into_string().unwrap()
    }
    pub fn ItemSoulCrystal(&'a self) -> u32 {
        self.row.columns[41].into_u32().copied().unwrap()
    }
    pub fn UnlockQuest(&'a self) -> u32 {
        self.row.columns[42].into_u32().copied().unwrap()
    }
    pub fn RelicQuest(&'a self) -> u32 {
        self.row.columns[43].into_u32().copied().unwrap()
    }
    pub fn Prerequisite(&'a self) -> u32 {
        self.row.columns[44].into_u32().copied().unwrap()
    }
    pub fn Unknown_70_1(&'a self) -> i32 {
        self.row.columns[23].into_i32().copied().unwrap()
    }
    pub fn Unknown_70_2(&'a self) -> i32 {
        self.row.columns[24].into_i32().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> i32 {
        self.row.columns[25].into_i32().copied().unwrap()
    }
    pub fn ItemStartingWeaponMainHand(&'a self) -> i32 {
        self.row.columns[31].into_i32().copied().unwrap()
    }
    pub fn ItemStartingWeaponOffHand(&'a self) -> i32 {
        self.row.columns[32].into_i32().copied().unwrap()
    }
    pub fn ModifierHitPoints(&'a self) -> u16 {
        self.row.columns[9].into_u16().copied().unwrap()
    }
    pub fn ModifierManaPoints(&'a self) -> u16 {
        self.row.columns[10].into_u16().copied().unwrap()
    }
    pub fn ModifierStrength(&'a self) -> u16 {
        self.row.columns[11].into_u16().copied().unwrap()
    }
    pub fn ModifierVitality(&'a self) -> u16 {
        self.row.columns[12].into_u16().copied().unwrap()
    }
    pub fn ModifierDexterity(&'a self) -> u16 {
        self.row.columns[13].into_u16().copied().unwrap()
    }
    pub fn ModifierIntelligence(&'a self) -> u16 {
        self.row.columns[14].into_u16().copied().unwrap()
    }
    pub fn ModifierMind(&'a self) -> u16 {
        self.row.columns[15].into_u16().copied().unwrap()
    }
    pub fn ModifierPiety(&'a self) -> u16 {
        self.row.columns[16].into_u16().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u16 {
        self.row.columns[17].into_u16().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u16 {
        self.row.columns[18].into_u16().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> u16 {
        self.row.columns[19].into_u16().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> u16 {
        self.row.columns[20].into_u16().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> u16 {
        self.row.columns[21].into_u16().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> u16 {
        self.row.columns[22].into_u16().copied().unwrap()
    }
    pub fn LimitBreak1(&'a self) -> u16 {
        self.row.columns[37].into_u16().copied().unwrap()
    }
    pub fn LimitBreak2(&'a self) -> u16 {
        self.row.columns[38].into_u16().copied().unwrap()
    }
    pub fn LimitBreak3(&'a self) -> u16 {
        self.row.columns[39].into_u16().copied().unwrap()
    }
    pub fn ClassJobCategory(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn JobIndex(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn PvPBaseParamValue(&'a self) -> u8 {
        self.row.columns[26].into_u8().copied().unwrap()
    }
    pub fn PvPActionSortRow(&'a self) -> u8 {
        self.row.columns[27].into_u8().copied().unwrap()
    }
    pub fn PvPInitialSelectActionTrait(&'a self) -> u8 {
        self.row.columns[28].into_u8().copied().unwrap()
    }
    pub fn ClassJobParent(&'a self) -> u8 {
        self.row.columns[29].into_u8().copied().unwrap()
    }
    pub fn Role(&'a self) -> u8 {
        self.row.columns[33].into_u8().copied().unwrap()
    }
    pub fn StartingTown(&'a self) -> u8 {
        self.row.columns[34].into_u8().copied().unwrap()
    }
    pub fn PrimaryStat(&'a self) -> u8 {
        self.row.columns[36].into_u8().copied().unwrap()
    }
    pub fn UIPriority(&'a self) -> u8 {
        self.row.columns[40].into_u8().copied().unwrap()
    }
    pub fn StartingLevel(&'a self) -> u8 {
        self.row.columns[45].into_u8().copied().unwrap()
    }
    pub fn PartyBonus(&'a self) -> u8 {
        self.row.columns[46].into_u8().copied().unwrap()
    }
    /// 1 = Tank
    /// 2 = Pure Healer
    /// 3 = Melee
    /// 4 = Physical Ranged
    /// 5 = Magical Ranged
    /// 6 = Barrier Healer
    ///
    pub fn JobType(&'a self) -> u8 {
        self.row.columns[47].into_u8().copied().unwrap()
    }
    pub fn ExpArrayIndex(&'a self) -> i8 {
        self.row.columns[4].into_i8().copied().unwrap()
    }
    pub fn BattleClassIndex(&'a self) -> i8 {
        self.row.columns[5].into_i8().copied().unwrap()
    }
    pub fn DohDolJobIndex(&'a self) -> i8 {
        self.row.columns[8].into_i8().copied().unwrap()
    }
    pub fn MonsterNote(&'a self) -> i8 {
        self.row.columns[35].into_i8().copied().unwrap()
    }
    pub fn IsLimitedJob(&'a self) -> bool {
        self.row.columns[48].into_bool().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> bool {
        self.row.columns[49].into_bool().copied().unwrap()
    }
}
