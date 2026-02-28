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
    index_mapping: Vec<usize>,
}
impl FateEventSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("FateEvent")?;
        let sheet = resolver.read_excel_sheet(&exh, "FateEvent", language)?;
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
impl<'a> StructuredSheet<'a> for FateEventSheet {
    type Row = FateEventRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a FateEventSheet {
    type Item = (u32, Vec<(u16, FateEventRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, FateEventSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, FateEventSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FateEventRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> FateEventRow<'a> {
    pub fn EventParameters(&'a self) -> [EventParametersElement<'a>; 8] {
        [
            EventParametersElement {
                Gesture: &self.row.columns[self.index_mapping[0]],
                LipSync: &self.row.columns[self.index_mapping[1]],
                Facial: &self.row.columns[self.index_mapping[2]],
                Shape: &self.row.columns[self.index_mapping[3]],
                Turn: &self.row.columns[self.index_mapping[4]],
                WidgetType: &self.row.columns[self.index_mapping[5]],
                IsAutoShake: &self.row.columns[self.index_mapping[6]],
            },
            EventParametersElement {
                Gesture: &self.row.columns[self.index_mapping[7]],
                LipSync: &self.row.columns[self.index_mapping[8]],
                Facial: &self.row.columns[self.index_mapping[9]],
                Shape: &self.row.columns[self.index_mapping[10]],
                Turn: &self.row.columns[self.index_mapping[11]],
                WidgetType: &self.row.columns[self.index_mapping[12]],
                IsAutoShake: &self.row.columns[self.index_mapping[13]],
            },
            EventParametersElement {
                Gesture: &self.row.columns[self.index_mapping[14]],
                LipSync: &self.row.columns[self.index_mapping[15]],
                Facial: &self.row.columns[self.index_mapping[16]],
                Shape: &self.row.columns[self.index_mapping[17]],
                Turn: &self.row.columns[self.index_mapping[18]],
                WidgetType: &self.row.columns[self.index_mapping[19]],
                IsAutoShake: &self.row.columns[self.index_mapping[20]],
            },
            EventParametersElement {
                Gesture: &self.row.columns[self.index_mapping[21]],
                LipSync: &self.row.columns[self.index_mapping[22]],
                Facial: &self.row.columns[self.index_mapping[23]],
                Shape: &self.row.columns[self.index_mapping[24]],
                Turn: &self.row.columns[self.index_mapping[25]],
                WidgetType: &self.row.columns[self.index_mapping[26]],
                IsAutoShake: &self.row.columns[self.index_mapping[27]],
            },
            EventParametersElement {
                Gesture: &self.row.columns[self.index_mapping[28]],
                LipSync: &self.row.columns[self.index_mapping[29]],
                Facial: &self.row.columns[self.index_mapping[30]],
                Shape: &self.row.columns[self.index_mapping[31]],
                Turn: &self.row.columns[self.index_mapping[32]],
                WidgetType: &self.row.columns[self.index_mapping[33]],
                IsAutoShake: &self.row.columns[self.index_mapping[34]],
            },
            EventParametersElement {
                Gesture: &self.row.columns[self.index_mapping[35]],
                LipSync: &self.row.columns[self.index_mapping[36]],
                Facial: &self.row.columns[self.index_mapping[37]],
                Shape: &self.row.columns[self.index_mapping[38]],
                Turn: &self.row.columns[self.index_mapping[39]],
                WidgetType: &self.row.columns[self.index_mapping[40]],
                IsAutoShake: &self.row.columns[self.index_mapping[41]],
            },
            EventParametersElement {
                Gesture: &self.row.columns[self.index_mapping[42]],
                LipSync: &self.row.columns[self.index_mapping[43]],
                Facial: &self.row.columns[self.index_mapping[44]],
                Shape: &self.row.columns[self.index_mapping[45]],
                Turn: &self.row.columns[self.index_mapping[46]],
                WidgetType: &self.row.columns[self.index_mapping[47]],
                IsAutoShake: &self.row.columns[self.index_mapping[48]],
            },
            EventParametersElement {
                Gesture: &self.row.columns[self.index_mapping[49]],
                LipSync: &self.row.columns[self.index_mapping[50]],
                Facial: &self.row.columns[self.index_mapping[51]],
                Shape: &self.row.columns[self.index_mapping[52]],
                Turn: &self.row.columns[self.index_mapping[53]],
                WidgetType: &self.row.columns[self.index_mapping[54]],
                IsAutoShake: &self.row.columns[self.index_mapping[55]],
            },
        ]
    }
    pub fn Text(&'a self) -> [&'a Field; 8] {
        [
            &self.row.columns[self.index_mapping[56]],
            &self.row.columns[self.index_mapping[57]],
            &self.row.columns[self.index_mapping[58]],
            &self.row.columns[self.index_mapping[59]],
            &self.row.columns[self.index_mapping[60]],
            &self.row.columns[self.index_mapping[61]],
            &self.row.columns[self.index_mapping[62]],
            &self.row.columns[self.index_mapping[63]],
        ]
    }
}
