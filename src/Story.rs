//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct StoryParamsElement<'a> {
    pub Instruction: &'a ColumnData,
    pub Argument: &'a ColumnData,
}
pub struct StoryDefineElement<'a> {
    pub CompletedQuest: [&'a ColumnData; 3],
    pub AcceptedQuest: [&'a ColumnData; 3],
    pub LayerSet: [&'a ColumnData; 2],
    pub Sequence: &'a ColumnData,
    pub CompletedQuestOperator: &'a ColumnData,
    pub AcceptedQuestOperator: &'a ColumnData,
    pub AcceptedQuestSequence: [&'a ColumnData; 3],
}
pub struct StoryListenerElement<'a> {
    pub Listener: &'a ColumnData,
    pub SequenceBegin: &'a ColumnData,
    pub SequenceEnd: &'a ColumnData,
}
#[derive(Debug, Clone)]
pub struct StorySheet {
    sheet: ExcelSheet,
}
impl StorySheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Story")?;
        let sheet = resolver.read_excel_sheet(exh, "Story", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<StoryRow> {
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
        Some(StoryRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<StoryRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<StoryRow> {
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
pub struct StoryRow {
    columns: Vec<ColumnData>,
}
impl StoryRow {
    pub fn StoryParams<'a>(&'a self) -> [StoryParamsElement<'a>; 40] {
        [
            StoryParamsElement {
                Instruction: &self.columns[0],
                Argument: &self.columns[1],
            },
            StoryParamsElement {
                Instruction: &self.columns[2],
                Argument: &self.columns[3],
            },
            StoryParamsElement {
                Instruction: &self.columns[4],
                Argument: &self.columns[5],
            },
            StoryParamsElement {
                Instruction: &self.columns[6],
                Argument: &self.columns[7],
            },
            StoryParamsElement {
                Instruction: &self.columns[8],
                Argument: &self.columns[9],
            },
            StoryParamsElement {
                Instruction: &self.columns[10],
                Argument: &self.columns[11],
            },
            StoryParamsElement {
                Instruction: &self.columns[12],
                Argument: &self.columns[13],
            },
            StoryParamsElement {
                Instruction: &self.columns[14],
                Argument: &self.columns[15],
            },
            StoryParamsElement {
                Instruction: &self.columns[16],
                Argument: &self.columns[17],
            },
            StoryParamsElement {
                Instruction: &self.columns[18],
                Argument: &self.columns[19],
            },
            StoryParamsElement {
                Instruction: &self.columns[20],
                Argument: &self.columns[21],
            },
            StoryParamsElement {
                Instruction: &self.columns[22],
                Argument: &self.columns[23],
            },
            StoryParamsElement {
                Instruction: &self.columns[24],
                Argument: &self.columns[25],
            },
            StoryParamsElement {
                Instruction: &self.columns[26],
                Argument: &self.columns[27],
            },
            StoryParamsElement {
                Instruction: &self.columns[28],
                Argument: &self.columns[29],
            },
            StoryParamsElement {
                Instruction: &self.columns[30],
                Argument: &self.columns[31],
            },
            StoryParamsElement {
                Instruction: &self.columns[32],
                Argument: &self.columns[33],
            },
            StoryParamsElement {
                Instruction: &self.columns[34],
                Argument: &self.columns[35],
            },
            StoryParamsElement {
                Instruction: &self.columns[36],
                Argument: &self.columns[37],
            },
            StoryParamsElement {
                Instruction: &self.columns[38],
                Argument: &self.columns[39],
            },
            StoryParamsElement {
                Instruction: &self.columns[40],
                Argument: &self.columns[41],
            },
            StoryParamsElement {
                Instruction: &self.columns[42],
                Argument: &self.columns[43],
            },
            StoryParamsElement {
                Instruction: &self.columns[44],
                Argument: &self.columns[45],
            },
            StoryParamsElement {
                Instruction: &self.columns[46],
                Argument: &self.columns[47],
            },
            StoryParamsElement {
                Instruction: &self.columns[48],
                Argument: &self.columns[49],
            },
            StoryParamsElement {
                Instruction: &self.columns[50],
                Argument: &self.columns[51],
            },
            StoryParamsElement {
                Instruction: &self.columns[52],
                Argument: &self.columns[53],
            },
            StoryParamsElement {
                Instruction: &self.columns[54],
                Argument: &self.columns[55],
            },
            StoryParamsElement {
                Instruction: &self.columns[56],
                Argument: &self.columns[57],
            },
            StoryParamsElement {
                Instruction: &self.columns[58],
                Argument: &self.columns[59],
            },
            StoryParamsElement {
                Instruction: &self.columns[60],
                Argument: &self.columns[61],
            },
            StoryParamsElement {
                Instruction: &self.columns[62],
                Argument: &self.columns[63],
            },
            StoryParamsElement {
                Instruction: &self.columns[64],
                Argument: &self.columns[65],
            },
            StoryParamsElement {
                Instruction: &self.columns[66],
                Argument: &self.columns[67],
            },
            StoryParamsElement {
                Instruction: &self.columns[68],
                Argument: &self.columns[69],
            },
            StoryParamsElement {
                Instruction: &self.columns[70],
                Argument: &self.columns[71],
            },
            StoryParamsElement {
                Instruction: &self.columns[72],
                Argument: &self.columns[73],
            },
            StoryParamsElement {
                Instruction: &self.columns[74],
                Argument: &self.columns[75],
            },
            StoryParamsElement {
                Instruction: &self.columns[76],
                Argument: &self.columns[77],
            },
            StoryParamsElement {
                Instruction: &self.columns[78],
                Argument: &self.columns[79],
            },
        ]
    }
    pub fn StoryDefine<'a>(&'a self) -> [StoryDefineElement<'a>; 110] {
        [
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[80],
                    &self.columns[81],
                    &self.columns[82],
                ],
                AcceptedQuest: [&self.columns[83], &self.columns[84], &self.columns[85]],
                LayerSet: [&self.columns[86], &self.columns[87]],
                Sequence: &self.columns[88],
                CompletedQuestOperator: &self.columns[89],
                AcceptedQuestOperator: &self.columns[90],
                AcceptedQuestSequence: [
                    &self.columns[91],
                    &self.columns[92],
                    &self.columns[93],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[94],
                    &self.columns[95],
                    &self.columns[96],
                ],
                AcceptedQuest: [&self.columns[97], &self.columns[98], &self.columns[99]],
                LayerSet: [&self.columns[100], &self.columns[101]],
                Sequence: &self.columns[102],
                CompletedQuestOperator: &self.columns[103],
                AcceptedQuestOperator: &self.columns[104],
                AcceptedQuestSequence: [
                    &self.columns[105],
                    &self.columns[106],
                    &self.columns[107],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[108],
                    &self.columns[109],
                    &self.columns[110],
                ],
                AcceptedQuest: [
                    &self.columns[111],
                    &self.columns[112],
                    &self.columns[113],
                ],
                LayerSet: [&self.columns[114], &self.columns[115]],
                Sequence: &self.columns[116],
                CompletedQuestOperator: &self.columns[117],
                AcceptedQuestOperator: &self.columns[118],
                AcceptedQuestSequence: [
                    &self.columns[119],
                    &self.columns[120],
                    &self.columns[121],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[122],
                    &self.columns[123],
                    &self.columns[124],
                ],
                AcceptedQuest: [
                    &self.columns[125],
                    &self.columns[126],
                    &self.columns[127],
                ],
                LayerSet: [&self.columns[128], &self.columns[129]],
                Sequence: &self.columns[130],
                CompletedQuestOperator: &self.columns[131],
                AcceptedQuestOperator: &self.columns[132],
                AcceptedQuestSequence: [
                    &self.columns[133],
                    &self.columns[134],
                    &self.columns[135],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[136],
                    &self.columns[137],
                    &self.columns[138],
                ],
                AcceptedQuest: [
                    &self.columns[139],
                    &self.columns[140],
                    &self.columns[141],
                ],
                LayerSet: [&self.columns[142], &self.columns[143]],
                Sequence: &self.columns[144],
                CompletedQuestOperator: &self.columns[145],
                AcceptedQuestOperator: &self.columns[146],
                AcceptedQuestSequence: [
                    &self.columns[147],
                    &self.columns[148],
                    &self.columns[149],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[150],
                    &self.columns[151],
                    &self.columns[152],
                ],
                AcceptedQuest: [
                    &self.columns[153],
                    &self.columns[154],
                    &self.columns[155],
                ],
                LayerSet: [&self.columns[156], &self.columns[157]],
                Sequence: &self.columns[158],
                CompletedQuestOperator: &self.columns[159],
                AcceptedQuestOperator: &self.columns[160],
                AcceptedQuestSequence: [
                    &self.columns[161],
                    &self.columns[162],
                    &self.columns[163],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[164],
                    &self.columns[165],
                    &self.columns[166],
                ],
                AcceptedQuest: [
                    &self.columns[167],
                    &self.columns[168],
                    &self.columns[169],
                ],
                LayerSet: [&self.columns[170], &self.columns[171]],
                Sequence: &self.columns[172],
                CompletedQuestOperator: &self.columns[173],
                AcceptedQuestOperator: &self.columns[174],
                AcceptedQuestSequence: [
                    &self.columns[175],
                    &self.columns[176],
                    &self.columns[177],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[178],
                    &self.columns[179],
                    &self.columns[180],
                ],
                AcceptedQuest: [
                    &self.columns[181],
                    &self.columns[182],
                    &self.columns[183],
                ],
                LayerSet: [&self.columns[184], &self.columns[185]],
                Sequence: &self.columns[186],
                CompletedQuestOperator: &self.columns[187],
                AcceptedQuestOperator: &self.columns[188],
                AcceptedQuestSequence: [
                    &self.columns[189],
                    &self.columns[190],
                    &self.columns[191],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[192],
                    &self.columns[193],
                    &self.columns[194],
                ],
                AcceptedQuest: [
                    &self.columns[195],
                    &self.columns[196],
                    &self.columns[197],
                ],
                LayerSet: [&self.columns[198], &self.columns[199]],
                Sequence: &self.columns[200],
                CompletedQuestOperator: &self.columns[201],
                AcceptedQuestOperator: &self.columns[202],
                AcceptedQuestSequence: [
                    &self.columns[203],
                    &self.columns[204],
                    &self.columns[205],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[206],
                    &self.columns[207],
                    &self.columns[208],
                ],
                AcceptedQuest: [
                    &self.columns[209],
                    &self.columns[210],
                    &self.columns[211],
                ],
                LayerSet: [&self.columns[212], &self.columns[213]],
                Sequence: &self.columns[214],
                CompletedQuestOperator: &self.columns[215],
                AcceptedQuestOperator: &self.columns[216],
                AcceptedQuestSequence: [
                    &self.columns[217],
                    &self.columns[218],
                    &self.columns[219],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[220],
                    &self.columns[221],
                    &self.columns[222],
                ],
                AcceptedQuest: [
                    &self.columns[223],
                    &self.columns[224],
                    &self.columns[225],
                ],
                LayerSet: [&self.columns[226], &self.columns[227]],
                Sequence: &self.columns[228],
                CompletedQuestOperator: &self.columns[229],
                AcceptedQuestOperator: &self.columns[230],
                AcceptedQuestSequence: [
                    &self.columns[231],
                    &self.columns[232],
                    &self.columns[233],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[234],
                    &self.columns[235],
                    &self.columns[236],
                ],
                AcceptedQuest: [
                    &self.columns[237],
                    &self.columns[238],
                    &self.columns[239],
                ],
                LayerSet: [&self.columns[240], &self.columns[241]],
                Sequence: &self.columns[242],
                CompletedQuestOperator: &self.columns[243],
                AcceptedQuestOperator: &self.columns[244],
                AcceptedQuestSequence: [
                    &self.columns[245],
                    &self.columns[246],
                    &self.columns[247],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[248],
                    &self.columns[249],
                    &self.columns[250],
                ],
                AcceptedQuest: [
                    &self.columns[251],
                    &self.columns[252],
                    &self.columns[253],
                ],
                LayerSet: [&self.columns[254], &self.columns[255]],
                Sequence: &self.columns[256],
                CompletedQuestOperator: &self.columns[257],
                AcceptedQuestOperator: &self.columns[258],
                AcceptedQuestSequence: [
                    &self.columns[259],
                    &self.columns[260],
                    &self.columns[261],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[262],
                    &self.columns[263],
                    &self.columns[264],
                ],
                AcceptedQuest: [
                    &self.columns[265],
                    &self.columns[266],
                    &self.columns[267],
                ],
                LayerSet: [&self.columns[268], &self.columns[269]],
                Sequence: &self.columns[270],
                CompletedQuestOperator: &self.columns[271],
                AcceptedQuestOperator: &self.columns[272],
                AcceptedQuestSequence: [
                    &self.columns[273],
                    &self.columns[274],
                    &self.columns[275],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[276],
                    &self.columns[277],
                    &self.columns[278],
                ],
                AcceptedQuest: [
                    &self.columns[279],
                    &self.columns[280],
                    &self.columns[281],
                ],
                LayerSet: [&self.columns[282], &self.columns[283]],
                Sequence: &self.columns[284],
                CompletedQuestOperator: &self.columns[285],
                AcceptedQuestOperator: &self.columns[286],
                AcceptedQuestSequence: [
                    &self.columns[287],
                    &self.columns[288],
                    &self.columns[289],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[290],
                    &self.columns[291],
                    &self.columns[292],
                ],
                AcceptedQuest: [
                    &self.columns[293],
                    &self.columns[294],
                    &self.columns[295],
                ],
                LayerSet: [&self.columns[296], &self.columns[297]],
                Sequence: &self.columns[298],
                CompletedQuestOperator: &self.columns[299],
                AcceptedQuestOperator: &self.columns[300],
                AcceptedQuestSequence: [
                    &self.columns[301],
                    &self.columns[302],
                    &self.columns[303],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[304],
                    &self.columns[305],
                    &self.columns[306],
                ],
                AcceptedQuest: [
                    &self.columns[307],
                    &self.columns[308],
                    &self.columns[309],
                ],
                LayerSet: [&self.columns[310], &self.columns[311]],
                Sequence: &self.columns[312],
                CompletedQuestOperator: &self.columns[313],
                AcceptedQuestOperator: &self.columns[314],
                AcceptedQuestSequence: [
                    &self.columns[315],
                    &self.columns[316],
                    &self.columns[317],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[318],
                    &self.columns[319],
                    &self.columns[320],
                ],
                AcceptedQuest: [
                    &self.columns[321],
                    &self.columns[322],
                    &self.columns[323],
                ],
                LayerSet: [&self.columns[324], &self.columns[325]],
                Sequence: &self.columns[326],
                CompletedQuestOperator: &self.columns[327],
                AcceptedQuestOperator: &self.columns[328],
                AcceptedQuestSequence: [
                    &self.columns[329],
                    &self.columns[330],
                    &self.columns[331],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[332],
                    &self.columns[333],
                    &self.columns[334],
                ],
                AcceptedQuest: [
                    &self.columns[335],
                    &self.columns[336],
                    &self.columns[337],
                ],
                LayerSet: [&self.columns[338], &self.columns[339]],
                Sequence: &self.columns[340],
                CompletedQuestOperator: &self.columns[341],
                AcceptedQuestOperator: &self.columns[342],
                AcceptedQuestSequence: [
                    &self.columns[343],
                    &self.columns[344],
                    &self.columns[345],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[346],
                    &self.columns[347],
                    &self.columns[348],
                ],
                AcceptedQuest: [
                    &self.columns[349],
                    &self.columns[350],
                    &self.columns[351],
                ],
                LayerSet: [&self.columns[352], &self.columns[353]],
                Sequence: &self.columns[354],
                CompletedQuestOperator: &self.columns[355],
                AcceptedQuestOperator: &self.columns[356],
                AcceptedQuestSequence: [
                    &self.columns[357],
                    &self.columns[358],
                    &self.columns[359],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[360],
                    &self.columns[361],
                    &self.columns[362],
                ],
                AcceptedQuest: [
                    &self.columns[363],
                    &self.columns[364],
                    &self.columns[365],
                ],
                LayerSet: [&self.columns[366], &self.columns[367]],
                Sequence: &self.columns[368],
                CompletedQuestOperator: &self.columns[369],
                AcceptedQuestOperator: &self.columns[370],
                AcceptedQuestSequence: [
                    &self.columns[371],
                    &self.columns[372],
                    &self.columns[373],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[374],
                    &self.columns[375],
                    &self.columns[376],
                ],
                AcceptedQuest: [
                    &self.columns[377],
                    &self.columns[378],
                    &self.columns[379],
                ],
                LayerSet: [&self.columns[380], &self.columns[381]],
                Sequence: &self.columns[382],
                CompletedQuestOperator: &self.columns[383],
                AcceptedQuestOperator: &self.columns[384],
                AcceptedQuestSequence: [
                    &self.columns[385],
                    &self.columns[386],
                    &self.columns[387],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[388],
                    &self.columns[389],
                    &self.columns[390],
                ],
                AcceptedQuest: [
                    &self.columns[391],
                    &self.columns[392],
                    &self.columns[393],
                ],
                LayerSet: [&self.columns[394], &self.columns[395]],
                Sequence: &self.columns[396],
                CompletedQuestOperator: &self.columns[397],
                AcceptedQuestOperator: &self.columns[398],
                AcceptedQuestSequence: [
                    &self.columns[399],
                    &self.columns[400],
                    &self.columns[401],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[402],
                    &self.columns[403],
                    &self.columns[404],
                ],
                AcceptedQuest: [
                    &self.columns[405],
                    &self.columns[406],
                    &self.columns[407],
                ],
                LayerSet: [&self.columns[408], &self.columns[409]],
                Sequence: &self.columns[410],
                CompletedQuestOperator: &self.columns[411],
                AcceptedQuestOperator: &self.columns[412],
                AcceptedQuestSequence: [
                    &self.columns[413],
                    &self.columns[414],
                    &self.columns[415],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[416],
                    &self.columns[417],
                    &self.columns[418],
                ],
                AcceptedQuest: [
                    &self.columns[419],
                    &self.columns[420],
                    &self.columns[421],
                ],
                LayerSet: [&self.columns[422], &self.columns[423]],
                Sequence: &self.columns[424],
                CompletedQuestOperator: &self.columns[425],
                AcceptedQuestOperator: &self.columns[426],
                AcceptedQuestSequence: [
                    &self.columns[427],
                    &self.columns[428],
                    &self.columns[429],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[430],
                    &self.columns[431],
                    &self.columns[432],
                ],
                AcceptedQuest: [
                    &self.columns[433],
                    &self.columns[434],
                    &self.columns[435],
                ],
                LayerSet: [&self.columns[436], &self.columns[437]],
                Sequence: &self.columns[438],
                CompletedQuestOperator: &self.columns[439],
                AcceptedQuestOperator: &self.columns[440],
                AcceptedQuestSequence: [
                    &self.columns[441],
                    &self.columns[442],
                    &self.columns[443],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[444],
                    &self.columns[445],
                    &self.columns[446],
                ],
                AcceptedQuest: [
                    &self.columns[447],
                    &self.columns[448],
                    &self.columns[449],
                ],
                LayerSet: [&self.columns[450], &self.columns[451]],
                Sequence: &self.columns[452],
                CompletedQuestOperator: &self.columns[453],
                AcceptedQuestOperator: &self.columns[454],
                AcceptedQuestSequence: [
                    &self.columns[455],
                    &self.columns[456],
                    &self.columns[457],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[458],
                    &self.columns[459],
                    &self.columns[460],
                ],
                AcceptedQuest: [
                    &self.columns[461],
                    &self.columns[462],
                    &self.columns[463],
                ],
                LayerSet: [&self.columns[464], &self.columns[465]],
                Sequence: &self.columns[466],
                CompletedQuestOperator: &self.columns[467],
                AcceptedQuestOperator: &self.columns[468],
                AcceptedQuestSequence: [
                    &self.columns[469],
                    &self.columns[470],
                    &self.columns[471],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[472],
                    &self.columns[473],
                    &self.columns[474],
                ],
                AcceptedQuest: [
                    &self.columns[475],
                    &self.columns[476],
                    &self.columns[477],
                ],
                LayerSet: [&self.columns[478], &self.columns[479]],
                Sequence: &self.columns[480],
                CompletedQuestOperator: &self.columns[481],
                AcceptedQuestOperator: &self.columns[482],
                AcceptedQuestSequence: [
                    &self.columns[483],
                    &self.columns[484],
                    &self.columns[485],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[486],
                    &self.columns[487],
                    &self.columns[488],
                ],
                AcceptedQuest: [
                    &self.columns[489],
                    &self.columns[490],
                    &self.columns[491],
                ],
                LayerSet: [&self.columns[492], &self.columns[493]],
                Sequence: &self.columns[494],
                CompletedQuestOperator: &self.columns[495],
                AcceptedQuestOperator: &self.columns[496],
                AcceptedQuestSequence: [
                    &self.columns[497],
                    &self.columns[498],
                    &self.columns[499],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[500],
                    &self.columns[501],
                    &self.columns[502],
                ],
                AcceptedQuest: [
                    &self.columns[503],
                    &self.columns[504],
                    &self.columns[505],
                ],
                LayerSet: [&self.columns[506], &self.columns[507]],
                Sequence: &self.columns[508],
                CompletedQuestOperator: &self.columns[509],
                AcceptedQuestOperator: &self.columns[510],
                AcceptedQuestSequence: [
                    &self.columns[511],
                    &self.columns[512],
                    &self.columns[513],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[514],
                    &self.columns[515],
                    &self.columns[516],
                ],
                AcceptedQuest: [
                    &self.columns[517],
                    &self.columns[518],
                    &self.columns[519],
                ],
                LayerSet: [&self.columns[520], &self.columns[521]],
                Sequence: &self.columns[522],
                CompletedQuestOperator: &self.columns[523],
                AcceptedQuestOperator: &self.columns[524],
                AcceptedQuestSequence: [
                    &self.columns[525],
                    &self.columns[526],
                    &self.columns[527],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[528],
                    &self.columns[529],
                    &self.columns[530],
                ],
                AcceptedQuest: [
                    &self.columns[531],
                    &self.columns[532],
                    &self.columns[533],
                ],
                LayerSet: [&self.columns[534], &self.columns[535]],
                Sequence: &self.columns[536],
                CompletedQuestOperator: &self.columns[537],
                AcceptedQuestOperator: &self.columns[538],
                AcceptedQuestSequence: [
                    &self.columns[539],
                    &self.columns[540],
                    &self.columns[541],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[542],
                    &self.columns[543],
                    &self.columns[544],
                ],
                AcceptedQuest: [
                    &self.columns[545],
                    &self.columns[546],
                    &self.columns[547],
                ],
                LayerSet: [&self.columns[548], &self.columns[549]],
                Sequence: &self.columns[550],
                CompletedQuestOperator: &self.columns[551],
                AcceptedQuestOperator: &self.columns[552],
                AcceptedQuestSequence: [
                    &self.columns[553],
                    &self.columns[554],
                    &self.columns[555],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[556],
                    &self.columns[557],
                    &self.columns[558],
                ],
                AcceptedQuest: [
                    &self.columns[559],
                    &self.columns[560],
                    &self.columns[561],
                ],
                LayerSet: [&self.columns[562], &self.columns[563]],
                Sequence: &self.columns[564],
                CompletedQuestOperator: &self.columns[565],
                AcceptedQuestOperator: &self.columns[566],
                AcceptedQuestSequence: [
                    &self.columns[567],
                    &self.columns[568],
                    &self.columns[569],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[570],
                    &self.columns[571],
                    &self.columns[572],
                ],
                AcceptedQuest: [
                    &self.columns[573],
                    &self.columns[574],
                    &self.columns[575],
                ],
                LayerSet: [&self.columns[576], &self.columns[577]],
                Sequence: &self.columns[578],
                CompletedQuestOperator: &self.columns[579],
                AcceptedQuestOperator: &self.columns[580],
                AcceptedQuestSequence: [
                    &self.columns[581],
                    &self.columns[582],
                    &self.columns[583],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[584],
                    &self.columns[585],
                    &self.columns[586],
                ],
                AcceptedQuest: [
                    &self.columns[587],
                    &self.columns[588],
                    &self.columns[589],
                ],
                LayerSet: [&self.columns[590], &self.columns[591]],
                Sequence: &self.columns[592],
                CompletedQuestOperator: &self.columns[593],
                AcceptedQuestOperator: &self.columns[594],
                AcceptedQuestSequence: [
                    &self.columns[595],
                    &self.columns[596],
                    &self.columns[597],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[598],
                    &self.columns[599],
                    &self.columns[600],
                ],
                AcceptedQuest: [
                    &self.columns[601],
                    &self.columns[602],
                    &self.columns[603],
                ],
                LayerSet: [&self.columns[604], &self.columns[605]],
                Sequence: &self.columns[606],
                CompletedQuestOperator: &self.columns[607],
                AcceptedQuestOperator: &self.columns[608],
                AcceptedQuestSequence: [
                    &self.columns[609],
                    &self.columns[610],
                    &self.columns[611],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[612],
                    &self.columns[613],
                    &self.columns[614],
                ],
                AcceptedQuest: [
                    &self.columns[615],
                    &self.columns[616],
                    &self.columns[617],
                ],
                LayerSet: [&self.columns[618], &self.columns[619]],
                Sequence: &self.columns[620],
                CompletedQuestOperator: &self.columns[621],
                AcceptedQuestOperator: &self.columns[622],
                AcceptedQuestSequence: [
                    &self.columns[623],
                    &self.columns[624],
                    &self.columns[625],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[626],
                    &self.columns[627],
                    &self.columns[628],
                ],
                AcceptedQuest: [
                    &self.columns[629],
                    &self.columns[630],
                    &self.columns[631],
                ],
                LayerSet: [&self.columns[632], &self.columns[633]],
                Sequence: &self.columns[634],
                CompletedQuestOperator: &self.columns[635],
                AcceptedQuestOperator: &self.columns[636],
                AcceptedQuestSequence: [
                    &self.columns[637],
                    &self.columns[638],
                    &self.columns[639],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[640],
                    &self.columns[641],
                    &self.columns[642],
                ],
                AcceptedQuest: [
                    &self.columns[643],
                    &self.columns[644],
                    &self.columns[645],
                ],
                LayerSet: [&self.columns[646], &self.columns[647]],
                Sequence: &self.columns[648],
                CompletedQuestOperator: &self.columns[649],
                AcceptedQuestOperator: &self.columns[650],
                AcceptedQuestSequence: [
                    &self.columns[651],
                    &self.columns[652],
                    &self.columns[653],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[654],
                    &self.columns[655],
                    &self.columns[656],
                ],
                AcceptedQuest: [
                    &self.columns[657],
                    &self.columns[658],
                    &self.columns[659],
                ],
                LayerSet: [&self.columns[660], &self.columns[661]],
                Sequence: &self.columns[662],
                CompletedQuestOperator: &self.columns[663],
                AcceptedQuestOperator: &self.columns[664],
                AcceptedQuestSequence: [
                    &self.columns[665],
                    &self.columns[666],
                    &self.columns[667],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[668],
                    &self.columns[669],
                    &self.columns[670],
                ],
                AcceptedQuest: [
                    &self.columns[671],
                    &self.columns[672],
                    &self.columns[673],
                ],
                LayerSet: [&self.columns[674], &self.columns[675]],
                Sequence: &self.columns[676],
                CompletedQuestOperator: &self.columns[677],
                AcceptedQuestOperator: &self.columns[678],
                AcceptedQuestSequence: [
                    &self.columns[679],
                    &self.columns[680],
                    &self.columns[681],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[682],
                    &self.columns[683],
                    &self.columns[684],
                ],
                AcceptedQuest: [
                    &self.columns[685],
                    &self.columns[686],
                    &self.columns[687],
                ],
                LayerSet: [&self.columns[688], &self.columns[689]],
                Sequence: &self.columns[690],
                CompletedQuestOperator: &self.columns[691],
                AcceptedQuestOperator: &self.columns[692],
                AcceptedQuestSequence: [
                    &self.columns[693],
                    &self.columns[694],
                    &self.columns[695],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[696],
                    &self.columns[697],
                    &self.columns[698],
                ],
                AcceptedQuest: [
                    &self.columns[699],
                    &self.columns[700],
                    &self.columns[701],
                ],
                LayerSet: [&self.columns[702], &self.columns[703]],
                Sequence: &self.columns[704],
                CompletedQuestOperator: &self.columns[705],
                AcceptedQuestOperator: &self.columns[706],
                AcceptedQuestSequence: [
                    &self.columns[707],
                    &self.columns[708],
                    &self.columns[709],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[710],
                    &self.columns[711],
                    &self.columns[712],
                ],
                AcceptedQuest: [
                    &self.columns[713],
                    &self.columns[714],
                    &self.columns[715],
                ],
                LayerSet: [&self.columns[716], &self.columns[717]],
                Sequence: &self.columns[718],
                CompletedQuestOperator: &self.columns[719],
                AcceptedQuestOperator: &self.columns[720],
                AcceptedQuestSequence: [
                    &self.columns[721],
                    &self.columns[722],
                    &self.columns[723],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[724],
                    &self.columns[725],
                    &self.columns[726],
                ],
                AcceptedQuest: [
                    &self.columns[727],
                    &self.columns[728],
                    &self.columns[729],
                ],
                LayerSet: [&self.columns[730], &self.columns[731]],
                Sequence: &self.columns[732],
                CompletedQuestOperator: &self.columns[733],
                AcceptedQuestOperator: &self.columns[734],
                AcceptedQuestSequence: [
                    &self.columns[735],
                    &self.columns[736],
                    &self.columns[737],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[738],
                    &self.columns[739],
                    &self.columns[740],
                ],
                AcceptedQuest: [
                    &self.columns[741],
                    &self.columns[742],
                    &self.columns[743],
                ],
                LayerSet: [&self.columns[744], &self.columns[745]],
                Sequence: &self.columns[746],
                CompletedQuestOperator: &self.columns[747],
                AcceptedQuestOperator: &self.columns[748],
                AcceptedQuestSequence: [
                    &self.columns[749],
                    &self.columns[750],
                    &self.columns[751],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[752],
                    &self.columns[753],
                    &self.columns[754],
                ],
                AcceptedQuest: [
                    &self.columns[755],
                    &self.columns[756],
                    &self.columns[757],
                ],
                LayerSet: [&self.columns[758], &self.columns[759]],
                Sequence: &self.columns[760],
                CompletedQuestOperator: &self.columns[761],
                AcceptedQuestOperator: &self.columns[762],
                AcceptedQuestSequence: [
                    &self.columns[763],
                    &self.columns[764],
                    &self.columns[765],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[766],
                    &self.columns[767],
                    &self.columns[768],
                ],
                AcceptedQuest: [
                    &self.columns[769],
                    &self.columns[770],
                    &self.columns[771],
                ],
                LayerSet: [&self.columns[772], &self.columns[773]],
                Sequence: &self.columns[774],
                CompletedQuestOperator: &self.columns[775],
                AcceptedQuestOperator: &self.columns[776],
                AcceptedQuestSequence: [
                    &self.columns[777],
                    &self.columns[778],
                    &self.columns[779],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[780],
                    &self.columns[781],
                    &self.columns[782],
                ],
                AcceptedQuest: [
                    &self.columns[783],
                    &self.columns[784],
                    &self.columns[785],
                ],
                LayerSet: [&self.columns[786], &self.columns[787]],
                Sequence: &self.columns[788],
                CompletedQuestOperator: &self.columns[789],
                AcceptedQuestOperator: &self.columns[790],
                AcceptedQuestSequence: [
                    &self.columns[791],
                    &self.columns[792],
                    &self.columns[793],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[794],
                    &self.columns[795],
                    &self.columns[796],
                ],
                AcceptedQuest: [
                    &self.columns[797],
                    &self.columns[798],
                    &self.columns[799],
                ],
                LayerSet: [&self.columns[800], &self.columns[801]],
                Sequence: &self.columns[802],
                CompletedQuestOperator: &self.columns[803],
                AcceptedQuestOperator: &self.columns[804],
                AcceptedQuestSequence: [
                    &self.columns[805],
                    &self.columns[806],
                    &self.columns[807],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[808],
                    &self.columns[809],
                    &self.columns[810],
                ],
                AcceptedQuest: [
                    &self.columns[811],
                    &self.columns[812],
                    &self.columns[813],
                ],
                LayerSet: [&self.columns[814], &self.columns[815]],
                Sequence: &self.columns[816],
                CompletedQuestOperator: &self.columns[817],
                AcceptedQuestOperator: &self.columns[818],
                AcceptedQuestSequence: [
                    &self.columns[819],
                    &self.columns[820],
                    &self.columns[821],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[822],
                    &self.columns[823],
                    &self.columns[824],
                ],
                AcceptedQuest: [
                    &self.columns[825],
                    &self.columns[826],
                    &self.columns[827],
                ],
                LayerSet: [&self.columns[828], &self.columns[829]],
                Sequence: &self.columns[830],
                CompletedQuestOperator: &self.columns[831],
                AcceptedQuestOperator: &self.columns[832],
                AcceptedQuestSequence: [
                    &self.columns[833],
                    &self.columns[834],
                    &self.columns[835],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[836],
                    &self.columns[837],
                    &self.columns[838],
                ],
                AcceptedQuest: [
                    &self.columns[839],
                    &self.columns[840],
                    &self.columns[841],
                ],
                LayerSet: [&self.columns[842], &self.columns[843]],
                Sequence: &self.columns[844],
                CompletedQuestOperator: &self.columns[845],
                AcceptedQuestOperator: &self.columns[846],
                AcceptedQuestSequence: [
                    &self.columns[847],
                    &self.columns[848],
                    &self.columns[849],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[850],
                    &self.columns[851],
                    &self.columns[852],
                ],
                AcceptedQuest: [
                    &self.columns[853],
                    &self.columns[854],
                    &self.columns[855],
                ],
                LayerSet: [&self.columns[856], &self.columns[857]],
                Sequence: &self.columns[858],
                CompletedQuestOperator: &self.columns[859],
                AcceptedQuestOperator: &self.columns[860],
                AcceptedQuestSequence: [
                    &self.columns[861],
                    &self.columns[862],
                    &self.columns[863],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[864],
                    &self.columns[865],
                    &self.columns[866],
                ],
                AcceptedQuest: [
                    &self.columns[867],
                    &self.columns[868],
                    &self.columns[869],
                ],
                LayerSet: [&self.columns[870], &self.columns[871]],
                Sequence: &self.columns[872],
                CompletedQuestOperator: &self.columns[873],
                AcceptedQuestOperator: &self.columns[874],
                AcceptedQuestSequence: [
                    &self.columns[875],
                    &self.columns[876],
                    &self.columns[877],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[878],
                    &self.columns[879],
                    &self.columns[880],
                ],
                AcceptedQuest: [
                    &self.columns[881],
                    &self.columns[882],
                    &self.columns[883],
                ],
                LayerSet: [&self.columns[884], &self.columns[885]],
                Sequence: &self.columns[886],
                CompletedQuestOperator: &self.columns[887],
                AcceptedQuestOperator: &self.columns[888],
                AcceptedQuestSequence: [
                    &self.columns[889],
                    &self.columns[890],
                    &self.columns[891],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[892],
                    &self.columns[893],
                    &self.columns[894],
                ],
                AcceptedQuest: [
                    &self.columns[895],
                    &self.columns[896],
                    &self.columns[897],
                ],
                LayerSet: [&self.columns[898], &self.columns[899]],
                Sequence: &self.columns[900],
                CompletedQuestOperator: &self.columns[901],
                AcceptedQuestOperator: &self.columns[902],
                AcceptedQuestSequence: [
                    &self.columns[903],
                    &self.columns[904],
                    &self.columns[905],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[906],
                    &self.columns[907],
                    &self.columns[908],
                ],
                AcceptedQuest: [
                    &self.columns[909],
                    &self.columns[910],
                    &self.columns[911],
                ],
                LayerSet: [&self.columns[912], &self.columns[913]],
                Sequence: &self.columns[914],
                CompletedQuestOperator: &self.columns[915],
                AcceptedQuestOperator: &self.columns[916],
                AcceptedQuestSequence: [
                    &self.columns[917],
                    &self.columns[918],
                    &self.columns[919],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[920],
                    &self.columns[921],
                    &self.columns[922],
                ],
                AcceptedQuest: [
                    &self.columns[923],
                    &self.columns[924],
                    &self.columns[925],
                ],
                LayerSet: [&self.columns[926], &self.columns[927]],
                Sequence: &self.columns[928],
                CompletedQuestOperator: &self.columns[929],
                AcceptedQuestOperator: &self.columns[930],
                AcceptedQuestSequence: [
                    &self.columns[931],
                    &self.columns[932],
                    &self.columns[933],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[934],
                    &self.columns[935],
                    &self.columns[936],
                ],
                AcceptedQuest: [
                    &self.columns[937],
                    &self.columns[938],
                    &self.columns[939],
                ],
                LayerSet: [&self.columns[940], &self.columns[941]],
                Sequence: &self.columns[942],
                CompletedQuestOperator: &self.columns[943],
                AcceptedQuestOperator: &self.columns[944],
                AcceptedQuestSequence: [
                    &self.columns[945],
                    &self.columns[946],
                    &self.columns[947],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[948],
                    &self.columns[949],
                    &self.columns[950],
                ],
                AcceptedQuest: [
                    &self.columns[951],
                    &self.columns[952],
                    &self.columns[953],
                ],
                LayerSet: [&self.columns[954], &self.columns[955]],
                Sequence: &self.columns[956],
                CompletedQuestOperator: &self.columns[957],
                AcceptedQuestOperator: &self.columns[958],
                AcceptedQuestSequence: [
                    &self.columns[959],
                    &self.columns[960],
                    &self.columns[961],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[962],
                    &self.columns[963],
                    &self.columns[964],
                ],
                AcceptedQuest: [
                    &self.columns[965],
                    &self.columns[966],
                    &self.columns[967],
                ],
                LayerSet: [&self.columns[968], &self.columns[969]],
                Sequence: &self.columns[970],
                CompletedQuestOperator: &self.columns[971],
                AcceptedQuestOperator: &self.columns[972],
                AcceptedQuestSequence: [
                    &self.columns[973],
                    &self.columns[974],
                    &self.columns[975],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[976],
                    &self.columns[977],
                    &self.columns[978],
                ],
                AcceptedQuest: [
                    &self.columns[979],
                    &self.columns[980],
                    &self.columns[981],
                ],
                LayerSet: [&self.columns[982], &self.columns[983]],
                Sequence: &self.columns[984],
                CompletedQuestOperator: &self.columns[985],
                AcceptedQuestOperator: &self.columns[986],
                AcceptedQuestSequence: [
                    &self.columns[987],
                    &self.columns[988],
                    &self.columns[989],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[990],
                    &self.columns[991],
                    &self.columns[992],
                ],
                AcceptedQuest: [
                    &self.columns[993],
                    &self.columns[994],
                    &self.columns[995],
                ],
                LayerSet: [&self.columns[996], &self.columns[997]],
                Sequence: &self.columns[998],
                CompletedQuestOperator: &self.columns[999],
                AcceptedQuestOperator: &self.columns[1000],
                AcceptedQuestSequence: [
                    &self.columns[1001],
                    &self.columns[1002],
                    &self.columns[1003],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1004],
                    &self.columns[1005],
                    &self.columns[1006],
                ],
                AcceptedQuest: [
                    &self.columns[1007],
                    &self.columns[1008],
                    &self.columns[1009],
                ],
                LayerSet: [&self.columns[1010], &self.columns[1011]],
                Sequence: &self.columns[1012],
                CompletedQuestOperator: &self.columns[1013],
                AcceptedQuestOperator: &self.columns[1014],
                AcceptedQuestSequence: [
                    &self.columns[1015],
                    &self.columns[1016],
                    &self.columns[1017],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1018],
                    &self.columns[1019],
                    &self.columns[1020],
                ],
                AcceptedQuest: [
                    &self.columns[1021],
                    &self.columns[1022],
                    &self.columns[1023],
                ],
                LayerSet: [&self.columns[1024], &self.columns[1025]],
                Sequence: &self.columns[1026],
                CompletedQuestOperator: &self.columns[1027],
                AcceptedQuestOperator: &self.columns[1028],
                AcceptedQuestSequence: [
                    &self.columns[1029],
                    &self.columns[1030],
                    &self.columns[1031],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1032],
                    &self.columns[1033],
                    &self.columns[1034],
                ],
                AcceptedQuest: [
                    &self.columns[1035],
                    &self.columns[1036],
                    &self.columns[1037],
                ],
                LayerSet: [&self.columns[1038], &self.columns[1039]],
                Sequence: &self.columns[1040],
                CompletedQuestOperator: &self.columns[1041],
                AcceptedQuestOperator: &self.columns[1042],
                AcceptedQuestSequence: [
                    &self.columns[1043],
                    &self.columns[1044],
                    &self.columns[1045],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1046],
                    &self.columns[1047],
                    &self.columns[1048],
                ],
                AcceptedQuest: [
                    &self.columns[1049],
                    &self.columns[1050],
                    &self.columns[1051],
                ],
                LayerSet: [&self.columns[1052], &self.columns[1053]],
                Sequence: &self.columns[1054],
                CompletedQuestOperator: &self.columns[1055],
                AcceptedQuestOperator: &self.columns[1056],
                AcceptedQuestSequence: [
                    &self.columns[1057],
                    &self.columns[1058],
                    &self.columns[1059],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1060],
                    &self.columns[1061],
                    &self.columns[1062],
                ],
                AcceptedQuest: [
                    &self.columns[1063],
                    &self.columns[1064],
                    &self.columns[1065],
                ],
                LayerSet: [&self.columns[1066], &self.columns[1067]],
                Sequence: &self.columns[1068],
                CompletedQuestOperator: &self.columns[1069],
                AcceptedQuestOperator: &self.columns[1070],
                AcceptedQuestSequence: [
                    &self.columns[1071],
                    &self.columns[1072],
                    &self.columns[1073],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1074],
                    &self.columns[1075],
                    &self.columns[1076],
                ],
                AcceptedQuest: [
                    &self.columns[1077],
                    &self.columns[1078],
                    &self.columns[1079],
                ],
                LayerSet: [&self.columns[1080], &self.columns[1081]],
                Sequence: &self.columns[1082],
                CompletedQuestOperator: &self.columns[1083],
                AcceptedQuestOperator: &self.columns[1084],
                AcceptedQuestSequence: [
                    &self.columns[1085],
                    &self.columns[1086],
                    &self.columns[1087],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1088],
                    &self.columns[1089],
                    &self.columns[1090],
                ],
                AcceptedQuest: [
                    &self.columns[1091],
                    &self.columns[1092],
                    &self.columns[1093],
                ],
                LayerSet: [&self.columns[1094], &self.columns[1095]],
                Sequence: &self.columns[1096],
                CompletedQuestOperator: &self.columns[1097],
                AcceptedQuestOperator: &self.columns[1098],
                AcceptedQuestSequence: [
                    &self.columns[1099],
                    &self.columns[1100],
                    &self.columns[1101],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1102],
                    &self.columns[1103],
                    &self.columns[1104],
                ],
                AcceptedQuest: [
                    &self.columns[1105],
                    &self.columns[1106],
                    &self.columns[1107],
                ],
                LayerSet: [&self.columns[1108], &self.columns[1109]],
                Sequence: &self.columns[1110],
                CompletedQuestOperator: &self.columns[1111],
                AcceptedQuestOperator: &self.columns[1112],
                AcceptedQuestSequence: [
                    &self.columns[1113],
                    &self.columns[1114],
                    &self.columns[1115],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1116],
                    &self.columns[1117],
                    &self.columns[1118],
                ],
                AcceptedQuest: [
                    &self.columns[1119],
                    &self.columns[1120],
                    &self.columns[1121],
                ],
                LayerSet: [&self.columns[1122], &self.columns[1123]],
                Sequence: &self.columns[1124],
                CompletedQuestOperator: &self.columns[1125],
                AcceptedQuestOperator: &self.columns[1126],
                AcceptedQuestSequence: [
                    &self.columns[1127],
                    &self.columns[1128],
                    &self.columns[1129],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1130],
                    &self.columns[1131],
                    &self.columns[1132],
                ],
                AcceptedQuest: [
                    &self.columns[1133],
                    &self.columns[1134],
                    &self.columns[1135],
                ],
                LayerSet: [&self.columns[1136], &self.columns[1137]],
                Sequence: &self.columns[1138],
                CompletedQuestOperator: &self.columns[1139],
                AcceptedQuestOperator: &self.columns[1140],
                AcceptedQuestSequence: [
                    &self.columns[1141],
                    &self.columns[1142],
                    &self.columns[1143],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1144],
                    &self.columns[1145],
                    &self.columns[1146],
                ],
                AcceptedQuest: [
                    &self.columns[1147],
                    &self.columns[1148],
                    &self.columns[1149],
                ],
                LayerSet: [&self.columns[1150], &self.columns[1151]],
                Sequence: &self.columns[1152],
                CompletedQuestOperator: &self.columns[1153],
                AcceptedQuestOperator: &self.columns[1154],
                AcceptedQuestSequence: [
                    &self.columns[1155],
                    &self.columns[1156],
                    &self.columns[1157],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1158],
                    &self.columns[1159],
                    &self.columns[1160],
                ],
                AcceptedQuest: [
                    &self.columns[1161],
                    &self.columns[1162],
                    &self.columns[1163],
                ],
                LayerSet: [&self.columns[1164], &self.columns[1165]],
                Sequence: &self.columns[1166],
                CompletedQuestOperator: &self.columns[1167],
                AcceptedQuestOperator: &self.columns[1168],
                AcceptedQuestSequence: [
                    &self.columns[1169],
                    &self.columns[1170],
                    &self.columns[1171],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1172],
                    &self.columns[1173],
                    &self.columns[1174],
                ],
                AcceptedQuest: [
                    &self.columns[1175],
                    &self.columns[1176],
                    &self.columns[1177],
                ],
                LayerSet: [&self.columns[1178], &self.columns[1179]],
                Sequence: &self.columns[1180],
                CompletedQuestOperator: &self.columns[1181],
                AcceptedQuestOperator: &self.columns[1182],
                AcceptedQuestSequence: [
                    &self.columns[1183],
                    &self.columns[1184],
                    &self.columns[1185],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1186],
                    &self.columns[1187],
                    &self.columns[1188],
                ],
                AcceptedQuest: [
                    &self.columns[1189],
                    &self.columns[1190],
                    &self.columns[1191],
                ],
                LayerSet: [&self.columns[1192], &self.columns[1193]],
                Sequence: &self.columns[1194],
                CompletedQuestOperator: &self.columns[1195],
                AcceptedQuestOperator: &self.columns[1196],
                AcceptedQuestSequence: [
                    &self.columns[1197],
                    &self.columns[1198],
                    &self.columns[1199],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1200],
                    &self.columns[1201],
                    &self.columns[1202],
                ],
                AcceptedQuest: [
                    &self.columns[1203],
                    &self.columns[1204],
                    &self.columns[1205],
                ],
                LayerSet: [&self.columns[1206], &self.columns[1207]],
                Sequence: &self.columns[1208],
                CompletedQuestOperator: &self.columns[1209],
                AcceptedQuestOperator: &self.columns[1210],
                AcceptedQuestSequence: [
                    &self.columns[1211],
                    &self.columns[1212],
                    &self.columns[1213],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1214],
                    &self.columns[1215],
                    &self.columns[1216],
                ],
                AcceptedQuest: [
                    &self.columns[1217],
                    &self.columns[1218],
                    &self.columns[1219],
                ],
                LayerSet: [&self.columns[1220], &self.columns[1221]],
                Sequence: &self.columns[1222],
                CompletedQuestOperator: &self.columns[1223],
                AcceptedQuestOperator: &self.columns[1224],
                AcceptedQuestSequence: [
                    &self.columns[1225],
                    &self.columns[1226],
                    &self.columns[1227],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1228],
                    &self.columns[1229],
                    &self.columns[1230],
                ],
                AcceptedQuest: [
                    &self.columns[1231],
                    &self.columns[1232],
                    &self.columns[1233],
                ],
                LayerSet: [&self.columns[1234], &self.columns[1235]],
                Sequence: &self.columns[1236],
                CompletedQuestOperator: &self.columns[1237],
                AcceptedQuestOperator: &self.columns[1238],
                AcceptedQuestSequence: [
                    &self.columns[1239],
                    &self.columns[1240],
                    &self.columns[1241],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1242],
                    &self.columns[1243],
                    &self.columns[1244],
                ],
                AcceptedQuest: [
                    &self.columns[1245],
                    &self.columns[1246],
                    &self.columns[1247],
                ],
                LayerSet: [&self.columns[1248], &self.columns[1249]],
                Sequence: &self.columns[1250],
                CompletedQuestOperator: &self.columns[1251],
                AcceptedQuestOperator: &self.columns[1252],
                AcceptedQuestSequence: [
                    &self.columns[1253],
                    &self.columns[1254],
                    &self.columns[1255],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1256],
                    &self.columns[1257],
                    &self.columns[1258],
                ],
                AcceptedQuest: [
                    &self.columns[1259],
                    &self.columns[1260],
                    &self.columns[1261],
                ],
                LayerSet: [&self.columns[1262], &self.columns[1263]],
                Sequence: &self.columns[1264],
                CompletedQuestOperator: &self.columns[1265],
                AcceptedQuestOperator: &self.columns[1266],
                AcceptedQuestSequence: [
                    &self.columns[1267],
                    &self.columns[1268],
                    &self.columns[1269],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1270],
                    &self.columns[1271],
                    &self.columns[1272],
                ],
                AcceptedQuest: [
                    &self.columns[1273],
                    &self.columns[1274],
                    &self.columns[1275],
                ],
                LayerSet: [&self.columns[1276], &self.columns[1277]],
                Sequence: &self.columns[1278],
                CompletedQuestOperator: &self.columns[1279],
                AcceptedQuestOperator: &self.columns[1280],
                AcceptedQuestSequence: [
                    &self.columns[1281],
                    &self.columns[1282],
                    &self.columns[1283],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1284],
                    &self.columns[1285],
                    &self.columns[1286],
                ],
                AcceptedQuest: [
                    &self.columns[1287],
                    &self.columns[1288],
                    &self.columns[1289],
                ],
                LayerSet: [&self.columns[1290], &self.columns[1291]],
                Sequence: &self.columns[1292],
                CompletedQuestOperator: &self.columns[1293],
                AcceptedQuestOperator: &self.columns[1294],
                AcceptedQuestSequence: [
                    &self.columns[1295],
                    &self.columns[1296],
                    &self.columns[1297],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1298],
                    &self.columns[1299],
                    &self.columns[1300],
                ],
                AcceptedQuest: [
                    &self.columns[1301],
                    &self.columns[1302],
                    &self.columns[1303],
                ],
                LayerSet: [&self.columns[1304], &self.columns[1305]],
                Sequence: &self.columns[1306],
                CompletedQuestOperator: &self.columns[1307],
                AcceptedQuestOperator: &self.columns[1308],
                AcceptedQuestSequence: [
                    &self.columns[1309],
                    &self.columns[1310],
                    &self.columns[1311],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1312],
                    &self.columns[1313],
                    &self.columns[1314],
                ],
                AcceptedQuest: [
                    &self.columns[1315],
                    &self.columns[1316],
                    &self.columns[1317],
                ],
                LayerSet: [&self.columns[1318], &self.columns[1319]],
                Sequence: &self.columns[1320],
                CompletedQuestOperator: &self.columns[1321],
                AcceptedQuestOperator: &self.columns[1322],
                AcceptedQuestSequence: [
                    &self.columns[1323],
                    &self.columns[1324],
                    &self.columns[1325],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1326],
                    &self.columns[1327],
                    &self.columns[1328],
                ],
                AcceptedQuest: [
                    &self.columns[1329],
                    &self.columns[1330],
                    &self.columns[1331],
                ],
                LayerSet: [&self.columns[1332], &self.columns[1333]],
                Sequence: &self.columns[1334],
                CompletedQuestOperator: &self.columns[1335],
                AcceptedQuestOperator: &self.columns[1336],
                AcceptedQuestSequence: [
                    &self.columns[1337],
                    &self.columns[1338],
                    &self.columns[1339],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1340],
                    &self.columns[1341],
                    &self.columns[1342],
                ],
                AcceptedQuest: [
                    &self.columns[1343],
                    &self.columns[1344],
                    &self.columns[1345],
                ],
                LayerSet: [&self.columns[1346], &self.columns[1347]],
                Sequence: &self.columns[1348],
                CompletedQuestOperator: &self.columns[1349],
                AcceptedQuestOperator: &self.columns[1350],
                AcceptedQuestSequence: [
                    &self.columns[1351],
                    &self.columns[1352],
                    &self.columns[1353],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1354],
                    &self.columns[1355],
                    &self.columns[1356],
                ],
                AcceptedQuest: [
                    &self.columns[1357],
                    &self.columns[1358],
                    &self.columns[1359],
                ],
                LayerSet: [&self.columns[1360], &self.columns[1361]],
                Sequence: &self.columns[1362],
                CompletedQuestOperator: &self.columns[1363],
                AcceptedQuestOperator: &self.columns[1364],
                AcceptedQuestSequence: [
                    &self.columns[1365],
                    &self.columns[1366],
                    &self.columns[1367],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1368],
                    &self.columns[1369],
                    &self.columns[1370],
                ],
                AcceptedQuest: [
                    &self.columns[1371],
                    &self.columns[1372],
                    &self.columns[1373],
                ],
                LayerSet: [&self.columns[1374], &self.columns[1375]],
                Sequence: &self.columns[1376],
                CompletedQuestOperator: &self.columns[1377],
                AcceptedQuestOperator: &self.columns[1378],
                AcceptedQuestSequence: [
                    &self.columns[1379],
                    &self.columns[1380],
                    &self.columns[1381],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1382],
                    &self.columns[1383],
                    &self.columns[1384],
                ],
                AcceptedQuest: [
                    &self.columns[1385],
                    &self.columns[1386],
                    &self.columns[1387],
                ],
                LayerSet: [&self.columns[1388], &self.columns[1389]],
                Sequence: &self.columns[1390],
                CompletedQuestOperator: &self.columns[1391],
                AcceptedQuestOperator: &self.columns[1392],
                AcceptedQuestSequence: [
                    &self.columns[1393],
                    &self.columns[1394],
                    &self.columns[1395],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1396],
                    &self.columns[1397],
                    &self.columns[1398],
                ],
                AcceptedQuest: [
                    &self.columns[1399],
                    &self.columns[1400],
                    &self.columns[1401],
                ],
                LayerSet: [&self.columns[1402], &self.columns[1403]],
                Sequence: &self.columns[1404],
                CompletedQuestOperator: &self.columns[1405],
                AcceptedQuestOperator: &self.columns[1406],
                AcceptedQuestSequence: [
                    &self.columns[1407],
                    &self.columns[1408],
                    &self.columns[1409],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1410],
                    &self.columns[1411],
                    &self.columns[1412],
                ],
                AcceptedQuest: [
                    &self.columns[1413],
                    &self.columns[1414],
                    &self.columns[1415],
                ],
                LayerSet: [&self.columns[1416], &self.columns[1417]],
                Sequence: &self.columns[1418],
                CompletedQuestOperator: &self.columns[1419],
                AcceptedQuestOperator: &self.columns[1420],
                AcceptedQuestSequence: [
                    &self.columns[1421],
                    &self.columns[1422],
                    &self.columns[1423],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1424],
                    &self.columns[1425],
                    &self.columns[1426],
                ],
                AcceptedQuest: [
                    &self.columns[1427],
                    &self.columns[1428],
                    &self.columns[1429],
                ],
                LayerSet: [&self.columns[1430], &self.columns[1431]],
                Sequence: &self.columns[1432],
                CompletedQuestOperator: &self.columns[1433],
                AcceptedQuestOperator: &self.columns[1434],
                AcceptedQuestSequence: [
                    &self.columns[1435],
                    &self.columns[1436],
                    &self.columns[1437],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1438],
                    &self.columns[1439],
                    &self.columns[1440],
                ],
                AcceptedQuest: [
                    &self.columns[1441],
                    &self.columns[1442],
                    &self.columns[1443],
                ],
                LayerSet: [&self.columns[1444], &self.columns[1445]],
                Sequence: &self.columns[1446],
                CompletedQuestOperator: &self.columns[1447],
                AcceptedQuestOperator: &self.columns[1448],
                AcceptedQuestSequence: [
                    &self.columns[1449],
                    &self.columns[1450],
                    &self.columns[1451],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1452],
                    &self.columns[1453],
                    &self.columns[1454],
                ],
                AcceptedQuest: [
                    &self.columns[1455],
                    &self.columns[1456],
                    &self.columns[1457],
                ],
                LayerSet: [&self.columns[1458], &self.columns[1459]],
                Sequence: &self.columns[1460],
                CompletedQuestOperator: &self.columns[1461],
                AcceptedQuestOperator: &self.columns[1462],
                AcceptedQuestSequence: [
                    &self.columns[1463],
                    &self.columns[1464],
                    &self.columns[1465],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1466],
                    &self.columns[1467],
                    &self.columns[1468],
                ],
                AcceptedQuest: [
                    &self.columns[1469],
                    &self.columns[1470],
                    &self.columns[1471],
                ],
                LayerSet: [&self.columns[1472], &self.columns[1473]],
                Sequence: &self.columns[1474],
                CompletedQuestOperator: &self.columns[1475],
                AcceptedQuestOperator: &self.columns[1476],
                AcceptedQuestSequence: [
                    &self.columns[1477],
                    &self.columns[1478],
                    &self.columns[1479],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1480],
                    &self.columns[1481],
                    &self.columns[1482],
                ],
                AcceptedQuest: [
                    &self.columns[1483],
                    &self.columns[1484],
                    &self.columns[1485],
                ],
                LayerSet: [&self.columns[1486], &self.columns[1487]],
                Sequence: &self.columns[1488],
                CompletedQuestOperator: &self.columns[1489],
                AcceptedQuestOperator: &self.columns[1490],
                AcceptedQuestSequence: [
                    &self.columns[1491],
                    &self.columns[1492],
                    &self.columns[1493],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1494],
                    &self.columns[1495],
                    &self.columns[1496],
                ],
                AcceptedQuest: [
                    &self.columns[1497],
                    &self.columns[1498],
                    &self.columns[1499],
                ],
                LayerSet: [&self.columns[1500], &self.columns[1501]],
                Sequence: &self.columns[1502],
                CompletedQuestOperator: &self.columns[1503],
                AcceptedQuestOperator: &self.columns[1504],
                AcceptedQuestSequence: [
                    &self.columns[1505],
                    &self.columns[1506],
                    &self.columns[1507],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1508],
                    &self.columns[1509],
                    &self.columns[1510],
                ],
                AcceptedQuest: [
                    &self.columns[1511],
                    &self.columns[1512],
                    &self.columns[1513],
                ],
                LayerSet: [&self.columns[1514], &self.columns[1515]],
                Sequence: &self.columns[1516],
                CompletedQuestOperator: &self.columns[1517],
                AcceptedQuestOperator: &self.columns[1518],
                AcceptedQuestSequence: [
                    &self.columns[1519],
                    &self.columns[1520],
                    &self.columns[1521],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1522],
                    &self.columns[1523],
                    &self.columns[1524],
                ],
                AcceptedQuest: [
                    &self.columns[1525],
                    &self.columns[1526],
                    &self.columns[1527],
                ],
                LayerSet: [&self.columns[1528], &self.columns[1529]],
                Sequence: &self.columns[1530],
                CompletedQuestOperator: &self.columns[1531],
                AcceptedQuestOperator: &self.columns[1532],
                AcceptedQuestSequence: [
                    &self.columns[1533],
                    &self.columns[1534],
                    &self.columns[1535],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1536],
                    &self.columns[1537],
                    &self.columns[1538],
                ],
                AcceptedQuest: [
                    &self.columns[1539],
                    &self.columns[1540],
                    &self.columns[1541],
                ],
                LayerSet: [&self.columns[1542], &self.columns[1543]],
                Sequence: &self.columns[1544],
                CompletedQuestOperator: &self.columns[1545],
                AcceptedQuestOperator: &self.columns[1546],
                AcceptedQuestSequence: [
                    &self.columns[1547],
                    &self.columns[1548],
                    &self.columns[1549],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1550],
                    &self.columns[1551],
                    &self.columns[1552],
                ],
                AcceptedQuest: [
                    &self.columns[1553],
                    &self.columns[1554],
                    &self.columns[1555],
                ],
                LayerSet: [&self.columns[1556], &self.columns[1557]],
                Sequence: &self.columns[1558],
                CompletedQuestOperator: &self.columns[1559],
                AcceptedQuestOperator: &self.columns[1560],
                AcceptedQuestSequence: [
                    &self.columns[1561],
                    &self.columns[1562],
                    &self.columns[1563],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1564],
                    &self.columns[1565],
                    &self.columns[1566],
                ],
                AcceptedQuest: [
                    &self.columns[1567],
                    &self.columns[1568],
                    &self.columns[1569],
                ],
                LayerSet: [&self.columns[1570], &self.columns[1571]],
                Sequence: &self.columns[1572],
                CompletedQuestOperator: &self.columns[1573],
                AcceptedQuestOperator: &self.columns[1574],
                AcceptedQuestSequence: [
                    &self.columns[1575],
                    &self.columns[1576],
                    &self.columns[1577],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1578],
                    &self.columns[1579],
                    &self.columns[1580],
                ],
                AcceptedQuest: [
                    &self.columns[1581],
                    &self.columns[1582],
                    &self.columns[1583],
                ],
                LayerSet: [&self.columns[1584], &self.columns[1585]],
                Sequence: &self.columns[1586],
                CompletedQuestOperator: &self.columns[1587],
                AcceptedQuestOperator: &self.columns[1588],
                AcceptedQuestSequence: [
                    &self.columns[1589],
                    &self.columns[1590],
                    &self.columns[1591],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1592],
                    &self.columns[1593],
                    &self.columns[1594],
                ],
                AcceptedQuest: [
                    &self.columns[1595],
                    &self.columns[1596],
                    &self.columns[1597],
                ],
                LayerSet: [&self.columns[1598], &self.columns[1599]],
                Sequence: &self.columns[1600],
                CompletedQuestOperator: &self.columns[1601],
                AcceptedQuestOperator: &self.columns[1602],
                AcceptedQuestSequence: [
                    &self.columns[1603],
                    &self.columns[1604],
                    &self.columns[1605],
                ],
            },
            StoryDefineElement {
                CompletedQuest: [
                    &self.columns[1606],
                    &self.columns[1607],
                    &self.columns[1608],
                ],
                AcceptedQuest: [
                    &self.columns[1609],
                    &self.columns[1610],
                    &self.columns[1611],
                ],
                LayerSet: [&self.columns[1612], &self.columns[1613]],
                Sequence: &self.columns[1614],
                CompletedQuestOperator: &self.columns[1615],
                AcceptedQuestOperator: &self.columns[1616],
                AcceptedQuestSequence: [
                    &self.columns[1617],
                    &self.columns[1618],
                    &self.columns[1619],
                ],
            },
        ]
    }
    pub fn StoryListener<'a>(&'a self) -> [StoryListenerElement<'a>; 80] {
        [
            StoryListenerElement {
                Listener: &self.columns[1620],
                SequenceBegin: &self.columns[1621],
                SequenceEnd: &self.columns[1622],
            },
            StoryListenerElement {
                Listener: &self.columns[1623],
                SequenceBegin: &self.columns[1624],
                SequenceEnd: &self.columns[1625],
            },
            StoryListenerElement {
                Listener: &self.columns[1626],
                SequenceBegin: &self.columns[1627],
                SequenceEnd: &self.columns[1628],
            },
            StoryListenerElement {
                Listener: &self.columns[1629],
                SequenceBegin: &self.columns[1630],
                SequenceEnd: &self.columns[1631],
            },
            StoryListenerElement {
                Listener: &self.columns[1632],
                SequenceBegin: &self.columns[1633],
                SequenceEnd: &self.columns[1634],
            },
            StoryListenerElement {
                Listener: &self.columns[1635],
                SequenceBegin: &self.columns[1636],
                SequenceEnd: &self.columns[1637],
            },
            StoryListenerElement {
                Listener: &self.columns[1638],
                SequenceBegin: &self.columns[1639],
                SequenceEnd: &self.columns[1640],
            },
            StoryListenerElement {
                Listener: &self.columns[1641],
                SequenceBegin: &self.columns[1642],
                SequenceEnd: &self.columns[1643],
            },
            StoryListenerElement {
                Listener: &self.columns[1644],
                SequenceBegin: &self.columns[1645],
                SequenceEnd: &self.columns[1646],
            },
            StoryListenerElement {
                Listener: &self.columns[1647],
                SequenceBegin: &self.columns[1648],
                SequenceEnd: &self.columns[1649],
            },
            StoryListenerElement {
                Listener: &self.columns[1650],
                SequenceBegin: &self.columns[1651],
                SequenceEnd: &self.columns[1652],
            },
            StoryListenerElement {
                Listener: &self.columns[1653],
                SequenceBegin: &self.columns[1654],
                SequenceEnd: &self.columns[1655],
            },
            StoryListenerElement {
                Listener: &self.columns[1656],
                SequenceBegin: &self.columns[1657],
                SequenceEnd: &self.columns[1658],
            },
            StoryListenerElement {
                Listener: &self.columns[1659],
                SequenceBegin: &self.columns[1660],
                SequenceEnd: &self.columns[1661],
            },
            StoryListenerElement {
                Listener: &self.columns[1662],
                SequenceBegin: &self.columns[1663],
                SequenceEnd: &self.columns[1664],
            },
            StoryListenerElement {
                Listener: &self.columns[1665],
                SequenceBegin: &self.columns[1666],
                SequenceEnd: &self.columns[1667],
            },
            StoryListenerElement {
                Listener: &self.columns[1668],
                SequenceBegin: &self.columns[1669],
                SequenceEnd: &self.columns[1670],
            },
            StoryListenerElement {
                Listener: &self.columns[1671],
                SequenceBegin: &self.columns[1672],
                SequenceEnd: &self.columns[1673],
            },
            StoryListenerElement {
                Listener: &self.columns[1674],
                SequenceBegin: &self.columns[1675],
                SequenceEnd: &self.columns[1676],
            },
            StoryListenerElement {
                Listener: &self.columns[1677],
                SequenceBegin: &self.columns[1678],
                SequenceEnd: &self.columns[1679],
            },
            StoryListenerElement {
                Listener: &self.columns[1680],
                SequenceBegin: &self.columns[1681],
                SequenceEnd: &self.columns[1682],
            },
            StoryListenerElement {
                Listener: &self.columns[1683],
                SequenceBegin: &self.columns[1684],
                SequenceEnd: &self.columns[1685],
            },
            StoryListenerElement {
                Listener: &self.columns[1686],
                SequenceBegin: &self.columns[1687],
                SequenceEnd: &self.columns[1688],
            },
            StoryListenerElement {
                Listener: &self.columns[1689],
                SequenceBegin: &self.columns[1690],
                SequenceEnd: &self.columns[1691],
            },
            StoryListenerElement {
                Listener: &self.columns[1692],
                SequenceBegin: &self.columns[1693],
                SequenceEnd: &self.columns[1694],
            },
            StoryListenerElement {
                Listener: &self.columns[1695],
                SequenceBegin: &self.columns[1696],
                SequenceEnd: &self.columns[1697],
            },
            StoryListenerElement {
                Listener: &self.columns[1698],
                SequenceBegin: &self.columns[1699],
                SequenceEnd: &self.columns[1700],
            },
            StoryListenerElement {
                Listener: &self.columns[1701],
                SequenceBegin: &self.columns[1702],
                SequenceEnd: &self.columns[1703],
            },
            StoryListenerElement {
                Listener: &self.columns[1704],
                SequenceBegin: &self.columns[1705],
                SequenceEnd: &self.columns[1706],
            },
            StoryListenerElement {
                Listener: &self.columns[1707],
                SequenceBegin: &self.columns[1708],
                SequenceEnd: &self.columns[1709],
            },
            StoryListenerElement {
                Listener: &self.columns[1710],
                SequenceBegin: &self.columns[1711],
                SequenceEnd: &self.columns[1712],
            },
            StoryListenerElement {
                Listener: &self.columns[1713],
                SequenceBegin: &self.columns[1714],
                SequenceEnd: &self.columns[1715],
            },
            StoryListenerElement {
                Listener: &self.columns[1716],
                SequenceBegin: &self.columns[1717],
                SequenceEnd: &self.columns[1718],
            },
            StoryListenerElement {
                Listener: &self.columns[1719],
                SequenceBegin: &self.columns[1720],
                SequenceEnd: &self.columns[1721],
            },
            StoryListenerElement {
                Listener: &self.columns[1722],
                SequenceBegin: &self.columns[1723],
                SequenceEnd: &self.columns[1724],
            },
            StoryListenerElement {
                Listener: &self.columns[1725],
                SequenceBegin: &self.columns[1726],
                SequenceEnd: &self.columns[1727],
            },
            StoryListenerElement {
                Listener: &self.columns[1728],
                SequenceBegin: &self.columns[1729],
                SequenceEnd: &self.columns[1730],
            },
            StoryListenerElement {
                Listener: &self.columns[1731],
                SequenceBegin: &self.columns[1732],
                SequenceEnd: &self.columns[1733],
            },
            StoryListenerElement {
                Listener: &self.columns[1734],
                SequenceBegin: &self.columns[1735],
                SequenceEnd: &self.columns[1736],
            },
            StoryListenerElement {
                Listener: &self.columns[1737],
                SequenceBegin: &self.columns[1738],
                SequenceEnd: &self.columns[1739],
            },
            StoryListenerElement {
                Listener: &self.columns[1740],
                SequenceBegin: &self.columns[1741],
                SequenceEnd: &self.columns[1742],
            },
            StoryListenerElement {
                Listener: &self.columns[1743],
                SequenceBegin: &self.columns[1744],
                SequenceEnd: &self.columns[1745],
            },
            StoryListenerElement {
                Listener: &self.columns[1746],
                SequenceBegin: &self.columns[1747],
                SequenceEnd: &self.columns[1748],
            },
            StoryListenerElement {
                Listener: &self.columns[1749],
                SequenceBegin: &self.columns[1750],
                SequenceEnd: &self.columns[1751],
            },
            StoryListenerElement {
                Listener: &self.columns[1752],
                SequenceBegin: &self.columns[1753],
                SequenceEnd: &self.columns[1754],
            },
            StoryListenerElement {
                Listener: &self.columns[1755],
                SequenceBegin: &self.columns[1756],
                SequenceEnd: &self.columns[1757],
            },
            StoryListenerElement {
                Listener: &self.columns[1758],
                SequenceBegin: &self.columns[1759],
                SequenceEnd: &self.columns[1760],
            },
            StoryListenerElement {
                Listener: &self.columns[1761],
                SequenceBegin: &self.columns[1762],
                SequenceEnd: &self.columns[1763],
            },
            StoryListenerElement {
                Listener: &self.columns[1764],
                SequenceBegin: &self.columns[1765],
                SequenceEnd: &self.columns[1766],
            },
            StoryListenerElement {
                Listener: &self.columns[1767],
                SequenceBegin: &self.columns[1768],
                SequenceEnd: &self.columns[1769],
            },
            StoryListenerElement {
                Listener: &self.columns[1770],
                SequenceBegin: &self.columns[1771],
                SequenceEnd: &self.columns[1772],
            },
            StoryListenerElement {
                Listener: &self.columns[1773],
                SequenceBegin: &self.columns[1774],
                SequenceEnd: &self.columns[1775],
            },
            StoryListenerElement {
                Listener: &self.columns[1776],
                SequenceBegin: &self.columns[1777],
                SequenceEnd: &self.columns[1778],
            },
            StoryListenerElement {
                Listener: &self.columns[1779],
                SequenceBegin: &self.columns[1780],
                SequenceEnd: &self.columns[1781],
            },
            StoryListenerElement {
                Listener: &self.columns[1782],
                SequenceBegin: &self.columns[1783],
                SequenceEnd: &self.columns[1784],
            },
            StoryListenerElement {
                Listener: &self.columns[1785],
                SequenceBegin: &self.columns[1786],
                SequenceEnd: &self.columns[1787],
            },
            StoryListenerElement {
                Listener: &self.columns[1788],
                SequenceBegin: &self.columns[1789],
                SequenceEnd: &self.columns[1790],
            },
            StoryListenerElement {
                Listener: &self.columns[1791],
                SequenceBegin: &self.columns[1792],
                SequenceEnd: &self.columns[1793],
            },
            StoryListenerElement {
                Listener: &self.columns[1794],
                SequenceBegin: &self.columns[1795],
                SequenceEnd: &self.columns[1796],
            },
            StoryListenerElement {
                Listener: &self.columns[1797],
                SequenceBegin: &self.columns[1798],
                SequenceEnd: &self.columns[1799],
            },
            StoryListenerElement {
                Listener: &self.columns[1800],
                SequenceBegin: &self.columns[1801],
                SequenceEnd: &self.columns[1802],
            },
            StoryListenerElement {
                Listener: &self.columns[1803],
                SequenceBegin: &self.columns[1804],
                SequenceEnd: &self.columns[1805],
            },
            StoryListenerElement {
                Listener: &self.columns[1806],
                SequenceBegin: &self.columns[1807],
                SequenceEnd: &self.columns[1808],
            },
            StoryListenerElement {
                Listener: &self.columns[1809],
                SequenceBegin: &self.columns[1810],
                SequenceEnd: &self.columns[1811],
            },
            StoryListenerElement {
                Listener: &self.columns[1812],
                SequenceBegin: &self.columns[1813],
                SequenceEnd: &self.columns[1814],
            },
            StoryListenerElement {
                Listener: &self.columns[1815],
                SequenceBegin: &self.columns[1816],
                SequenceEnd: &self.columns[1817],
            },
            StoryListenerElement {
                Listener: &self.columns[1818],
                SequenceBegin: &self.columns[1819],
                SequenceEnd: &self.columns[1820],
            },
            StoryListenerElement {
                Listener: &self.columns[1821],
                SequenceBegin: &self.columns[1822],
                SequenceEnd: &self.columns[1823],
            },
            StoryListenerElement {
                Listener: &self.columns[1824],
                SequenceBegin: &self.columns[1825],
                SequenceEnd: &self.columns[1826],
            },
            StoryListenerElement {
                Listener: &self.columns[1827],
                SequenceBegin: &self.columns[1828],
                SequenceEnd: &self.columns[1829],
            },
            StoryListenerElement {
                Listener: &self.columns[1830],
                SequenceBegin: &self.columns[1831],
                SequenceEnd: &self.columns[1832],
            },
            StoryListenerElement {
                Listener: &self.columns[1833],
                SequenceBegin: &self.columns[1834],
                SequenceEnd: &self.columns[1835],
            },
            StoryListenerElement {
                Listener: &self.columns[1836],
                SequenceBegin: &self.columns[1837],
                SequenceEnd: &self.columns[1838],
            },
            StoryListenerElement {
                Listener: &self.columns[1839],
                SequenceBegin: &self.columns[1840],
                SequenceEnd: &self.columns[1841],
            },
            StoryListenerElement {
                Listener: &self.columns[1842],
                SequenceBegin: &self.columns[1843],
                SequenceEnd: &self.columns[1844],
            },
            StoryListenerElement {
                Listener: &self.columns[1845],
                SequenceBegin: &self.columns[1846],
                SequenceEnd: &self.columns[1847],
            },
            StoryListenerElement {
                Listener: &self.columns[1848],
                SequenceBegin: &self.columns[1849],
                SequenceEnd: &self.columns[1850],
            },
            StoryListenerElement {
                Listener: &self.columns[1851],
                SequenceBegin: &self.columns[1852],
                SequenceEnd: &self.columns[1853],
            },
            StoryListenerElement {
                Listener: &self.columns[1854],
                SequenceBegin: &self.columns[1855],
                SequenceEnd: &self.columns[1856],
            },
            StoryListenerElement {
                Listener: &self.columns[1857],
                SequenceBegin: &self.columns[1858],
                SequenceEnd: &self.columns[1859],
            },
        ]
    }
    pub fn Script<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1860]
    }
    pub fn LayerSetTerritoryType<'a>(&'a self) -> [&'a ColumnData; 2] {
        [&self.columns[1861], &self.columns[1862]]
    }
}
