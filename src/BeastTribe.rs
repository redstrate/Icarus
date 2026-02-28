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
pub struct BeastTribeSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl BeastTribeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BeastTribe")?;
        let sheet = resolver.read_excel_sheet(&exh, "BeastTribe", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<BeastTribeRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BeastTribeRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for BeastTribeSheet {
    type Row = BeastTribeRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a BeastTribeSheet {
    type Item = (u32, Vec<(u16, BeastTribeRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, BeastTribeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BeastTribeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BeastTribeRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> BeastTribeRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Plural(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn NameRelation(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn Adjective(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn PossessivePronoun(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn StartsWithVowel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn Pronoun(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn Article(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn DEF(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn IconReputation(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn Icon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn IntersocietalQuest(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn Level(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn CurrencyItem(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn MinLevel(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn BeastRankBonus(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn MaxRank(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn Expansion(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn DisplayOrder(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
}
