//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct OvooDataElement {
    pub EmptyIcon: u32,
    pub MaelstromIcon: u32,
    pub TwinAdderIcon: u32,
    pub ImmortalFlamesIcon: u32,
    pub Unknown0: u8,
    pub Unknown1: u8,
    pub Unknown2: u8,
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
impl<'a> StructuredSheet<'a> for Frontline03Sheet {
    type Row = Frontline03Row<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
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
}
impl<'a> Frontline03Row<'a> {
    pub fn OvooData(&'a self) -> [OvooDataElement; 3] {
        [
            OvooDataElement {
                EmptyIcon: self.row.columns[9].into_u32().copied().unwrap(),
                MaelstromIcon: self.row.columns[12].into_u32().copied().unwrap(),
                TwinAdderIcon: self.row.columns[15].into_u32().copied().unwrap(),
                ImmortalFlamesIcon: self.row.columns[18].into_u32().copied().unwrap(),
                Unknown0: self.row.columns[0].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[3].into_u8().copied().unwrap(),
                Unknown2: self.row.columns[6].into_u8().copied().unwrap(),
            },
            OvooDataElement {
                EmptyIcon: self.row.columns[10].into_u32().copied().unwrap(),
                MaelstromIcon: self.row.columns[13].into_u32().copied().unwrap(),
                TwinAdderIcon: self.row.columns[16].into_u32().copied().unwrap(),
                ImmortalFlamesIcon: self.row.columns[19].into_u32().copied().unwrap(),
                Unknown0: self.row.columns[1].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[4].into_u8().copied().unwrap(),
                Unknown2: self.row.columns[7].into_u8().copied().unwrap(),
            },
            OvooDataElement {
                EmptyIcon: self.row.columns[11].into_u32().copied().unwrap(),
                MaelstromIcon: self.row.columns[14].into_u32().copied().unwrap(),
                TwinAdderIcon: self.row.columns[17].into_u32().copied().unwrap(),
                ImmortalFlamesIcon: self.row.columns[20].into_u32().copied().unwrap(),
                Unknown0: self.row.columns[2].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[5].into_u8().copied().unwrap(),
                Unknown2: self.row.columns[8].into_u8().copied().unwrap(),
            },
        ]
    }
}
