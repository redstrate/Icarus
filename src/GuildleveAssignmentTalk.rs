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
pub struct GuildleveAssignmentTalkSheet {
    sheet: Sheet,
}
impl GuildleveAssignmentTalkSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GuildleveAssignmentTalk")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "GuildleveAssignmentTalk", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GuildleveAssignmentTalkRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<GuildleveAssignmentTalkRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for GuildleveAssignmentTalkSheet {
    type Row = GuildleveAssignmentTalkRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a GuildleveAssignmentTalkSheet {
    type Item = (u32, Vec<(u16, GuildleveAssignmentTalkRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, GuildleveAssignmentTalkSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GuildleveAssignmentTalkSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GuildleveAssignmentTalkRow<'a> {
    row: &'a Row,
}
impl<'a> GuildleveAssignmentTalkRow<'a> {
    pub fn Unknown0(&'a self) -> u32 {
        self.row.columns[10].into_u32().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> i32 {
        self.row.columns[25].into_i32().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> i8 {
        self.row.columns[15].into_i8().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> i8 {
        self.row.columns[20].into_i8().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> bool {
        self.row.columns[0].into_bool().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> u32 {
        self.row.columns[11].into_u32().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> i32 {
        self.row.columns[26].into_i32().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> i8 {
        self.row.columns[16].into_i8().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> i8 {
        self.row.columns[21].into_i8().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> bool {
        self.row.columns[1].into_bool().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> u32 {
        self.row.columns[12].into_u32().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> i32 {
        self.row.columns[27].into_i32().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> i8 {
        self.row.columns[17].into_i8().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> i8 {
        self.row.columns[22].into_i8().copied().unwrap()
    }
    pub fn Unknown17(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
    pub fn Unknown18(&'a self) -> u32 {
        self.row.columns[13].into_u32().copied().unwrap()
    }
    pub fn Unknown19(&'a self) -> i32 {
        self.row.columns[28].into_i32().copied().unwrap()
    }
    pub fn Unknown20(&'a self) -> u8 {
        self.row.columns[8].into_u8().copied().unwrap()
    }
    pub fn Unknown21(&'a self) -> i8 {
        self.row.columns[18].into_i8().copied().unwrap()
    }
    pub fn Unknown22(&'a self) -> i8 {
        self.row.columns[23].into_i8().copied().unwrap()
    }
    pub fn Unknown23(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    pub fn Unknown24(&'a self) -> u32 {
        self.row.columns[14].into_u32().copied().unwrap()
    }
    pub fn Unknown25(&'a self) -> i32 {
        self.row.columns[29].into_i32().copied().unwrap()
    }
    pub fn Unknown26(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    pub fn Unknown27(&'a self) -> i8 {
        self.row.columns[19].into_i8().copied().unwrap()
    }
    pub fn Unknown28(&'a self) -> i8 {
        self.row.columns[24].into_i8().copied().unwrap()
    }
    pub fn Unknown29(&'a self) -> bool {
        self.row.columns[4].into_bool().copied().unwrap()
    }
    pub fn Talk(&'a self) -> [&'a str; 8] {
        [
            self.row.columns[30].into_string().unwrap(),
            self.row.columns[31].into_string().unwrap(),
            self.row.columns[32].into_string().unwrap(),
            self.row.columns[33].into_string().unwrap(),
            self.row.columns[34].into_string().unwrap(),
            self.row.columns[35].into_string().unwrap(),
            self.row.columns[36].into_string().unwrap(),
            self.row.columns[37].into_string().unwrap(),
        ]
    }
}
