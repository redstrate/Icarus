//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct WeeklyLotBonusParamElement<'a> {
    pub Unknown0: &'a ColumnData,
    pub WeeklyLotBonusThreshold: &'a ColumnData,
    pub Unknown1: &'a ColumnData,
}
pub struct WeeklyLotBonusSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl WeeklyLotBonusSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "WeeklyLotBonus")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "WeeklyLotBonus", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<WeeklyLotBonusRow> {
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
        Some(WeeklyLotBonusRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<WeeklyLotBonusRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<WeeklyLotBonusRow> {
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
pub struct WeeklyLotBonusRow {
    columns: Vec<ColumnData>,
}
impl WeeklyLotBonusRow {
    pub fn WeeklyLotBonusParam<'a>(&'a self) -> [WeeklyLotBonusParamElement<'a>; 32] {
        [
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[0],
                WeeklyLotBonusThreshold: &self.columns[1],
                Unknown1: &self.columns[2],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[3],
                WeeklyLotBonusThreshold: &self.columns[4],
                Unknown1: &self.columns[5],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[6],
                WeeklyLotBonusThreshold: &self.columns[7],
                Unknown1: &self.columns[8],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[9],
                WeeklyLotBonusThreshold: &self.columns[10],
                Unknown1: &self.columns[11],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[12],
                WeeklyLotBonusThreshold: &self.columns[13],
                Unknown1: &self.columns[14],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[15],
                WeeklyLotBonusThreshold: &self.columns[16],
                Unknown1: &self.columns[17],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[18],
                WeeklyLotBonusThreshold: &self.columns[19],
                Unknown1: &self.columns[20],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[21],
                WeeklyLotBonusThreshold: &self.columns[22],
                Unknown1: &self.columns[23],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[24],
                WeeklyLotBonusThreshold: &self.columns[25],
                Unknown1: &self.columns[26],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[27],
                WeeklyLotBonusThreshold: &self.columns[28],
                Unknown1: &self.columns[29],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[30],
                WeeklyLotBonusThreshold: &self.columns[31],
                Unknown1: &self.columns[32],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[33],
                WeeklyLotBonusThreshold: &self.columns[34],
                Unknown1: &self.columns[35],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[36],
                WeeklyLotBonusThreshold: &self.columns[37],
                Unknown1: &self.columns[38],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[39],
                WeeklyLotBonusThreshold: &self.columns[40],
                Unknown1: &self.columns[41],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[42],
                WeeklyLotBonusThreshold: &self.columns[43],
                Unknown1: &self.columns[44],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[45],
                WeeklyLotBonusThreshold: &self.columns[46],
                Unknown1: &self.columns[47],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[48],
                WeeklyLotBonusThreshold: &self.columns[49],
                Unknown1: &self.columns[50],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[51],
                WeeklyLotBonusThreshold: &self.columns[52],
                Unknown1: &self.columns[53],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[54],
                WeeklyLotBonusThreshold: &self.columns[55],
                Unknown1: &self.columns[56],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[57],
                WeeklyLotBonusThreshold: &self.columns[58],
                Unknown1: &self.columns[59],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[60],
                WeeklyLotBonusThreshold: &self.columns[61],
                Unknown1: &self.columns[62],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[63],
                WeeklyLotBonusThreshold: &self.columns[64],
                Unknown1: &self.columns[65],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[66],
                WeeklyLotBonusThreshold: &self.columns[67],
                Unknown1: &self.columns[68],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[69],
                WeeklyLotBonusThreshold: &self.columns[70],
                Unknown1: &self.columns[71],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[72],
                WeeklyLotBonusThreshold: &self.columns[73],
                Unknown1: &self.columns[74],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[75],
                WeeklyLotBonusThreshold: &self.columns[76],
                Unknown1: &self.columns[77],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[78],
                WeeklyLotBonusThreshold: &self.columns[79],
                Unknown1: &self.columns[80],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[81],
                WeeklyLotBonusThreshold: &self.columns[82],
                Unknown1: &self.columns[83],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[84],
                WeeklyLotBonusThreshold: &self.columns[85],
                Unknown1: &self.columns[86],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[87],
                WeeklyLotBonusThreshold: &self.columns[88],
                Unknown1: &self.columns[89],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[90],
                WeeklyLotBonusThreshold: &self.columns[91],
                Unknown1: &self.columns[92],
            },
            WeeklyLotBonusParamElement {
                Unknown0: &self.columns[93],
                WeeklyLotBonusThreshold: &self.columns[94],
                Unknown1: &self.columns[95],
            },
        ]
    }
}
