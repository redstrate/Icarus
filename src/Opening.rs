//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct VariablesElement<'a> {
    Name: &'a ColumnData,
    Value: &'a ColumnData,
}
pub struct OpeningSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl OpeningSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "Opening")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "Opening", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<OpeningRow> {
        let column_defs = &self.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<ColumnData>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(OpeningRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<OpeningRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => row,
                ExcelRowKind::SubRows(rows) => &rows.first()?.1,
            };
            return self.read_row(row);
        }
        None
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<OpeningRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => return None,
                ExcelRowKind::SubRows(subrows) => {
                    &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
                }
            };
            return self.read_row(row);
        }
        None
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.row_count
    }
}
pub struct OpeningRow {
    columns: Vec<ColumnData>,
}
impl OpeningRow {
    pub fn Variables<'a>(&'a self) -> [VariablesElement<'a>; 40] {
        [
            VariablesElement {
                Name: &self.columns[0],
                Value: &self.columns[1],
            },
            VariablesElement {
                Name: &self.columns[2],
                Value: &self.columns[3],
            },
            VariablesElement {
                Name: &self.columns[4],
                Value: &self.columns[5],
            },
            VariablesElement {
                Name: &self.columns[6],
                Value: &self.columns[7],
            },
            VariablesElement {
                Name: &self.columns[8],
                Value: &self.columns[9],
            },
            VariablesElement {
                Name: &self.columns[10],
                Value: &self.columns[11],
            },
            VariablesElement {
                Name: &self.columns[12],
                Value: &self.columns[13],
            },
            VariablesElement {
                Name: &self.columns[14],
                Value: &self.columns[15],
            },
            VariablesElement {
                Name: &self.columns[16],
                Value: &self.columns[17],
            },
            VariablesElement {
                Name: &self.columns[18],
                Value: &self.columns[19],
            },
            VariablesElement {
                Name: &self.columns[20],
                Value: &self.columns[21],
            },
            VariablesElement {
                Name: &self.columns[22],
                Value: &self.columns[23],
            },
            VariablesElement {
                Name: &self.columns[24],
                Value: &self.columns[25],
            },
            VariablesElement {
                Name: &self.columns[26],
                Value: &self.columns[27],
            },
            VariablesElement {
                Name: &self.columns[28],
                Value: &self.columns[29],
            },
            VariablesElement {
                Name: &self.columns[30],
                Value: &self.columns[31],
            },
            VariablesElement {
                Name: &self.columns[32],
                Value: &self.columns[33],
            },
            VariablesElement {
                Name: &self.columns[34],
                Value: &self.columns[35],
            },
            VariablesElement {
                Name: &self.columns[36],
                Value: &self.columns[37],
            },
            VariablesElement {
                Name: &self.columns[38],
                Value: &self.columns[39],
            },
            VariablesElement {
                Name: &self.columns[40],
                Value: &self.columns[41],
            },
            VariablesElement {
                Name: &self.columns[42],
                Value: &self.columns[43],
            },
            VariablesElement {
                Name: &self.columns[44],
                Value: &self.columns[45],
            },
            VariablesElement {
                Name: &self.columns[46],
                Value: &self.columns[47],
            },
            VariablesElement {
                Name: &self.columns[48],
                Value: &self.columns[49],
            },
            VariablesElement {
                Name: &self.columns[50],
                Value: &self.columns[51],
            },
            VariablesElement {
                Name: &self.columns[52],
                Value: &self.columns[53],
            },
            VariablesElement {
                Name: &self.columns[54],
                Value: &self.columns[55],
            },
            VariablesElement {
                Name: &self.columns[56],
                Value: &self.columns[57],
            },
            VariablesElement {
                Name: &self.columns[58],
                Value: &self.columns[59],
            },
            VariablesElement {
                Name: &self.columns[60],
                Value: &self.columns[61],
            },
            VariablesElement {
                Name: &self.columns[62],
                Value: &self.columns[63],
            },
            VariablesElement {
                Name: &self.columns[64],
                Value: &self.columns[65],
            },
            VariablesElement {
                Name: &self.columns[66],
                Value: &self.columns[67],
            },
            VariablesElement {
                Name: &self.columns[68],
                Value: &self.columns[69],
            },
            VariablesElement {
                Name: &self.columns[70],
                Value: &self.columns[71],
            },
            VariablesElement {
                Name: &self.columns[72],
                Value: &self.columns[73],
            },
            VariablesElement {
                Name: &self.columns[74],
                Value: &self.columns[75],
            },
            VariablesElement {
                Name: &self.columns[76],
                Value: &self.columns[77],
            },
            VariablesElement {
                Name: &self.columns[78],
                Value: &self.columns[79],
            },
        ]
    }
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[80]
    }
    pub fn Quest<'a>(&'a self) -> &'a ColumnData {
        &self.columns[81]
    }
}
