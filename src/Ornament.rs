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
pub struct OrnamentSheet {
    sheet: Sheet,
}
impl OrnamentSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Ornament")?;
        let sheet = resolver.read_excel_sheet(&exh, "Ornament", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<OrnamentRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<OrnamentRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for OrnamentSheet {
    type Row = OrnamentRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Singular: row
                .columns[8]
                .into_string()
                .cloned()
                .expect("Expected column 8 to be a string!"),
            Plural: row
                .columns[10]
                .into_string()
                .cloned()
                .expect("Expected column 10 to be a string!"),
            Adjective: row
                .columns[9]
                .into_i8()
                .copied()
                .expect("Expected column 9 to be a int8!"),
            PossessivePronoun: row
                .columns[11]
                .into_i8()
                .copied()
                .expect("Expected column 11 to be a int8!"),
            StartsWithVowel: row
                .columns[12]
                .into_i8()
                .copied()
                .expect("Expected column 12 to be a int8!"),
            Unknown0: row
                .columns[13]
                .into_i8()
                .copied()
                .expect("Expected column 13 to be a int8!"),
            Pronoun: row
                .columns[14]
                .into_i8()
                .copied()
                .expect("Expected column 14 to be a int8!"),
            Article: row
                .columns[15]
                .into_i8()
                .copied()
                .expect("Expected column 15 to be a int8!"),
            Model: row
                .columns[0]
                .into_u16()
                .copied()
                .expect("Expected column 0 to be a uint16!"),
            Action: row
                .columns[4]
                .into_u16()
                .copied()
                .expect("Expected column 4 to be a uint16!"),
            Icon: row
                .columns[6]
                .into_u16()
                .copied()
                .expect("Expected column 6 to be a uint16!"),
            Transient: row
                .columns[7]
                .into_u16()
                .copied()
                .expect("Expected column 7 to be a uint16!"),
            Order: row
                .columns[5]
                .into_i16()
                .copied()
                .expect("Expected column 5 to be a int16!"),
            CustomizeGroup: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown3: row
                .columns[2]
                .into_u8()
                .copied()
                .expect("Expected column 2 to be a uint8!"),
            Unknown4: row
                .columns[3]
                .into_u8()
                .copied()
                .expect("Expected column 3 to be a uint8!"),
        })
    }
}
impl<'a> IntoIterator for &'a OrnamentSheet {
    type Item = (u32, Vec<(u16, OrnamentRow)>);
    type IntoIter = StructuredSheetIterator<'a, OrnamentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, OrnamentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct OrnamentRow {
    ///""
    pub Singular: String,
    ///""
    pub Plural: String,
    ///""
    pub Adjective: i8,
    ///""
    pub PossessivePronoun: i8,
    ///""
    pub StartsWithVowel: i8,
    ///""
    pub Unknown0: i8,
    ///""
    pub Pronoun: i8,
    ///""
    pub Article: i8,
    ///""
    pub Model: u16,
    ///""
    pub Action: u16,
    ///""
    pub Icon: u16,
    ///""
    pub Transient: u16,
    ///""
    pub Order: i16,
    ///""
    pub CustomizeGroup: u8,
    ///""
    pub Unknown3: u8,
    ///""
    pub Unknown4: u8,
}
