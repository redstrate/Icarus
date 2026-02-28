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
    index_mapping: Vec<usize>,
}
impl ContentMemberTypeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("ContentMemberType")?;
        let sheet = resolver.read_excel_sheet(&exh, "ContentMemberType", language)?;
        let mut index_mapping: Vec<(usize, &ExcelColumnDefinition)> = sheet
            .exh
            .column_definitions
            .iter()
            .enumerate()
            .collect();
        index_mapping.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let index_mapping: Vec<usize> = index_mapping
            .iter()
            .map(|(index, _)| *index)
            .collect();
        Ok(Self { sheet, index_mapping })
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
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
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
    index_mapping: Vec<usize>,
}
impl<'a> ContentMemberTypeRow<'a> {
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    /// Used as parameter for Addon#16461.
    pub fn AllianceRegistrationMinPlayers(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn MembersPerParty(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn PartyCount(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    /// For example, used for The Occult Crescent: South Horn, to allow queuing with alliance parties. If 0, use PartyCount.
    pub fn AlliancePartyCount(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    /// Unknown party count. If 0, use PartyCount.
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn TanksPerParty(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn HealersPerParty(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn MeleesPerParty(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn RangedPerParty(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn Unknown16(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    /// Displays Addon#10826.
    pub fn AllowAllianceRegistration(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn Unknown10(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    /// If true, displays MeleesPerParty and RangedPerParty separately (Addon#102593) instead of combining them (Addon#2513).
    pub fn SeparatedDpsRoles(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    /// If true, displays Addon#102599 instead of Addon#2513 or Addon#102593.
    pub fn RoleIndependent(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn Unknown12(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    /// Displays Addon#102617.
    pub fn OnlyPvPTeamMembers(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    /// Displays Addon#10829.
    pub fn AllowLimitedJobs(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
}
