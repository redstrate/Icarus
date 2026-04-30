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
pub struct IKDFishParamSheet {
    sheet: Sheet,
}
impl IKDFishParamSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("IKDFishParam")?;
        let sheet = resolver.read_excel_sheet(&exh, "IKDFishParam", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<IKDFishParamRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<IKDFishParamRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for IKDFishParamSheet {
    type Row = IKDFishParamRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        Some(Self::Row {
            Fish: row
                .columns[0]
                .into_u32()
                .copied()
                .expect("Expected column 0 to be a uint32!"),
            IKDContentBonus: [
                row
                    .columns[1]
                    .into_u8()
                    .copied()
                    .expect("Expected column 1 to be a uint8!"),
                row
                    .columns[2]
                    .into_u8()
                    .copied()
                    .expect("Expected column 2 to be a uint8!"),
            ],
        })
    }
}
impl<'a> IntoIterator for &'a IKDFishParamSheet {
    type Item = (u32, Vec<(u16, IKDFishParamRow)>);
    type IntoIter = StructuredSheetIterator<'a, IKDFishParamSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, IKDFishParamSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct IKDFishParamRow {
    ///""
    pub Fish: u32,
    ///""
    pub IKDContentBonus: [u8; 2],
}
