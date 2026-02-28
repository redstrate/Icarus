//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct SupplyDataElement<'a> {
    pub Item: [&'a Field; 3],
    pub ItemCount: [&'a Field; 3],
}
#[derive(Debug, Clone)]
pub struct GCSupplyDutySheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl GCSupplyDutySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GCSupplyDuty")?;
        let sheet = resolver.read_excel_sheet(&exh, "GCSupplyDuty", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<GCSupplyDutyRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GCSupplyDutyRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for GCSupplyDutySheet {
    type Row = GCSupplyDutyRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a GCSupplyDutySheet {
    type Item = (u32, Vec<(u16, GCSupplyDutyRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, GCSupplyDutySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GCSupplyDutySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GCSupplyDutyRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> GCSupplyDutyRow<'a> {
    pub fn SupplyData(&'a self) -> [SupplyDataElement<'a>; 11] {
        [
            SupplyDataElement {
                Item: [
                    &self.row.columns[self.index_mapping[0]],
                    &self.row.columns[self.index_mapping[1]],
                    &self.row.columns[self.index_mapping[2]],
                ],
                ItemCount: [
                    &self.row.columns[self.index_mapping[3]],
                    &self.row.columns[self.index_mapping[4]],
                    &self.row.columns[self.index_mapping[5]],
                ],
            },
            SupplyDataElement {
                Item: [
                    &self.row.columns[self.index_mapping[6]],
                    &self.row.columns[self.index_mapping[7]],
                    &self.row.columns[self.index_mapping[8]],
                ],
                ItemCount: [
                    &self.row.columns[self.index_mapping[9]],
                    &self.row.columns[self.index_mapping[10]],
                    &self.row.columns[self.index_mapping[11]],
                ],
            },
            SupplyDataElement {
                Item: [
                    &self.row.columns[self.index_mapping[12]],
                    &self.row.columns[self.index_mapping[13]],
                    &self.row.columns[self.index_mapping[14]],
                ],
                ItemCount: [
                    &self.row.columns[self.index_mapping[15]],
                    &self.row.columns[self.index_mapping[16]],
                    &self.row.columns[self.index_mapping[17]],
                ],
            },
            SupplyDataElement {
                Item: [
                    &self.row.columns[self.index_mapping[18]],
                    &self.row.columns[self.index_mapping[19]],
                    &self.row.columns[self.index_mapping[20]],
                ],
                ItemCount: [
                    &self.row.columns[self.index_mapping[21]],
                    &self.row.columns[self.index_mapping[22]],
                    &self.row.columns[self.index_mapping[23]],
                ],
            },
            SupplyDataElement {
                Item: [
                    &self.row.columns[self.index_mapping[24]],
                    &self.row.columns[self.index_mapping[25]],
                    &self.row.columns[self.index_mapping[26]],
                ],
                ItemCount: [
                    &self.row.columns[self.index_mapping[27]],
                    &self.row.columns[self.index_mapping[28]],
                    &self.row.columns[self.index_mapping[29]],
                ],
            },
            SupplyDataElement {
                Item: [
                    &self.row.columns[self.index_mapping[30]],
                    &self.row.columns[self.index_mapping[31]],
                    &self.row.columns[self.index_mapping[32]],
                ],
                ItemCount: [
                    &self.row.columns[self.index_mapping[33]],
                    &self.row.columns[self.index_mapping[34]],
                    &self.row.columns[self.index_mapping[35]],
                ],
            },
            SupplyDataElement {
                Item: [
                    &self.row.columns[self.index_mapping[36]],
                    &self.row.columns[self.index_mapping[37]],
                    &self.row.columns[self.index_mapping[38]],
                ],
                ItemCount: [
                    &self.row.columns[self.index_mapping[39]],
                    &self.row.columns[self.index_mapping[40]],
                    &self.row.columns[self.index_mapping[41]],
                ],
            },
            SupplyDataElement {
                Item: [
                    &self.row.columns[self.index_mapping[42]],
                    &self.row.columns[self.index_mapping[43]],
                    &self.row.columns[self.index_mapping[44]],
                ],
                ItemCount: [
                    &self.row.columns[self.index_mapping[45]],
                    &self.row.columns[self.index_mapping[46]],
                    &self.row.columns[self.index_mapping[47]],
                ],
            },
            SupplyDataElement {
                Item: [
                    &self.row.columns[self.index_mapping[48]],
                    &self.row.columns[self.index_mapping[49]],
                    &self.row.columns[self.index_mapping[50]],
                ],
                ItemCount: [
                    &self.row.columns[self.index_mapping[51]],
                    &self.row.columns[self.index_mapping[52]],
                    &self.row.columns[self.index_mapping[53]],
                ],
            },
            SupplyDataElement {
                Item: [
                    &self.row.columns[self.index_mapping[54]],
                    &self.row.columns[self.index_mapping[55]],
                    &self.row.columns[self.index_mapping[56]],
                ],
                ItemCount: [
                    &self.row.columns[self.index_mapping[57]],
                    &self.row.columns[self.index_mapping[58]],
                    &self.row.columns[self.index_mapping[59]],
                ],
            },
            SupplyDataElement {
                Item: [
                    &self.row.columns[self.index_mapping[60]],
                    &self.row.columns[self.index_mapping[61]],
                    &self.row.columns[self.index_mapping[62]],
                ],
                ItemCount: [
                    &self.row.columns[self.index_mapping[63]],
                    &self.row.columns[self.index_mapping[64]],
                    &self.row.columns[self.index_mapping[65]],
                ],
            },
        ]
    }
}
