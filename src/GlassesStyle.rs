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
pub struct GlassesStyleSheet {
    sheet: Sheet,
}
impl GlassesStyleSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GlassesStyle")?;
        let sheet = resolver.read_excel_sheet(&exh, "GlassesStyle", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<GlassesStyleRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GlassesStyleRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for GlassesStyleSheet {
    type Row = GlassesStyleRow;
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
impl<'a> IntoIterator for &'a GlassesStyleSheet {
    type Item = (u32, Vec<(u16, GlassesStyleRow)>);
    type IntoIter = StructuredSheetIterator<'a, GlassesStyleSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GlassesStyleSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GlassesStyleRow {
    columns: Vec<Field>,
}
impl GlassesStyleRow {
    pub fn Singular<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Plural<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Unknown_70_1<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Unknown_70_2<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn Unknown_70_3<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn Unknown_70_4<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Unknown_70_5<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn Unknown_70_6<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn Icon<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn Order<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn Glasses<'a>(&'a self) -> [&'a Field; 12] {
        [
            &self.columns[11],
            &self.columns[12],
            &self.columns[13],
            &self.columns[14],
            &self.columns[15],
            &self.columns[16],
            &self.columns[17],
            &self.columns[18],
            &self.columns[19],
            &self.columns[20],
            &self.columns[21],
            &self.columns[22],
        ]
    }
    pub fn Unknown_70_7<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
}
