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
pub struct CSBonusContentSheet {
    sheet: Sheet,
}
impl CSBonusContentSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("CSBonusContent")?;
        let sheet = resolver.read_excel_sheet(&exh, "CSBonusContent", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<CSBonusContentRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<CSBonusContentRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for CSBonusContentSheet {
    type Row = CSBonusContentRow;
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
impl<'a> IntoIterator for &'a CSBonusContentSheet {
    type Item = (u32, Vec<(u16, CSBonusContentRow)>);
    type IntoIter = StructuredSheetIterator<'a, CSBonusContentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, CSBonusContentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CSBonusContentRow {
    columns: Vec<Field>,
}
impl CSBonusContentRow {
    pub fn Score1<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Score2<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn Score3<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Score4<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Score5<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn Content0<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn Content1<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Score0<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn ContentType<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn RewardCount0<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn RewardCount1<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn RewardCount2<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn RewardCount3<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn RewardCount4<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
}
