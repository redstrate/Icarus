//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct SatisfactionNpcParamsElement<'a> {
    pub SupplyIndex: &'a ColumnData,
    pub Item: [&'a ColumnData; 3],
    pub SatisfactionRequired: &'a ColumnData,
    pub ItemCount: [&'a ColumnData; 3],
    pub IsHQ: [&'a ColumnData; 3],
}
pub struct RankParamsElement<'a> {
    pub ImageId: &'a ColumnData,
    pub Unknown1: &'a ColumnData,
    pub Quest: &'a ColumnData,
}
#[derive(Debug, Clone)]
pub struct SatisfactionNpcSheet {
    sheet: ExcelSheet,
}
impl SatisfactionNpcSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("SatisfactionNpc")?;
        let sheet = resolver.read_excel_sheet(&exh, "SatisfactionNpc", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<SatisfactionNpcRow> {
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
        Some(SatisfactionNpcRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<SatisfactionNpcRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<SatisfactionNpcRow> {
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
pub struct SatisfactionNpcRow {
    columns: Vec<ColumnData>,
}
impl SatisfactionNpcRow {
    pub fn SatisfactionNpcParams<'a>(&'a self) -> [SatisfactionNpcParamsElement<'a>; 6] {
        [
            SatisfactionNpcParamsElement {
                SupplyIndex: &self.columns[0],
                Item: [&self.columns[1], &self.columns[2], &self.columns[3]],
                SatisfactionRequired: &self.columns[4],
                ItemCount: [&self.columns[5], &self.columns[6], &self.columns[7]],
                IsHQ: [&self.columns[8], &self.columns[9], &self.columns[10]],
            },
            SatisfactionNpcParamsElement {
                SupplyIndex: &self.columns[11],
                Item: [&self.columns[12], &self.columns[13], &self.columns[14]],
                SatisfactionRequired: &self.columns[15],
                ItemCount: [&self.columns[16], &self.columns[17], &self.columns[18]],
                IsHQ: [&self.columns[19], &self.columns[20], &self.columns[21]],
            },
            SatisfactionNpcParamsElement {
                SupplyIndex: &self.columns[22],
                Item: [&self.columns[23], &self.columns[24], &self.columns[25]],
                SatisfactionRequired: &self.columns[26],
                ItemCount: [&self.columns[27], &self.columns[28], &self.columns[29]],
                IsHQ: [&self.columns[30], &self.columns[31], &self.columns[32]],
            },
            SatisfactionNpcParamsElement {
                SupplyIndex: &self.columns[33],
                Item: [&self.columns[34], &self.columns[35], &self.columns[36]],
                SatisfactionRequired: &self.columns[37],
                ItemCount: [&self.columns[38], &self.columns[39], &self.columns[40]],
                IsHQ: [&self.columns[41], &self.columns[42], &self.columns[43]],
            },
            SatisfactionNpcParamsElement {
                SupplyIndex: &self.columns[44],
                Item: [&self.columns[45], &self.columns[46], &self.columns[47]],
                SatisfactionRequired: &self.columns[48],
                ItemCount: [&self.columns[49], &self.columns[50], &self.columns[51]],
                IsHQ: [&self.columns[52], &self.columns[53], &self.columns[54]],
            },
            SatisfactionNpcParamsElement {
                SupplyIndex: &self.columns[55],
                Item: [&self.columns[56], &self.columns[57], &self.columns[58]],
                SatisfactionRequired: &self.columns[59],
                ItemCount: [&self.columns[60], &self.columns[61], &self.columns[62]],
                IsHQ: [&self.columns[63], &self.columns[64], &self.columns[65]],
            },
        ]
    }
    pub fn RankParams<'a>(&'a self) -> [RankParamsElement<'a>; 6] {
        [
            RankParamsElement {
                ImageId: &self.columns[66],
                Unknown1: &self.columns[67],
                Quest: &self.columns[68],
            },
            RankParamsElement {
                ImageId: &self.columns[69],
                Unknown1: &self.columns[70],
                Quest: &self.columns[71],
            },
            RankParamsElement {
                ImageId: &self.columns[72],
                Unknown1: &self.columns[73],
                Quest: &self.columns[74],
            },
            RankParamsElement {
                ImageId: &self.columns[75],
                Unknown1: &self.columns[76],
                Quest: &self.columns[77],
            },
            RankParamsElement {
                ImageId: &self.columns[78],
                Unknown1: &self.columns[79],
                Quest: &self.columns[80],
            },
            RankParamsElement {
                ImageId: &self.columns[81],
                Unknown1: &self.columns[82],
                Quest: &self.columns[83],
            },
        ]
    }
    pub fn Level<'a>(&'a self) -> &'a ColumnData {
        &self.columns[84]
    }
    pub fn Npc<'a>(&'a self) -> &'a ColumnData {
        &self.columns[85]
    }
    pub fn QuestRequired<'a>(&'a self) -> &'a ColumnData {
        &self.columns[86]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[87]
    }
    pub fn LevelUnlock<'a>(&'a self) -> &'a ColumnData {
        &self.columns[88]
    }
    pub fn DeliveriesPerWeek<'a>(&'a self) -> &'a ColumnData {
        &self.columns[89]
    }
    pub fn GlamourIndex<'a>(&'a self) -> &'a ColumnData {
        &self.columns[90]
    }
    pub fn Unknown19<'a>(&'a self) -> &'a ColumnData {
        &self.columns[91]
    }
    pub fn Unknown20<'a>(&'a self) -> &'a ColumnData {
        &self.columns[92]
    }
}
