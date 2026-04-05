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
pub struct TripleTriadCardResidentSheet {
    sheet: Sheet,
}
impl TripleTriadCardResidentSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TripleTriadCardResident")?;
        let sheet = resolver
            .read_excel_sheet(&exh, "TripleTriadCardResident", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<TripleTriadCardResidentRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<TripleTriadCardResidentRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for TripleTriadCardResidentSheet {
    type Row = TripleTriadCardResidentRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a TripleTriadCardResidentSheet {
    type Item = (u32, Vec<(u16, TripleTriadCardResidentRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, TripleTriadCardResidentSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TripleTriadCardResidentSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TripleTriadCardResidentRow<'a> {
    row: &'a Row,
}
impl<'a> TripleTriadCardResidentRow<'a> {
    pub fn Acquisition(&'a self) -> u32 {
        self.row.columns[13].into_u32().copied().unwrap()
    }
    pub fn Location(&'a self) -> u32 {
        self.row.columns[14].into_u32().copied().unwrap()
    }
    pub fn Quest(&'a self) -> u32 {
        self.row.columns[15].into_u32().copied().unwrap()
    }
    pub fn Unknown0(&'a self) -> u16 {
        self.row.columns[0].into_u16().copied().unwrap()
    }
    pub fn SaleValue(&'a self) -> u16 {
        self.row.columns[7].into_u16().copied().unwrap()
    }
    pub fn Order(&'a self) -> u16 {
        self.row.columns[9].into_u16().copied().unwrap()
    }
    pub fn Top(&'a self) -> u8 {
        self.row.columns[1].into_u8().copied().unwrap()
    }
    pub fn Bottom(&'a self) -> u8 {
        self.row.columns[2].into_u8().copied().unwrap()
    }
    pub fn Left(&'a self) -> u8 {
        self.row.columns[3].into_u8().copied().unwrap()
    }
    pub fn Right(&'a self) -> u8 {
        self.row.columns[4].into_u8().copied().unwrap()
    }
    pub fn TripleTriadCardRarity(&'a self) -> u8 {
        self.row.columns[5].into_u8().copied().unwrap()
    }
    pub fn TripleTriadCardType(&'a self) -> u8 {
        self.row.columns[6].into_u8().copied().unwrap()
    }
    pub fn SortKey(&'a self) -> u8 {
        self.row.columns[8].into_u8().copied().unwrap()
    }
    pub fn UIPriority(&'a self) -> u8 {
        self.row.columns[10].into_u8().copied().unwrap()
    }
    pub fn AcquisitionType(&'a self) -> u8 {
        self.row.columns[12].into_u8().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> bool {
        self.row.columns[11].into_bool().copied().unwrap()
    }
}
