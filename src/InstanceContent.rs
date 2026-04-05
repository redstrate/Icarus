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
impl<'a> StructuredSheet<'a> for InstanceContentSheet {
    type Row = InstanceContentRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a InstanceContentSheet {
    type Item = (u32, Vec<(u16, InstanceContentRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, InstanceContentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, InstanceContentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct InstanceContentRow<'a> {
    row: &'a Row,
}
impl<'a> InstanceContentRow<'a> {
    pub fn NewPlayerBonusGil(&'a self) -> u32 {
        self.row.columns[17].into_u32().copied().unwrap()
    }
    pub fn NewPlayerBonusExp(&'a self) -> u32 {
        self.row.columns[18].into_u32().copied().unwrap()
    }
    pub fn FinalBossExp(&'a self) -> u32 {
        self.row.columns[21].into_u32().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u32 {
        self.row.columns[23].into_u32().copied().unwrap()
    }
    pub fn BossExp(&'a self) -> [u32; 5] {
        [
            self.row.columns[27].into_u32().copied().unwrap(),
            self.row.columns[28].into_u32().copied().unwrap(),
            self.row.columns[29].into_u32().copied().unwrap(),
            self.row.columns[30].into_u32().copied().unwrap(),
            self.row.columns[31].into_u32().copied().unwrap(),
        ]
    }
    pub fn InstanceClearExp(&'a self) -> u32 {
        self.row.columns[47].into_u32().copied().unwrap()
    }
    pub fn InstanceClearGil(&'a self) -> u32 {
        self.row.columns[48].into_u32().copied().unwrap()
    }
    pub fn InstanceContentRewardItem(&'a self) -> u32 {
        self.row.columns[49].into_u32().copied().unwrap()
    }
    pub fn NewPlayerBonusA(&'a self) -> u16 {
        self.row.columns[19].into_u16().copied().unwrap()
    }
    pub fn NewPlayerBonusB(&'a self) -> u16 {
        self.row.columns[20].into_u16().copied().unwrap()
    }
    pub fn FinalBossCurrencyA(&'a self) -> u16 {
        self.row.columns[24].into_u16().copied().unwrap()
    }
    pub fn FinalBossCurrencyB(&'a self) -> u16 {
        self.row.columns[25].into_u16().copied().unwrap()
    }
    pub fn FinalBossCurrencyC(&'a self) -> u16 {
        self.row.columns[26].into_u16().copied().unwrap()
    }
    pub fn BossCurrencyA(&'a self) -> [u16; 5] {
        [
            self.row.columns[32].into_u16().copied().unwrap(),
            self.row.columns[33].into_u16().copied().unwrap(),
            self.row.columns[34].into_u16().copied().unwrap(),
            self.row.columns[35].into_u16().copied().unwrap(),
            self.row.columns[36].into_u16().copied().unwrap(),
        ]
    }
    pub fn BossCurrencyB(&'a self) -> [u16; 5] {
        [
            self.row.columns[37].into_u16().copied().unwrap(),
            self.row.columns[38].into_u16().copied().unwrap(),
            self.row.columns[39].into_u16().copied().unwrap(),
            self.row.columns[40].into_u16().copied().unwrap(),
            self.row.columns[41].into_u16().copied().unwrap(),
        ]
    }
    pub fn BossCurrencyC(&'a self) -> [u16; 5] {
        [
            self.row.columns[42].into_u16().copied().unwrap(),
            self.row.columns[43].into_u16().copied().unwrap(),
            self.row.columns[44].into_u16().copied().unwrap(),
            self.row.columns[45].into_u16().copied().unwrap(),
            self.row.columns[46].into_u16().copied().unwrap(),
        ]
    }
    pub fn Unknown1(&'a self) -> u16 {
        self.row.columns[50].into_u16().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> u8 {
        self.row.columns[22].into_u8().copied().unwrap()
    }
    pub fn LimitedTimeBonus(&'a self) -> bool {
        self.row.columns[51].into_bool().copied().unwrap()
    }
    pub fn Cutscene(&'a self) -> u32 {
        self.row.columns[6].into_u32().copied().unwrap()
    }
    pub fn LGBEventRange(&'a self) -> u32 {
        self.row.columns[7].into_u32().copied().unwrap()
    }
    pub fn InstanceContentTextDataBossStart(&'a self) -> u32 {
        self.row.columns[11].into_u32().copied().unwrap()
    }
    pub fn InstanceContentTextDataBossEnd(&'a self) -> u32 {
        self.row.columns[12].into_u32().copied().unwrap()
    }
    pub fn BNpcBaseBoss(&'a self) -> u32 {
        self.row.columns[13].into_u32().copied().unwrap()
    }
    pub fn InstanceContentTextDataObjectiveStart(&'a self) -> u32 {
        self.row.columns[14].into_u32().copied().unwrap()
    }
    pub fn InstanceContentTextDataObjectiveEnd(&'a self) -> u32 {
        self.row.columns[15].into_u32().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u32 {
        self.row.columns[52].into_u32().copied().unwrap()
    }
    pub fn ReqInstance(&'a self) -> u32 {
        self.row.columns[55].into_u32().copied().unwrap()
    }
    pub fn InstanceContentBuff(&'a self) -> i32 {
        self.row.columns[53].into_i32().copied().unwrap()
    }
    pub fn TimeLimitmin(&'a self) -> u16 {
        self.row.columns[2].into_u16().copied().unwrap()
    }
    pub fn BGM(&'a self) -> u16 {
        self.row.columns[4].into_u16().copied().unwrap()
    }
    pub fn WinBGM(&'a self) -> u16 {
        self.row.columns[5].into_u16().copied().unwrap()
    }
    pub fn ContentFinderCondition(&'a self) -> u16 {
        self.row.columns[8].into_u16().copied().unwrap()
    }
    pub fn SortKey(&'a self) -> u16 {
        self.row.columns[16].into_u16().copied().unwrap()
    }
    pub fn ContentRoute(&'a self) -> u16 {
        self.row.columns[63].into_u16().copied().unwrap()
    }
    pub fn ContentDirectorManagedSG(&'a self) -> u16 {
        self.row.columns[64].into_u16().copied().unwrap()
    }
    pub fn ContentTodo(&'a self) -> u16 {
        self.row.columns[65].into_u16().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> u16 {
        self.row.columns[67].into_u16().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> u16 {
        self.row.columns[68].into_u16().copied().unwrap()
    }
    pub fn ContentEventItem(&'a self) -> u16 {
        self.row.columns[70].into_u16().copied().unwrap()
    }
    pub fn ContentDirectorBattleTalk(&'a self) -> u16 {
        self.row.columns[71].into_u16().copied().unwrap()
    }
    pub fn PartyCondition(&'a self) -> i16 {
        self.row.columns[56].into_i16().copied().unwrap()
    }
    pub fn InstanceContentType(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn WeekRestriction(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn Colosseum(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> u8 {
        self.row.columns[57].into_u8().copied().unwrap()
    }
    pub fn QTE1(&'a self) -> u8 {
        self.row.columns[58].into_u8().copied().unwrap()
    }
    pub fn QTE2(&'a self) -> u8 {
        self.row.columns[59].into_u8().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> u8 {
        self.row.columns[60].into_u8().copied().unwrap()
    }
    pub fn ContentAttributeRect(&'a self) -> u8 {
        self.row.columns[61].into_u8().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> u8 {
        self.row.columns[66].into_u8().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> bool {
        self.row.columns[10].into_bool().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> bool {
        self.row.columns[54].into_bool().copied().unwrap()
    }
    pub fn Unknown17(&'a self) -> bool {
        self.row.columns[62].into_bool().copied().unwrap()
    }
    pub fn Unknown18(&'a self) -> bool {
        self.row.columns[69].into_bool().copied().unwrap()
    }
    pub fn AllowPhoenixDown(&'a self) -> bool {
        self.row.columns[72].into_bool().copied().unwrap()
    }
}
