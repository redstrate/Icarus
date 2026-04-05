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
pub struct GoldSaucerTalkSheet {
    sheet: Sheet,
}
impl GoldSaucerTalkSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GoldSaucerTalk")?;
        let sheet = resolver.read_excel_sheet(&exh, "GoldSaucerTalk", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GoldSaucerTalkRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GoldSaucerTalkRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for GoldSaucerTalkSheet {
    type Row = GoldSaucerTalkRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a GoldSaucerTalkSheet {
    type Item = (u32, Vec<(u16, GoldSaucerTalkRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, GoldSaucerTalkSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GoldSaucerTalkSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GoldSaucerTalkRow<'a> {
    row: &'a Row,
}
impl<'a> GoldSaucerTalkRow<'a> {
    pub fn Message(&'a self) -> &'a str {
        self.row.columns[17].into_string().unwrap()
    }
    pub fn ChoicesText(&'a self) -> [&'a str; 10] {
        [
            self.row.columns[18].into_string().unwrap(),
            self.row.columns[19].into_string().unwrap(),
            self.row.columns[20].into_string().unwrap(),
            self.row.columns[21].into_string().unwrap(),
            self.row.columns[22].into_string().unwrap(),
            self.row.columns[23].into_string().unwrap(),
            self.row.columns[24].into_string().unwrap(),
            self.row.columns[25].into_string().unwrap(),
            self.row.columns[26].into_string().unwrap(),
            self.row.columns[27].into_string().unwrap(),
        ]
    }
    /// The next GoldSaucerTalk message.
    pub fn NextTalk(&'a self) -> u16 {
        self.row.columns[3].into_u16().copied().unwrap()
    }
    pub fn ActionTimeline(&'a self) -> u16 {
        self.row.columns[5].into_u16().copied().unwrap()
    }
    /// The next GoldSaucerTalk for the ChoicesText of the same index.
    pub fn ChoicesTalk(&'a self) -> [u16; 10] {
        [
            self.row.columns[7].into_u16().copied().unwrap(),
            self.row.columns[8].into_u16().copied().unwrap(),
            self.row.columns[9].into_u16().copied().unwrap(),
            self.row.columns[10].into_u16().copied().unwrap(),
            self.row.columns[11].into_u16().copied().unwrap(),
            self.row.columns[12].into_u16().copied().unwrap(),
            self.row.columns[13].into_u16().copied().unwrap(),
            self.row.columns[14].into_u16().copied().unwrap(),
            self.row.columns[15].into_u16().copied().unwrap(),
            self.row.columns[16].into_u16().copied().unwrap(),
        ]
    }
    pub fn Unknown23(&'a self) -> u8 {
        self.row.columns[0].into_u8().copied().unwrap()
    }
    pub fn Unknown24(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn Unknown25(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    pub fn Unknown26(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
    pub fn Unknown27(&'a self) -> bool {
        self.row.columns[6].into_bool().copied().unwrap()
    }
}
