//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
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
}
impl WKSCosmoToolClassSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WKSCosmoToolClass")?;
        let sheet = resolver.read_excel_sheet(&exh, "WKSCosmoToolClass", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for WKSCosmoToolClassSheet {
    type Row = WKSCosmoToolClassRow;
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
impl<'a> IntoIterator for &'a WKSCosmoToolClassSheet {
    type Item = (u32, Vec<(u16, WKSCosmoToolClassRow)>);
    type IntoIter = StructuredSheetIterator<'a, WKSCosmoToolClassSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WKSCosmoToolClassSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WKSCosmoToolClassRow {
    columns: Vec<Field>,
}
impl WKSCosmoToolClassRow {
    pub fn Stages<'a>(&'a self) -> [StagesElement<'a>; 17] {
        [
            StagesElement {
                Unknown0: &self.columns[0],
                Item: &self.columns[1],
                Name: &self.columns[2],
            },
            StagesElement {
                Unknown0: &self.columns[3],
                Item: &self.columns[4],
                Name: &self.columns[5],
            },
            StagesElement {
                Unknown0: &self.columns[6],
                Item: &self.columns[7],
                Name: &self.columns[8],
            },
            StagesElement {
                Unknown0: &self.columns[9],
                Item: &self.columns[10],
                Name: &self.columns[11],
            },
            StagesElement {
                Unknown0: &self.columns[12],
                Item: &self.columns[13],
                Name: &self.columns[14],
            },
            StagesElement {
                Unknown0: &self.columns[15],
                Item: &self.columns[16],
                Name: &self.columns[17],
            },
            StagesElement {
                Unknown0: &self.columns[18],
                Item: &self.columns[19],
                Name: &self.columns[20],
            },
            StagesElement {
                Unknown0: &self.columns[21],
                Item: &self.columns[22],
                Name: &self.columns[23],
            },
            StagesElement {
                Unknown0: &self.columns[24],
                Item: &self.columns[25],
                Name: &self.columns[26],
            },
            StagesElement {
                Unknown0: &self.columns[27],
                Item: &self.columns[28],
                Name: &self.columns[29],
            },
            StagesElement {
                Unknown0: &self.columns[30],
                Item: &self.columns[31],
                Name: &self.columns[32],
            },
            StagesElement {
                Unknown0: &self.columns[33],
                Item: &self.columns[34],
                Name: &self.columns[35],
            },
            StagesElement {
                Unknown0: &self.columns[36],
                Item: &self.columns[37],
                Name: &self.columns[38],
            },
            StagesElement {
                Unknown0: &self.columns[39],
                Item: &self.columns[40],
                Name: &self.columns[41],
            },
            StagesElement {
                Unknown0: &self.columns[42],
                Item: &self.columns[43],
                Name: &self.columns[44],
            },
            StagesElement {
                Unknown0: &self.columns[45],
                Item: &self.columns[46],
                Name: &self.columns[47],
            },
            StagesElement {
                Unknown0: &self.columns[48],
                Item: &self.columns[49],
                Name: &self.columns[50],
            },
        ]
    }
    pub fn Types<'a>(&'a self) -> [TypesElement<'a>; 6] {
        [
            TypesElement {
                Icon: &self.columns[51],
                Name: &self.columns[52],
                CosmicName: &self.columns[53],
            },
            TypesElement {
                Icon: &self.columns[54],
                Name: &self.columns[55],
                CosmicName: &self.columns[56],
            },
            TypesElement {
                Icon: &self.columns[57],
                Name: &self.columns[58],
                CosmicName: &self.columns[59],
            },
            TypesElement {
                Icon: &self.columns[60],
                Name: &self.columns[61],
                CosmicName: &self.columns[62],
            },
            TypesElement {
                Icon: &self.columns[63],
                Name: &self.columns[64],
                CosmicName: &self.columns[65],
            },
            TypesElement {
                Icon: &self.columns[66],
                Name: &self.columns[67],
                CosmicName: &self.columns[68],
            },
        ]
    }
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[69]
    }
    pub fn DataAmount<'a>(&'a self) -> &'a Field {
        &self.columns[70]
    }
}
