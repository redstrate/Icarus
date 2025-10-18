//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct QuestRedoParamElement<'a> {
    Quest: &'a ColumnData,
    UnknownParam: &'a ColumnData,
}
pub struct QuestRedoSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl QuestRedoSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "QuestRedo")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "QuestRedo", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<QuestRedoRow> {
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
        Some(QuestRedoRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<QuestRedoRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestRedoRow> {
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
pub struct QuestRedoRow {
    columns: Vec<ColumnData>,
}
impl QuestRedoRow {
    pub fn QuestRedoParam<'a>(&'a self) -> [QuestRedoParamElement<'a>; 32] {
        [
            QuestRedoParamElement {
                Quest: &self.columns[0],
                UnknownParam: &self.columns[1],
            },
            QuestRedoParamElement {
                Quest: &self.columns[2],
                UnknownParam: &self.columns[3],
            },
            QuestRedoParamElement {
                Quest: &self.columns[4],
                UnknownParam: &self.columns[5],
            },
            QuestRedoParamElement {
                Quest: &self.columns[6],
                UnknownParam: &self.columns[7],
            },
            QuestRedoParamElement {
                Quest: &self.columns[8],
                UnknownParam: &self.columns[9],
            },
            QuestRedoParamElement {
                Quest: &self.columns[10],
                UnknownParam: &self.columns[11],
            },
            QuestRedoParamElement {
                Quest: &self.columns[12],
                UnknownParam: &self.columns[13],
            },
            QuestRedoParamElement {
                Quest: &self.columns[14],
                UnknownParam: &self.columns[15],
            },
            QuestRedoParamElement {
                Quest: &self.columns[16],
                UnknownParam: &self.columns[17],
            },
            QuestRedoParamElement {
                Quest: &self.columns[18],
                UnknownParam: &self.columns[19],
            },
            QuestRedoParamElement {
                Quest: &self.columns[20],
                UnknownParam: &self.columns[21],
            },
            QuestRedoParamElement {
                Quest: &self.columns[22],
                UnknownParam: &self.columns[23],
            },
            QuestRedoParamElement {
                Quest: &self.columns[24],
                UnknownParam: &self.columns[25],
            },
            QuestRedoParamElement {
                Quest: &self.columns[26],
                UnknownParam: &self.columns[27],
            },
            QuestRedoParamElement {
                Quest: &self.columns[28],
                UnknownParam: &self.columns[29],
            },
            QuestRedoParamElement {
                Quest: &self.columns[30],
                UnknownParam: &self.columns[31],
            },
            QuestRedoParamElement {
                Quest: &self.columns[32],
                UnknownParam: &self.columns[33],
            },
            QuestRedoParamElement {
                Quest: &self.columns[34],
                UnknownParam: &self.columns[35],
            },
            QuestRedoParamElement {
                Quest: &self.columns[36],
                UnknownParam: &self.columns[37],
            },
            QuestRedoParamElement {
                Quest: &self.columns[38],
                UnknownParam: &self.columns[39],
            },
            QuestRedoParamElement {
                Quest: &self.columns[40],
                UnknownParam: &self.columns[41],
            },
            QuestRedoParamElement {
                Quest: &self.columns[42],
                UnknownParam: &self.columns[43],
            },
            QuestRedoParamElement {
                Quest: &self.columns[44],
                UnknownParam: &self.columns[45],
            },
            QuestRedoParamElement {
                Quest: &self.columns[46],
                UnknownParam: &self.columns[47],
            },
            QuestRedoParamElement {
                Quest: &self.columns[48],
                UnknownParam: &self.columns[49],
            },
            QuestRedoParamElement {
                Quest: &self.columns[50],
                UnknownParam: &self.columns[51],
            },
            QuestRedoParamElement {
                Quest: &self.columns[52],
                UnknownParam: &self.columns[53],
            },
            QuestRedoParamElement {
                Quest: &self.columns[54],
                UnknownParam: &self.columns[55],
            },
            QuestRedoParamElement {
                Quest: &self.columns[56],
                UnknownParam: &self.columns[57],
            },
            QuestRedoParamElement {
                Quest: &self.columns[58],
                UnknownParam: &self.columns[59],
            },
            QuestRedoParamElement {
                Quest: &self.columns[60],
                UnknownParam: &self.columns[61],
            },
            QuestRedoParamElement {
                Quest: &self.columns[62],
                UnknownParam: &self.columns[63],
            },
        ]
    }
    pub fn FinalQuest<'a>(&'a self) -> &'a ColumnData {
        &self.columns[64]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[65]
    }
    pub fn Chapter<'a>(&'a self) -> &'a ColumnData {
        &self.columns[66]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[67]
    }
}
