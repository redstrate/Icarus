//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct HWDCrafterSupplyParamsElement<'a> {
    pub ItemTradeIn: &'a ColumnData,
    pub BaseCollectableRating: &'a ColumnData,
    pub MidCollectableRating: &'a ColumnData,
    pub HighCollectableRating: &'a ColumnData,
    pub BaseCollectableReward: &'a ColumnData,
    pub MidCollectableReward: &'a ColumnData,
    pub HighCollectableReward: &'a ColumnData,
    pub BaseCollectableRewardPostPhase: &'a ColumnData,
    pub MidCollectableRewardPostPhase: &'a ColumnData,
    pub HighCollectableRewardPostPhase: &'a ColumnData,
    pub Level: &'a ColumnData,
    pub LevelMax: &'a ColumnData,
    pub Unknown0: &'a ColumnData,
    pub TermName: &'a ColumnData,
}
pub struct HWDCrafterSupplySheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl HWDCrafterSupplySheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "HWDCrafterSupply")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(
                    read_excel_sheet(resource, "HWDCrafterSupply", &exh, language, i)?,
                );
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<HWDCrafterSupplyRow> {
        let column_defs = &self.exh.column_definitions;
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
        Some(HWDCrafterSupplyRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<HWDCrafterSupplyRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => row,
                ExcelRowKind::SubRows(rows) => &rows.first()?.1,
            };
            return self.read_row(row);
        }
        None
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<HWDCrafterSupplyRow> {
        for page in &self.pages {
            let Some(row) = &page.get_row(row_id) else {
                continue;
            };
            let row = match row {
                ExcelRowKind::SingleRow(row) => return None,
                ExcelRowKind::SubRows(subrows) => {
                    &subrows.iter().filter(|(id, _)| *id == subrow_id).next()?.1
                }
            };
            return self.read_row(row);
        }
        None
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.row_count
    }
}
pub struct HWDCrafterSupplyRow {
    columns: Vec<ColumnData>,
}
impl HWDCrafterSupplyRow {
    pub fn HWDCrafterSupplyParams<'a>(
        &'a self,
    ) -> [HWDCrafterSupplyParamsElement<'a>; 23] {
        [
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[0],
                BaseCollectableRating: &self.columns[1],
                MidCollectableRating: &self.columns[2],
                HighCollectableRating: &self.columns[3],
                BaseCollectableReward: &self.columns[4],
                MidCollectableReward: &self.columns[5],
                HighCollectableReward: &self.columns[6],
                BaseCollectableRewardPostPhase: &self.columns[7],
                MidCollectableRewardPostPhase: &self.columns[8],
                HighCollectableRewardPostPhase: &self.columns[9],
                Level: &self.columns[10],
                LevelMax: &self.columns[11],
                Unknown0: &self.columns[12],
                TermName: &self.columns[13],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[14],
                BaseCollectableRating: &self.columns[15],
                MidCollectableRating: &self.columns[16],
                HighCollectableRating: &self.columns[17],
                BaseCollectableReward: &self.columns[18],
                MidCollectableReward: &self.columns[19],
                HighCollectableReward: &self.columns[20],
                BaseCollectableRewardPostPhase: &self.columns[21],
                MidCollectableRewardPostPhase: &self.columns[22],
                HighCollectableRewardPostPhase: &self.columns[23],
                Level: &self.columns[24],
                LevelMax: &self.columns[25],
                Unknown0: &self.columns[26],
                TermName: &self.columns[27],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[28],
                BaseCollectableRating: &self.columns[29],
                MidCollectableRating: &self.columns[30],
                HighCollectableRating: &self.columns[31],
                BaseCollectableReward: &self.columns[32],
                MidCollectableReward: &self.columns[33],
                HighCollectableReward: &self.columns[34],
                BaseCollectableRewardPostPhase: &self.columns[35],
                MidCollectableRewardPostPhase: &self.columns[36],
                HighCollectableRewardPostPhase: &self.columns[37],
                Level: &self.columns[38],
                LevelMax: &self.columns[39],
                Unknown0: &self.columns[40],
                TermName: &self.columns[41],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[42],
                BaseCollectableRating: &self.columns[43],
                MidCollectableRating: &self.columns[44],
                HighCollectableRating: &self.columns[45],
                BaseCollectableReward: &self.columns[46],
                MidCollectableReward: &self.columns[47],
                HighCollectableReward: &self.columns[48],
                BaseCollectableRewardPostPhase: &self.columns[49],
                MidCollectableRewardPostPhase: &self.columns[50],
                HighCollectableRewardPostPhase: &self.columns[51],
                Level: &self.columns[52],
                LevelMax: &self.columns[53],
                Unknown0: &self.columns[54],
                TermName: &self.columns[55],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[56],
                BaseCollectableRating: &self.columns[57],
                MidCollectableRating: &self.columns[58],
                HighCollectableRating: &self.columns[59],
                BaseCollectableReward: &self.columns[60],
                MidCollectableReward: &self.columns[61],
                HighCollectableReward: &self.columns[62],
                BaseCollectableRewardPostPhase: &self.columns[63],
                MidCollectableRewardPostPhase: &self.columns[64],
                HighCollectableRewardPostPhase: &self.columns[65],
                Level: &self.columns[66],
                LevelMax: &self.columns[67],
                Unknown0: &self.columns[68],
                TermName: &self.columns[69],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[70],
                BaseCollectableRating: &self.columns[71],
                MidCollectableRating: &self.columns[72],
                HighCollectableRating: &self.columns[73],
                BaseCollectableReward: &self.columns[74],
                MidCollectableReward: &self.columns[75],
                HighCollectableReward: &self.columns[76],
                BaseCollectableRewardPostPhase: &self.columns[77],
                MidCollectableRewardPostPhase: &self.columns[78],
                HighCollectableRewardPostPhase: &self.columns[79],
                Level: &self.columns[80],
                LevelMax: &self.columns[81],
                Unknown0: &self.columns[82],
                TermName: &self.columns[83],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[84],
                BaseCollectableRating: &self.columns[85],
                MidCollectableRating: &self.columns[86],
                HighCollectableRating: &self.columns[87],
                BaseCollectableReward: &self.columns[88],
                MidCollectableReward: &self.columns[89],
                HighCollectableReward: &self.columns[90],
                BaseCollectableRewardPostPhase: &self.columns[91],
                MidCollectableRewardPostPhase: &self.columns[92],
                HighCollectableRewardPostPhase: &self.columns[93],
                Level: &self.columns[94],
                LevelMax: &self.columns[95],
                Unknown0: &self.columns[96],
                TermName: &self.columns[97],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[98],
                BaseCollectableRating: &self.columns[99],
                MidCollectableRating: &self.columns[100],
                HighCollectableRating: &self.columns[101],
                BaseCollectableReward: &self.columns[102],
                MidCollectableReward: &self.columns[103],
                HighCollectableReward: &self.columns[104],
                BaseCollectableRewardPostPhase: &self.columns[105],
                MidCollectableRewardPostPhase: &self.columns[106],
                HighCollectableRewardPostPhase: &self.columns[107],
                Level: &self.columns[108],
                LevelMax: &self.columns[109],
                Unknown0: &self.columns[110],
                TermName: &self.columns[111],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[112],
                BaseCollectableRating: &self.columns[113],
                MidCollectableRating: &self.columns[114],
                HighCollectableRating: &self.columns[115],
                BaseCollectableReward: &self.columns[116],
                MidCollectableReward: &self.columns[117],
                HighCollectableReward: &self.columns[118],
                BaseCollectableRewardPostPhase: &self.columns[119],
                MidCollectableRewardPostPhase: &self.columns[120],
                HighCollectableRewardPostPhase: &self.columns[121],
                Level: &self.columns[122],
                LevelMax: &self.columns[123],
                Unknown0: &self.columns[124],
                TermName: &self.columns[125],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[126],
                BaseCollectableRating: &self.columns[127],
                MidCollectableRating: &self.columns[128],
                HighCollectableRating: &self.columns[129],
                BaseCollectableReward: &self.columns[130],
                MidCollectableReward: &self.columns[131],
                HighCollectableReward: &self.columns[132],
                BaseCollectableRewardPostPhase: &self.columns[133],
                MidCollectableRewardPostPhase: &self.columns[134],
                HighCollectableRewardPostPhase: &self.columns[135],
                Level: &self.columns[136],
                LevelMax: &self.columns[137],
                Unknown0: &self.columns[138],
                TermName: &self.columns[139],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[140],
                BaseCollectableRating: &self.columns[141],
                MidCollectableRating: &self.columns[142],
                HighCollectableRating: &self.columns[143],
                BaseCollectableReward: &self.columns[144],
                MidCollectableReward: &self.columns[145],
                HighCollectableReward: &self.columns[146],
                BaseCollectableRewardPostPhase: &self.columns[147],
                MidCollectableRewardPostPhase: &self.columns[148],
                HighCollectableRewardPostPhase: &self.columns[149],
                Level: &self.columns[150],
                LevelMax: &self.columns[151],
                Unknown0: &self.columns[152],
                TermName: &self.columns[153],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[154],
                BaseCollectableRating: &self.columns[155],
                MidCollectableRating: &self.columns[156],
                HighCollectableRating: &self.columns[157],
                BaseCollectableReward: &self.columns[158],
                MidCollectableReward: &self.columns[159],
                HighCollectableReward: &self.columns[160],
                BaseCollectableRewardPostPhase: &self.columns[161],
                MidCollectableRewardPostPhase: &self.columns[162],
                HighCollectableRewardPostPhase: &self.columns[163],
                Level: &self.columns[164],
                LevelMax: &self.columns[165],
                Unknown0: &self.columns[166],
                TermName: &self.columns[167],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[168],
                BaseCollectableRating: &self.columns[169],
                MidCollectableRating: &self.columns[170],
                HighCollectableRating: &self.columns[171],
                BaseCollectableReward: &self.columns[172],
                MidCollectableReward: &self.columns[173],
                HighCollectableReward: &self.columns[174],
                BaseCollectableRewardPostPhase: &self.columns[175],
                MidCollectableRewardPostPhase: &self.columns[176],
                HighCollectableRewardPostPhase: &self.columns[177],
                Level: &self.columns[178],
                LevelMax: &self.columns[179],
                Unknown0: &self.columns[180],
                TermName: &self.columns[181],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[182],
                BaseCollectableRating: &self.columns[183],
                MidCollectableRating: &self.columns[184],
                HighCollectableRating: &self.columns[185],
                BaseCollectableReward: &self.columns[186],
                MidCollectableReward: &self.columns[187],
                HighCollectableReward: &self.columns[188],
                BaseCollectableRewardPostPhase: &self.columns[189],
                MidCollectableRewardPostPhase: &self.columns[190],
                HighCollectableRewardPostPhase: &self.columns[191],
                Level: &self.columns[192],
                LevelMax: &self.columns[193],
                Unknown0: &self.columns[194],
                TermName: &self.columns[195],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[196],
                BaseCollectableRating: &self.columns[197],
                MidCollectableRating: &self.columns[198],
                HighCollectableRating: &self.columns[199],
                BaseCollectableReward: &self.columns[200],
                MidCollectableReward: &self.columns[201],
                HighCollectableReward: &self.columns[202],
                BaseCollectableRewardPostPhase: &self.columns[203],
                MidCollectableRewardPostPhase: &self.columns[204],
                HighCollectableRewardPostPhase: &self.columns[205],
                Level: &self.columns[206],
                LevelMax: &self.columns[207],
                Unknown0: &self.columns[208],
                TermName: &self.columns[209],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[210],
                BaseCollectableRating: &self.columns[211],
                MidCollectableRating: &self.columns[212],
                HighCollectableRating: &self.columns[213],
                BaseCollectableReward: &self.columns[214],
                MidCollectableReward: &self.columns[215],
                HighCollectableReward: &self.columns[216],
                BaseCollectableRewardPostPhase: &self.columns[217],
                MidCollectableRewardPostPhase: &self.columns[218],
                HighCollectableRewardPostPhase: &self.columns[219],
                Level: &self.columns[220],
                LevelMax: &self.columns[221],
                Unknown0: &self.columns[222],
                TermName: &self.columns[223],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[224],
                BaseCollectableRating: &self.columns[225],
                MidCollectableRating: &self.columns[226],
                HighCollectableRating: &self.columns[227],
                BaseCollectableReward: &self.columns[228],
                MidCollectableReward: &self.columns[229],
                HighCollectableReward: &self.columns[230],
                BaseCollectableRewardPostPhase: &self.columns[231],
                MidCollectableRewardPostPhase: &self.columns[232],
                HighCollectableRewardPostPhase: &self.columns[233],
                Level: &self.columns[234],
                LevelMax: &self.columns[235],
                Unknown0: &self.columns[236],
                TermName: &self.columns[237],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[238],
                BaseCollectableRating: &self.columns[239],
                MidCollectableRating: &self.columns[240],
                HighCollectableRating: &self.columns[241],
                BaseCollectableReward: &self.columns[242],
                MidCollectableReward: &self.columns[243],
                HighCollectableReward: &self.columns[244],
                BaseCollectableRewardPostPhase: &self.columns[245],
                MidCollectableRewardPostPhase: &self.columns[246],
                HighCollectableRewardPostPhase: &self.columns[247],
                Level: &self.columns[248],
                LevelMax: &self.columns[249],
                Unknown0: &self.columns[250],
                TermName: &self.columns[251],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[252],
                BaseCollectableRating: &self.columns[253],
                MidCollectableRating: &self.columns[254],
                HighCollectableRating: &self.columns[255],
                BaseCollectableReward: &self.columns[256],
                MidCollectableReward: &self.columns[257],
                HighCollectableReward: &self.columns[258],
                BaseCollectableRewardPostPhase: &self.columns[259],
                MidCollectableRewardPostPhase: &self.columns[260],
                HighCollectableRewardPostPhase: &self.columns[261],
                Level: &self.columns[262],
                LevelMax: &self.columns[263],
                Unknown0: &self.columns[264],
                TermName: &self.columns[265],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[266],
                BaseCollectableRating: &self.columns[267],
                MidCollectableRating: &self.columns[268],
                HighCollectableRating: &self.columns[269],
                BaseCollectableReward: &self.columns[270],
                MidCollectableReward: &self.columns[271],
                HighCollectableReward: &self.columns[272],
                BaseCollectableRewardPostPhase: &self.columns[273],
                MidCollectableRewardPostPhase: &self.columns[274],
                HighCollectableRewardPostPhase: &self.columns[275],
                Level: &self.columns[276],
                LevelMax: &self.columns[277],
                Unknown0: &self.columns[278],
                TermName: &self.columns[279],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[280],
                BaseCollectableRating: &self.columns[281],
                MidCollectableRating: &self.columns[282],
                HighCollectableRating: &self.columns[283],
                BaseCollectableReward: &self.columns[284],
                MidCollectableReward: &self.columns[285],
                HighCollectableReward: &self.columns[286],
                BaseCollectableRewardPostPhase: &self.columns[287],
                MidCollectableRewardPostPhase: &self.columns[288],
                HighCollectableRewardPostPhase: &self.columns[289],
                Level: &self.columns[290],
                LevelMax: &self.columns[291],
                Unknown0: &self.columns[292],
                TermName: &self.columns[293],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[294],
                BaseCollectableRating: &self.columns[295],
                MidCollectableRating: &self.columns[296],
                HighCollectableRating: &self.columns[297],
                BaseCollectableReward: &self.columns[298],
                MidCollectableReward: &self.columns[299],
                HighCollectableReward: &self.columns[300],
                BaseCollectableRewardPostPhase: &self.columns[301],
                MidCollectableRewardPostPhase: &self.columns[302],
                HighCollectableRewardPostPhase: &self.columns[303],
                Level: &self.columns[304],
                LevelMax: &self.columns[305],
                Unknown0: &self.columns[306],
                TermName: &self.columns[307],
            },
            HWDCrafterSupplyParamsElement {
                ItemTradeIn: &self.columns[308],
                BaseCollectableRating: &self.columns[309],
                MidCollectableRating: &self.columns[310],
                HighCollectableRating: &self.columns[311],
                BaseCollectableReward: &self.columns[312],
                MidCollectableReward: &self.columns[313],
                HighCollectableReward: &self.columns[314],
                BaseCollectableRewardPostPhase: &self.columns[315],
                MidCollectableRewardPostPhase: &self.columns[316],
                HighCollectableRewardPostPhase: &self.columns[317],
                Level: &self.columns[318],
                LevelMax: &self.columns[319],
                Unknown0: &self.columns[320],
                TermName: &self.columns[321],
            },
        ]
    }
}
