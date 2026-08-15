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
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
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
impl StructuredSheet for ContentMemberTypeSheet {
    type Row = ContentMemberTypeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Unknown0: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown1: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            AllianceRegistrationMinPlayers: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            Unknown3: row
                .columns[6]
                .into_u8()
                .copied()
                .expect("Expected column 6 to be a uint8!"),
            MembersPerParty: row
                .columns[7]
                .into_u8()
                .copied()
                .expect("Expected column 7 to be a uint8!"),
            PartyCount: row
                .columns[8]
                .into_u8()
                .copied()
                .expect("Expected column 8 to be a uint8!"),
            AlliancePartyCount: row
                .columns[9]
                .into_u8()
                .copied()
                .expect("Expected column 9 to be a uint8!"),
            Unknown6: row
                .columns[10]
                .into_u8()
                .copied()
                .expect("Expected column 10 to be a uint8!"),
            TanksPerParty: row
                .columns[11]
                .into_u8()
                .copied()
                .expect("Expected column 11 to be a uint8!"),
            HealersPerParty: row
                .columns[12]
                .into_u8()
                .copied()
                .expect("Expected column 12 to be a uint8!"),
            MeleesPerParty: row
                .columns[13]
                .into_u8()
                .copied()
                .expect("Expected column 13 to be a uint8!"),
            RangedPerParty: row
                .columns[14]
                .into_u8()
                .copied()
                .expect("Expected column 14 to be a uint8!"),
            Unknown16: row
                .columns[17]
                .into_u8()
                .copied()
                .expect("Expected column 17 to be a uint8!"),
            Unknown7: row
                .columns[0]
                .into_bool()
                .copied()
                .expect("Expected column 0 to be a bool!"),
            AllowAllianceRegistration: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
            Unknown10: row
                .columns[3]
                .into_bool()
                .copied()
                .expect("Expected column 3 to be a bool!"),
            SeparatedDpsRoles: row
                .columns[15]
                .into_bool()
                .copied()
                .expect("Expected column 15 to be a bool!"),
            RoleIndependent: row
                .columns[16]
                .into_bool()
                .copied()
                .expect("Expected column 16 to be a bool!"),
            Unknown12: row
                .columns[18]
                .into_bool()
                .copied()
                .expect("Expected column 18 to be a bool!"),
            OnlyPvPTeamMembers: row
                .columns[19]
                .into_bool()
                .copied()
                .expect("Expected column 19 to be a bool!"),
            AllowLimitedJobs: row
                .columns[20]
                .into_bool()
                .copied()
                .expect("Expected column 20 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a ContentMemberTypeSheet {
    type Item = (u32, Vec<(u16, ContentMemberTypeRow)>);
    type IntoIter = StructuredSheetIterator<'a, ContentMemberTypeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, ContentMemberTypeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ContentMemberTypeRow {
    ///""
    pub Unknown0: u8,
    ///""
    pub Unknown1: u8,
    ///"Used as parameter for Addon#16461."
    pub AllianceRegistrationMinPlayers: u8,
    ///""
    pub Unknown3: u8,
    ///""
    pub MembersPerParty: u8,
    ///""
    pub PartyCount: u8,
    ///"For example, used for The Occult Crescent: South Horn, to allow queuing with alliance parties. If 0, use PartyCount."
    pub AlliancePartyCount: u8,
    ///"Unknown party count. If 0, use PartyCount."
    pub Unknown6: u8,
    ///""
    pub TanksPerParty: u8,
    ///""
    pub HealersPerParty: u8,
    ///""
    pub MeleesPerParty: u8,
    ///""
    pub RangedPerParty: u8,
    ///""
    pub Unknown16: u8,
    ///""
    pub Unknown7: bool,
    ///"Displays Addon#10826."
    pub AllowAllianceRegistration: bool,
    ///""
    pub Unknown10: bool,
    ///"If true, displays MeleesPerParty and RangedPerParty separately (Addon#102593) instead of combining them (Addon#2513)."
    pub SeparatedDpsRoles: bool,
    ///"If true, displays Addon#102599 instead of Addon#2513 or Addon#102593."
    pub RoleIndependent: bool,
    ///""
    pub Unknown12: bool,
    ///"Displays Addon#102617."
    pub OnlyPvPTeamMembers: bool,
    ///"Displays Addon#10829."
    pub AllowLimitedJobs: bool,
}
