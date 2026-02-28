//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct VariablesElement<'a> {
    pub Name: &'a Field,
    pub Value: &'a Field,
}
#[derive(Debug, Clone)]
pub struct OpeningSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl OpeningSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Opening")?;
        let sheet = resolver.read_excel_sheet(&exh, "Opening", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<OpeningRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<OpeningRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for OpeningSheet {
    type Row = OpeningRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a OpeningSheet {
    type Item = (u32, Vec<(u16, OpeningRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, OpeningSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, OpeningSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct OpeningRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> OpeningRow<'a> {
    pub fn Variables(&'a self) -> [VariablesElement<'a>; 40] {
        [
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[0]],
                Value: &self.row.columns[self.index_mapping[1]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[2]],
                Value: &self.row.columns[self.index_mapping[3]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[4]],
                Value: &self.row.columns[self.index_mapping[5]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[6]],
                Value: &self.row.columns[self.index_mapping[7]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[8]],
                Value: &self.row.columns[self.index_mapping[9]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[10]],
                Value: &self.row.columns[self.index_mapping[11]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[12]],
                Value: &self.row.columns[self.index_mapping[13]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[14]],
                Value: &self.row.columns[self.index_mapping[15]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[16]],
                Value: &self.row.columns[self.index_mapping[17]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[18]],
                Value: &self.row.columns[self.index_mapping[19]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[20]],
                Value: &self.row.columns[self.index_mapping[21]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[22]],
                Value: &self.row.columns[self.index_mapping[23]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[24]],
                Value: &self.row.columns[self.index_mapping[25]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[26]],
                Value: &self.row.columns[self.index_mapping[27]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[28]],
                Value: &self.row.columns[self.index_mapping[29]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[30]],
                Value: &self.row.columns[self.index_mapping[31]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[32]],
                Value: &self.row.columns[self.index_mapping[33]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[34]],
                Value: &self.row.columns[self.index_mapping[35]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[36]],
                Value: &self.row.columns[self.index_mapping[37]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[38]],
                Value: &self.row.columns[self.index_mapping[39]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[40]],
                Value: &self.row.columns[self.index_mapping[41]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[42]],
                Value: &self.row.columns[self.index_mapping[43]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[44]],
                Value: &self.row.columns[self.index_mapping[45]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[46]],
                Value: &self.row.columns[self.index_mapping[47]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[48]],
                Value: &self.row.columns[self.index_mapping[49]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[50]],
                Value: &self.row.columns[self.index_mapping[51]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[52]],
                Value: &self.row.columns[self.index_mapping[53]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[54]],
                Value: &self.row.columns[self.index_mapping[55]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[56]],
                Value: &self.row.columns[self.index_mapping[57]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[58]],
                Value: &self.row.columns[self.index_mapping[59]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[60]],
                Value: &self.row.columns[self.index_mapping[61]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[62]],
                Value: &self.row.columns[self.index_mapping[63]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[64]],
                Value: &self.row.columns[self.index_mapping[65]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[66]],
                Value: &self.row.columns[self.index_mapping[67]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[68]],
                Value: &self.row.columns[self.index_mapping[69]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[70]],
                Value: &self.row.columns[self.index_mapping[71]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[72]],
                Value: &self.row.columns[self.index_mapping[73]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[74]],
                Value: &self.row.columns[self.index_mapping[75]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[76]],
                Value: &self.row.columns[self.index_mapping[77]],
            },
            VariablesElement {
                Name: &self.row.columns[self.index_mapping[78]],
                Value: &self.row.columns[self.index_mapping[79]],
            },
        ]
    }
    pub fn Name(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[80]]
    }
    pub fn Quest(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[81]]
    }
}
