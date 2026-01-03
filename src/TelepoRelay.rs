//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct RelaysElement<'a> {
    pub EnterTerritory: &'a ColumnData,
    pub ExitTerritory: &'a ColumnData,
    pub Cost: &'a ColumnData,
}
#[derive(Debug, Clone)]
pub struct TelepoRelaySheet {
    sheet: ExcelSheet,
}
impl TelepoRelaySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TelepoRelay")?;
        let sheet = resolver.read_excel_sheet(exh, "TelepoRelay", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<TelepoRelayRow> {
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
        Some(TelepoRelayRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<TelepoRelayRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<TelepoRelayRow> {
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
pub struct TelepoRelayRow {
    columns: Vec<ColumnData>,
}
impl TelepoRelayRow {
    pub fn Relays<'a>(&'a self) -> [RelaysElement<'a>; 9] {
        [
            RelaysElement {
                EnterTerritory: &self.columns[0],
                ExitTerritory: &self.columns[1],
                Cost: &self.columns[2],
            },
            RelaysElement {
                EnterTerritory: &self.columns[3],
                ExitTerritory: &self.columns[4],
                Cost: &self.columns[5],
            },
            RelaysElement {
                EnterTerritory: &self.columns[6],
                ExitTerritory: &self.columns[7],
                Cost: &self.columns[8],
            },
            RelaysElement {
                EnterTerritory: &self.columns[9],
                ExitTerritory: &self.columns[10],
                Cost: &self.columns[11],
            },
            RelaysElement {
                EnterTerritory: &self.columns[12],
                ExitTerritory: &self.columns[13],
                Cost: &self.columns[14],
            },
            RelaysElement {
                EnterTerritory: &self.columns[15],
                ExitTerritory: &self.columns[16],
                Cost: &self.columns[17],
            },
            RelaysElement {
                EnterTerritory: &self.columns[18],
                ExitTerritory: &self.columns[19],
                Cost: &self.columns[20],
            },
            RelaysElement {
                EnterTerritory: &self.columns[21],
                ExitTerritory: &self.columns[22],
                Cost: &self.columns[23],
            },
            RelaysElement {
                EnterTerritory: &self.columns[24],
                ExitTerritory: &self.columns[25],
                Cost: &self.columns[26],
            },
        ]
    }
    pub fn Unknown_70<'a>(&'a self) -> &'a ColumnData {
        &self.columns[27]
    }
}
