//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct GFATEParamsElement<'a> {
    LGBPopRange: &'a ColumnData,
    Icon: &'a ColumnData,
    Unknown0: &'a ColumnData,
    Unknown1: &'a ColumnData,
    Unknown2: &'a ColumnData,
}
pub struct GFATESheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl GFATESheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "GFATE")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "GFATE", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<GFATERow> {
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
        Some(GFATERow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<GFATERow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<GFATERow> {
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
pub struct GFATERow {
    columns: Vec<ColumnData>,
}
impl GFATERow {
    pub fn GFATEParams<'a>(&'a self) -> [GFATEParamsElement<'a>; 15] {
        [
            GFATEParamsElement {
                LGBPopRange: &self.columns[0],
                Icon: &self.columns[1],
                Unknown0: &self.columns[2],
                Unknown1: &self.columns[3],
                Unknown2: &self.columns[4],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[5],
                Icon: &self.columns[6],
                Unknown0: &self.columns[7],
                Unknown1: &self.columns[8],
                Unknown2: &self.columns[9],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[10],
                Icon: &self.columns[11],
                Unknown0: &self.columns[12],
                Unknown1: &self.columns[13],
                Unknown2: &self.columns[14],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[15],
                Icon: &self.columns[16],
                Unknown0: &self.columns[17],
                Unknown1: &self.columns[18],
                Unknown2: &self.columns[19],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[20],
                Icon: &self.columns[21],
                Unknown0: &self.columns[22],
                Unknown1: &self.columns[23],
                Unknown2: &self.columns[24],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[25],
                Icon: &self.columns[26],
                Unknown0: &self.columns[27],
                Unknown1: &self.columns[28],
                Unknown2: &self.columns[29],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[30],
                Icon: &self.columns[31],
                Unknown0: &self.columns[32],
                Unknown1: &self.columns[33],
                Unknown2: &self.columns[34],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[35],
                Icon: &self.columns[36],
                Unknown0: &self.columns[37],
                Unknown1: &self.columns[38],
                Unknown2: &self.columns[39],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[40],
                Icon: &self.columns[41],
                Unknown0: &self.columns[42],
                Unknown1: &self.columns[43],
                Unknown2: &self.columns[44],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[45],
                Icon: &self.columns[46],
                Unknown0: &self.columns[47],
                Unknown1: &self.columns[48],
                Unknown2: &self.columns[49],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[50],
                Icon: &self.columns[51],
                Unknown0: &self.columns[52],
                Unknown1: &self.columns[53],
                Unknown2: &self.columns[54],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[55],
                Icon: &self.columns[56],
                Unknown0: &self.columns[57],
                Unknown1: &self.columns[58],
                Unknown2: &self.columns[59],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[60],
                Icon: &self.columns[61],
                Unknown0: &self.columns[62],
                Unknown1: &self.columns[63],
                Unknown2: &self.columns[64],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[65],
                Icon: &self.columns[66],
                Unknown0: &self.columns[67],
                Unknown1: &self.columns[68],
                Unknown2: &self.columns[69],
            },
            GFATEParamsElement {
                LGBPopRange: &self.columns[70],
                Icon: &self.columns[71],
                Unknown0: &self.columns[72],
                Unknown1: &self.columns[73],
                Unknown2: &self.columns[74],
            },
        ]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[75]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[76]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[77]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[78]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[79]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[80]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[81]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[82]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[83]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[84]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[85]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[86]
    }
}
