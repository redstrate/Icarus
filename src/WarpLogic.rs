//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct WarpParamsElement<'a> {
    pub Function: &'a Field,
    pub Argument: &'a Field,
}
#[derive(Debug, Clone)]
pub struct WarpLogicSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl WarpLogicSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WarpLogic")?;
        let sheet = resolver.read_excel_sheet(&exh, "WarpLogic", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<WarpLogicRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WarpLogicRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for WarpLogicSheet {
    type Row = WarpLogicRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a WarpLogicSheet {
    type Item = (u32, Vec<(u16, WarpLogicRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WarpLogicSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WarpLogicSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WarpLogicRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> WarpLogicRow<'a> {
    pub fn WarpParams(&'a self) -> [WarpParamsElement<'a>; 10] {
        [
            WarpParamsElement {
                Function: &self.row.columns[self.index_mapping[0]],
                Argument: &self.row.columns[self.index_mapping[1]],
            },
            WarpParamsElement {
                Function: &self.row.columns[self.index_mapping[2]],
                Argument: &self.row.columns[self.index_mapping[3]],
            },
            WarpParamsElement {
                Function: &self.row.columns[self.index_mapping[4]],
                Argument: &self.row.columns[self.index_mapping[5]],
            },
            WarpParamsElement {
                Function: &self.row.columns[self.index_mapping[6]],
                Argument: &self.row.columns[self.index_mapping[7]],
            },
            WarpParamsElement {
                Function: &self.row.columns[self.index_mapping[8]],
                Argument: &self.row.columns[self.index_mapping[9]],
            },
            WarpParamsElement {
                Function: &self.row.columns[self.index_mapping[10]],
                Argument: &self.row.columns[self.index_mapping[11]],
            },
            WarpParamsElement {
                Function: &self.row.columns[self.index_mapping[12]],
                Argument: &self.row.columns[self.index_mapping[13]],
            },
            WarpParamsElement {
                Function: &self.row.columns[self.index_mapping[14]],
                Argument: &self.row.columns[self.index_mapping[15]],
            },
            WarpParamsElement {
                Function: &self.row.columns[self.index_mapping[16]],
                Argument: &self.row.columns[self.index_mapping[17]],
            },
            WarpParamsElement {
                Function: &self.row.columns[self.index_mapping[18]],
                Argument: &self.row.columns[self.index_mapping[19]],
            },
        ]
    }
    pub fn Question(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn ResponseYes(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn ResponseNo(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn WarpName(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn CanSkipCutscene(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
}
