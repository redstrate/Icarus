//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct ScriptElement<'a> {
    pub ScriptInstruction: &'a ColumnData,
    pub ScriptArg: &'a ColumnData,
}
pub struct CustomTalkSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl CustomTalkSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "CustomTalk")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "CustomTalk", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<CustomTalkRow> {
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
        Some(CustomTalkRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<CustomTalkRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<CustomTalkRow> {
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
pub struct CustomTalkRow {
    columns: Vec<ColumnData>,
}
impl CustomTalkRow {
    pub fn Script<'a>(&'a self) -> [ScriptElement<'a>; 30] {
        [
            ScriptElement {
                ScriptInstruction: &self.columns[0],
                ScriptArg: &self.columns[1],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[2],
                ScriptArg: &self.columns[3],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[4],
                ScriptArg: &self.columns[5],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[6],
                ScriptArg: &self.columns[7],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[8],
                ScriptArg: &self.columns[9],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[10],
                ScriptArg: &self.columns[11],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[12],
                ScriptArg: &self.columns[13],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[14],
                ScriptArg: &self.columns[15],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[16],
                ScriptArg: &self.columns[17],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[18],
                ScriptArg: &self.columns[19],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[20],
                ScriptArg: &self.columns[21],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[22],
                ScriptArg: &self.columns[23],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[24],
                ScriptArg: &self.columns[25],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[26],
                ScriptArg: &self.columns[27],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[28],
                ScriptArg: &self.columns[29],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[30],
                ScriptArg: &self.columns[31],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[32],
                ScriptArg: &self.columns[33],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[34],
                ScriptArg: &self.columns[35],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[36],
                ScriptArg: &self.columns[37],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[38],
                ScriptArg: &self.columns[39],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[40],
                ScriptArg: &self.columns[41],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[42],
                ScriptArg: &self.columns[43],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[44],
                ScriptArg: &self.columns[45],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[46],
                ScriptArg: &self.columns[47],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[48],
                ScriptArg: &self.columns[49],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[50],
                ScriptArg: &self.columns[51],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[52],
                ScriptArg: &self.columns[53],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[54],
                ScriptArg: &self.columns[55],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[56],
                ScriptArg: &self.columns[57],
            },
            ScriptElement {
                ScriptInstruction: &self.columns[58],
                ScriptArg: &self.columns[59],
            },
        ]
    }
    pub fn MainOption<'a>(&'a self) -> &'a ColumnData {
        &self.columns[60]
    }
    pub fn SubOption<'a>(&'a self) -> &'a ColumnData {
        &self.columns[61]
    }
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[62]
    }
    pub fn IconActor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[63]
    }
    pub fn IconMap<'a>(&'a self) -> &'a ColumnData {
        &self.columns[64]
    }
    pub fn SpecialLinks<'a>(&'a self) -> &'a ColumnData {
        &self.columns[65]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[66]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[67]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[68]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[69]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[70]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[71]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[72]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[73]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[74]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[75]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[76]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[77]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[78]
    }
}
