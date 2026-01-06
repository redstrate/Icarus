//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
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
impl StructuredSheet for ContentMemberTypeSheet {
    type Row = ContentMemberTypeRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
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
#[derive(Debug, Clone)]
pub struct ContentMemberTypeRow {
    columns: Vec<Field>,
}
impl ContentMemberTypeRow {
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    /// Used as parameter for Addon#16461.
    pub fn AllianceRegistrationMinPlayers<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn MembersPerParty<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn PartyCount<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    /// For example, used for The Occult Crescent: South Horn, to allow queuing with alliance parties. If 0, use PartyCount.
    pub fn AlliancePartyCount<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    /// Unknown party count. If 0, use PartyCount.
    pub fn Unknown6<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn TanksPerParty<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn HealersPerParty<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn MeleesPerParty<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn RangedPerParty<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    /// Displays Addon#10826.
    pub fn AllowAllianceRegistration<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    /// If true, displays MeleesPerParty and RangedPerParty separately (Addon#102593) instead of combining them (Addon#2513).
    pub fn SeparatedDpsRoles<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    /// If true, displays Addon#102599 instead of Addon#2513 or Addon#102593.
    pub fn RoleIndependent<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    /// Displays Addon#102617.
    pub fn OnlyPvPTeamMembers<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    /// Displays Addon#10829.
    pub fn AllowLimitedJobs<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
}
