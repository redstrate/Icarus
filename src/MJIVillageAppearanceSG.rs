//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
};
pub struct VillageAppearanceDataElement<'a> {
    pub UnknownParam: &'a Field,
    pub SGB: &'a Field,
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
impl StructuredSheet for MJIVillageAppearanceSGSheet {
    type Row = MJIVillageAppearanceSGRow;
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
impl<'a> IntoIterator for &'a MJIVillageAppearanceSGSheet {
    type Item = (u32, Vec<(u16, MJIVillageAppearanceSGRow)>);
    type IntoIter = StructuredSheetIterator<'a, MJIVillageAppearanceSGSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, MJIVillageAppearanceSGSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MJIVillageAppearanceSGRow {
    columns: Vec<Field>,
}
impl MJIVillageAppearanceSGRow {
    pub fn VillageAppearanceData<'a>(&'a self) -> [VillageAppearanceDataElement<'a>; 5] {
        [
            VillageAppearanceDataElement {
                UnknownParam: &self.columns[0],
                SGB: &self.columns[1],
            },
            VillageAppearanceDataElement {
                UnknownParam: &self.columns[2],
                SGB: &self.columns[3],
            },
            VillageAppearanceDataElement {
                UnknownParam: &self.columns[4],
                SGB: &self.columns[5],
            },
            VillageAppearanceDataElement {
                UnknownParam: &self.columns[6],
                SGB: &self.columns[7],
            },
            VillageAppearanceDataElement {
                UnknownParam: &self.columns[8],
                SGB: &self.columns[9],
            },
        ]
    }
}
