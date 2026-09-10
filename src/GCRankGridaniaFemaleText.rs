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
pub struct GCRankGridaniaFemaleTextSheet {
    sheet: Sheet,
}
impl GCRankGridaniaFemaleTextSheet {
    /// The ID of the first row, on the first page.
    pub const STARTING_ROW: u32 = 0u32;
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GCRankGridaniaFemaleText")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "GCRankGridaniaFemaleText", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GCRankGridaniaFemaleTextRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<GCRankGridaniaFemaleTextRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GCRankGridaniaFemaleTextSheet {
    type Row = GCRankGridaniaFemaleTextRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Singular: row
                .columns[0]
                .into_string()
                .cloned()
                .expect("Expected column 0 to be a string!"),
            Plural: row
                .columns[2]
                .into_string()
                .cloned()
                .expect("Expected column 2 to be a string!"),
            NameRank: row
                .columns[8]
                .into_string()
                .cloned()
                .expect("Expected column 8 to be a string!"),
            Unknown0: row
                .columns[9]
                .into_string()
                .cloned()
                .expect("Expected column 9 to be a string!"),
            Adjective: row
                .columns[1]
                .into_i8()
                .copied()
                .expect("Expected column 1 to be a int8!"),
            PossessivePronoun: row
                .columns[3]
                .into_i8()
                .copied()
                .expect("Expected column 3 to be a int8!"),
            StartsWithVowel: row
                .columns[4]
                .into_i8()
                .copied()
                .expect("Expected column 4 to be a int8!"),
            Countability: row
                .columns[5]
                .into_i8()
                .copied()
                .expect("Expected column 5 to be a int8!"),
            Pronoun: row
                .columns[6]
                .into_i8()
                .copied()
                .expect("Expected column 6 to be a int8!"),
            Article: row
                .columns[7]
                .into_i8()
                .copied()
                .expect("Expected column 7 to be a int8!"),
        })
    }
}
impl<'a> IntoIterator for &'a GCRankGridaniaFemaleTextSheet {
    type Item = (u32, Vec<(u16, GCRankGridaniaFemaleTextRow)>);
    type IntoIter = StructuredSheetIterator<'a, GCRankGridaniaFemaleTextSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GCRankGridaniaFemaleTextSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct GCRankGridaniaFemaleTextRow {
    ///""
    pub Singular: String,
    ///""
    pub Plural: String,
    ///""
    pub NameRank: String,
    ///""
    pub Unknown0: String,
    ///""
    pub Adjective: i8,
    ///""
    pub PossessivePronoun: i8,
    ///""
    pub StartsWithVowel: i8,
    ///""
    pub Countability: i8,
    ///""
    pub Pronoun: i8,
    ///""
    pub Article: i8,
}
