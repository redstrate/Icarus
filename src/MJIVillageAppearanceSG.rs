//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VillageAppearanceDataElement {
    pub UnknownParam: u32,
    pub SGB: u16,
}
#[derive(Debug, Clone)]
pub struct MJIVillageAppearanceSGSheet {
    sheet: Sheet,
}
impl MJIVillageAppearanceSGSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("MJIVillageAppearanceSG")?;
        let sheet = resolver.read_excel_sheet(&exh, "MJIVillageAppearanceSG", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<MJIVillageAppearanceSGRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<MJIVillageAppearanceSGRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for MJIVillageAppearanceSGSheet {
    type Row = MJIVillageAppearanceSGRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a MJIVillageAppearanceSGSheet {
    type Item = (u32, Vec<(u16, MJIVillageAppearanceSGRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, MJIVillageAppearanceSGSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MJIVillageAppearanceSGSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MJIVillageAppearanceSGRow<'a> {
    row: &'a Row,
}
impl<'a> MJIVillageAppearanceSGRow<'a> {
    pub fn VillageAppearanceData(&'a self) -> [VillageAppearanceDataElement; 5] {
        [
            VillageAppearanceDataElement {
                UnknownParam: self.row.columns[5].into_u32().copied().unwrap(),
                SGB: self.row.columns[0].into_u16().copied().unwrap(),
            },
            VillageAppearanceDataElement {
                UnknownParam: self.row.columns[6].into_u32().copied().unwrap(),
                SGB: self.row.columns[1].into_u16().copied().unwrap(),
            },
            VillageAppearanceDataElement {
                UnknownParam: self.row.columns[7].into_u32().copied().unwrap(),
                SGB: self.row.columns[2].into_u16().copied().unwrap(),
            },
            VillageAppearanceDataElement {
                UnknownParam: self.row.columns[8].into_u32().copied().unwrap(),
                SGB: self.row.columns[3].into_u16().copied().unwrap(),
            },
            VillageAppearanceDataElement {
                UnknownParam: self.row.columns[9].into_u32().copied().unwrap(),
                SGB: self.row.columns[4].into_u16().copied().unwrap(),
            },
        ]
    }
}
