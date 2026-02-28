//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct ItemDataElement<'a> {
    pub Item: &'a Field,
    pub Cost: &'a Field,
    pub FCRankRequired: &'a Field,
}
#[derive(Debug, Clone)]
pub struct FccShopSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl FccShopSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("FccShop")?;
        let sheet = resolver.read_excel_sheet(&exh, "FccShop", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<FccShopRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<FccShopRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for FccShopSheet {
    type Row = FccShopRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a FccShopSheet {
    type Item = (u32, Vec<(u16, FccShopRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, FccShopSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, FccShopSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FccShopRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> FccShopRow<'a> {
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn ItemData(&'a self) -> [ItemDataElement<'a>; 10] {
        [
            ItemDataElement {
                Item: &self.row.columns[self.index_mapping[1]],
                Cost: &self.row.columns[self.index_mapping[2]],
                FCRankRequired: &self.row.columns[self.index_mapping[3]],
            },
            ItemDataElement {
                Item: &self.row.columns[self.index_mapping[4]],
                Cost: &self.row.columns[self.index_mapping[5]],
                FCRankRequired: &self.row.columns[self.index_mapping[6]],
            },
            ItemDataElement {
                Item: &self.row.columns[self.index_mapping[7]],
                Cost: &self.row.columns[self.index_mapping[8]],
                FCRankRequired: &self.row.columns[self.index_mapping[9]],
            },
            ItemDataElement {
                Item: &self.row.columns[self.index_mapping[10]],
                Cost: &self.row.columns[self.index_mapping[11]],
                FCRankRequired: &self.row.columns[self.index_mapping[12]],
            },
            ItemDataElement {
                Item: &self.row.columns[self.index_mapping[13]],
                Cost: &self.row.columns[self.index_mapping[14]],
                FCRankRequired: &self.row.columns[self.index_mapping[15]],
            },
            ItemDataElement {
                Item: &self.row.columns[self.index_mapping[16]],
                Cost: &self.row.columns[self.index_mapping[17]],
                FCRankRequired: &self.row.columns[self.index_mapping[18]],
            },
            ItemDataElement {
                Item: &self.row.columns[self.index_mapping[19]],
                Cost: &self.row.columns[self.index_mapping[20]],
                FCRankRequired: &self.row.columns[self.index_mapping[21]],
            },
            ItemDataElement {
                Item: &self.row.columns[self.index_mapping[22]],
                Cost: &self.row.columns[self.index_mapping[23]],
                FCRankRequired: &self.row.columns[self.index_mapping[24]],
            },
            ItemDataElement {
                Item: &self.row.columns[self.index_mapping[25]],
                Cost: &self.row.columns[self.index_mapping[26]],
                FCRankRequired: &self.row.columns[self.index_mapping[27]],
            },
            ItemDataElement {
                Item: &self.row.columns[self.index_mapping[28]],
                Cost: &self.row.columns[self.index_mapping[29]],
                FCRankRequired: &self.row.columns[self.index_mapping[30]],
            },
        ]
    }
}
