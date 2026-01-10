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
pub struct PresetCameraAdjustSheet {
    sheet: Sheet,
}
impl PresetCameraAdjustSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PresetCameraAdjust")?;
        let sheet = resolver.read_excel_sheet(&exh, "PresetCameraAdjust", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PresetCameraAdjustRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PresetCameraAdjustRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for PresetCameraAdjustSheet {
    type Row = PresetCameraAdjustRow;
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
impl<'a> IntoIterator for &'a PresetCameraAdjustSheet {
    type Item = (u32, Vec<(u16, PresetCameraAdjustRow)>);
    type IntoIter = StructuredSheetIterator<'a, PresetCameraAdjustSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PresetCameraAdjustSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PresetCameraAdjustRow {
    columns: Vec<Field>,
}
impl PresetCameraAdjustRow {
    pub fn Hyur_M<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Hyur_F<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn Elezen_M<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Elezen_F<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Lalafell_M<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn Lalafell_F<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn Miqote_M<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Miqote_F<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn Roe_M<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn Roe_F<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn Hrothgar_M<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn Hrothgar_F<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn Viera_M<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn Viera_F<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn Unknown_70<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
}
