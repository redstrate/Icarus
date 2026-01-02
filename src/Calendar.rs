//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct CalendarStructElement<'a> {
    pub Month: &'a ColumnData,
    pub Day: &'a ColumnData,
}
pub struct CalendarSheet {
    sheet: ExcelSheet,
}
impl CalendarSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Calendar")?;
        let sheet = resolver.read_excel_sheet(exh, "Calendar", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<CalendarRow> {
        let column_defs = &self.sheet.exh.column_definitions;
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
        Some(CalendarRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<CalendarRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<CalendarRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => return None,
            ExcelRowKind::SubRows(subrows) => {
                &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
            }
        };
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
pub struct CalendarRow {
    columns: Vec<ColumnData>,
}
impl CalendarRow {
    pub fn CalendarStruct<'a>(&'a self) -> [CalendarStructElement<'a>; 32] {
        [
            CalendarStructElement {
                Month: &self.columns[0],
                Day: &self.columns[1],
            },
            CalendarStructElement {
                Month: &self.columns[2],
                Day: &self.columns[3],
            },
            CalendarStructElement {
                Month: &self.columns[4],
                Day: &self.columns[5],
            },
            CalendarStructElement {
                Month: &self.columns[6],
                Day: &self.columns[7],
            },
            CalendarStructElement {
                Month: &self.columns[8],
                Day: &self.columns[9],
            },
            CalendarStructElement {
                Month: &self.columns[10],
                Day: &self.columns[11],
            },
            CalendarStructElement {
                Month: &self.columns[12],
                Day: &self.columns[13],
            },
            CalendarStructElement {
                Month: &self.columns[14],
                Day: &self.columns[15],
            },
            CalendarStructElement {
                Month: &self.columns[16],
                Day: &self.columns[17],
            },
            CalendarStructElement {
                Month: &self.columns[18],
                Day: &self.columns[19],
            },
            CalendarStructElement {
                Month: &self.columns[20],
                Day: &self.columns[21],
            },
            CalendarStructElement {
                Month: &self.columns[22],
                Day: &self.columns[23],
            },
            CalendarStructElement {
                Month: &self.columns[24],
                Day: &self.columns[25],
            },
            CalendarStructElement {
                Month: &self.columns[26],
                Day: &self.columns[27],
            },
            CalendarStructElement {
                Month: &self.columns[28],
                Day: &self.columns[29],
            },
            CalendarStructElement {
                Month: &self.columns[30],
                Day: &self.columns[31],
            },
            CalendarStructElement {
                Month: &self.columns[32],
                Day: &self.columns[33],
            },
            CalendarStructElement {
                Month: &self.columns[34],
                Day: &self.columns[35],
            },
            CalendarStructElement {
                Month: &self.columns[36],
                Day: &self.columns[37],
            },
            CalendarStructElement {
                Month: &self.columns[38],
                Day: &self.columns[39],
            },
            CalendarStructElement {
                Month: &self.columns[40],
                Day: &self.columns[41],
            },
            CalendarStructElement {
                Month: &self.columns[42],
                Day: &self.columns[43],
            },
            CalendarStructElement {
                Month: &self.columns[44],
                Day: &self.columns[45],
            },
            CalendarStructElement {
                Month: &self.columns[46],
                Day: &self.columns[47],
            },
            CalendarStructElement {
                Month: &self.columns[48],
                Day: &self.columns[49],
            },
            CalendarStructElement {
                Month: &self.columns[50],
                Day: &self.columns[51],
            },
            CalendarStructElement {
                Month: &self.columns[52],
                Day: &self.columns[53],
            },
            CalendarStructElement {
                Month: &self.columns[54],
                Day: &self.columns[55],
            },
            CalendarStructElement {
                Month: &self.columns[56],
                Day: &self.columns[57],
            },
            CalendarStructElement {
                Month: &self.columns[58],
                Day: &self.columns[59],
            },
            CalendarStructElement {
                Month: &self.columns[60],
                Day: &self.columns[61],
            },
            CalendarStructElement {
                Month: &self.columns[62],
                Day: &self.columns[63],
            },
        ]
    }
}
