//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct StagesElement<'a> {
    pub Unknown0: &'a Field,
    pub Item: &'a Field,
    pub Name: &'a Field,
}
pub struct TypesElement<'a> {
    pub Icon: &'a Field,
    pub Name: &'a Field,
    pub CosmicName: &'a Field,
}
#[derive(Debug, Clone)]
pub struct WKSCosmoToolClassSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl WKSCosmoToolClassSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSCosmoToolClass")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSCosmoToolClass", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<WKSCosmoToolClassRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WKSCosmoToolClassRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for WKSCosmoToolClassSheet {
    type Row = WKSCosmoToolClassRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a WKSCosmoToolClassSheet {
    type Item = (u32, Vec<(u16, WKSCosmoToolClassRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WKSCosmoToolClassSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSCosmoToolClassSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WKSCosmoToolClassRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> WKSCosmoToolClassRow<'a> {
    pub fn Stages(&'a self) -> [StagesElement<'a>; 17] {
        [
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[0]],
                Item: &self.row.columns[self.index_mapping[1]],
                Name: &self.row.columns[self.index_mapping[2]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[3]],
                Item: &self.row.columns[self.index_mapping[4]],
                Name: &self.row.columns[self.index_mapping[5]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[6]],
                Item: &self.row.columns[self.index_mapping[7]],
                Name: &self.row.columns[self.index_mapping[8]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[9]],
                Item: &self.row.columns[self.index_mapping[10]],
                Name: &self.row.columns[self.index_mapping[11]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[12]],
                Item: &self.row.columns[self.index_mapping[13]],
                Name: &self.row.columns[self.index_mapping[14]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[15]],
                Item: &self.row.columns[self.index_mapping[16]],
                Name: &self.row.columns[self.index_mapping[17]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[18]],
                Item: &self.row.columns[self.index_mapping[19]],
                Name: &self.row.columns[self.index_mapping[20]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[21]],
                Item: &self.row.columns[self.index_mapping[22]],
                Name: &self.row.columns[self.index_mapping[23]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[24]],
                Item: &self.row.columns[self.index_mapping[25]],
                Name: &self.row.columns[self.index_mapping[26]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[27]],
                Item: &self.row.columns[self.index_mapping[28]],
                Name: &self.row.columns[self.index_mapping[29]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[30]],
                Item: &self.row.columns[self.index_mapping[31]],
                Name: &self.row.columns[self.index_mapping[32]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[33]],
                Item: &self.row.columns[self.index_mapping[34]],
                Name: &self.row.columns[self.index_mapping[35]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[36]],
                Item: &self.row.columns[self.index_mapping[37]],
                Name: &self.row.columns[self.index_mapping[38]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[39]],
                Item: &self.row.columns[self.index_mapping[40]],
                Name: &self.row.columns[self.index_mapping[41]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[42]],
                Item: &self.row.columns[self.index_mapping[43]],
                Name: &self.row.columns[self.index_mapping[44]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[45]],
                Item: &self.row.columns[self.index_mapping[46]],
                Name: &self.row.columns[self.index_mapping[47]],
            },
            StagesElement {
                Unknown0: &self.row.columns[self.index_mapping[48]],
                Item: &self.row.columns[self.index_mapping[49]],
                Name: &self.row.columns[self.index_mapping[50]],
            },
        ]
    }
    pub fn Types(&'a self) -> [TypesElement<'a>; 6] {
        [
            TypesElement {
                Icon: &self.row.columns[self.index_mapping[51]],
                Name: &self.row.columns[self.index_mapping[52]],
                CosmicName: &self.row.columns[self.index_mapping[53]],
            },
            TypesElement {
                Icon: &self.row.columns[self.index_mapping[54]],
                Name: &self.row.columns[self.index_mapping[55]],
                CosmicName: &self.row.columns[self.index_mapping[56]],
            },
            TypesElement {
                Icon: &self.row.columns[self.index_mapping[57]],
                Name: &self.row.columns[self.index_mapping[58]],
                CosmicName: &self.row.columns[self.index_mapping[59]],
            },
            TypesElement {
                Icon: &self.row.columns[self.index_mapping[60]],
                Name: &self.row.columns[self.index_mapping[61]],
                CosmicName: &self.row.columns[self.index_mapping[62]],
            },
            TypesElement {
                Icon: &self.row.columns[self.index_mapping[63]],
                Name: &self.row.columns[self.index_mapping[64]],
                CosmicName: &self.row.columns[self.index_mapping[65]],
            },
            TypesElement {
                Icon: &self.row.columns[self.index_mapping[66]],
                Name: &self.row.columns[self.index_mapping[67]],
                CosmicName: &self.row.columns[self.index_mapping[68]],
            },
        ]
    }
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[69]]
    }
    pub fn DataAmount(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[70]]
    }
}
