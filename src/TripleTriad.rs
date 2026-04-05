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
pub struct TripleTriadSheet {
    sheet: Sheet,
}
impl TripleTriadSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TripleTriad")?;
        let sheet = resolver.read_excel_sheet(&exh, "TripleTriad", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<TripleTriadRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<TripleTriadRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for TripleTriadSheet {
    type Row = TripleTriadRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a TripleTriadSheet {
    type Item = (u32, Vec<(u16, TripleTriadRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, TripleTriadSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TripleTriadSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TripleTriadRow<'a> {
    row: &'a Row,
}
impl<'a> TripleTriadRow<'a> {
    pub fn ItemPossibleReward(&'a self) -> [u32; 4] {
        [
            self.row.columns[26].into_u32().copied().unwrap(),
            self.row.columns[27].into_u32().copied().unwrap(),
            self.row.columns[28].into_u32().copied().unwrap(),
            self.row.columns[29].into_u32().copied().unwrap(),
        ]
    }
    pub fn PreviousQuest(&'a self) -> [u32; 3] {
        [
            self.row.columns[15].into_u32().copied().unwrap(),
            self.row.columns[16].into_u32().copied().unwrap(),
            self.row.columns[17].into_u32().copied().unwrap(),
        ]
    }
    pub fn DefaultTalkChallenge(&'a self) -> u32 {
        self.row.columns[20].into_u32().copied().unwrap()
    }
    pub fn DefaultTalkUnavailable(&'a self) -> u32 {
        self.row.columns[21].into_u32().copied().unwrap()
    }
    pub fn DefaultTalkNPCWin(&'a self) -> u32 {
        self.row.columns[22].into_u32().copied().unwrap()
    }
    pub fn DefaultTalkDraw(&'a self) -> u32 {
        self.row.columns[23].into_u32().copied().unwrap()
    }
    pub fn DefaultTalkPCWin(&'a self) -> u32 {
        self.row.columns[24].into_u32().copied().unwrap()
    }
    pub fn TripleTriadCardFixed(&'a self) -> [u16; 5] {
        [
            self.row.columns[0].into_u16().copied().unwrap(),
            self.row.columns[1].into_u16().copied().unwrap(),
            self.row.columns[2].into_u16().copied().unwrap(),
            self.row.columns[3].into_u16().copied().unwrap(),
            self.row.columns[4].into_u16().copied().unwrap(),
        ]
    }
    pub fn TripleTriadCardVariable(&'a self) -> [u16; 5] {
        [
            self.row.columns[5].into_u16().copied().unwrap(),
            self.row.columns[6].into_u16().copied().unwrap(),
            self.row.columns[7].into_u16().copied().unwrap(),
            self.row.columns[8].into_u16().copied().unwrap(),
            self.row.columns[9].into_u16().copied().unwrap(),
        ]
    }
    pub fn Fee(&'a self) -> u16 {
        self.row.columns[13].into_u16().copied().unwrap()
    }
    pub fn StartTime(&'a self) -> u16 {
        self.row.columns[18].into_u16().copied().unwrap()
    }
    pub fn EndTime(&'a self) -> u16 {
        self.row.columns[19].into_u16().copied().unwrap()
    }
    pub fn TripleTriadRule(&'a self) -> [u8; 2] {
        [
            self.row.columns[10].into_u8().copied().unwrap(),
            self.row.columns[11].into_u8().copied().unwrap(),
        ]
    }
    pub fn PreviousQuestJoin(&'a self) -> u8 {
        self.row.columns[14].into_u8().copied().unwrap()
    }
    pub fn UsesRegionalRules(&'a self) -> bool {
        self.row.columns[12].into_bool().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> bool {
        self.row.columns[25].into_bool().copied().unwrap()
    }
}
