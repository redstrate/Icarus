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
pub struct SupplyDataElement {
    pub Item: [i32; 3],
    pub ItemCount: [u8; 3],
}
#[derive(Debug, Clone)]
pub struct GCSupplyDutySheet {
    sheet: Sheet,
}
impl GCSupplyDutySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GCSupplyDuty")?;
        let sheet = resolver.read_excel_sheet(&exh, "GCSupplyDuty", language)?;
        Ok(Self { sheet })
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
        Some(Self::Row { row })
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
}
impl<'a> GCSupplyDutyRow<'a> {
    pub fn SupplyData(&'a self) -> [SupplyDataElement; 11] {
        [
            SupplyDataElement {
                Item: [
                    self.row.columns[0].into_i32().copied().unwrap(),
                    self.row.columns[2].into_i32().copied().unwrap(),
                    self.row.columns[4].into_i32().copied().unwrap(),
                ],
                ItemCount: [
                    self.row.columns[1].into_u8().copied().unwrap(),
                    self.row.columns[3].into_u8().copied().unwrap(),
                    self.row.columns[5].into_u8().copied().unwrap(),
                ],
            },
            SupplyDataElement {
                Item: [
                    self.row.columns[6].into_i32().copied().unwrap(),
                    self.row.columns[8].into_i32().copied().unwrap(),
                    self.row.columns[10].into_i32().copied().unwrap(),
                ],
                ItemCount: [
                    self.row.columns[7].into_u8().copied().unwrap(),
                    self.row.columns[9].into_u8().copied().unwrap(),
                    self.row.columns[11].into_u8().copied().unwrap(),
                ],
            },
            SupplyDataElement {
                Item: [
                    self.row.columns[12].into_i32().copied().unwrap(),
                    self.row.columns[14].into_i32().copied().unwrap(),
                    self.row.columns[16].into_i32().copied().unwrap(),
                ],
                ItemCount: [
                    self.row.columns[13].into_u8().copied().unwrap(),
                    self.row.columns[15].into_u8().copied().unwrap(),
                    self.row.columns[17].into_u8().copied().unwrap(),
                ],
            },
            SupplyDataElement {
                Item: [
                    self.row.columns[18].into_i32().copied().unwrap(),
                    self.row.columns[20].into_i32().copied().unwrap(),
                    self.row.columns[22].into_i32().copied().unwrap(),
                ],
                ItemCount: [
                    self.row.columns[19].into_u8().copied().unwrap(),
                    self.row.columns[21].into_u8().copied().unwrap(),
                    self.row.columns[23].into_u8().copied().unwrap(),
                ],
            },
            SupplyDataElement {
                Item: [
                    self.row.columns[24].into_i32().copied().unwrap(),
                    self.row.columns[26].into_i32().copied().unwrap(),
                    self.row.columns[28].into_i32().copied().unwrap(),
                ],
                ItemCount: [
                    self.row.columns[25].into_u8().copied().unwrap(),
                    self.row.columns[27].into_u8().copied().unwrap(),
                    self.row.columns[29].into_u8().copied().unwrap(),
                ],
            },
            SupplyDataElement {
                Item: [
                    self.row.columns[30].into_i32().copied().unwrap(),
                    self.row.columns[32].into_i32().copied().unwrap(),
                    self.row.columns[34].into_i32().copied().unwrap(),
                ],
                ItemCount: [
                    self.row.columns[31].into_u8().copied().unwrap(),
                    self.row.columns[33].into_u8().copied().unwrap(),
                    self.row.columns[35].into_u8().copied().unwrap(),
                ],
            },
            SupplyDataElement {
                Item: [
                    self.row.columns[36].into_i32().copied().unwrap(),
                    self.row.columns[38].into_i32().copied().unwrap(),
                    self.row.columns[40].into_i32().copied().unwrap(),
                ],
                ItemCount: [
                    self.row.columns[37].into_u8().copied().unwrap(),
                    self.row.columns[39].into_u8().copied().unwrap(),
                    self.row.columns[41].into_u8().copied().unwrap(),
                ],
            },
            SupplyDataElement {
                Item: [
                    self.row.columns[42].into_i32().copied().unwrap(),
                    self.row.columns[44].into_i32().copied().unwrap(),
                    self.row.columns[46].into_i32().copied().unwrap(),
                ],
                ItemCount: [
                    self.row.columns[43].into_u8().copied().unwrap(),
                    self.row.columns[45].into_u8().copied().unwrap(),
                    self.row.columns[47].into_u8().copied().unwrap(),
                ],
            },
            SupplyDataElement {
                Item: [
                    self.row.columns[48].into_i32().copied().unwrap(),
                    self.row.columns[50].into_i32().copied().unwrap(),
                    self.row.columns[52].into_i32().copied().unwrap(),
                ],
                ItemCount: [
                    self.row.columns[49].into_u8().copied().unwrap(),
                    self.row.columns[51].into_u8().copied().unwrap(),
                    self.row.columns[53].into_u8().copied().unwrap(),
                ],
            },
            SupplyDataElement {
                Item: [
                    self.row.columns[54].into_i32().copied().unwrap(),
                    self.row.columns[56].into_i32().copied().unwrap(),
                    self.row.columns[58].into_i32().copied().unwrap(),
                ],
                ItemCount: [
                    self.row.columns[55].into_u8().copied().unwrap(),
                    self.row.columns[57].into_u8().copied().unwrap(),
                    self.row.columns[59].into_u8().copied().unwrap(),
                ],
            },
            SupplyDataElement {
                Item: [
                    self.row.columns[60].into_i32().copied().unwrap(),
                    self.row.columns[62].into_i32().copied().unwrap(),
                    self.row.columns[64].into_i32().copied().unwrap(),
                ],
                ItemCount: [
                    self.row.columns[61].into_u8().copied().unwrap(),
                    self.row.columns[63].into_u8().copied().unwrap(),
                    self.row.columns[65].into_u8().copied().unwrap(),
                ],
            },
        ]
    }
}
