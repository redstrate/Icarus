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
}
impl ContentRouletteSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ContentRoulette")?;
        let sheet = resolver.read_excel_sheet(&exh, "ContentRoulette", language)?;
        Ok(Self { sheet })
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
        Some(Self::Row { row })
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
}
impl<'a> ContentRouletteRow<'a> {
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Category(&'a self) -> &'a str {
        self.row.columns[1].into_string().unwrap()
    }
    pub fn Unknown0(&'a self) -> &'a str {
        self.row.columns[2].into_string().unwrap()
    }
    pub fn Description(&'a self) -> &'a str {
        self.row.columns[3].into_string().unwrap()
    }
    pub fn DutyType(&'a self) -> &'a str {
        self.row.columns[4].into_string().unwrap()
    }
    /// This would show Addon#102618, but the row is empty.
    pub fn Unknown1(&'a self) -> u32 {
        self.row.columns[6].into_u32().copied().unwrap()
    }
    pub fn Image(&'a self) -> u32 {
        self.row.columns[16].into_u32().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u32 {
        self.row.columns[21].into_u32().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u32 {
        self.row.columns[23].into_u32().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> i32 {
        self.row.columns[47].into_i32().copied().unwrap()
    }
    pub fn ItemLevelRequired(&'a self) -> u16 {
        self.row.columns[13].into_u16().copied().unwrap()
    }
    pub fn ItemLevelSync(&'a self) -> u16 {
        self.row.columns[15].into_u16().copied().unwrap()
    }
    pub fn RewardTomeA(&'a self) -> u16 {
        self.row.columns[18].into_u16().copied().unwrap()
    }
    pub fn RewardTomeB(&'a self) -> u16 {
        self.row.columns[19].into_u16().copied().unwrap()
    }
    pub fn RewardTomeC(&'a self) -> u16 {
        self.row.columns[20].into_u16().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> u16 {
        self.row.columns[22].into_u16().copied().unwrap()
    }
    pub fn InstanceContent(&'a self) -> u16 {
        self.row.columns[41].into_u16().copied().unwrap()
    }
    pub fn RequiredExVersion(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn OpenRule(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn RequiredLevel(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
    pub fn SyncedFromLevel(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn ContentRouletteRoleBonus(&'a self) -> u8 {
        self.row.columns[17].into_u8().copied().unwrap()
    }
    pub fn SortKey(&'a self) -> u8 {
        self.row.columns[24].into_u8().copied().unwrap()
    }
    pub fn ClassJobCategory(&'a self) -> u8 {
        self.row.columns[25].into_u8().copied().unwrap()
    }
    pub fn ContentMemberType(&'a self) -> u8 {
        self.row.columns[26].into_u8().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> u8 {
        self.row.columns[27].into_u8().copied().unwrap()
    }
    pub fn QueueMaxPlayers(&'a self) -> u8 {
        self.row.columns[28].into_u8().copied().unwrap()
    }
    pub fn ContentType(&'a self) -> u8 {
        self.row.columns[31].into_u8().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> u8 {
        self.row.columns[32].into_u8().copied().unwrap()
    }
    /// In minutes.
    pub fn TimeLimit(&'a self) -> u8 {
        self.row.columns[33].into_u8().copied().unwrap()
    }
    /// In minutes. If 0, only TimeLimit is displayed.
    pub fn TimeLimitMax(&'a self) -> u8 {
        self.row.columns[34].into_u8().copied().unwrap()
    }
    pub fn LootModeType(&'a self) -> u8 {
        self.row.columns[40].into_u8().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> u8 {
        self.row.columns[43].into_u8().copied().unwrap()
    }
    /// Index in PlayerState.PenaltyTimestamps
    pub fn PenaltyTimestampArrayIndex(&'a self) -> u8 {
        self.row.columns[48].into_u8().copied().unwrap()
    }
    /// Index in PlayerState.ContentRouletteCompletion
    pub fn CompletionArrayIndex(&'a self) -> i8 {
        self.row.columns[29].into_i8().copied().unwrap()
    }
    pub fn IsGoldSaucer(&'a self) -> bool {
        self.row.columns[7].into_bool().copied().unwrap()
    }
    pub fn IsInDutyFinder(&'a self) -> bool {
        self.row.columns[8].into_bool().copied().unwrap()
    }
    pub fn IsPvP(&'a self) -> bool {
        self.row.columns[10].into_bool().copied().unwrap()
    }
    /// Displays Addon#2828.
    pub fn AppliesHighestAverageDutyItemLevel(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
    pub fn Unknown18(&'a self) -> bool {
        self.row.columns[30].into_bool().copied().unwrap()
    }
    pub fn AllowConsumableItems(&'a self) -> bool {
        self.row.columns[35].into_bool().copied().unwrap()
    }
    pub fn AllowPhoenixDown(&'a self) -> bool {
        self.row.columns[36].into_bool().copied().unwrap()
    }
    pub fn AllowReplacement(&'a self) -> bool {
        self.row.columns[37].into_bool().copied().unwrap()
    }
    pub fn RatedMatch(&'a self) -> bool {
        self.row.columns[38].into_bool().copied().unwrap()
    }
    pub fn Rated(&'a self) -> bool {
        self.row.columns[39].into_bool().copied().unwrap()
    }
    /// This would show Addon#10833, but the row does not exist.
    pub fn Unknown22(&'a self) -> bool {
        self.row.columns[42].into_bool().copied().unwrap()
    }
    pub fn Unknown23(&'a self) -> bool {
        self.row.columns[44].into_bool().copied().unwrap()
    }
    pub fn Unknown24(&'a self) -> bool {
        self.row.columns[45].into_bool().copied().unwrap()
    }
    pub fn IsRegistrationAllowedFromAnyDataCenter(&'a self) -> bool {
        self.row.columns[46].into_bool().copied().unwrap()
    }
}
