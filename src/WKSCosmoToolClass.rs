//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct StagesElement {
    pub Unknown0: u32,
    pub Item: i32,
    pub Name: u16,
}
pub struct TypesElement {
    pub Icon: u32,
    pub Name: u16,
    pub CosmicName: u16,
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
impl<'a> StructuredSheet<'a> for WKSCosmoToolClassSheet {
    type Row = WKSCosmoToolClassRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
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
}
impl<'a> WKSCosmoToolClassRow<'a> {
    pub fn Stages(&'a self) -> [StagesElement; 17] {
        [
            StagesElement {
                Unknown0: self.row.columns[34].into_u32().copied().unwrap(),
                Item: self.row.columns[17].into_i32().copied().unwrap(),
                Name: self.row.columns[0].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[35].into_u32().copied().unwrap(),
                Item: self.row.columns[18].into_i32().copied().unwrap(),
                Name: self.row.columns[1].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[36].into_u32().copied().unwrap(),
                Item: self.row.columns[19].into_i32().copied().unwrap(),
                Name: self.row.columns[2].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[37].into_u32().copied().unwrap(),
                Item: self.row.columns[20].into_i32().copied().unwrap(),
                Name: self.row.columns[3].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[38].into_u32().copied().unwrap(),
                Item: self.row.columns[21].into_i32().copied().unwrap(),
                Name: self.row.columns[4].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[39].into_u32().copied().unwrap(),
                Item: self.row.columns[22].into_i32().copied().unwrap(),
                Name: self.row.columns[5].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[40].into_u32().copied().unwrap(),
                Item: self.row.columns[23].into_i32().copied().unwrap(),
                Name: self.row.columns[6].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[41].into_u32().copied().unwrap(),
                Item: self.row.columns[24].into_i32().copied().unwrap(),
                Name: self.row.columns[7].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[42].into_u32().copied().unwrap(),
                Item: self.row.columns[25].into_i32().copied().unwrap(),
                Name: self.row.columns[8].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[43].into_u32().copied().unwrap(),
                Item: self.row.columns[26].into_i32().copied().unwrap(),
                Name: self.row.columns[9].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[44].into_u32().copied().unwrap(),
                Item: self.row.columns[27].into_i32().copied().unwrap(),
                Name: self.row.columns[10].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[45].into_u32().copied().unwrap(),
                Item: self.row.columns[28].into_i32().copied().unwrap(),
                Name: self.row.columns[11].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[46].into_u32().copied().unwrap(),
                Item: self.row.columns[29].into_i32().copied().unwrap(),
                Name: self.row.columns[12].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[47].into_u32().copied().unwrap(),
                Item: self.row.columns[30].into_i32().copied().unwrap(),
                Name: self.row.columns[13].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[48].into_u32().copied().unwrap(),
                Item: self.row.columns[31].into_i32().copied().unwrap(),
                Name: self.row.columns[14].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[49].into_u32().copied().unwrap(),
                Item: self.row.columns[32].into_i32().copied().unwrap(),
                Name: self.row.columns[15].into_u16().copied().unwrap(),
            },
            StagesElement {
                Unknown0: self.row.columns[50].into_u32().copied().unwrap(),
                Item: self.row.columns[33].into_i32().copied().unwrap(),
                Name: self.row.columns[16].into_u16().copied().unwrap(),
            },
        ]
    }
    pub fn Types(&'a self) -> [TypesElement; 6] {
        [
            TypesElement {
                Icon: self.row.columns[63].into_u32().copied().unwrap(),
                Name: self.row.columns[51].into_u16().copied().unwrap(),
                CosmicName: self.row.columns[57].into_u16().copied().unwrap(),
            },
            TypesElement {
                Icon: self.row.columns[64].into_u32().copied().unwrap(),
                Name: self.row.columns[52].into_u16().copied().unwrap(),
                CosmicName: self.row.columns[58].into_u16().copied().unwrap(),
            },
            TypesElement {
                Icon: self.row.columns[65].into_u32().copied().unwrap(),
                Name: self.row.columns[53].into_u16().copied().unwrap(),
                CosmicName: self.row.columns[59].into_u16().copied().unwrap(),
            },
            TypesElement {
                Icon: self.row.columns[66].into_u32().copied().unwrap(),
                Name: self.row.columns[54].into_u16().copied().unwrap(),
                CosmicName: self.row.columns[60].into_u16().copied().unwrap(),
            },
            TypesElement {
                Icon: self.row.columns[67].into_u32().copied().unwrap(),
                Name: self.row.columns[55].into_u16().copied().unwrap(),
                CosmicName: self.row.columns[61].into_u16().copied().unwrap(),
            },
            TypesElement {
                Icon: self.row.columns[68].into_u32().copied().unwrap(),
                Name: self.row.columns[56].into_u16().copied().unwrap(),
                CosmicName: self.row.columns[62].into_u16().copied().unwrap(),
            },
        ]
    }
    pub fn Name(&'a self) -> u16 {
        self.row.columns[70].into_u16().copied().unwrap()
    }
    pub fn DataAmount(&'a self) -> u8 {
        self.row.columns[69].into_u8().copied().unwrap()
    }
}
