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
pub struct TiltParamSheet {
    sheet: Sheet,
}
impl TiltParamSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TiltParam")?;
        let sheet = resolver.read_excel_sheet(&exh, "TiltParam", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<TiltParamRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<TiltParamRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for TiltParamSheet {
    type Row = TiltParamRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            TiltSpeed: row
                .columns[3]
                .into_f32()
                .copied()
                .expect("Expected column 3 to be a float32!"),
            RotationOrigin: row
                .columns[0]
                .into_u8()
                .copied()
                .expect("Expected column 0 to be a uint8!"),
            TiltAngle: row
                .columns[1]
                .into_u8()
                .copied()
                .expect("Expected column 1 to be a uint8!"),
            Unknown3: row
                .columns[4]
                .into_u8()
                .copied()
                .expect("Expected column 4 to be a uint8!"),
            Unknown4: row
                .columns[5]
                .into_u8()
                .copied()
                .expect("Expected column 5 to be a uint8!"),
            ReverseRotation: row
                .columns[2]
                .into_bool()
                .copied()
                .expect("Expected column 2 to be a bool!"),
        })
    }
}
impl<'a> IntoIterator for &'a TiltParamSheet {
    type Item = (u32, Vec<(u16, TiltParamRow)>);
    type IntoIter = StructuredSheetIterator<'a, TiltParamSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TiltParamSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct TiltParamRow {
    ///""
    pub TiltSpeed: f32,
    ///""
    pub RotationOrigin: u8,
    ///""
    pub TiltAngle: u8,
    ///""
    pub Unknown3: u8,
    ///""
    pub Unknown4: u8,
    ///""
    pub ReverseRotation: bool,
}
