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
}
impl BeastTribeSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BeastTribe")?;
        let sheet = resolver.read_excel_sheet(&exh, "BeastTribe", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for BeastTribeSheet {
    type Row = BeastTribeRow;
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
impl<'a> IntoIterator for &'a BeastTribeSheet {
    type Item = (u32, Vec<(u16, BeastTribeRow)>);
    type IntoIter = StructuredSheetIterator<'a, BeastTribeSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BeastTribeSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BeastTribeRow {
    columns: Vec<Field>,
}
impl BeastTribeRow {
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Plural<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn NameRelation<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Adjective<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn PossessivePronoun<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn StartsWithVowel<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn Pronoun<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Article<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn DEF<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn IconReputation<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn Icon<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn IntersocietalQuest<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn Level<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn CurrencyItem<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn MinLevel<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn BeastRankBonus<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn MaxRank<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn Expansion<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn DisplayOrder<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
}
