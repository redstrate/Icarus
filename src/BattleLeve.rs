//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
};
pub struct LeveDataElement<'a> {
    pub BNpcName: &'a Field,
    pub ToDoNumberInvolved: &'a Field,
    pub ToDoParam: [&'a Field; 5],
    pub BaseID: &'a Field,
    pub ItemsInvolved: &'a Field,
    pub EnemyLevel: &'a Field,
    pub ItemsInvolvedQty: &'a Field,
    pub ItemDropRate: &'a Field,
    pub NumOfAppearance: [&'a Field; 8],
}
#[derive(Debug, Clone)]
pub struct BattleLeveSheet {
    sheet: Sheet,
}
impl BattleLeveSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("BattleLeve")?;
        let sheet = resolver.read_excel_sheet(&exh, "BattleLeve", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<BattleLeveRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<BattleLeveRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for BattleLeveSheet {
    type Row = BattleLeveRow;
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
impl<'a> IntoIterator for &'a BattleLeveSheet {
    type Item = (u32, Vec<(u16, BattleLeveRow)>);
    type IntoIter = StructuredSheetIterator<'a, BattleLeveSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, BattleLeveSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BattleLeveRow {
    columns: Vec<Field>,
}
impl BattleLeveRow {
    pub fn Time<'a>(&'a self) -> [&'a Field; 8] {
        [
            &self.columns[0],
            &self.columns[1],
            &self.columns[2],
            &self.columns[3],
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
        ]
    }
    pub fn LeveData<'a>(&'a self) -> [LeveDataElement<'a>; 8] {
        [
            LeveDataElement {
                BNpcName: &self.columns[8],
                ToDoNumberInvolved: &self.columns[9],
                ToDoParam: [
                    &self.columns[10],
                    &self.columns[11],
                    &self.columns[12],
                    &self.columns[13],
                    &self.columns[14],
                ],
                BaseID: &self.columns[15],
                ItemsInvolved: &self.columns[16],
                EnemyLevel: &self.columns[17],
                ItemsInvolvedQty: &self.columns[18],
                ItemDropRate: &self.columns[19],
                NumOfAppearance: [
                    &self.columns[20],
                    &self.columns[21],
                    &self.columns[22],
                    &self.columns[23],
                    &self.columns[24],
                    &self.columns[25],
                    &self.columns[26],
                    &self.columns[27],
                ],
            },
            LeveDataElement {
                BNpcName: &self.columns[28],
                ToDoNumberInvolved: &self.columns[29],
                ToDoParam: [
                    &self.columns[30],
                    &self.columns[31],
                    &self.columns[32],
                    &self.columns[33],
                    &self.columns[34],
                ],
                BaseID: &self.columns[35],
                ItemsInvolved: &self.columns[36],
                EnemyLevel: &self.columns[37],
                ItemsInvolvedQty: &self.columns[38],
                ItemDropRate: &self.columns[39],
                NumOfAppearance: [
                    &self.columns[40],
                    &self.columns[41],
                    &self.columns[42],
                    &self.columns[43],
                    &self.columns[44],
                    &self.columns[45],
                    &self.columns[46],
                    &self.columns[47],
                ],
            },
            LeveDataElement {
                BNpcName: &self.columns[48],
                ToDoNumberInvolved: &self.columns[49],
                ToDoParam: [
                    &self.columns[50],
                    &self.columns[51],
                    &self.columns[52],
                    &self.columns[53],
                    &self.columns[54],
                ],
                BaseID: &self.columns[55],
                ItemsInvolved: &self.columns[56],
                EnemyLevel: &self.columns[57],
                ItemsInvolvedQty: &self.columns[58],
                ItemDropRate: &self.columns[59],
                NumOfAppearance: [
                    &self.columns[60],
                    &self.columns[61],
                    &self.columns[62],
                    &self.columns[63],
                    &self.columns[64],
                    &self.columns[65],
                    &self.columns[66],
                    &self.columns[67],
                ],
            },
            LeveDataElement {
                BNpcName: &self.columns[68],
                ToDoNumberInvolved: &self.columns[69],
                ToDoParam: [
                    &self.columns[70],
                    &self.columns[71],
                    &self.columns[72],
                    &self.columns[73],
                    &self.columns[74],
                ],
                BaseID: &self.columns[75],
                ItemsInvolved: &self.columns[76],
                EnemyLevel: &self.columns[77],
                ItemsInvolvedQty: &self.columns[78],
                ItemDropRate: &self.columns[79],
                NumOfAppearance: [
                    &self.columns[80],
                    &self.columns[81],
                    &self.columns[82],
                    &self.columns[83],
                    &self.columns[84],
                    &self.columns[85],
                    &self.columns[86],
                    &self.columns[87],
                ],
            },
            LeveDataElement {
                BNpcName: &self.columns[88],
                ToDoNumberInvolved: &self.columns[89],
                ToDoParam: [
                    &self.columns[90],
                    &self.columns[91],
                    &self.columns[92],
                    &self.columns[93],
                    &self.columns[94],
                ],
                BaseID: &self.columns[95],
                ItemsInvolved: &self.columns[96],
                EnemyLevel: &self.columns[97],
                ItemsInvolvedQty: &self.columns[98],
                ItemDropRate: &self.columns[99],
                NumOfAppearance: [
                    &self.columns[100],
                    &self.columns[101],
                    &self.columns[102],
                    &self.columns[103],
                    &self.columns[104],
                    &self.columns[105],
                    &self.columns[106],
                    &self.columns[107],
                ],
            },
            LeveDataElement {
                BNpcName: &self.columns[108],
                ToDoNumberInvolved: &self.columns[109],
                ToDoParam: [
                    &self.columns[110],
                    &self.columns[111],
                    &self.columns[112],
                    &self.columns[113],
                    &self.columns[114],
                ],
                BaseID: &self.columns[115],
                ItemsInvolved: &self.columns[116],
                EnemyLevel: &self.columns[117],
                ItemsInvolvedQty: &self.columns[118],
                ItemDropRate: &self.columns[119],
                NumOfAppearance: [
                    &self.columns[120],
                    &self.columns[121],
                    &self.columns[122],
                    &self.columns[123],
                    &self.columns[124],
                    &self.columns[125],
                    &self.columns[126],
                    &self.columns[127],
                ],
            },
            LeveDataElement {
                BNpcName: &self.columns[128],
                ToDoNumberInvolved: &self.columns[129],
                ToDoParam: [
                    &self.columns[130],
                    &self.columns[131],
                    &self.columns[132],
                    &self.columns[133],
                    &self.columns[134],
                ],
                BaseID: &self.columns[135],
                ItemsInvolved: &self.columns[136],
                EnemyLevel: &self.columns[137],
                ItemsInvolvedQty: &self.columns[138],
                ItemDropRate: &self.columns[139],
                NumOfAppearance: [
                    &self.columns[140],
                    &self.columns[141],
                    &self.columns[142],
                    &self.columns[143],
                    &self.columns[144],
                    &self.columns[145],
                    &self.columns[146],
                    &self.columns[147],
                ],
            },
            LeveDataElement {
                BNpcName: &self.columns[148],
                ToDoNumberInvolved: &self.columns[149],
                ToDoParam: [
                    &self.columns[150],
                    &self.columns[151],
                    &self.columns[152],
                    &self.columns[153],
                    &self.columns[154],
                ],
                BaseID: &self.columns[155],
                ItemsInvolved: &self.columns[156],
                EnemyLevel: &self.columns[157],
                ItemsInvolvedQty: &self.columns[158],
                ItemDropRate: &self.columns[159],
                NumOfAppearance: [
                    &self.columns[160],
                    &self.columns[161],
                    &self.columns[162],
                    &self.columns[163],
                    &self.columns[164],
                    &self.columns[165],
                    &self.columns[166],
                    &self.columns[167],
                ],
            },
        ]
    }
    pub fn ToDoSequence<'a>(&'a self) -> [&'a Field; 8] {
        [
            &self.columns[168],
            &self.columns[169],
            &self.columns[170],
            &self.columns[171],
            &self.columns[172],
            &self.columns[173],
            &self.columns[174],
            &self.columns[175],
        ]
    }
    pub fn Rule<'a>(&'a self) -> &'a Field {
        &self.columns[176]
    }
    pub fn Objectives<'a>(&'a self) -> [&'a Field; 3] {
        [&self.columns[177], &self.columns[178], &self.columns[179]]
    }
    pub fn Help<'a>(&'a self) -> [&'a Field; 2] {
        [&self.columns[180], &self.columns[181]]
    }
    pub fn Variant<'a>(&'a self) -> &'a Field {
        &self.columns[182]
    }
}
