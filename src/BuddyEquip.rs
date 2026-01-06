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
pub struct BuddyEquipSheet {
    sheet: Sheet,
}
impl BuddyEquipSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BuddyEquip")?;
        let sheet = resolver.read_excel_sheet(&exh, "BuddyEquip", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BuddyEquipRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BuddyEquipRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BuddyEquipSheet {
    type Row = BuddyEquipRow;
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
impl<'a> IntoIterator for &'a BuddyEquipSheet {
    type Item = (u32, Vec<(u16, BuddyEquipRow)>);
    type IntoIter = StructuredSheetIterator<'a, BuddyEquipSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BuddyEquipSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BuddyEquipRow {
    columns: Vec<Field>,
}
impl BuddyEquipRow {
    pub fn Singular<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Plural<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn Name<'a>(&'a self) -> &'a Field {
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
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Pronoun<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn Article<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn ModelTop<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn ModelBody<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn ModelLegs<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn IconHead<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn IconBody<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn IconLegs<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn GrandCompany<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn Order<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
}
