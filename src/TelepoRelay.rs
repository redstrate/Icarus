//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct RelaysElement<'a> {
    pub EnterTerritory: &'a Field,
    pub ExitTerritory: &'a Field,
    pub Cost: &'a Field,
}
#[derive(Debug, Clone)]
pub struct TelepoRelaySheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl TelepoRelaySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TelepoRelay")?;
        let sheet = resolver.read_excel_sheet(&exh, "TelepoRelay", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<TelepoRelayRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<TelepoRelayRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for TelepoRelaySheet {
    type Row = TelepoRelayRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a TelepoRelaySheet {
    type Item = (u32, Vec<(u16, TelepoRelayRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, TelepoRelaySheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, TelepoRelaySheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TelepoRelayRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> TelepoRelayRow<'a> {
    pub fn Relays(&'a self) -> [RelaysElement<'a>; 9] {
        [
            RelaysElement {
                EnterTerritory: &self.row.columns[self.index_mapping[0]],
                ExitTerritory: &self.row.columns[self.index_mapping[1]],
                Cost: &self.row.columns[self.index_mapping[2]],
            },
            RelaysElement {
                EnterTerritory: &self.row.columns[self.index_mapping[3]],
                ExitTerritory: &self.row.columns[self.index_mapping[4]],
                Cost: &self.row.columns[self.index_mapping[5]],
            },
            RelaysElement {
                EnterTerritory: &self.row.columns[self.index_mapping[6]],
                ExitTerritory: &self.row.columns[self.index_mapping[7]],
                Cost: &self.row.columns[self.index_mapping[8]],
            },
            RelaysElement {
                EnterTerritory: &self.row.columns[self.index_mapping[9]],
                ExitTerritory: &self.row.columns[self.index_mapping[10]],
                Cost: &self.row.columns[self.index_mapping[11]],
            },
            RelaysElement {
                EnterTerritory: &self.row.columns[self.index_mapping[12]],
                ExitTerritory: &self.row.columns[self.index_mapping[13]],
                Cost: &self.row.columns[self.index_mapping[14]],
            },
            RelaysElement {
                EnterTerritory: &self.row.columns[self.index_mapping[15]],
                ExitTerritory: &self.row.columns[self.index_mapping[16]],
                Cost: &self.row.columns[self.index_mapping[17]],
            },
            RelaysElement {
                EnterTerritory: &self.row.columns[self.index_mapping[18]],
                ExitTerritory: &self.row.columns[self.index_mapping[19]],
                Cost: &self.row.columns[self.index_mapping[20]],
            },
            RelaysElement {
                EnterTerritory: &self.row.columns[self.index_mapping[21]],
                ExitTerritory: &self.row.columns[self.index_mapping[22]],
                Cost: &self.row.columns[self.index_mapping[23]],
            },
            RelaysElement {
                EnterTerritory: &self.row.columns[self.index_mapping[24]],
                ExitTerritory: &self.row.columns[self.index_mapping[25]],
                Cost: &self.row.columns[self.index_mapping[26]],
            },
        ]
    }
    pub fn Unknown_70(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
}
