//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct EventParametersElement<'a> {
    pub Gesture: &'a Field,
    pub LipSync: &'a Field,
    pub Facial: &'a Field,
    pub Shape: &'a Field,
    pub Turn: &'a Field,
    pub WidgetType: &'a Field,
    pub IsAutoShake: &'a Field,
}
#[derive(Debug, Clone)]
pub struct FateEventSheet {
    sheet: Sheet,
}
impl FateEventSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("FateEvent")?;
        let sheet = resolver.read_excel_sheet(&exh, "FateEvent", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<FateEventRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<FateEventRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for FateEventSheet {
    type Row = FateEventRow;
    fn read_row(&self, row: &Row) -> Option<Self::Row> {
        let column_defs = &self.sheet.exh.column_definitions;
        let mut zipped: Vec<_> = row
            .columns
            .clone()
            .into_iter()
            .zip(column_defs)
            .collect();
        zipped.sort_by(|(_, a_col), (_, b_col)| a_col.offset.cmp(&b_col.offset));
        let (columns, _): (Vec<Field>, Vec<ExcelColumnDefinition>) = zipped
            .into_iter()
            .unzip();
        Some(Self::Row { columns })
    }
}
impl<'a> IntoIterator for &'a FateEventSheet {
    type Item = (u32, Vec<(u16, FateEventRow)>);
    type IntoIter = StructuredSheetIterator<'a, FateEventSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, FateEventSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FateEventRow {
    columns: Vec<Field>,
}
impl FateEventRow {
    pub fn EventParameters<'a>(&'a self) -> [EventParametersElement<'a>; 8] {
        [
            EventParametersElement {
                Gesture: &self.columns[0],
                LipSync: &self.columns[1],
                Facial: &self.columns[2],
                Shape: &self.columns[3],
                Turn: &self.columns[4],
                WidgetType: &self.columns[5],
                IsAutoShake: &self.columns[6],
            },
            EventParametersElement {
                Gesture: &self.columns[7],
                LipSync: &self.columns[8],
                Facial: &self.columns[9],
                Shape: &self.columns[10],
                Turn: &self.columns[11],
                WidgetType: &self.columns[12],
                IsAutoShake: &self.columns[13],
            },
            EventParametersElement {
                Gesture: &self.columns[14],
                LipSync: &self.columns[15],
                Facial: &self.columns[16],
                Shape: &self.columns[17],
                Turn: &self.columns[18],
                WidgetType: &self.columns[19],
                IsAutoShake: &self.columns[20],
            },
            EventParametersElement {
                Gesture: &self.columns[21],
                LipSync: &self.columns[22],
                Facial: &self.columns[23],
                Shape: &self.columns[24],
                Turn: &self.columns[25],
                WidgetType: &self.columns[26],
                IsAutoShake: &self.columns[27],
            },
            EventParametersElement {
                Gesture: &self.columns[28],
                LipSync: &self.columns[29],
                Facial: &self.columns[30],
                Shape: &self.columns[31],
                Turn: &self.columns[32],
                WidgetType: &self.columns[33],
                IsAutoShake: &self.columns[34],
            },
            EventParametersElement {
                Gesture: &self.columns[35],
                LipSync: &self.columns[36],
                Facial: &self.columns[37],
                Shape: &self.columns[38],
                Turn: &self.columns[39],
                WidgetType: &self.columns[40],
                IsAutoShake: &self.columns[41],
            },
            EventParametersElement {
                Gesture: &self.columns[42],
                LipSync: &self.columns[43],
                Facial: &self.columns[44],
                Shape: &self.columns[45],
                Turn: &self.columns[46],
                WidgetType: &self.columns[47],
                IsAutoShake: &self.columns[48],
            },
            EventParametersElement {
                Gesture: &self.columns[49],
                LipSync: &self.columns[50],
                Facial: &self.columns[51],
                Shape: &self.columns[52],
                Turn: &self.columns[53],
                WidgetType: &self.columns[54],
                IsAutoShake: &self.columns[55],
            },
        ]
    }
    pub fn Text<'a>(&'a self) -> [&'a Field; 8] {
        [
            &self.columns[56],
            &self.columns[57],
            &self.columns[58],
            &self.columns[59],
            &self.columns[60],
            &self.columns[61],
            &self.columns[62],
            &self.columns[63],
        ]
    }
}
