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
}
impl WarpLogicSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WarpLogic")?;
        let sheet = resolver.read_excel_sheet(&exh, "WarpLogic", language)?;
        Ok(Self { sheet })
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
impl StructuredSheet for WarpLogicSheet {
    type Row = WarpLogicRow;
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
impl<'a> IntoIterator for &'a WarpLogicSheet {
    type Item = (u32, Vec<(u16, WarpLogicRow)>);
    type IntoIter = StructuredSheetIterator<'a, WarpLogicSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WarpLogicSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WarpLogicRow {
    columns: Vec<Field>,
}
impl WarpLogicRow {
    pub fn WarpParams<'a>(&'a self) -> [WarpParamsElement<'a>; 10] {
        [
            WarpParamsElement {
                Function: &self.columns[0],
                Argument: &self.columns[1],
            },
            WarpParamsElement {
                Function: &self.columns[2],
                Argument: &self.columns[3],
            },
            WarpParamsElement {
                Function: &self.columns[4],
                Argument: &self.columns[5],
            },
            WarpParamsElement {
                Function: &self.columns[6],
                Argument: &self.columns[7],
            },
            WarpParamsElement {
                Function: &self.columns[8],
                Argument: &self.columns[9],
            },
            WarpParamsElement {
                Function: &self.columns[10],
                Argument: &self.columns[11],
            },
            WarpParamsElement {
                Function: &self.columns[12],
                Argument: &self.columns[13],
            },
            WarpParamsElement {
                Function: &self.columns[14],
                Argument: &self.columns[15],
            },
            WarpParamsElement {
                Function: &self.columns[16],
                Argument: &self.columns[17],
            },
            WarpParamsElement {
                Function: &self.columns[18],
                Argument: &self.columns[19],
            },
        ]
    }
    pub fn Question<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn ResponseYes<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn ResponseNo<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn WarpName<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn CanSkipCutscene<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
}
