//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RelaysElement {
    pub EnterTerritory: u16,
    pub ExitTerritory: u16,
    pub Cost: u16,
}
#[derive(Debug, Clone)]
pub struct TelepoRelaySheet {
    sheet: Sheet,
}
impl TelepoRelaySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("TelepoRelay")?;
        let sheet = resolver.read_excel_sheet(&exh, "TelepoRelay", language)?;
        Ok(Self { sheet })
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
        Some(Self::Row { row })
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
}
impl<'a> TelepoRelayRow<'a> {
    pub fn Relays(&'a self) -> [RelaysElement; 9] {
        [
            RelaysElement {
                EnterTerritory: self.row.columns[0].into_u16().copied().unwrap(),
                ExitTerritory: self.row.columns[9].into_u16().copied().unwrap(),
                Cost: self.row.columns[18].into_u16().copied().unwrap(),
            },
            RelaysElement {
                EnterTerritory: self.row.columns[1].into_u16().copied().unwrap(),
                ExitTerritory: self.row.columns[10].into_u16().copied().unwrap(),
                Cost: self.row.columns[19].into_u16().copied().unwrap(),
            },
            RelaysElement {
                EnterTerritory: self.row.columns[2].into_u16().copied().unwrap(),
                ExitTerritory: self.row.columns[11].into_u16().copied().unwrap(),
                Cost: self.row.columns[20].into_u16().copied().unwrap(),
            },
            RelaysElement {
                EnterTerritory: self.row.columns[3].into_u16().copied().unwrap(),
                ExitTerritory: self.row.columns[12].into_u16().copied().unwrap(),
                Cost: self.row.columns[21].into_u16().copied().unwrap(),
            },
            RelaysElement {
                EnterTerritory: self.row.columns[4].into_u16().copied().unwrap(),
                ExitTerritory: self.row.columns[13].into_u16().copied().unwrap(),
                Cost: self.row.columns[22].into_u16().copied().unwrap(),
            },
            RelaysElement {
                EnterTerritory: self.row.columns[5].into_u16().copied().unwrap(),
                ExitTerritory: self.row.columns[14].into_u16().copied().unwrap(),
                Cost: self.row.columns[23].into_u16().copied().unwrap(),
            },
            RelaysElement {
                EnterTerritory: self.row.columns[6].into_u16().copied().unwrap(),
                ExitTerritory: self.row.columns[15].into_u16().copied().unwrap(),
                Cost: self.row.columns[24].into_u16().copied().unwrap(),
            },
            RelaysElement {
                EnterTerritory: self.row.columns[7].into_u16().copied().unwrap(),
                ExitTerritory: self.row.columns[16].into_u16().copied().unwrap(),
                Cost: self.row.columns[25].into_u16().copied().unwrap(),
            },
            RelaysElement {
                EnterTerritory: self.row.columns[8].into_u16().copied().unwrap(),
                ExitTerritory: self.row.columns[17].into_u16().copied().unwrap(),
                Cost: self.row.columns[26].into_u16().copied().unwrap(),
            },
        ]
    }
    pub fn Unknown_70(&'a self) -> u32 {
        self.row.columns[27].into_u32().copied().unwrap()
    }
}
