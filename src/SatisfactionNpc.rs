//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct SatisfactionNpcParamsElement<'a> {
    pub SupplyIndex: &'a Field,
    pub Item: [&'a Field; 3],
    pub SatisfactionRequired: &'a Field,
    pub ItemCount: [&'a Field; 3],
    pub IsHQ: [&'a Field; 3],
}
pub struct RankParamsElement<'a> {
    pub ImageId: &'a Field,
    pub Unknown1: &'a Field,
    pub Quest: &'a Field,
}
#[derive(Debug, Clone)]
pub struct SatisfactionNpcSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl SatisfactionNpcSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SatisfactionNpc")?;
        let sheet = resolver.read_excel_sheet(&exh, "SatisfactionNpc", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<SatisfactionNpcRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<SatisfactionNpcRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for SatisfactionNpcSheet {
    type Row = SatisfactionNpcRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a SatisfactionNpcSheet {
    type Item = (u32, Vec<(u16, SatisfactionNpcRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, SatisfactionNpcSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, SatisfactionNpcSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SatisfactionNpcRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> SatisfactionNpcRow<'a> {
    pub fn SatisfactionNpcParams(&'a self) -> [SatisfactionNpcParamsElement<'a>; 6] {
        [
            SatisfactionNpcParamsElement {
                SupplyIndex: &self.row.columns[self.index_mapping[0]],
                Item: [
                    &self.row.columns[self.index_mapping[1]],
                    &self.row.columns[self.index_mapping[2]],
                    &self.row.columns[self.index_mapping[3]],
                ],
                SatisfactionRequired: &self.row.columns[self.index_mapping[4]],
                ItemCount: [
                    &self.row.columns[self.index_mapping[5]],
                    &self.row.columns[self.index_mapping[6]],
                    &self.row.columns[self.index_mapping[7]],
                ],
                IsHQ: [
                    &self.row.columns[self.index_mapping[8]],
                    &self.row.columns[self.index_mapping[9]],
                    &self.row.columns[self.index_mapping[10]],
                ],
            },
            SatisfactionNpcParamsElement {
                SupplyIndex: &self.row.columns[self.index_mapping[11]],
                Item: [
                    &self.row.columns[self.index_mapping[12]],
                    &self.row.columns[self.index_mapping[13]],
                    &self.row.columns[self.index_mapping[14]],
                ],
                SatisfactionRequired: &self.row.columns[self.index_mapping[15]],
                ItemCount: [
                    &self.row.columns[self.index_mapping[16]],
                    &self.row.columns[self.index_mapping[17]],
                    &self.row.columns[self.index_mapping[18]],
                ],
                IsHQ: [
                    &self.row.columns[self.index_mapping[19]],
                    &self.row.columns[self.index_mapping[20]],
                    &self.row.columns[self.index_mapping[21]],
                ],
            },
            SatisfactionNpcParamsElement {
                SupplyIndex: &self.row.columns[self.index_mapping[22]],
                Item: [
                    &self.row.columns[self.index_mapping[23]],
                    &self.row.columns[self.index_mapping[24]],
                    &self.row.columns[self.index_mapping[25]],
                ],
                SatisfactionRequired: &self.row.columns[self.index_mapping[26]],
                ItemCount: [
                    &self.row.columns[self.index_mapping[27]],
                    &self.row.columns[self.index_mapping[28]],
                    &self.row.columns[self.index_mapping[29]],
                ],
                IsHQ: [
                    &self.row.columns[self.index_mapping[30]],
                    &self.row.columns[self.index_mapping[31]],
                    &self.row.columns[self.index_mapping[32]],
                ],
            },
            SatisfactionNpcParamsElement {
                SupplyIndex: &self.row.columns[self.index_mapping[33]],
                Item: [
                    &self.row.columns[self.index_mapping[34]],
                    &self.row.columns[self.index_mapping[35]],
                    &self.row.columns[self.index_mapping[36]],
                ],
                SatisfactionRequired: &self.row.columns[self.index_mapping[37]],
                ItemCount: [
                    &self.row.columns[self.index_mapping[38]],
                    &self.row.columns[self.index_mapping[39]],
                    &self.row.columns[self.index_mapping[40]],
                ],
                IsHQ: [
                    &self.row.columns[self.index_mapping[41]],
                    &self.row.columns[self.index_mapping[42]],
                    &self.row.columns[self.index_mapping[43]],
                ],
            },
            SatisfactionNpcParamsElement {
                SupplyIndex: &self.row.columns[self.index_mapping[44]],
                Item: [
                    &self.row.columns[self.index_mapping[45]],
                    &self.row.columns[self.index_mapping[46]],
                    &self.row.columns[self.index_mapping[47]],
                ],
                SatisfactionRequired: &self.row.columns[self.index_mapping[48]],
                ItemCount: [
                    &self.row.columns[self.index_mapping[49]],
                    &self.row.columns[self.index_mapping[50]],
                    &self.row.columns[self.index_mapping[51]],
                ],
                IsHQ: [
                    &self.row.columns[self.index_mapping[52]],
                    &self.row.columns[self.index_mapping[53]],
                    &self.row.columns[self.index_mapping[54]],
                ],
            },
            SatisfactionNpcParamsElement {
                SupplyIndex: &self.row.columns[self.index_mapping[55]],
                Item: [
                    &self.row.columns[self.index_mapping[56]],
                    &self.row.columns[self.index_mapping[57]],
                    &self.row.columns[self.index_mapping[58]],
                ],
                SatisfactionRequired: &self.row.columns[self.index_mapping[59]],
                ItemCount: [
                    &self.row.columns[self.index_mapping[60]],
                    &self.row.columns[self.index_mapping[61]],
                    &self.row.columns[self.index_mapping[62]],
                ],
                IsHQ: [
                    &self.row.columns[self.index_mapping[63]],
                    &self.row.columns[self.index_mapping[64]],
                    &self.row.columns[self.index_mapping[65]],
                ],
            },
        ]
    }
    pub fn RankParams(&'a self) -> [RankParamsElement<'a>; 6] {
        [
            RankParamsElement {
                ImageId: &self.row.columns[self.index_mapping[66]],
                Unknown1: &self.row.columns[self.index_mapping[67]],
                Quest: &self.row.columns[self.index_mapping[68]],
            },
            RankParamsElement {
                ImageId: &self.row.columns[self.index_mapping[69]],
                Unknown1: &self.row.columns[self.index_mapping[70]],
                Quest: &self.row.columns[self.index_mapping[71]],
            },
            RankParamsElement {
                ImageId: &self.row.columns[self.index_mapping[72]],
                Unknown1: &self.row.columns[self.index_mapping[73]],
                Quest: &self.row.columns[self.index_mapping[74]],
            },
            RankParamsElement {
                ImageId: &self.row.columns[self.index_mapping[75]],
                Unknown1: &self.row.columns[self.index_mapping[76]],
                Quest: &self.row.columns[self.index_mapping[77]],
            },
            RankParamsElement {
                ImageId: &self.row.columns[self.index_mapping[78]],
                Unknown1: &self.row.columns[self.index_mapping[79]],
                Quest: &self.row.columns[self.index_mapping[80]],
            },
            RankParamsElement {
                ImageId: &self.row.columns[self.index_mapping[81]],
                Unknown1: &self.row.columns[self.index_mapping[82]],
                Quest: &self.row.columns[self.index_mapping[83]],
            },
        ]
    }
    pub fn Level(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[84]]
    }
    pub fn Npc(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[85]]
    }
    pub fn QuestRequired(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[86]]
    }
    pub fn Icon(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[87]]
    }
    pub fn LevelUnlock(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[88]]
    }
    pub fn DeliveriesPerWeek(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[89]]
    }
    pub fn GlamourIndex(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[90]]
    }
    pub fn Unknown19(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[91]]
    }
    pub fn Unknown20(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[92]]
    }
}
