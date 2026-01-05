//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct ObjectiveIconElement<'a> {
    pub LayoutId: &'a ColumnData,
    pub Icon: &'a ColumnData,
}
#[derive(Debug, Clone)]
pub struct FateSheet {
    sheet: ExcelSheet,
}
impl FateSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Fate")?;
        let sheet = resolver.read_excel_sheet(&exh, "Fate", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<FateRow> {
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
        Some(FateRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<FateRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<FateRow> {
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
pub struct FateRow {
    columns: Vec<ColumnData>,
}
impl FateRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn Description<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1]
    }
    pub fn Objective<'a>(&'a self) -> &'a ColumnData {
        &self.columns[2]
    }
    pub fn StatusText<'a>(&'a self) -> [&'a ColumnData; 3] {
        [&self.columns[3], &self.columns[4], &self.columns[5]]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[6]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[7]
    }
    pub fn ReqEventItem<'a>(&'a self) -> &'a ColumnData {
        &self.columns[8]
    }
    pub fn TurnInEventItem<'a>(&'a self) -> &'a ColumnData {
        &self.columns[9]
    }
    pub fn Unknown2<'a>(&'a self) -> [&'a ColumnData; 3] {
        [&self.columns[10], &self.columns[11], &self.columns[12]]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[13]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a ColumnData {
        &self.columns[14]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[15]
    }
    pub fn ObjectiveIcon<'a>(&'a self) -> [ObjectiveIconElement<'a>; 32] {
        [
            ObjectiveIconElement {
                LayoutId: &self.columns[16],
                Icon: &self.columns[17],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[18],
                Icon: &self.columns[19],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[20],
                Icon: &self.columns[21],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[22],
                Icon: &self.columns[23],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[24],
                Icon: &self.columns[25],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[26],
                Icon: &self.columns[27],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[28],
                Icon: &self.columns[29],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[30],
                Icon: &self.columns[31],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[32],
                Icon: &self.columns[33],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[34],
                Icon: &self.columns[35],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[36],
                Icon: &self.columns[37],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[38],
                Icon: &self.columns[39],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[40],
                Icon: &self.columns[41],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[42],
                Icon: &self.columns[43],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[44],
                Icon: &self.columns[45],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[46],
                Icon: &self.columns[47],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[48],
                Icon: &self.columns[49],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[50],
                Icon: &self.columns[51],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[52],
                Icon: &self.columns[53],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[54],
                Icon: &self.columns[55],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[56],
                Icon: &self.columns[57],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[58],
                Icon: &self.columns[59],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[60],
                Icon: &self.columns[61],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[62],
                Icon: &self.columns[63],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[64],
                Icon: &self.columns[65],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[66],
                Icon: &self.columns[67],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[68],
                Icon: &self.columns[69],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[70],
                Icon: &self.columns[71],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[72],
                Icon: &self.columns[73],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[74],
                Icon: &self.columns[75],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[76],
                Icon: &self.columns[77],
            },
            ObjectiveIconElement {
                LayoutId: &self.columns[78],
                Icon: &self.columns[79],
            },
        ]
    }
    pub fn Location<'a>(&'a self) -> &'a ColumnData {
        &self.columns[80]
    }
    pub fn EventItem<'a>(&'a self) -> &'a ColumnData {
        &self.columns[81]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[82]
    }
    pub fn MapIcon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[83]
    }
    pub fn InactiveMapIcon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[84]
    }
    pub fn LGBGuardNPCLocation<'a>(&'a self) -> &'a ColumnData {
        &self.columns[85]
    }
    pub fn RequiredQuest<'a>(&'a self) -> &'a ColumnData {
        &self.columns[86]
    }
    pub fn FATEChain<'a>(&'a self) -> &'a ColumnData {
        &self.columns[87]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a ColumnData {
        &self.columns[88]
    }
    pub fn FateRuleEx<'a>(&'a self) -> &'a ColumnData {
        &self.columns[89]
    }
    pub fn Music<'a>(&'a self) -> &'a ColumnData {
        &self.columns[90]
    }
    pub fn ScreenImageAccept<'a>(&'a self) -> &'a ColumnData {
        &self.columns[91]
    }
    pub fn ScreenImageComplete<'a>(&'a self) -> &'a ColumnData {
        &self.columns[92]
    }
    pub fn ScreenImageFailed<'a>(&'a self) -> &'a ColumnData {
        &self.columns[93]
    }
    pub fn GivenStatus<'a>(&'a self) -> &'a ColumnData {
        &self.columns[94]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[95]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[96]
    }
    pub fn EurekaFate<'a>(&'a self) -> &'a ColumnData {
        &self.columns[97]
    }
    pub fn Rule<'a>(&'a self) -> &'a ColumnData {
        &self.columns[98]
    }
    pub fn ClassJobLevel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[99]
    }
    pub fn ClassJobLevelMax<'a>(&'a self) -> &'a ColumnData {
        &self.columns[100]
    }
    pub fn StatusValue<'a>(&'a self) -> [&'a ColumnData; 3] {
        [&self.columns[101], &self.columns[102], &self.columns[103]]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[104]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[105]
    }
    pub fn SpecialFate<'a>(&'a self) -> &'a ColumnData {
        &self.columns[106]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[107]
    }
    pub fn AdventEvent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[108]
    }
    pub fn MoonFaireEvent<'a>(&'a self) -> &'a ColumnData {
        &self.columns[109]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[110]
    }
}
