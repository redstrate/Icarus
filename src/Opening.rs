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
    pub Name: &'a str,
    pub Value: u32,
}
#[derive(Debug, Clone)]
pub struct OpeningSheet {
    sheet: Sheet,
}
impl OpeningSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Opening")?;
        let sheet = resolver.read_excel_sheet(&exh, "Opening", language)?;
        Ok(Self { sheet })
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
        Some(Self::Row { row })
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
}
impl<'a> OpeningRow<'a> {
    pub fn Variables(&'a self) -> [VariablesElement<'a>; 40] {
        [
            VariablesElement {
                Name: self.row.columns[2].into_string().unwrap(),
                Value: self.row.columns[42].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[3].into_string().unwrap(),
                Value: self.row.columns[43].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[4].into_string().unwrap(),
                Value: self.row.columns[44].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[5].into_string().unwrap(),
                Value: self.row.columns[45].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[6].into_string().unwrap(),
                Value: self.row.columns[46].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[7].into_string().unwrap(),
                Value: self.row.columns[47].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[8].into_string().unwrap(),
                Value: self.row.columns[48].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[9].into_string().unwrap(),
                Value: self.row.columns[49].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[10].into_string().unwrap(),
                Value: self.row.columns[50].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[11].into_string().unwrap(),
                Value: self.row.columns[51].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[12].into_string().unwrap(),
                Value: self.row.columns[52].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[13].into_string().unwrap(),
                Value: self.row.columns[53].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[14].into_string().unwrap(),
                Value: self.row.columns[54].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[15].into_string().unwrap(),
                Value: self.row.columns[55].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[16].into_string().unwrap(),
                Value: self.row.columns[56].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[17].into_string().unwrap(),
                Value: self.row.columns[57].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[18].into_string().unwrap(),
                Value: self.row.columns[58].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[19].into_string().unwrap(),
                Value: self.row.columns[59].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[20].into_string().unwrap(),
                Value: self.row.columns[60].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[21].into_string().unwrap(),
                Value: self.row.columns[61].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[22].into_string().unwrap(),
                Value: self.row.columns[62].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[23].into_string().unwrap(),
                Value: self.row.columns[63].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[24].into_string().unwrap(),
                Value: self.row.columns[64].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[25].into_string().unwrap(),
                Value: self.row.columns[65].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[26].into_string().unwrap(),
                Value: self.row.columns[66].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[27].into_string().unwrap(),
                Value: self.row.columns[67].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[28].into_string().unwrap(),
                Value: self.row.columns[68].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[29].into_string().unwrap(),
                Value: self.row.columns[69].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[30].into_string().unwrap(),
                Value: self.row.columns[70].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[31].into_string().unwrap(),
                Value: self.row.columns[71].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[32].into_string().unwrap(),
                Value: self.row.columns[72].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[33].into_string().unwrap(),
                Value: self.row.columns[73].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[34].into_string().unwrap(),
                Value: self.row.columns[74].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[35].into_string().unwrap(),
                Value: self.row.columns[75].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[36].into_string().unwrap(),
                Value: self.row.columns[76].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[37].into_string().unwrap(),
                Value: self.row.columns[77].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[38].into_string().unwrap(),
                Value: self.row.columns[78].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[39].into_string().unwrap(),
                Value: self.row.columns[79].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[40].into_string().unwrap(),
                Value: self.row.columns[80].into_u32().copied().unwrap(),
            },
            VariablesElement {
                Name: self.row.columns[41].into_string().unwrap(),
                Value: self.row.columns[81].into_u32().copied().unwrap(),
            },
        ]
    }
    pub fn Name(&'a self) -> &'a str {
        self.row.columns[0].into_string().unwrap()
    }
    pub fn Quest(&'a self) -> u32 {
        self.row.columns[1].into_u32().copied().unwrap()
    }
}
