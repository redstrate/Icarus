//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct OvooDataElement<'a> {
    pub EmptyIcon: &'a Field,
    pub MaelstromIcon: &'a Field,
    pub TwinAdderIcon: &'a Field,
    pub ImmortalFlamesIcon: &'a Field,
    pub Unknown0: &'a Field,
    pub Unknown1: &'a Field,
    pub Unknown2: &'a Field,
}
#[derive(Debug, Clone)]
pub struct Frontline03Sheet {
    sheet: Sheet,
}
impl Frontline03Sheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Frontline03")?;
        let sheet = resolver.read_excel_sheet(&exh, "Frontline03", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<Frontline03Row> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<Frontline03Row> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for Frontline03Sheet {
    type Row = Frontline03Row;
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
impl<'a> IntoIterator for &'a Frontline03Sheet {
    type Item = (u32, Vec<(u16, Frontline03Row)>);
    type IntoIter = StructuredSheetIterator<'a, Frontline03Sheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, Frontline03Sheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Frontline03Row {
    columns: Vec<Field>,
}
impl Frontline03Row {
    pub fn OvooData<'a>(&'a self) -> [OvooDataElement<'a>; 3] {
        [
            OvooDataElement {
                EmptyIcon: &self.columns[0],
                MaelstromIcon: &self.columns[1],
                TwinAdderIcon: &self.columns[2],
                ImmortalFlamesIcon: &self.columns[3],
                Unknown0: &self.columns[4],
                Unknown1: &self.columns[5],
                Unknown2: &self.columns[6],
            },
            OvooDataElement {
                EmptyIcon: &self.columns[7],
                MaelstromIcon: &self.columns[8],
                TwinAdderIcon: &self.columns[9],
                ImmortalFlamesIcon: &self.columns[10],
                Unknown0: &self.columns[11],
                Unknown1: &self.columns[12],
                Unknown2: &self.columns[13],
            },
            OvooDataElement {
                EmptyIcon: &self.columns[14],
                MaelstromIcon: &self.columns[15],
                TwinAdderIcon: &self.columns[16],
                ImmortalFlamesIcon: &self.columns[17],
                Unknown0: &self.columns[18],
                Unknown1: &self.columns[19],
                Unknown2: &self.columns[20],
            },
        ]
    }
}
