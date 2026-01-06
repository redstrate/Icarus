//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
};
pub struct SupplyDataElement<'a> {
    pub Item: [&'a Field; 3],
    pub ItemCount: [&'a Field; 3],
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
impl StructuredSheet for GCSupplyDutySheet {
    type Row = GCSupplyDutyRow;
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
impl<'a> IntoIterator for &'a GCSupplyDutySheet {
    type Item = (u32, Vec<(u16, GCSupplyDutyRow)>);
    type IntoIter = StructuredSheetIterator<'a, GCSupplyDutySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GCSupplyDutySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GCSupplyDutyRow {
    columns: Vec<Field>,
}
impl GCSupplyDutyRow {
    pub fn SupplyData<'a>(&'a self) -> [SupplyDataElement<'a>; 11] {
        [
            SupplyDataElement {
                Item: [&self.columns[0], &self.columns[1], &self.columns[2]],
                ItemCount: [&self.columns[3], &self.columns[4], &self.columns[5]],
            },
            SupplyDataElement {
                Item: [&self.columns[6], &self.columns[7], &self.columns[8]],
                ItemCount: [&self.columns[9], &self.columns[10], &self.columns[11]],
            },
            SupplyDataElement {
                Item: [&self.columns[12], &self.columns[13], &self.columns[14]],
                ItemCount: [&self.columns[15], &self.columns[16], &self.columns[17]],
            },
            SupplyDataElement {
                Item: [&self.columns[18], &self.columns[19], &self.columns[20]],
                ItemCount: [&self.columns[21], &self.columns[22], &self.columns[23]],
            },
            SupplyDataElement {
                Item: [&self.columns[24], &self.columns[25], &self.columns[26]],
                ItemCount: [&self.columns[27], &self.columns[28], &self.columns[29]],
            },
            SupplyDataElement {
                Item: [&self.columns[30], &self.columns[31], &self.columns[32]],
                ItemCount: [&self.columns[33], &self.columns[34], &self.columns[35]],
            },
            SupplyDataElement {
                Item: [&self.columns[36], &self.columns[37], &self.columns[38]],
                ItemCount: [&self.columns[39], &self.columns[40], &self.columns[41]],
            },
            SupplyDataElement {
                Item: [&self.columns[42], &self.columns[43], &self.columns[44]],
                ItemCount: [&self.columns[45], &self.columns[46], &self.columns[47]],
            },
            SupplyDataElement {
                Item: [&self.columns[48], &self.columns[49], &self.columns[50]],
                ItemCount: [&self.columns[51], &self.columns[52], &self.columns[53]],
            },
            SupplyDataElement {
                Item: [&self.columns[54], &self.columns[55], &self.columns[56]],
                ItemCount: [&self.columns[57], &self.columns[58], &self.columns[59]],
            },
            SupplyDataElement {
                Item: [&self.columns[60], &self.columns[61], &self.columns[62]],
                ItemCount: [&self.columns[63], &self.columns[64], &self.columns[65]],
            },
        ]
    }
}
