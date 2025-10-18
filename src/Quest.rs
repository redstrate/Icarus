//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct QuestParamsElement<'a> {
    ScriptInstruction: &'a ColumnData,
    ScriptArg: &'a ColumnData,
}
pub struct QuestListenerParamsElement<'a> {
    Listener: &'a ColumnData,
    ConditionValue: &'a ColumnData,
    Behavior: &'a ColumnData,
    ActorSpawnSeq: &'a ColumnData,
    ActorDespawnSeq: &'a ColumnData,
    QuestUInt8A: &'a ColumnData,
    ConditionType: &'a ColumnData,
    ConditionOperator: &'a ColumnData,
    VisibleBool: &'a ColumnData,
    ConditionBool: &'a ColumnData,
    ItemBool: &'a ColumnData,
    AnnounceBool: &'a ColumnData,
    BehaviorBool: &'a ColumnData,
    AcceptBool: &'a ColumnData,
    QualifiedBool: &'a ColumnData,
    CanTargetBool: &'a ColumnData,
}
pub struct TodoParamsElement<'a> {
    ToDoLocation: [&'a ColumnData; 8],
    ToDoCompleteSeq: &'a ColumnData,
    ToDoQty: &'a ColumnData,
    CountableNum: &'a ColumnData,
}
pub struct QuestSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl QuestSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "Quest")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages.push(read_excel_sheet(resource, "Quest", &exh, language, i)?);
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<QuestRow> {
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
        Some(QuestRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<QuestRow> {
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
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestRow> {
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
pub struct QuestRow {
    columns: Vec<ColumnData>,
}
impl QuestRow {
    pub fn Name<'a>(&'a self) -> &'a ColumnData {
        &self.columns[0]
    }
    pub fn QuestParams<'a>(&'a self) -> [QuestParamsElement<'a>; 50] {
        [
            QuestParamsElement {
                ScriptInstruction: &self.columns[1],
                ScriptArg: &self.columns[2],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[3],
                ScriptArg: &self.columns[4],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[5],
                ScriptArg: &self.columns[6],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[7],
                ScriptArg: &self.columns[8],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[9],
                ScriptArg: &self.columns[10],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[11],
                ScriptArg: &self.columns[12],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[13],
                ScriptArg: &self.columns[14],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[15],
                ScriptArg: &self.columns[16],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[17],
                ScriptArg: &self.columns[18],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[19],
                ScriptArg: &self.columns[20],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[21],
                ScriptArg: &self.columns[22],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[23],
                ScriptArg: &self.columns[24],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[25],
                ScriptArg: &self.columns[26],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[27],
                ScriptArg: &self.columns[28],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[29],
                ScriptArg: &self.columns[30],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[31],
                ScriptArg: &self.columns[32],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[33],
                ScriptArg: &self.columns[34],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[35],
                ScriptArg: &self.columns[36],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[37],
                ScriptArg: &self.columns[38],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[39],
                ScriptArg: &self.columns[40],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[41],
                ScriptArg: &self.columns[42],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[43],
                ScriptArg: &self.columns[44],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[45],
                ScriptArg: &self.columns[46],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[47],
                ScriptArg: &self.columns[48],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[49],
                ScriptArg: &self.columns[50],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[51],
                ScriptArg: &self.columns[52],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[53],
                ScriptArg: &self.columns[54],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[55],
                ScriptArg: &self.columns[56],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[57],
                ScriptArg: &self.columns[58],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[59],
                ScriptArg: &self.columns[60],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[61],
                ScriptArg: &self.columns[62],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[63],
                ScriptArg: &self.columns[64],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[65],
                ScriptArg: &self.columns[66],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[67],
                ScriptArg: &self.columns[68],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[69],
                ScriptArg: &self.columns[70],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[71],
                ScriptArg: &self.columns[72],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[73],
                ScriptArg: &self.columns[74],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[75],
                ScriptArg: &self.columns[76],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[77],
                ScriptArg: &self.columns[78],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[79],
                ScriptArg: &self.columns[80],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[81],
                ScriptArg: &self.columns[82],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[83],
                ScriptArg: &self.columns[84],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[85],
                ScriptArg: &self.columns[86],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[87],
                ScriptArg: &self.columns[88],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[89],
                ScriptArg: &self.columns[90],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[91],
                ScriptArg: &self.columns[92],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[93],
                ScriptArg: &self.columns[94],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[95],
                ScriptArg: &self.columns[96],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[97],
                ScriptArg: &self.columns[98],
            },
            QuestParamsElement {
                ScriptInstruction: &self.columns[99],
                ScriptArg: &self.columns[100],
            },
        ]
    }
    pub fn QuestListenerParams<'a>(&'a self) -> [QuestListenerParamsElement<'a>; 64] {
        [
            QuestListenerParamsElement {
                Listener: &self.columns[101],
                ConditionValue: &self.columns[102],
                Behavior: &self.columns[103],
                ActorSpawnSeq: &self.columns[104],
                ActorDespawnSeq: &self.columns[105],
                QuestUInt8A: &self.columns[106],
                ConditionType: &self.columns[107],
                ConditionOperator: &self.columns[108],
                VisibleBool: &self.columns[109],
                ConditionBool: &self.columns[110],
                ItemBool: &self.columns[111],
                AnnounceBool: &self.columns[112],
                BehaviorBool: &self.columns[113],
                AcceptBool: &self.columns[114],
                QualifiedBool: &self.columns[115],
                CanTargetBool: &self.columns[116],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[117],
                ConditionValue: &self.columns[118],
                Behavior: &self.columns[119],
                ActorSpawnSeq: &self.columns[120],
                ActorDespawnSeq: &self.columns[121],
                QuestUInt8A: &self.columns[122],
                ConditionType: &self.columns[123],
                ConditionOperator: &self.columns[124],
                VisibleBool: &self.columns[125],
                ConditionBool: &self.columns[126],
                ItemBool: &self.columns[127],
                AnnounceBool: &self.columns[128],
                BehaviorBool: &self.columns[129],
                AcceptBool: &self.columns[130],
                QualifiedBool: &self.columns[131],
                CanTargetBool: &self.columns[132],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[133],
                ConditionValue: &self.columns[134],
                Behavior: &self.columns[135],
                ActorSpawnSeq: &self.columns[136],
                ActorDespawnSeq: &self.columns[137],
                QuestUInt8A: &self.columns[138],
                ConditionType: &self.columns[139],
                ConditionOperator: &self.columns[140],
                VisibleBool: &self.columns[141],
                ConditionBool: &self.columns[142],
                ItemBool: &self.columns[143],
                AnnounceBool: &self.columns[144],
                BehaviorBool: &self.columns[145],
                AcceptBool: &self.columns[146],
                QualifiedBool: &self.columns[147],
                CanTargetBool: &self.columns[148],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[149],
                ConditionValue: &self.columns[150],
                Behavior: &self.columns[151],
                ActorSpawnSeq: &self.columns[152],
                ActorDespawnSeq: &self.columns[153],
                QuestUInt8A: &self.columns[154],
                ConditionType: &self.columns[155],
                ConditionOperator: &self.columns[156],
                VisibleBool: &self.columns[157],
                ConditionBool: &self.columns[158],
                ItemBool: &self.columns[159],
                AnnounceBool: &self.columns[160],
                BehaviorBool: &self.columns[161],
                AcceptBool: &self.columns[162],
                QualifiedBool: &self.columns[163],
                CanTargetBool: &self.columns[164],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[165],
                ConditionValue: &self.columns[166],
                Behavior: &self.columns[167],
                ActorSpawnSeq: &self.columns[168],
                ActorDespawnSeq: &self.columns[169],
                QuestUInt8A: &self.columns[170],
                ConditionType: &self.columns[171],
                ConditionOperator: &self.columns[172],
                VisibleBool: &self.columns[173],
                ConditionBool: &self.columns[174],
                ItemBool: &self.columns[175],
                AnnounceBool: &self.columns[176],
                BehaviorBool: &self.columns[177],
                AcceptBool: &self.columns[178],
                QualifiedBool: &self.columns[179],
                CanTargetBool: &self.columns[180],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[181],
                ConditionValue: &self.columns[182],
                Behavior: &self.columns[183],
                ActorSpawnSeq: &self.columns[184],
                ActorDespawnSeq: &self.columns[185],
                QuestUInt8A: &self.columns[186],
                ConditionType: &self.columns[187],
                ConditionOperator: &self.columns[188],
                VisibleBool: &self.columns[189],
                ConditionBool: &self.columns[190],
                ItemBool: &self.columns[191],
                AnnounceBool: &self.columns[192],
                BehaviorBool: &self.columns[193],
                AcceptBool: &self.columns[194],
                QualifiedBool: &self.columns[195],
                CanTargetBool: &self.columns[196],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[197],
                ConditionValue: &self.columns[198],
                Behavior: &self.columns[199],
                ActorSpawnSeq: &self.columns[200],
                ActorDespawnSeq: &self.columns[201],
                QuestUInt8A: &self.columns[202],
                ConditionType: &self.columns[203],
                ConditionOperator: &self.columns[204],
                VisibleBool: &self.columns[205],
                ConditionBool: &self.columns[206],
                ItemBool: &self.columns[207],
                AnnounceBool: &self.columns[208],
                BehaviorBool: &self.columns[209],
                AcceptBool: &self.columns[210],
                QualifiedBool: &self.columns[211],
                CanTargetBool: &self.columns[212],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[213],
                ConditionValue: &self.columns[214],
                Behavior: &self.columns[215],
                ActorSpawnSeq: &self.columns[216],
                ActorDespawnSeq: &self.columns[217],
                QuestUInt8A: &self.columns[218],
                ConditionType: &self.columns[219],
                ConditionOperator: &self.columns[220],
                VisibleBool: &self.columns[221],
                ConditionBool: &self.columns[222],
                ItemBool: &self.columns[223],
                AnnounceBool: &self.columns[224],
                BehaviorBool: &self.columns[225],
                AcceptBool: &self.columns[226],
                QualifiedBool: &self.columns[227],
                CanTargetBool: &self.columns[228],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[229],
                ConditionValue: &self.columns[230],
                Behavior: &self.columns[231],
                ActorSpawnSeq: &self.columns[232],
                ActorDespawnSeq: &self.columns[233],
                QuestUInt8A: &self.columns[234],
                ConditionType: &self.columns[235],
                ConditionOperator: &self.columns[236],
                VisibleBool: &self.columns[237],
                ConditionBool: &self.columns[238],
                ItemBool: &self.columns[239],
                AnnounceBool: &self.columns[240],
                BehaviorBool: &self.columns[241],
                AcceptBool: &self.columns[242],
                QualifiedBool: &self.columns[243],
                CanTargetBool: &self.columns[244],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[245],
                ConditionValue: &self.columns[246],
                Behavior: &self.columns[247],
                ActorSpawnSeq: &self.columns[248],
                ActorDespawnSeq: &self.columns[249],
                QuestUInt8A: &self.columns[250],
                ConditionType: &self.columns[251],
                ConditionOperator: &self.columns[252],
                VisibleBool: &self.columns[253],
                ConditionBool: &self.columns[254],
                ItemBool: &self.columns[255],
                AnnounceBool: &self.columns[256],
                BehaviorBool: &self.columns[257],
                AcceptBool: &self.columns[258],
                QualifiedBool: &self.columns[259],
                CanTargetBool: &self.columns[260],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[261],
                ConditionValue: &self.columns[262],
                Behavior: &self.columns[263],
                ActorSpawnSeq: &self.columns[264],
                ActorDespawnSeq: &self.columns[265],
                QuestUInt8A: &self.columns[266],
                ConditionType: &self.columns[267],
                ConditionOperator: &self.columns[268],
                VisibleBool: &self.columns[269],
                ConditionBool: &self.columns[270],
                ItemBool: &self.columns[271],
                AnnounceBool: &self.columns[272],
                BehaviorBool: &self.columns[273],
                AcceptBool: &self.columns[274],
                QualifiedBool: &self.columns[275],
                CanTargetBool: &self.columns[276],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[277],
                ConditionValue: &self.columns[278],
                Behavior: &self.columns[279],
                ActorSpawnSeq: &self.columns[280],
                ActorDespawnSeq: &self.columns[281],
                QuestUInt8A: &self.columns[282],
                ConditionType: &self.columns[283],
                ConditionOperator: &self.columns[284],
                VisibleBool: &self.columns[285],
                ConditionBool: &self.columns[286],
                ItemBool: &self.columns[287],
                AnnounceBool: &self.columns[288],
                BehaviorBool: &self.columns[289],
                AcceptBool: &self.columns[290],
                QualifiedBool: &self.columns[291],
                CanTargetBool: &self.columns[292],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[293],
                ConditionValue: &self.columns[294],
                Behavior: &self.columns[295],
                ActorSpawnSeq: &self.columns[296],
                ActorDespawnSeq: &self.columns[297],
                QuestUInt8A: &self.columns[298],
                ConditionType: &self.columns[299],
                ConditionOperator: &self.columns[300],
                VisibleBool: &self.columns[301],
                ConditionBool: &self.columns[302],
                ItemBool: &self.columns[303],
                AnnounceBool: &self.columns[304],
                BehaviorBool: &self.columns[305],
                AcceptBool: &self.columns[306],
                QualifiedBool: &self.columns[307],
                CanTargetBool: &self.columns[308],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[309],
                ConditionValue: &self.columns[310],
                Behavior: &self.columns[311],
                ActorSpawnSeq: &self.columns[312],
                ActorDespawnSeq: &self.columns[313],
                QuestUInt8A: &self.columns[314],
                ConditionType: &self.columns[315],
                ConditionOperator: &self.columns[316],
                VisibleBool: &self.columns[317],
                ConditionBool: &self.columns[318],
                ItemBool: &self.columns[319],
                AnnounceBool: &self.columns[320],
                BehaviorBool: &self.columns[321],
                AcceptBool: &self.columns[322],
                QualifiedBool: &self.columns[323],
                CanTargetBool: &self.columns[324],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[325],
                ConditionValue: &self.columns[326],
                Behavior: &self.columns[327],
                ActorSpawnSeq: &self.columns[328],
                ActorDespawnSeq: &self.columns[329],
                QuestUInt8A: &self.columns[330],
                ConditionType: &self.columns[331],
                ConditionOperator: &self.columns[332],
                VisibleBool: &self.columns[333],
                ConditionBool: &self.columns[334],
                ItemBool: &self.columns[335],
                AnnounceBool: &self.columns[336],
                BehaviorBool: &self.columns[337],
                AcceptBool: &self.columns[338],
                QualifiedBool: &self.columns[339],
                CanTargetBool: &self.columns[340],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[341],
                ConditionValue: &self.columns[342],
                Behavior: &self.columns[343],
                ActorSpawnSeq: &self.columns[344],
                ActorDespawnSeq: &self.columns[345],
                QuestUInt8A: &self.columns[346],
                ConditionType: &self.columns[347],
                ConditionOperator: &self.columns[348],
                VisibleBool: &self.columns[349],
                ConditionBool: &self.columns[350],
                ItemBool: &self.columns[351],
                AnnounceBool: &self.columns[352],
                BehaviorBool: &self.columns[353],
                AcceptBool: &self.columns[354],
                QualifiedBool: &self.columns[355],
                CanTargetBool: &self.columns[356],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[357],
                ConditionValue: &self.columns[358],
                Behavior: &self.columns[359],
                ActorSpawnSeq: &self.columns[360],
                ActorDespawnSeq: &self.columns[361],
                QuestUInt8A: &self.columns[362],
                ConditionType: &self.columns[363],
                ConditionOperator: &self.columns[364],
                VisibleBool: &self.columns[365],
                ConditionBool: &self.columns[366],
                ItemBool: &self.columns[367],
                AnnounceBool: &self.columns[368],
                BehaviorBool: &self.columns[369],
                AcceptBool: &self.columns[370],
                QualifiedBool: &self.columns[371],
                CanTargetBool: &self.columns[372],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[373],
                ConditionValue: &self.columns[374],
                Behavior: &self.columns[375],
                ActorSpawnSeq: &self.columns[376],
                ActorDespawnSeq: &self.columns[377],
                QuestUInt8A: &self.columns[378],
                ConditionType: &self.columns[379],
                ConditionOperator: &self.columns[380],
                VisibleBool: &self.columns[381],
                ConditionBool: &self.columns[382],
                ItemBool: &self.columns[383],
                AnnounceBool: &self.columns[384],
                BehaviorBool: &self.columns[385],
                AcceptBool: &self.columns[386],
                QualifiedBool: &self.columns[387],
                CanTargetBool: &self.columns[388],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[389],
                ConditionValue: &self.columns[390],
                Behavior: &self.columns[391],
                ActorSpawnSeq: &self.columns[392],
                ActorDespawnSeq: &self.columns[393],
                QuestUInt8A: &self.columns[394],
                ConditionType: &self.columns[395],
                ConditionOperator: &self.columns[396],
                VisibleBool: &self.columns[397],
                ConditionBool: &self.columns[398],
                ItemBool: &self.columns[399],
                AnnounceBool: &self.columns[400],
                BehaviorBool: &self.columns[401],
                AcceptBool: &self.columns[402],
                QualifiedBool: &self.columns[403],
                CanTargetBool: &self.columns[404],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[405],
                ConditionValue: &self.columns[406],
                Behavior: &self.columns[407],
                ActorSpawnSeq: &self.columns[408],
                ActorDespawnSeq: &self.columns[409],
                QuestUInt8A: &self.columns[410],
                ConditionType: &self.columns[411],
                ConditionOperator: &self.columns[412],
                VisibleBool: &self.columns[413],
                ConditionBool: &self.columns[414],
                ItemBool: &self.columns[415],
                AnnounceBool: &self.columns[416],
                BehaviorBool: &self.columns[417],
                AcceptBool: &self.columns[418],
                QualifiedBool: &self.columns[419],
                CanTargetBool: &self.columns[420],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[421],
                ConditionValue: &self.columns[422],
                Behavior: &self.columns[423],
                ActorSpawnSeq: &self.columns[424],
                ActorDespawnSeq: &self.columns[425],
                QuestUInt8A: &self.columns[426],
                ConditionType: &self.columns[427],
                ConditionOperator: &self.columns[428],
                VisibleBool: &self.columns[429],
                ConditionBool: &self.columns[430],
                ItemBool: &self.columns[431],
                AnnounceBool: &self.columns[432],
                BehaviorBool: &self.columns[433],
                AcceptBool: &self.columns[434],
                QualifiedBool: &self.columns[435],
                CanTargetBool: &self.columns[436],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[437],
                ConditionValue: &self.columns[438],
                Behavior: &self.columns[439],
                ActorSpawnSeq: &self.columns[440],
                ActorDespawnSeq: &self.columns[441],
                QuestUInt8A: &self.columns[442],
                ConditionType: &self.columns[443],
                ConditionOperator: &self.columns[444],
                VisibleBool: &self.columns[445],
                ConditionBool: &self.columns[446],
                ItemBool: &self.columns[447],
                AnnounceBool: &self.columns[448],
                BehaviorBool: &self.columns[449],
                AcceptBool: &self.columns[450],
                QualifiedBool: &self.columns[451],
                CanTargetBool: &self.columns[452],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[453],
                ConditionValue: &self.columns[454],
                Behavior: &self.columns[455],
                ActorSpawnSeq: &self.columns[456],
                ActorDespawnSeq: &self.columns[457],
                QuestUInt8A: &self.columns[458],
                ConditionType: &self.columns[459],
                ConditionOperator: &self.columns[460],
                VisibleBool: &self.columns[461],
                ConditionBool: &self.columns[462],
                ItemBool: &self.columns[463],
                AnnounceBool: &self.columns[464],
                BehaviorBool: &self.columns[465],
                AcceptBool: &self.columns[466],
                QualifiedBool: &self.columns[467],
                CanTargetBool: &self.columns[468],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[469],
                ConditionValue: &self.columns[470],
                Behavior: &self.columns[471],
                ActorSpawnSeq: &self.columns[472],
                ActorDespawnSeq: &self.columns[473],
                QuestUInt8A: &self.columns[474],
                ConditionType: &self.columns[475],
                ConditionOperator: &self.columns[476],
                VisibleBool: &self.columns[477],
                ConditionBool: &self.columns[478],
                ItemBool: &self.columns[479],
                AnnounceBool: &self.columns[480],
                BehaviorBool: &self.columns[481],
                AcceptBool: &self.columns[482],
                QualifiedBool: &self.columns[483],
                CanTargetBool: &self.columns[484],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[485],
                ConditionValue: &self.columns[486],
                Behavior: &self.columns[487],
                ActorSpawnSeq: &self.columns[488],
                ActorDespawnSeq: &self.columns[489],
                QuestUInt8A: &self.columns[490],
                ConditionType: &self.columns[491],
                ConditionOperator: &self.columns[492],
                VisibleBool: &self.columns[493],
                ConditionBool: &self.columns[494],
                ItemBool: &self.columns[495],
                AnnounceBool: &self.columns[496],
                BehaviorBool: &self.columns[497],
                AcceptBool: &self.columns[498],
                QualifiedBool: &self.columns[499],
                CanTargetBool: &self.columns[500],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[501],
                ConditionValue: &self.columns[502],
                Behavior: &self.columns[503],
                ActorSpawnSeq: &self.columns[504],
                ActorDespawnSeq: &self.columns[505],
                QuestUInt8A: &self.columns[506],
                ConditionType: &self.columns[507],
                ConditionOperator: &self.columns[508],
                VisibleBool: &self.columns[509],
                ConditionBool: &self.columns[510],
                ItemBool: &self.columns[511],
                AnnounceBool: &self.columns[512],
                BehaviorBool: &self.columns[513],
                AcceptBool: &self.columns[514],
                QualifiedBool: &self.columns[515],
                CanTargetBool: &self.columns[516],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[517],
                ConditionValue: &self.columns[518],
                Behavior: &self.columns[519],
                ActorSpawnSeq: &self.columns[520],
                ActorDespawnSeq: &self.columns[521],
                QuestUInt8A: &self.columns[522],
                ConditionType: &self.columns[523],
                ConditionOperator: &self.columns[524],
                VisibleBool: &self.columns[525],
                ConditionBool: &self.columns[526],
                ItemBool: &self.columns[527],
                AnnounceBool: &self.columns[528],
                BehaviorBool: &self.columns[529],
                AcceptBool: &self.columns[530],
                QualifiedBool: &self.columns[531],
                CanTargetBool: &self.columns[532],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[533],
                ConditionValue: &self.columns[534],
                Behavior: &self.columns[535],
                ActorSpawnSeq: &self.columns[536],
                ActorDespawnSeq: &self.columns[537],
                QuestUInt8A: &self.columns[538],
                ConditionType: &self.columns[539],
                ConditionOperator: &self.columns[540],
                VisibleBool: &self.columns[541],
                ConditionBool: &self.columns[542],
                ItemBool: &self.columns[543],
                AnnounceBool: &self.columns[544],
                BehaviorBool: &self.columns[545],
                AcceptBool: &self.columns[546],
                QualifiedBool: &self.columns[547],
                CanTargetBool: &self.columns[548],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[549],
                ConditionValue: &self.columns[550],
                Behavior: &self.columns[551],
                ActorSpawnSeq: &self.columns[552],
                ActorDespawnSeq: &self.columns[553],
                QuestUInt8A: &self.columns[554],
                ConditionType: &self.columns[555],
                ConditionOperator: &self.columns[556],
                VisibleBool: &self.columns[557],
                ConditionBool: &self.columns[558],
                ItemBool: &self.columns[559],
                AnnounceBool: &self.columns[560],
                BehaviorBool: &self.columns[561],
                AcceptBool: &self.columns[562],
                QualifiedBool: &self.columns[563],
                CanTargetBool: &self.columns[564],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[565],
                ConditionValue: &self.columns[566],
                Behavior: &self.columns[567],
                ActorSpawnSeq: &self.columns[568],
                ActorDespawnSeq: &self.columns[569],
                QuestUInt8A: &self.columns[570],
                ConditionType: &self.columns[571],
                ConditionOperator: &self.columns[572],
                VisibleBool: &self.columns[573],
                ConditionBool: &self.columns[574],
                ItemBool: &self.columns[575],
                AnnounceBool: &self.columns[576],
                BehaviorBool: &self.columns[577],
                AcceptBool: &self.columns[578],
                QualifiedBool: &self.columns[579],
                CanTargetBool: &self.columns[580],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[581],
                ConditionValue: &self.columns[582],
                Behavior: &self.columns[583],
                ActorSpawnSeq: &self.columns[584],
                ActorDespawnSeq: &self.columns[585],
                QuestUInt8A: &self.columns[586],
                ConditionType: &self.columns[587],
                ConditionOperator: &self.columns[588],
                VisibleBool: &self.columns[589],
                ConditionBool: &self.columns[590],
                ItemBool: &self.columns[591],
                AnnounceBool: &self.columns[592],
                BehaviorBool: &self.columns[593],
                AcceptBool: &self.columns[594],
                QualifiedBool: &self.columns[595],
                CanTargetBool: &self.columns[596],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[597],
                ConditionValue: &self.columns[598],
                Behavior: &self.columns[599],
                ActorSpawnSeq: &self.columns[600],
                ActorDespawnSeq: &self.columns[601],
                QuestUInt8A: &self.columns[602],
                ConditionType: &self.columns[603],
                ConditionOperator: &self.columns[604],
                VisibleBool: &self.columns[605],
                ConditionBool: &self.columns[606],
                ItemBool: &self.columns[607],
                AnnounceBool: &self.columns[608],
                BehaviorBool: &self.columns[609],
                AcceptBool: &self.columns[610],
                QualifiedBool: &self.columns[611],
                CanTargetBool: &self.columns[612],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[613],
                ConditionValue: &self.columns[614],
                Behavior: &self.columns[615],
                ActorSpawnSeq: &self.columns[616],
                ActorDespawnSeq: &self.columns[617],
                QuestUInt8A: &self.columns[618],
                ConditionType: &self.columns[619],
                ConditionOperator: &self.columns[620],
                VisibleBool: &self.columns[621],
                ConditionBool: &self.columns[622],
                ItemBool: &self.columns[623],
                AnnounceBool: &self.columns[624],
                BehaviorBool: &self.columns[625],
                AcceptBool: &self.columns[626],
                QualifiedBool: &self.columns[627],
                CanTargetBool: &self.columns[628],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[629],
                ConditionValue: &self.columns[630],
                Behavior: &self.columns[631],
                ActorSpawnSeq: &self.columns[632],
                ActorDespawnSeq: &self.columns[633],
                QuestUInt8A: &self.columns[634],
                ConditionType: &self.columns[635],
                ConditionOperator: &self.columns[636],
                VisibleBool: &self.columns[637],
                ConditionBool: &self.columns[638],
                ItemBool: &self.columns[639],
                AnnounceBool: &self.columns[640],
                BehaviorBool: &self.columns[641],
                AcceptBool: &self.columns[642],
                QualifiedBool: &self.columns[643],
                CanTargetBool: &self.columns[644],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[645],
                ConditionValue: &self.columns[646],
                Behavior: &self.columns[647],
                ActorSpawnSeq: &self.columns[648],
                ActorDespawnSeq: &self.columns[649],
                QuestUInt8A: &self.columns[650],
                ConditionType: &self.columns[651],
                ConditionOperator: &self.columns[652],
                VisibleBool: &self.columns[653],
                ConditionBool: &self.columns[654],
                ItemBool: &self.columns[655],
                AnnounceBool: &self.columns[656],
                BehaviorBool: &self.columns[657],
                AcceptBool: &self.columns[658],
                QualifiedBool: &self.columns[659],
                CanTargetBool: &self.columns[660],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[661],
                ConditionValue: &self.columns[662],
                Behavior: &self.columns[663],
                ActorSpawnSeq: &self.columns[664],
                ActorDespawnSeq: &self.columns[665],
                QuestUInt8A: &self.columns[666],
                ConditionType: &self.columns[667],
                ConditionOperator: &self.columns[668],
                VisibleBool: &self.columns[669],
                ConditionBool: &self.columns[670],
                ItemBool: &self.columns[671],
                AnnounceBool: &self.columns[672],
                BehaviorBool: &self.columns[673],
                AcceptBool: &self.columns[674],
                QualifiedBool: &self.columns[675],
                CanTargetBool: &self.columns[676],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[677],
                ConditionValue: &self.columns[678],
                Behavior: &self.columns[679],
                ActorSpawnSeq: &self.columns[680],
                ActorDespawnSeq: &self.columns[681],
                QuestUInt8A: &self.columns[682],
                ConditionType: &self.columns[683],
                ConditionOperator: &self.columns[684],
                VisibleBool: &self.columns[685],
                ConditionBool: &self.columns[686],
                ItemBool: &self.columns[687],
                AnnounceBool: &self.columns[688],
                BehaviorBool: &self.columns[689],
                AcceptBool: &self.columns[690],
                QualifiedBool: &self.columns[691],
                CanTargetBool: &self.columns[692],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[693],
                ConditionValue: &self.columns[694],
                Behavior: &self.columns[695],
                ActorSpawnSeq: &self.columns[696],
                ActorDespawnSeq: &self.columns[697],
                QuestUInt8A: &self.columns[698],
                ConditionType: &self.columns[699],
                ConditionOperator: &self.columns[700],
                VisibleBool: &self.columns[701],
                ConditionBool: &self.columns[702],
                ItemBool: &self.columns[703],
                AnnounceBool: &self.columns[704],
                BehaviorBool: &self.columns[705],
                AcceptBool: &self.columns[706],
                QualifiedBool: &self.columns[707],
                CanTargetBool: &self.columns[708],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[709],
                ConditionValue: &self.columns[710],
                Behavior: &self.columns[711],
                ActorSpawnSeq: &self.columns[712],
                ActorDespawnSeq: &self.columns[713],
                QuestUInt8A: &self.columns[714],
                ConditionType: &self.columns[715],
                ConditionOperator: &self.columns[716],
                VisibleBool: &self.columns[717],
                ConditionBool: &self.columns[718],
                ItemBool: &self.columns[719],
                AnnounceBool: &self.columns[720],
                BehaviorBool: &self.columns[721],
                AcceptBool: &self.columns[722],
                QualifiedBool: &self.columns[723],
                CanTargetBool: &self.columns[724],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[725],
                ConditionValue: &self.columns[726],
                Behavior: &self.columns[727],
                ActorSpawnSeq: &self.columns[728],
                ActorDespawnSeq: &self.columns[729],
                QuestUInt8A: &self.columns[730],
                ConditionType: &self.columns[731],
                ConditionOperator: &self.columns[732],
                VisibleBool: &self.columns[733],
                ConditionBool: &self.columns[734],
                ItemBool: &self.columns[735],
                AnnounceBool: &self.columns[736],
                BehaviorBool: &self.columns[737],
                AcceptBool: &self.columns[738],
                QualifiedBool: &self.columns[739],
                CanTargetBool: &self.columns[740],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[741],
                ConditionValue: &self.columns[742],
                Behavior: &self.columns[743],
                ActorSpawnSeq: &self.columns[744],
                ActorDespawnSeq: &self.columns[745],
                QuestUInt8A: &self.columns[746],
                ConditionType: &self.columns[747],
                ConditionOperator: &self.columns[748],
                VisibleBool: &self.columns[749],
                ConditionBool: &self.columns[750],
                ItemBool: &self.columns[751],
                AnnounceBool: &self.columns[752],
                BehaviorBool: &self.columns[753],
                AcceptBool: &self.columns[754],
                QualifiedBool: &self.columns[755],
                CanTargetBool: &self.columns[756],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[757],
                ConditionValue: &self.columns[758],
                Behavior: &self.columns[759],
                ActorSpawnSeq: &self.columns[760],
                ActorDespawnSeq: &self.columns[761],
                QuestUInt8A: &self.columns[762],
                ConditionType: &self.columns[763],
                ConditionOperator: &self.columns[764],
                VisibleBool: &self.columns[765],
                ConditionBool: &self.columns[766],
                ItemBool: &self.columns[767],
                AnnounceBool: &self.columns[768],
                BehaviorBool: &self.columns[769],
                AcceptBool: &self.columns[770],
                QualifiedBool: &self.columns[771],
                CanTargetBool: &self.columns[772],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[773],
                ConditionValue: &self.columns[774],
                Behavior: &self.columns[775],
                ActorSpawnSeq: &self.columns[776],
                ActorDespawnSeq: &self.columns[777],
                QuestUInt8A: &self.columns[778],
                ConditionType: &self.columns[779],
                ConditionOperator: &self.columns[780],
                VisibleBool: &self.columns[781],
                ConditionBool: &self.columns[782],
                ItemBool: &self.columns[783],
                AnnounceBool: &self.columns[784],
                BehaviorBool: &self.columns[785],
                AcceptBool: &self.columns[786],
                QualifiedBool: &self.columns[787],
                CanTargetBool: &self.columns[788],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[789],
                ConditionValue: &self.columns[790],
                Behavior: &self.columns[791],
                ActorSpawnSeq: &self.columns[792],
                ActorDespawnSeq: &self.columns[793],
                QuestUInt8A: &self.columns[794],
                ConditionType: &self.columns[795],
                ConditionOperator: &self.columns[796],
                VisibleBool: &self.columns[797],
                ConditionBool: &self.columns[798],
                ItemBool: &self.columns[799],
                AnnounceBool: &self.columns[800],
                BehaviorBool: &self.columns[801],
                AcceptBool: &self.columns[802],
                QualifiedBool: &self.columns[803],
                CanTargetBool: &self.columns[804],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[805],
                ConditionValue: &self.columns[806],
                Behavior: &self.columns[807],
                ActorSpawnSeq: &self.columns[808],
                ActorDespawnSeq: &self.columns[809],
                QuestUInt8A: &self.columns[810],
                ConditionType: &self.columns[811],
                ConditionOperator: &self.columns[812],
                VisibleBool: &self.columns[813],
                ConditionBool: &self.columns[814],
                ItemBool: &self.columns[815],
                AnnounceBool: &self.columns[816],
                BehaviorBool: &self.columns[817],
                AcceptBool: &self.columns[818],
                QualifiedBool: &self.columns[819],
                CanTargetBool: &self.columns[820],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[821],
                ConditionValue: &self.columns[822],
                Behavior: &self.columns[823],
                ActorSpawnSeq: &self.columns[824],
                ActorDespawnSeq: &self.columns[825],
                QuestUInt8A: &self.columns[826],
                ConditionType: &self.columns[827],
                ConditionOperator: &self.columns[828],
                VisibleBool: &self.columns[829],
                ConditionBool: &self.columns[830],
                ItemBool: &self.columns[831],
                AnnounceBool: &self.columns[832],
                BehaviorBool: &self.columns[833],
                AcceptBool: &self.columns[834],
                QualifiedBool: &self.columns[835],
                CanTargetBool: &self.columns[836],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[837],
                ConditionValue: &self.columns[838],
                Behavior: &self.columns[839],
                ActorSpawnSeq: &self.columns[840],
                ActorDespawnSeq: &self.columns[841],
                QuestUInt8A: &self.columns[842],
                ConditionType: &self.columns[843],
                ConditionOperator: &self.columns[844],
                VisibleBool: &self.columns[845],
                ConditionBool: &self.columns[846],
                ItemBool: &self.columns[847],
                AnnounceBool: &self.columns[848],
                BehaviorBool: &self.columns[849],
                AcceptBool: &self.columns[850],
                QualifiedBool: &self.columns[851],
                CanTargetBool: &self.columns[852],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[853],
                ConditionValue: &self.columns[854],
                Behavior: &self.columns[855],
                ActorSpawnSeq: &self.columns[856],
                ActorDespawnSeq: &self.columns[857],
                QuestUInt8A: &self.columns[858],
                ConditionType: &self.columns[859],
                ConditionOperator: &self.columns[860],
                VisibleBool: &self.columns[861],
                ConditionBool: &self.columns[862],
                ItemBool: &self.columns[863],
                AnnounceBool: &self.columns[864],
                BehaviorBool: &self.columns[865],
                AcceptBool: &self.columns[866],
                QualifiedBool: &self.columns[867],
                CanTargetBool: &self.columns[868],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[869],
                ConditionValue: &self.columns[870],
                Behavior: &self.columns[871],
                ActorSpawnSeq: &self.columns[872],
                ActorDespawnSeq: &self.columns[873],
                QuestUInt8A: &self.columns[874],
                ConditionType: &self.columns[875],
                ConditionOperator: &self.columns[876],
                VisibleBool: &self.columns[877],
                ConditionBool: &self.columns[878],
                ItemBool: &self.columns[879],
                AnnounceBool: &self.columns[880],
                BehaviorBool: &self.columns[881],
                AcceptBool: &self.columns[882],
                QualifiedBool: &self.columns[883],
                CanTargetBool: &self.columns[884],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[885],
                ConditionValue: &self.columns[886],
                Behavior: &self.columns[887],
                ActorSpawnSeq: &self.columns[888],
                ActorDespawnSeq: &self.columns[889],
                QuestUInt8A: &self.columns[890],
                ConditionType: &self.columns[891],
                ConditionOperator: &self.columns[892],
                VisibleBool: &self.columns[893],
                ConditionBool: &self.columns[894],
                ItemBool: &self.columns[895],
                AnnounceBool: &self.columns[896],
                BehaviorBool: &self.columns[897],
                AcceptBool: &self.columns[898],
                QualifiedBool: &self.columns[899],
                CanTargetBool: &self.columns[900],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[901],
                ConditionValue: &self.columns[902],
                Behavior: &self.columns[903],
                ActorSpawnSeq: &self.columns[904],
                ActorDespawnSeq: &self.columns[905],
                QuestUInt8A: &self.columns[906],
                ConditionType: &self.columns[907],
                ConditionOperator: &self.columns[908],
                VisibleBool: &self.columns[909],
                ConditionBool: &self.columns[910],
                ItemBool: &self.columns[911],
                AnnounceBool: &self.columns[912],
                BehaviorBool: &self.columns[913],
                AcceptBool: &self.columns[914],
                QualifiedBool: &self.columns[915],
                CanTargetBool: &self.columns[916],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[917],
                ConditionValue: &self.columns[918],
                Behavior: &self.columns[919],
                ActorSpawnSeq: &self.columns[920],
                ActorDespawnSeq: &self.columns[921],
                QuestUInt8A: &self.columns[922],
                ConditionType: &self.columns[923],
                ConditionOperator: &self.columns[924],
                VisibleBool: &self.columns[925],
                ConditionBool: &self.columns[926],
                ItemBool: &self.columns[927],
                AnnounceBool: &self.columns[928],
                BehaviorBool: &self.columns[929],
                AcceptBool: &self.columns[930],
                QualifiedBool: &self.columns[931],
                CanTargetBool: &self.columns[932],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[933],
                ConditionValue: &self.columns[934],
                Behavior: &self.columns[935],
                ActorSpawnSeq: &self.columns[936],
                ActorDespawnSeq: &self.columns[937],
                QuestUInt8A: &self.columns[938],
                ConditionType: &self.columns[939],
                ConditionOperator: &self.columns[940],
                VisibleBool: &self.columns[941],
                ConditionBool: &self.columns[942],
                ItemBool: &self.columns[943],
                AnnounceBool: &self.columns[944],
                BehaviorBool: &self.columns[945],
                AcceptBool: &self.columns[946],
                QualifiedBool: &self.columns[947],
                CanTargetBool: &self.columns[948],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[949],
                ConditionValue: &self.columns[950],
                Behavior: &self.columns[951],
                ActorSpawnSeq: &self.columns[952],
                ActorDespawnSeq: &self.columns[953],
                QuestUInt8A: &self.columns[954],
                ConditionType: &self.columns[955],
                ConditionOperator: &self.columns[956],
                VisibleBool: &self.columns[957],
                ConditionBool: &self.columns[958],
                ItemBool: &self.columns[959],
                AnnounceBool: &self.columns[960],
                BehaviorBool: &self.columns[961],
                AcceptBool: &self.columns[962],
                QualifiedBool: &self.columns[963],
                CanTargetBool: &self.columns[964],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[965],
                ConditionValue: &self.columns[966],
                Behavior: &self.columns[967],
                ActorSpawnSeq: &self.columns[968],
                ActorDespawnSeq: &self.columns[969],
                QuestUInt8A: &self.columns[970],
                ConditionType: &self.columns[971],
                ConditionOperator: &self.columns[972],
                VisibleBool: &self.columns[973],
                ConditionBool: &self.columns[974],
                ItemBool: &self.columns[975],
                AnnounceBool: &self.columns[976],
                BehaviorBool: &self.columns[977],
                AcceptBool: &self.columns[978],
                QualifiedBool: &self.columns[979],
                CanTargetBool: &self.columns[980],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[981],
                ConditionValue: &self.columns[982],
                Behavior: &self.columns[983],
                ActorSpawnSeq: &self.columns[984],
                ActorDespawnSeq: &self.columns[985],
                QuestUInt8A: &self.columns[986],
                ConditionType: &self.columns[987],
                ConditionOperator: &self.columns[988],
                VisibleBool: &self.columns[989],
                ConditionBool: &self.columns[990],
                ItemBool: &self.columns[991],
                AnnounceBool: &self.columns[992],
                BehaviorBool: &self.columns[993],
                AcceptBool: &self.columns[994],
                QualifiedBool: &self.columns[995],
                CanTargetBool: &self.columns[996],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[997],
                ConditionValue: &self.columns[998],
                Behavior: &self.columns[999],
                ActorSpawnSeq: &self.columns[1000],
                ActorDespawnSeq: &self.columns[1001],
                QuestUInt8A: &self.columns[1002],
                ConditionType: &self.columns[1003],
                ConditionOperator: &self.columns[1004],
                VisibleBool: &self.columns[1005],
                ConditionBool: &self.columns[1006],
                ItemBool: &self.columns[1007],
                AnnounceBool: &self.columns[1008],
                BehaviorBool: &self.columns[1009],
                AcceptBool: &self.columns[1010],
                QualifiedBool: &self.columns[1011],
                CanTargetBool: &self.columns[1012],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1013],
                ConditionValue: &self.columns[1014],
                Behavior: &self.columns[1015],
                ActorSpawnSeq: &self.columns[1016],
                ActorDespawnSeq: &self.columns[1017],
                QuestUInt8A: &self.columns[1018],
                ConditionType: &self.columns[1019],
                ConditionOperator: &self.columns[1020],
                VisibleBool: &self.columns[1021],
                ConditionBool: &self.columns[1022],
                ItemBool: &self.columns[1023],
                AnnounceBool: &self.columns[1024],
                BehaviorBool: &self.columns[1025],
                AcceptBool: &self.columns[1026],
                QualifiedBool: &self.columns[1027],
                CanTargetBool: &self.columns[1028],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1029],
                ConditionValue: &self.columns[1030],
                Behavior: &self.columns[1031],
                ActorSpawnSeq: &self.columns[1032],
                ActorDespawnSeq: &self.columns[1033],
                QuestUInt8A: &self.columns[1034],
                ConditionType: &self.columns[1035],
                ConditionOperator: &self.columns[1036],
                VisibleBool: &self.columns[1037],
                ConditionBool: &self.columns[1038],
                ItemBool: &self.columns[1039],
                AnnounceBool: &self.columns[1040],
                BehaviorBool: &self.columns[1041],
                AcceptBool: &self.columns[1042],
                QualifiedBool: &self.columns[1043],
                CanTargetBool: &self.columns[1044],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1045],
                ConditionValue: &self.columns[1046],
                Behavior: &self.columns[1047],
                ActorSpawnSeq: &self.columns[1048],
                ActorDespawnSeq: &self.columns[1049],
                QuestUInt8A: &self.columns[1050],
                ConditionType: &self.columns[1051],
                ConditionOperator: &self.columns[1052],
                VisibleBool: &self.columns[1053],
                ConditionBool: &self.columns[1054],
                ItemBool: &self.columns[1055],
                AnnounceBool: &self.columns[1056],
                BehaviorBool: &self.columns[1057],
                AcceptBool: &self.columns[1058],
                QualifiedBool: &self.columns[1059],
                CanTargetBool: &self.columns[1060],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1061],
                ConditionValue: &self.columns[1062],
                Behavior: &self.columns[1063],
                ActorSpawnSeq: &self.columns[1064],
                ActorDespawnSeq: &self.columns[1065],
                QuestUInt8A: &self.columns[1066],
                ConditionType: &self.columns[1067],
                ConditionOperator: &self.columns[1068],
                VisibleBool: &self.columns[1069],
                ConditionBool: &self.columns[1070],
                ItemBool: &self.columns[1071],
                AnnounceBool: &self.columns[1072],
                BehaviorBool: &self.columns[1073],
                AcceptBool: &self.columns[1074],
                QualifiedBool: &self.columns[1075],
                CanTargetBool: &self.columns[1076],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1077],
                ConditionValue: &self.columns[1078],
                Behavior: &self.columns[1079],
                ActorSpawnSeq: &self.columns[1080],
                ActorDespawnSeq: &self.columns[1081],
                QuestUInt8A: &self.columns[1082],
                ConditionType: &self.columns[1083],
                ConditionOperator: &self.columns[1084],
                VisibleBool: &self.columns[1085],
                ConditionBool: &self.columns[1086],
                ItemBool: &self.columns[1087],
                AnnounceBool: &self.columns[1088],
                BehaviorBool: &self.columns[1089],
                AcceptBool: &self.columns[1090],
                QualifiedBool: &self.columns[1091],
                CanTargetBool: &self.columns[1092],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1093],
                ConditionValue: &self.columns[1094],
                Behavior: &self.columns[1095],
                ActorSpawnSeq: &self.columns[1096],
                ActorDespawnSeq: &self.columns[1097],
                QuestUInt8A: &self.columns[1098],
                ConditionType: &self.columns[1099],
                ConditionOperator: &self.columns[1100],
                VisibleBool: &self.columns[1101],
                ConditionBool: &self.columns[1102],
                ItemBool: &self.columns[1103],
                AnnounceBool: &self.columns[1104],
                BehaviorBool: &self.columns[1105],
                AcceptBool: &self.columns[1106],
                QualifiedBool: &self.columns[1107],
                CanTargetBool: &self.columns[1108],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1109],
                ConditionValue: &self.columns[1110],
                Behavior: &self.columns[1111],
                ActorSpawnSeq: &self.columns[1112],
                ActorDespawnSeq: &self.columns[1113],
                QuestUInt8A: &self.columns[1114],
                ConditionType: &self.columns[1115],
                ConditionOperator: &self.columns[1116],
                VisibleBool: &self.columns[1117],
                ConditionBool: &self.columns[1118],
                ItemBool: &self.columns[1119],
                AnnounceBool: &self.columns[1120],
                BehaviorBool: &self.columns[1121],
                AcceptBool: &self.columns[1122],
                QualifiedBool: &self.columns[1123],
                CanTargetBool: &self.columns[1124],
            },
        ]
    }
    pub fn TodoParams<'a>(&'a self) -> [TodoParamsElement<'a>; 24] {
        [
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1125],
                    &self.columns[1126],
                    &self.columns[1127],
                    &self.columns[1128],
                    &self.columns[1129],
                    &self.columns[1130],
                    &self.columns[1131],
                    &self.columns[1132],
                ],
                ToDoCompleteSeq: &self.columns[1133],
                ToDoQty: &self.columns[1134],
                CountableNum: &self.columns[1135],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1136],
                    &self.columns[1137],
                    &self.columns[1138],
                    &self.columns[1139],
                    &self.columns[1140],
                    &self.columns[1141],
                    &self.columns[1142],
                    &self.columns[1143],
                ],
                ToDoCompleteSeq: &self.columns[1144],
                ToDoQty: &self.columns[1145],
                CountableNum: &self.columns[1146],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1147],
                    &self.columns[1148],
                    &self.columns[1149],
                    &self.columns[1150],
                    &self.columns[1151],
                    &self.columns[1152],
                    &self.columns[1153],
                    &self.columns[1154],
                ],
                ToDoCompleteSeq: &self.columns[1155],
                ToDoQty: &self.columns[1156],
                CountableNum: &self.columns[1157],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1158],
                    &self.columns[1159],
                    &self.columns[1160],
                    &self.columns[1161],
                    &self.columns[1162],
                    &self.columns[1163],
                    &self.columns[1164],
                    &self.columns[1165],
                ],
                ToDoCompleteSeq: &self.columns[1166],
                ToDoQty: &self.columns[1167],
                CountableNum: &self.columns[1168],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1169],
                    &self.columns[1170],
                    &self.columns[1171],
                    &self.columns[1172],
                    &self.columns[1173],
                    &self.columns[1174],
                    &self.columns[1175],
                    &self.columns[1176],
                ],
                ToDoCompleteSeq: &self.columns[1177],
                ToDoQty: &self.columns[1178],
                CountableNum: &self.columns[1179],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1180],
                    &self.columns[1181],
                    &self.columns[1182],
                    &self.columns[1183],
                    &self.columns[1184],
                    &self.columns[1185],
                    &self.columns[1186],
                    &self.columns[1187],
                ],
                ToDoCompleteSeq: &self.columns[1188],
                ToDoQty: &self.columns[1189],
                CountableNum: &self.columns[1190],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1191],
                    &self.columns[1192],
                    &self.columns[1193],
                    &self.columns[1194],
                    &self.columns[1195],
                    &self.columns[1196],
                    &self.columns[1197],
                    &self.columns[1198],
                ],
                ToDoCompleteSeq: &self.columns[1199],
                ToDoQty: &self.columns[1200],
                CountableNum: &self.columns[1201],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1202],
                    &self.columns[1203],
                    &self.columns[1204],
                    &self.columns[1205],
                    &self.columns[1206],
                    &self.columns[1207],
                    &self.columns[1208],
                    &self.columns[1209],
                ],
                ToDoCompleteSeq: &self.columns[1210],
                ToDoQty: &self.columns[1211],
                CountableNum: &self.columns[1212],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1213],
                    &self.columns[1214],
                    &self.columns[1215],
                    &self.columns[1216],
                    &self.columns[1217],
                    &self.columns[1218],
                    &self.columns[1219],
                    &self.columns[1220],
                ],
                ToDoCompleteSeq: &self.columns[1221],
                ToDoQty: &self.columns[1222],
                CountableNum: &self.columns[1223],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1224],
                    &self.columns[1225],
                    &self.columns[1226],
                    &self.columns[1227],
                    &self.columns[1228],
                    &self.columns[1229],
                    &self.columns[1230],
                    &self.columns[1231],
                ],
                ToDoCompleteSeq: &self.columns[1232],
                ToDoQty: &self.columns[1233],
                CountableNum: &self.columns[1234],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1235],
                    &self.columns[1236],
                    &self.columns[1237],
                    &self.columns[1238],
                    &self.columns[1239],
                    &self.columns[1240],
                    &self.columns[1241],
                    &self.columns[1242],
                ],
                ToDoCompleteSeq: &self.columns[1243],
                ToDoQty: &self.columns[1244],
                CountableNum: &self.columns[1245],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1246],
                    &self.columns[1247],
                    &self.columns[1248],
                    &self.columns[1249],
                    &self.columns[1250],
                    &self.columns[1251],
                    &self.columns[1252],
                    &self.columns[1253],
                ],
                ToDoCompleteSeq: &self.columns[1254],
                ToDoQty: &self.columns[1255],
                CountableNum: &self.columns[1256],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1257],
                    &self.columns[1258],
                    &self.columns[1259],
                    &self.columns[1260],
                    &self.columns[1261],
                    &self.columns[1262],
                    &self.columns[1263],
                    &self.columns[1264],
                ],
                ToDoCompleteSeq: &self.columns[1265],
                ToDoQty: &self.columns[1266],
                CountableNum: &self.columns[1267],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1268],
                    &self.columns[1269],
                    &self.columns[1270],
                    &self.columns[1271],
                    &self.columns[1272],
                    &self.columns[1273],
                    &self.columns[1274],
                    &self.columns[1275],
                ],
                ToDoCompleteSeq: &self.columns[1276],
                ToDoQty: &self.columns[1277],
                CountableNum: &self.columns[1278],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1279],
                    &self.columns[1280],
                    &self.columns[1281],
                    &self.columns[1282],
                    &self.columns[1283],
                    &self.columns[1284],
                    &self.columns[1285],
                    &self.columns[1286],
                ],
                ToDoCompleteSeq: &self.columns[1287],
                ToDoQty: &self.columns[1288],
                CountableNum: &self.columns[1289],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1290],
                    &self.columns[1291],
                    &self.columns[1292],
                    &self.columns[1293],
                    &self.columns[1294],
                    &self.columns[1295],
                    &self.columns[1296],
                    &self.columns[1297],
                ],
                ToDoCompleteSeq: &self.columns[1298],
                ToDoQty: &self.columns[1299],
                CountableNum: &self.columns[1300],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1301],
                    &self.columns[1302],
                    &self.columns[1303],
                    &self.columns[1304],
                    &self.columns[1305],
                    &self.columns[1306],
                    &self.columns[1307],
                    &self.columns[1308],
                ],
                ToDoCompleteSeq: &self.columns[1309],
                ToDoQty: &self.columns[1310],
                CountableNum: &self.columns[1311],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1312],
                    &self.columns[1313],
                    &self.columns[1314],
                    &self.columns[1315],
                    &self.columns[1316],
                    &self.columns[1317],
                    &self.columns[1318],
                    &self.columns[1319],
                ],
                ToDoCompleteSeq: &self.columns[1320],
                ToDoQty: &self.columns[1321],
                CountableNum: &self.columns[1322],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1323],
                    &self.columns[1324],
                    &self.columns[1325],
                    &self.columns[1326],
                    &self.columns[1327],
                    &self.columns[1328],
                    &self.columns[1329],
                    &self.columns[1330],
                ],
                ToDoCompleteSeq: &self.columns[1331],
                ToDoQty: &self.columns[1332],
                CountableNum: &self.columns[1333],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1334],
                    &self.columns[1335],
                    &self.columns[1336],
                    &self.columns[1337],
                    &self.columns[1338],
                    &self.columns[1339],
                    &self.columns[1340],
                    &self.columns[1341],
                ],
                ToDoCompleteSeq: &self.columns[1342],
                ToDoQty: &self.columns[1343],
                CountableNum: &self.columns[1344],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1345],
                    &self.columns[1346],
                    &self.columns[1347],
                    &self.columns[1348],
                    &self.columns[1349],
                    &self.columns[1350],
                    &self.columns[1351],
                    &self.columns[1352],
                ],
                ToDoCompleteSeq: &self.columns[1353],
                ToDoQty: &self.columns[1354],
                CountableNum: &self.columns[1355],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1356],
                    &self.columns[1357],
                    &self.columns[1358],
                    &self.columns[1359],
                    &self.columns[1360],
                    &self.columns[1361],
                    &self.columns[1362],
                    &self.columns[1363],
                ],
                ToDoCompleteSeq: &self.columns[1364],
                ToDoQty: &self.columns[1365],
                CountableNum: &self.columns[1366],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1367],
                    &self.columns[1368],
                    &self.columns[1369],
                    &self.columns[1370],
                    &self.columns[1371],
                    &self.columns[1372],
                    &self.columns[1373],
                    &self.columns[1374],
                ],
                ToDoCompleteSeq: &self.columns[1375],
                ToDoQty: &self.columns[1376],
                CountableNum: &self.columns[1377],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1378],
                    &self.columns[1379],
                    &self.columns[1380],
                    &self.columns[1381],
                    &self.columns[1382],
                    &self.columns[1383],
                    &self.columns[1384],
                    &self.columns[1385],
                ],
                ToDoCompleteSeq: &self.columns[1386],
                ToDoQty: &self.columns[1387],
                CountableNum: &self.columns[1388],
            },
        ]
    }
    pub fn GilReward<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1389]
    }
    pub fn CurrencyReward<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1390]
    }
    pub fn CurrencyRewardCount<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1391]
    }
    pub fn Reward<'a>(&'a self) -> [&'a ColumnData; 7] {
        [
            &self.columns[1392],
            &self.columns[1393],
            &self.columns[1394],
            &self.columns[1395],
            &self.columns[1396],
            &self.columns[1397],
            &self.columns[1398],
        ]
    }
    pub fn OptionalItemReward<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[1399],
            &self.columns[1400],
            &self.columns[1401],
            &self.columns[1402],
            &self.columns[1403],
        ]
    }
    pub fn InstanceContentUnlock<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1404]
    }
    pub fn ExpFactor<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1405]
    }
    pub fn EmoteReward<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1406]
    }
    pub fn ActionReward<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1407]
    }
    pub fn SystemReward<'a>(&'a self) -> [&'a ColumnData; 2] {
        [&self.columns[1408], &self.columns[1409]]
    }
    pub fn GCTypeReward<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1410]
    }
    pub fn ItemCatalyst<'a>(&'a self) -> [&'a ColumnData; 3] {
        [&self.columns[1411], &self.columns[1412], &self.columns[1413]]
    }
    pub fn ItemCountCatalyst<'a>(&'a self) -> [&'a ColumnData; 3] {
        [&self.columns[1414], &self.columns[1415], &self.columns[1416]]
    }
    pub fn ItemRewardType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1417]
    }
    pub fn ItemCountReward<'a>(&'a self) -> [&'a ColumnData; 7] {
        [
            &self.columns[1418],
            &self.columns[1419],
            &self.columns[1420],
            &self.columns[1421],
            &self.columns[1422],
            &self.columns[1423],
            &self.columns[1424],
        ]
    }
    pub fn RewardStain<'a>(&'a self) -> [&'a ColumnData; 7] {
        [
            &self.columns[1425],
            &self.columns[1426],
            &self.columns[1427],
            &self.columns[1428],
            &self.columns[1429],
            &self.columns[1430],
            &self.columns[1431],
        ]
    }
    pub fn OptionalItemCountReward<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[1432],
            &self.columns[1433],
            &self.columns[1434],
            &self.columns[1435],
            &self.columns[1436],
        ]
    }
    pub fn OptionalItemStainReward<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[1437],
            &self.columns[1438],
            &self.columns[1439],
            &self.columns[1440],
            &self.columns[1441],
        ]
    }
    pub fn GeneralActionReward<'a>(&'a self) -> [&'a ColumnData; 2] {
        [&self.columns[1442], &self.columns[1443]]
    }
    pub fn OtherReward<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1444]
    }
    pub fn Tomestone<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1445]
    }
    pub fn TomestoneReward<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1446]
    }
    pub fn TomestoneCountReward<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1447]
    }
    pub fn ReputationReward<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1448]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1449]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1450]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1451]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1452]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1453]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1454]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1455]
    }
    pub fn OptionalItemIsHQReward<'a>(&'a self) -> [&'a ColumnData; 5] {
        [
            &self.columns[1456],
            &self.columns[1457],
            &self.columns[1458],
            &self.columns[1459],
            &self.columns[1460],
        ]
    }
    pub fn Id<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1461]
    }
    pub fn PreviousQuest<'a>(&'a self) -> [&'a ColumnData; 3] {
        [&self.columns[1462], &self.columns[1463], &self.columns[1464]]
    }
    pub fn QuestLock<'a>(&'a self) -> [&'a ColumnData; 2] {
        [&self.columns[1465], &self.columns[1466]]
    }
    pub fn InstanceContent<'a>(&'a self) -> [&'a ColumnData; 3] {
        [&self.columns[1467], &self.columns[1468], &self.columns[1469]]
    }
    pub fn IssuerStart<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1470]
    }
    pub fn IssuerLocation<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1471]
    }
    pub fn TargetEnd<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1472]
    }
    pub fn JournalGenre<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1473]
    }
    pub fn Icon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1474]
    }
    pub fn IconSpecial<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1475]
    }
    pub fn MountRequired<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1476]
    }
    pub fn ClassJobLevel<'a>(&'a self) -> [&'a ColumnData; 2] {
        [&self.columns[1477], &self.columns[1478]]
    }
    pub fn Header<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1479]
    }
    pub fn BellStart<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1480]
    }
    pub fn BellEnd<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1481]
    }
    pub fn BeastReputationValue<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1482]
    }
    pub fn ClientBehavior<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1483]
    }
    pub fn QuestClassJobSupply<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1484]
    }
    pub fn PlaceName<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1485]
    }
    pub fn SortKey<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1486]
    }
    pub fn Expansion<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1487]
    }
    pub fn ClassJobCategory0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1488]
    }
    pub fn QuestLevelOffset<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1489]
    }
    pub fn ClassJobCategory1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1490]
    }
    pub fn PreviousQuestJoin<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1491]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1492]
    }
    pub fn QuestLockJoin<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1493]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1494]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1495]
    }
    pub fn ClassJobUnlock<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1496]
    }
    pub fn GrandCompany<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1497]
    }
    pub fn GrandCompanyRank<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1498]
    }
    pub fn InstanceContentJoin<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1499]
    }
    pub fn Festival<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1500]
    }
    pub fn FestivalBegin<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1501]
    }
    pub fn FestivalEnd<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1502]
    }
    pub fn BeastTribe<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1503]
    }
    pub fn BeastReputationRank<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1504]
    }
    pub fn SatisfactionNpc<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1505]
    }
    pub fn SatisfactionLevel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1506]
    }
    pub fn DeliveryQuest<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1507]
    }
    pub fn RepeatIntervalType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1508]
    }
    pub fn QuestRepeatFlag<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1509]
    }
    pub fn Type<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1510]
    }
    pub fn Unknown_70<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1511]
    }
    pub fn LevelMax<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1512]
    }
    pub fn ClassJobRequired<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1513]
    }
    pub fn QuestRewardOtherDisplay<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1514]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1515]
    }
    pub fn EventIconType<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1516]
    }
    /// 1/2 - normal daily beast tribe quests, 3 - 'exclusive' (if player's rank is not greater than max rank requirement of quests offered by npc, exactly one of the available quests will be from this pool)
    pub fn DailyQuestPool<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1517]
    }
    pub fn IsHouseRequired<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1518]
    }
    pub fn IsRepeatable<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1519]
    }
    pub fn CanCancel<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1520]
    }
    pub fn Introduction<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1521]
    }
    pub fn HideOfferIcon<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1522]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1523]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a ColumnData {
        &self.columns[1524]
    }
}
