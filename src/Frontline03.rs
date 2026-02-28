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
    index_mapping: Vec<usize>,
}
impl Frontline03Sheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Frontline03")?;
        let sheet = resolver.read_excel_sheet(&exh, "Frontline03", language)?;
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
impl<'a> StructuredSheet<'a> for Frontline03Sheet {
    type Row = Frontline03Row<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a Frontline03Sheet {
    type Item = (u32, Vec<(u16, Frontline03Row<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, Frontline03Sheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, Frontline03Sheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Frontline03Row<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> Frontline03Row<'a> {
    pub fn OvooData(&'a self) -> [OvooDataElement<'a>; 3] {
        [
            OvooDataElement {
                EmptyIcon: &self.row.columns[self.index_mapping[0]],
                MaelstromIcon: &self.row.columns[self.index_mapping[1]],
                TwinAdderIcon: &self.row.columns[self.index_mapping[2]],
                ImmortalFlamesIcon: &self.row.columns[self.index_mapping[3]],
                Unknown0: &self.row.columns[self.index_mapping[4]],
                Unknown1: &self.row.columns[self.index_mapping[5]],
                Unknown2: &self.row.columns[self.index_mapping[6]],
            },
            OvooDataElement {
                EmptyIcon: &self.row.columns[self.index_mapping[7]],
                MaelstromIcon: &self.row.columns[self.index_mapping[8]],
                TwinAdderIcon: &self.row.columns[self.index_mapping[9]],
                ImmortalFlamesIcon: &self.row.columns[self.index_mapping[10]],
                Unknown0: &self.row.columns[self.index_mapping[11]],
                Unknown1: &self.row.columns[self.index_mapping[12]],
                Unknown2: &self.row.columns[self.index_mapping[13]],
            },
            OvooDataElement {
                EmptyIcon: &self.row.columns[self.index_mapping[14]],
                MaelstromIcon: &self.row.columns[self.index_mapping[15]],
                TwinAdderIcon: &self.row.columns[self.index_mapping[16]],
                ImmortalFlamesIcon: &self.row.columns[self.index_mapping[17]],
                Unknown0: &self.row.columns[self.index_mapping[18]],
                Unknown1: &self.row.columns[self.index_mapping[19]],
                Unknown2: &self.row.columns[self.index_mapping[20]],
            },
        ]
    }
}
