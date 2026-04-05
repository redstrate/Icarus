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
pub struct Frontline01Sheet {
    sheet: Sheet,
}
impl Frontline01Sheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Frontline01")?;
        let sheet = resolver.read_excel_sheet(&exh, "Frontline01", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<Frontline01Row> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<Frontline01Row> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for Frontline01Sheet {
    type Row = Frontline01Row<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a Frontline01Sheet {
    type Item = (u32, Vec<(u16, Frontline01Row<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, Frontline01Sheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, Frontline01Sheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Frontline01Row<'a> {
    row: &'a Row,
}
impl<'a> Frontline01Row<'a> {
    pub fn Unknown0(&'a self) -> u32 {
        self.row.columns[0].into_u32().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> u32 {
        self.row.columns[18].into_u32().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> u32 {
        self.row.columns[27].into_u32().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> u16 {
        self.row.columns[9].into_u16().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> u32 {
        self.row.columns[1].into_u32().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> u32 {
        self.row.columns[19].into_u32().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> u32 {
        self.row.columns[28].into_u32().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> u16 {
        self.row.columns[10].into_u16().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> u32 {
        self.row.columns[2].into_u32().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> u32 {
        self.row.columns[20].into_u32().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> u32 {
        self.row.columns[29].into_u32().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> u16 {
        self.row.columns[11].into_u16().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> u32 {
        self.row.columns[3].into_u32().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> u32 {
        self.row.columns[21].into_u32().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> u32 {
        self.row.columns[30].into_u32().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> u16 {
        self.row.columns[12].into_u16().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> u32 {
        self.row.columns[4].into_u32().copied().unwrap()
    }
    pub fn Unknown17(&'a self) -> u32 {
        self.row.columns[22].into_u32().copied().unwrap()
    }
    pub fn Unknown18(&'a self) -> u32 {
        self.row.columns[31].into_u32().copied().unwrap()
    }
    pub fn Unknown19(&'a self) -> u16 {
        self.row.columns[13].into_u16().copied().unwrap()
    }
    pub fn Unknown20(&'a self) -> u32 {
        self.row.columns[5].into_u32().copied().unwrap()
    }
    pub fn Unknown21(&'a self) -> u32 {
        self.row.columns[23].into_u32().copied().unwrap()
    }
    pub fn Unknown22(&'a self) -> u32 {
        self.row.columns[32].into_u32().copied().unwrap()
    }
    pub fn Unknown23(&'a self) -> u16 {
        self.row.columns[14].into_u16().copied().unwrap()
    }
    pub fn Unknown24(&'a self) -> u32 {
        self.row.columns[6].into_u32().copied().unwrap()
    }
    pub fn Unknown25(&'a self) -> u32 {
        self.row.columns[24].into_u32().copied().unwrap()
    }
    pub fn Unknown26(&'a self) -> u32 {
        self.row.columns[33].into_u32().copied().unwrap()
    }
    pub fn Unknown27(&'a self) -> u16 {
        self.row.columns[15].into_u16().copied().unwrap()
    }
    pub fn Unknown28(&'a self) -> u32 {
        self.row.columns[7].into_u32().copied().unwrap()
    }
    pub fn Unknown29(&'a self) -> u32 {
        self.row.columns[25].into_u32().copied().unwrap()
    }
    pub fn Unknown30(&'a self) -> u32 {
        self.row.columns[34].into_u32().copied().unwrap()
    }
    pub fn Unknown31(&'a self) -> u16 {
        self.row.columns[16].into_u16().copied().unwrap()
    }
    pub fn Unknown32(&'a self) -> u32 {
        self.row.columns[8].into_u32().copied().unwrap()
    }
    pub fn Unknown33(&'a self) -> u32 {
        self.row.columns[26].into_u32().copied().unwrap()
    }
    pub fn Unknown34(&'a self) -> u32 {
        self.row.columns[35].into_u32().copied().unwrap()
    }
    pub fn Unknown35(&'a self) -> u16 {
        self.row.columns[17].into_u16().copied().unwrap()
    }
}
