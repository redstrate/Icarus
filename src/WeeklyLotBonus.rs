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
pub struct WeeklyLotBonusParamElement {
    pub Unknown0: u16,
    pub WeeklyLotBonusThreshold: u8,
    pub Unknown1: u8,
}
#[derive(Debug, Clone)]
pub struct WeeklyLotBonusSheet {
    sheet: Sheet,
}
impl WeeklyLotBonusSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("WeeklyLotBonus")?;
        let sheet = resolver.read_excel_sheet(&exh, "WeeklyLotBonus", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<WeeklyLotBonusRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<WeeklyLotBonusRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for WeeklyLotBonusSheet {
    type Row = WeeklyLotBonusRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row { row })
    }
}
impl<'a> IntoIterator for &'a WeeklyLotBonusSheet {
    type Item = (u32, Vec<(u16, WeeklyLotBonusRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, WeeklyLotBonusSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, WeeklyLotBonusSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct WeeklyLotBonusRow<'a> {
    row: &'a Row,
}
impl<'a> WeeklyLotBonusRow<'a> {
    pub fn WeeklyLotBonusParam(&'a self) -> [WeeklyLotBonusParamElement; 32] {
        [
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[32].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self.row.columns[0].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[64].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[33].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self.row.columns[1].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[65].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[34].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self.row.columns[2].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[66].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[35].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self.row.columns[3].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[67].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[36].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self.row.columns[4].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[68].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[37].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self.row.columns[5].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[69].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[38].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self.row.columns[6].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[70].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[39].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self.row.columns[7].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[71].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[40].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self.row.columns[8].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[72].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[41].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self.row.columns[9].into_u8().copied().unwrap(),
                Unknown1: self.row.columns[73].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[42].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[10]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[74].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[43].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[11]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[75].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[44].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[12]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[76].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[45].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[13]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[77].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[46].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[14]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[78].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[47].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[15]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[79].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[48].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[16]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[80].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[49].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[17]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[81].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[50].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[18]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[82].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[51].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[19]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[83].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[52].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[20]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[84].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[53].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[21]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[85].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[54].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[22]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[86].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[55].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[23]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[87].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[56].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[24]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[88].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[57].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[25]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[89].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[58].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[26]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[90].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[59].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[27]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[91].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[60].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[28]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[92].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[61].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[29]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[93].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[62].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[30]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[94].into_u8().copied().unwrap(),
            },
            WeeklyLotBonusParamElement {
                Unknown0: self.row.columns[63].into_u16().copied().unwrap(),
                WeeklyLotBonusThreshold: self
                    .row
                    .columns[31]
                    .into_u8()
                    .copied()
                    .unwrap(),
                Unknown1: self.row.columns[95].into_u8().copied().unwrap(),
            },
        ]
    }
}
