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
pub struct ContentMemberTypeSheet {
    sheet: Sheet,
}
impl ContentMemberTypeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ContentMemberType")?;
        let sheet = resolver.read_excel_sheet(&exh, "ContentMemberType", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<ContentMemberTypeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<ContentMemberTypeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for ContentMemberTypeSheet {
    type Row = ContentMemberTypeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a ContentMemberTypeSheet {
    type Item = (u32, Vec<(u16, ContentMemberTypeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, ContentMemberTypeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ContentMemberTypeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ContentMemberTypeRow<'a> {
    row: &'a Row,
}
impl<'a> ContentMemberTypeRow<'a> {
    pub fn Unknown0(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    /// Used as parameter for Addon#16461.
    pub fn AllianceRegistrationMinPlayers(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn MembersPerParty(&'a self) -> u8 {
        self.row.columns[7].into_u8().copied().unwrap()
    }
    pub fn PartyCount(&'a self) -> u8 {
        self.row.columns[8].into_u8().copied().unwrap()
    }
    /// For example, used for The Occult Crescent: South Horn, to allow queuing with alliance parties. If 0, use PartyCount.
    pub fn AlliancePartyCount(&'a self) -> u8 {
        self.row.columns[9].into_u8().copied().unwrap()
    }
    /// Unknown party count. If 0, use PartyCount.
    pub fn Unknown6(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
    pub fn TanksPerParty(&'a self) -> u8 {
        self.row.columns[11].into_u8().copied().unwrap()
    }
    pub fn HealersPerParty(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn MeleesPerParty(&'a self) -> u8 {
        self.row.columns[13].into_u8().copied().unwrap()
    }
    pub fn RangedPerParty(&'a self) -> u8 {
        self.row.columns[14].into_u8().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> u8 {
        self.row.columns[17].into_u8().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> bool {
        self.row.columns[0].into_bool().copied().unwrap()
    }
    /// Displays Addon#10826.
    pub fn AllowAllianceRegistration(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    /// If true, displays MeleesPerParty and RangedPerParty separately (Addon#102593) instead of combining them (Addon#2513).
    pub fn SeparatedDpsRoles(&'a self) -> bool {
        self.row.columns[15].into_bool().copied().unwrap()
    }
    /// If true, displays Addon#102599 instead of Addon#2513 or Addon#102593.
    pub fn RoleIndependent(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> bool {
        self.row.columns[18].into_bool().copied().unwrap()
    }
    /// Displays Addon#102617.
    pub fn OnlyPvPTeamMembers(&'a self) -> bool {
        self.row.columns[19].into_bool().copied().unwrap()
    }
    /// Displays Addon#10829.
    pub fn AllowLimitedJobs(&'a self) -> bool {
        self.row.columns[20].into_bool().copied().unwrap()
    }
}
