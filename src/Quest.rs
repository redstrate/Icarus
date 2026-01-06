//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    common::Language,
};
pub struct QuestParamsElement<'a> {
    pub ScriptInstruction: &'a Field,
    pub ScriptArg: &'a Field,
}
pub struct QuestListenerParamsElement<'a> {
    pub Listener: &'a Field,
    pub ConditionValue: &'a Field,
    pub Behavior: &'a Field,
    pub ActorSpawnSeq: &'a Field,
    pub ActorDespawnSeq: &'a Field,
    pub Unknown0: &'a Field,
    pub Unknown1: &'a Field,
    pub QuestUInt8A: &'a Field,
    pub ConditionType: &'a Field,
    pub ConditionOperator: &'a Field,
    pub VisibleBool: &'a Field,
    pub ConditionBool: &'a Field,
    pub ItemBool: &'a Field,
    pub AnnounceBool: &'a Field,
    pub BehaviorBool: &'a Field,
    pub AcceptBool: &'a Field,
    pub QualifiedBool: &'a Field,
    pub CanTargetBool: &'a Field,
}
pub struct TodoParamsElement<'a> {
    pub ToDoLocation: [&'a Field; 8],
    pub ToDoCompleteSeq: &'a Field,
    pub ToDoQty: &'a Field,
    pub CountableNum: &'a Field,
}
#[derive(Debug, Clone)]
pub struct QuestSheet {
    sheet: Sheet,
}
impl QuestSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Quest")?;
        let sheet = resolver.read_excel_sheet(&exh, "Quest", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<QuestRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for QuestSheet {
    type Row = QuestRow;
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
impl<'a> IntoIterator for &'a QuestSheet {
    type Item = (u32, Vec<(u16, QuestRow)>);
    type IntoIter = StructuredSheetIterator<'a, QuestSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, QuestSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct QuestRow {
    columns: Vec<Field>,
}
impl QuestRow {
    pub fn Name<'a>(&'a self) -> &'a Field {
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
                Unknown0: &self.columns[106],
                Unknown1: &self.columns[107],
                QuestUInt8A: &self.columns[108],
                ConditionType: &self.columns[109],
                ConditionOperator: &self.columns[110],
                VisibleBool: &self.columns[111],
                ConditionBool: &self.columns[112],
                ItemBool: &self.columns[113],
                AnnounceBool: &self.columns[114],
                BehaviorBool: &self.columns[115],
                AcceptBool: &self.columns[116],
                QualifiedBool: &self.columns[117],
                CanTargetBool: &self.columns[118],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[119],
                ConditionValue: &self.columns[120],
                Behavior: &self.columns[121],
                ActorSpawnSeq: &self.columns[122],
                ActorDespawnSeq: &self.columns[123],
                Unknown0: &self.columns[124],
                Unknown1: &self.columns[125],
                QuestUInt8A: &self.columns[126],
                ConditionType: &self.columns[127],
                ConditionOperator: &self.columns[128],
                VisibleBool: &self.columns[129],
                ConditionBool: &self.columns[130],
                ItemBool: &self.columns[131],
                AnnounceBool: &self.columns[132],
                BehaviorBool: &self.columns[133],
                AcceptBool: &self.columns[134],
                QualifiedBool: &self.columns[135],
                CanTargetBool: &self.columns[136],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[137],
                ConditionValue: &self.columns[138],
                Behavior: &self.columns[139],
                ActorSpawnSeq: &self.columns[140],
                ActorDespawnSeq: &self.columns[141],
                Unknown0: &self.columns[142],
                Unknown1: &self.columns[143],
                QuestUInt8A: &self.columns[144],
                ConditionType: &self.columns[145],
                ConditionOperator: &self.columns[146],
                VisibleBool: &self.columns[147],
                ConditionBool: &self.columns[148],
                ItemBool: &self.columns[149],
                AnnounceBool: &self.columns[150],
                BehaviorBool: &self.columns[151],
                AcceptBool: &self.columns[152],
                QualifiedBool: &self.columns[153],
                CanTargetBool: &self.columns[154],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[155],
                ConditionValue: &self.columns[156],
                Behavior: &self.columns[157],
                ActorSpawnSeq: &self.columns[158],
                ActorDespawnSeq: &self.columns[159],
                Unknown0: &self.columns[160],
                Unknown1: &self.columns[161],
                QuestUInt8A: &self.columns[162],
                ConditionType: &self.columns[163],
                ConditionOperator: &self.columns[164],
                VisibleBool: &self.columns[165],
                ConditionBool: &self.columns[166],
                ItemBool: &self.columns[167],
                AnnounceBool: &self.columns[168],
                BehaviorBool: &self.columns[169],
                AcceptBool: &self.columns[170],
                QualifiedBool: &self.columns[171],
                CanTargetBool: &self.columns[172],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[173],
                ConditionValue: &self.columns[174],
                Behavior: &self.columns[175],
                ActorSpawnSeq: &self.columns[176],
                ActorDespawnSeq: &self.columns[177],
                Unknown0: &self.columns[178],
                Unknown1: &self.columns[179],
                QuestUInt8A: &self.columns[180],
                ConditionType: &self.columns[181],
                ConditionOperator: &self.columns[182],
                VisibleBool: &self.columns[183],
                ConditionBool: &self.columns[184],
                ItemBool: &self.columns[185],
                AnnounceBool: &self.columns[186],
                BehaviorBool: &self.columns[187],
                AcceptBool: &self.columns[188],
                QualifiedBool: &self.columns[189],
                CanTargetBool: &self.columns[190],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[191],
                ConditionValue: &self.columns[192],
                Behavior: &self.columns[193],
                ActorSpawnSeq: &self.columns[194],
                ActorDespawnSeq: &self.columns[195],
                Unknown0: &self.columns[196],
                Unknown1: &self.columns[197],
                QuestUInt8A: &self.columns[198],
                ConditionType: &self.columns[199],
                ConditionOperator: &self.columns[200],
                VisibleBool: &self.columns[201],
                ConditionBool: &self.columns[202],
                ItemBool: &self.columns[203],
                AnnounceBool: &self.columns[204],
                BehaviorBool: &self.columns[205],
                AcceptBool: &self.columns[206],
                QualifiedBool: &self.columns[207],
                CanTargetBool: &self.columns[208],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[209],
                ConditionValue: &self.columns[210],
                Behavior: &self.columns[211],
                ActorSpawnSeq: &self.columns[212],
                ActorDespawnSeq: &self.columns[213],
                Unknown0: &self.columns[214],
                Unknown1: &self.columns[215],
                QuestUInt8A: &self.columns[216],
                ConditionType: &self.columns[217],
                ConditionOperator: &self.columns[218],
                VisibleBool: &self.columns[219],
                ConditionBool: &self.columns[220],
                ItemBool: &self.columns[221],
                AnnounceBool: &self.columns[222],
                BehaviorBool: &self.columns[223],
                AcceptBool: &self.columns[224],
                QualifiedBool: &self.columns[225],
                CanTargetBool: &self.columns[226],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[227],
                ConditionValue: &self.columns[228],
                Behavior: &self.columns[229],
                ActorSpawnSeq: &self.columns[230],
                ActorDespawnSeq: &self.columns[231],
                Unknown0: &self.columns[232],
                Unknown1: &self.columns[233],
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
                Unknown0: &self.columns[250],
                Unknown1: &self.columns[251],
                QuestUInt8A: &self.columns[252],
                ConditionType: &self.columns[253],
                ConditionOperator: &self.columns[254],
                VisibleBool: &self.columns[255],
                ConditionBool: &self.columns[256],
                ItemBool: &self.columns[257],
                AnnounceBool: &self.columns[258],
                BehaviorBool: &self.columns[259],
                AcceptBool: &self.columns[260],
                QualifiedBool: &self.columns[261],
                CanTargetBool: &self.columns[262],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[263],
                ConditionValue: &self.columns[264],
                Behavior: &self.columns[265],
                ActorSpawnSeq: &self.columns[266],
                ActorDespawnSeq: &self.columns[267],
                Unknown0: &self.columns[268],
                Unknown1: &self.columns[269],
                QuestUInt8A: &self.columns[270],
                ConditionType: &self.columns[271],
                ConditionOperator: &self.columns[272],
                VisibleBool: &self.columns[273],
                ConditionBool: &self.columns[274],
                ItemBool: &self.columns[275],
                AnnounceBool: &self.columns[276],
                BehaviorBool: &self.columns[277],
                AcceptBool: &self.columns[278],
                QualifiedBool: &self.columns[279],
                CanTargetBool: &self.columns[280],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[281],
                ConditionValue: &self.columns[282],
                Behavior: &self.columns[283],
                ActorSpawnSeq: &self.columns[284],
                ActorDespawnSeq: &self.columns[285],
                Unknown0: &self.columns[286],
                Unknown1: &self.columns[287],
                QuestUInt8A: &self.columns[288],
                ConditionType: &self.columns[289],
                ConditionOperator: &self.columns[290],
                VisibleBool: &self.columns[291],
                ConditionBool: &self.columns[292],
                ItemBool: &self.columns[293],
                AnnounceBool: &self.columns[294],
                BehaviorBool: &self.columns[295],
                AcceptBool: &self.columns[296],
                QualifiedBool: &self.columns[297],
                CanTargetBool: &self.columns[298],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[299],
                ConditionValue: &self.columns[300],
                Behavior: &self.columns[301],
                ActorSpawnSeq: &self.columns[302],
                ActorDespawnSeq: &self.columns[303],
                Unknown0: &self.columns[304],
                Unknown1: &self.columns[305],
                QuestUInt8A: &self.columns[306],
                ConditionType: &self.columns[307],
                ConditionOperator: &self.columns[308],
                VisibleBool: &self.columns[309],
                ConditionBool: &self.columns[310],
                ItemBool: &self.columns[311],
                AnnounceBool: &self.columns[312],
                BehaviorBool: &self.columns[313],
                AcceptBool: &self.columns[314],
                QualifiedBool: &self.columns[315],
                CanTargetBool: &self.columns[316],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[317],
                ConditionValue: &self.columns[318],
                Behavior: &self.columns[319],
                ActorSpawnSeq: &self.columns[320],
                ActorDespawnSeq: &self.columns[321],
                Unknown0: &self.columns[322],
                Unknown1: &self.columns[323],
                QuestUInt8A: &self.columns[324],
                ConditionType: &self.columns[325],
                ConditionOperator: &self.columns[326],
                VisibleBool: &self.columns[327],
                ConditionBool: &self.columns[328],
                ItemBool: &self.columns[329],
                AnnounceBool: &self.columns[330],
                BehaviorBool: &self.columns[331],
                AcceptBool: &self.columns[332],
                QualifiedBool: &self.columns[333],
                CanTargetBool: &self.columns[334],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[335],
                ConditionValue: &self.columns[336],
                Behavior: &self.columns[337],
                ActorSpawnSeq: &self.columns[338],
                ActorDespawnSeq: &self.columns[339],
                Unknown0: &self.columns[340],
                Unknown1: &self.columns[341],
                QuestUInt8A: &self.columns[342],
                ConditionType: &self.columns[343],
                ConditionOperator: &self.columns[344],
                VisibleBool: &self.columns[345],
                ConditionBool: &self.columns[346],
                ItemBool: &self.columns[347],
                AnnounceBool: &self.columns[348],
                BehaviorBool: &self.columns[349],
                AcceptBool: &self.columns[350],
                QualifiedBool: &self.columns[351],
                CanTargetBool: &self.columns[352],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[353],
                ConditionValue: &self.columns[354],
                Behavior: &self.columns[355],
                ActorSpawnSeq: &self.columns[356],
                ActorDespawnSeq: &self.columns[357],
                Unknown0: &self.columns[358],
                Unknown1: &self.columns[359],
                QuestUInt8A: &self.columns[360],
                ConditionType: &self.columns[361],
                ConditionOperator: &self.columns[362],
                VisibleBool: &self.columns[363],
                ConditionBool: &self.columns[364],
                ItemBool: &self.columns[365],
                AnnounceBool: &self.columns[366],
                BehaviorBool: &self.columns[367],
                AcceptBool: &self.columns[368],
                QualifiedBool: &self.columns[369],
                CanTargetBool: &self.columns[370],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[371],
                ConditionValue: &self.columns[372],
                Behavior: &self.columns[373],
                ActorSpawnSeq: &self.columns[374],
                ActorDespawnSeq: &self.columns[375],
                Unknown0: &self.columns[376],
                Unknown1: &self.columns[377],
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
                Unknown0: &self.columns[394],
                Unknown1: &self.columns[395],
                QuestUInt8A: &self.columns[396],
                ConditionType: &self.columns[397],
                ConditionOperator: &self.columns[398],
                VisibleBool: &self.columns[399],
                ConditionBool: &self.columns[400],
                ItemBool: &self.columns[401],
                AnnounceBool: &self.columns[402],
                BehaviorBool: &self.columns[403],
                AcceptBool: &self.columns[404],
                QualifiedBool: &self.columns[405],
                CanTargetBool: &self.columns[406],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[407],
                ConditionValue: &self.columns[408],
                Behavior: &self.columns[409],
                ActorSpawnSeq: &self.columns[410],
                ActorDespawnSeq: &self.columns[411],
                Unknown0: &self.columns[412],
                Unknown1: &self.columns[413],
                QuestUInt8A: &self.columns[414],
                ConditionType: &self.columns[415],
                ConditionOperator: &self.columns[416],
                VisibleBool: &self.columns[417],
                ConditionBool: &self.columns[418],
                ItemBool: &self.columns[419],
                AnnounceBool: &self.columns[420],
                BehaviorBool: &self.columns[421],
                AcceptBool: &self.columns[422],
                QualifiedBool: &self.columns[423],
                CanTargetBool: &self.columns[424],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[425],
                ConditionValue: &self.columns[426],
                Behavior: &self.columns[427],
                ActorSpawnSeq: &self.columns[428],
                ActorDespawnSeq: &self.columns[429],
                Unknown0: &self.columns[430],
                Unknown1: &self.columns[431],
                QuestUInt8A: &self.columns[432],
                ConditionType: &self.columns[433],
                ConditionOperator: &self.columns[434],
                VisibleBool: &self.columns[435],
                ConditionBool: &self.columns[436],
                ItemBool: &self.columns[437],
                AnnounceBool: &self.columns[438],
                BehaviorBool: &self.columns[439],
                AcceptBool: &self.columns[440],
                QualifiedBool: &self.columns[441],
                CanTargetBool: &self.columns[442],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[443],
                ConditionValue: &self.columns[444],
                Behavior: &self.columns[445],
                ActorSpawnSeq: &self.columns[446],
                ActorDespawnSeq: &self.columns[447],
                Unknown0: &self.columns[448],
                Unknown1: &self.columns[449],
                QuestUInt8A: &self.columns[450],
                ConditionType: &self.columns[451],
                ConditionOperator: &self.columns[452],
                VisibleBool: &self.columns[453],
                ConditionBool: &self.columns[454],
                ItemBool: &self.columns[455],
                AnnounceBool: &self.columns[456],
                BehaviorBool: &self.columns[457],
                AcceptBool: &self.columns[458],
                QualifiedBool: &self.columns[459],
                CanTargetBool: &self.columns[460],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[461],
                ConditionValue: &self.columns[462],
                Behavior: &self.columns[463],
                ActorSpawnSeq: &self.columns[464],
                ActorDespawnSeq: &self.columns[465],
                Unknown0: &self.columns[466],
                Unknown1: &self.columns[467],
                QuestUInt8A: &self.columns[468],
                ConditionType: &self.columns[469],
                ConditionOperator: &self.columns[470],
                VisibleBool: &self.columns[471],
                ConditionBool: &self.columns[472],
                ItemBool: &self.columns[473],
                AnnounceBool: &self.columns[474],
                BehaviorBool: &self.columns[475],
                AcceptBool: &self.columns[476],
                QualifiedBool: &self.columns[477],
                CanTargetBool: &self.columns[478],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[479],
                ConditionValue: &self.columns[480],
                Behavior: &self.columns[481],
                ActorSpawnSeq: &self.columns[482],
                ActorDespawnSeq: &self.columns[483],
                Unknown0: &self.columns[484],
                Unknown1: &self.columns[485],
                QuestUInt8A: &self.columns[486],
                ConditionType: &self.columns[487],
                ConditionOperator: &self.columns[488],
                VisibleBool: &self.columns[489],
                ConditionBool: &self.columns[490],
                ItemBool: &self.columns[491],
                AnnounceBool: &self.columns[492],
                BehaviorBool: &self.columns[493],
                AcceptBool: &self.columns[494],
                QualifiedBool: &self.columns[495],
                CanTargetBool: &self.columns[496],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[497],
                ConditionValue: &self.columns[498],
                Behavior: &self.columns[499],
                ActorSpawnSeq: &self.columns[500],
                ActorDespawnSeq: &self.columns[501],
                Unknown0: &self.columns[502],
                Unknown1: &self.columns[503],
                QuestUInt8A: &self.columns[504],
                ConditionType: &self.columns[505],
                ConditionOperator: &self.columns[506],
                VisibleBool: &self.columns[507],
                ConditionBool: &self.columns[508],
                ItemBool: &self.columns[509],
                AnnounceBool: &self.columns[510],
                BehaviorBool: &self.columns[511],
                AcceptBool: &self.columns[512],
                QualifiedBool: &self.columns[513],
                CanTargetBool: &self.columns[514],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[515],
                ConditionValue: &self.columns[516],
                Behavior: &self.columns[517],
                ActorSpawnSeq: &self.columns[518],
                ActorDespawnSeq: &self.columns[519],
                Unknown0: &self.columns[520],
                Unknown1: &self.columns[521],
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
                Unknown0: &self.columns[538],
                Unknown1: &self.columns[539],
                QuestUInt8A: &self.columns[540],
                ConditionType: &self.columns[541],
                ConditionOperator: &self.columns[542],
                VisibleBool: &self.columns[543],
                ConditionBool: &self.columns[544],
                ItemBool: &self.columns[545],
                AnnounceBool: &self.columns[546],
                BehaviorBool: &self.columns[547],
                AcceptBool: &self.columns[548],
                QualifiedBool: &self.columns[549],
                CanTargetBool: &self.columns[550],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[551],
                ConditionValue: &self.columns[552],
                Behavior: &self.columns[553],
                ActorSpawnSeq: &self.columns[554],
                ActorDespawnSeq: &self.columns[555],
                Unknown0: &self.columns[556],
                Unknown1: &self.columns[557],
                QuestUInt8A: &self.columns[558],
                ConditionType: &self.columns[559],
                ConditionOperator: &self.columns[560],
                VisibleBool: &self.columns[561],
                ConditionBool: &self.columns[562],
                ItemBool: &self.columns[563],
                AnnounceBool: &self.columns[564],
                BehaviorBool: &self.columns[565],
                AcceptBool: &self.columns[566],
                QualifiedBool: &self.columns[567],
                CanTargetBool: &self.columns[568],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[569],
                ConditionValue: &self.columns[570],
                Behavior: &self.columns[571],
                ActorSpawnSeq: &self.columns[572],
                ActorDespawnSeq: &self.columns[573],
                Unknown0: &self.columns[574],
                Unknown1: &self.columns[575],
                QuestUInt8A: &self.columns[576],
                ConditionType: &self.columns[577],
                ConditionOperator: &self.columns[578],
                VisibleBool: &self.columns[579],
                ConditionBool: &self.columns[580],
                ItemBool: &self.columns[581],
                AnnounceBool: &self.columns[582],
                BehaviorBool: &self.columns[583],
                AcceptBool: &self.columns[584],
                QualifiedBool: &self.columns[585],
                CanTargetBool: &self.columns[586],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[587],
                ConditionValue: &self.columns[588],
                Behavior: &self.columns[589],
                ActorSpawnSeq: &self.columns[590],
                ActorDespawnSeq: &self.columns[591],
                Unknown0: &self.columns[592],
                Unknown1: &self.columns[593],
                QuestUInt8A: &self.columns[594],
                ConditionType: &self.columns[595],
                ConditionOperator: &self.columns[596],
                VisibleBool: &self.columns[597],
                ConditionBool: &self.columns[598],
                ItemBool: &self.columns[599],
                AnnounceBool: &self.columns[600],
                BehaviorBool: &self.columns[601],
                AcceptBool: &self.columns[602],
                QualifiedBool: &self.columns[603],
                CanTargetBool: &self.columns[604],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[605],
                ConditionValue: &self.columns[606],
                Behavior: &self.columns[607],
                ActorSpawnSeq: &self.columns[608],
                ActorDespawnSeq: &self.columns[609],
                Unknown0: &self.columns[610],
                Unknown1: &self.columns[611],
                QuestUInt8A: &self.columns[612],
                ConditionType: &self.columns[613],
                ConditionOperator: &self.columns[614],
                VisibleBool: &self.columns[615],
                ConditionBool: &self.columns[616],
                ItemBool: &self.columns[617],
                AnnounceBool: &self.columns[618],
                BehaviorBool: &self.columns[619],
                AcceptBool: &self.columns[620],
                QualifiedBool: &self.columns[621],
                CanTargetBool: &self.columns[622],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[623],
                ConditionValue: &self.columns[624],
                Behavior: &self.columns[625],
                ActorSpawnSeq: &self.columns[626],
                ActorDespawnSeq: &self.columns[627],
                Unknown0: &self.columns[628],
                Unknown1: &self.columns[629],
                QuestUInt8A: &self.columns[630],
                ConditionType: &self.columns[631],
                ConditionOperator: &self.columns[632],
                VisibleBool: &self.columns[633],
                ConditionBool: &self.columns[634],
                ItemBool: &self.columns[635],
                AnnounceBool: &self.columns[636],
                BehaviorBool: &self.columns[637],
                AcceptBool: &self.columns[638],
                QualifiedBool: &self.columns[639],
                CanTargetBool: &self.columns[640],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[641],
                ConditionValue: &self.columns[642],
                Behavior: &self.columns[643],
                ActorSpawnSeq: &self.columns[644],
                ActorDespawnSeq: &self.columns[645],
                Unknown0: &self.columns[646],
                Unknown1: &self.columns[647],
                QuestUInt8A: &self.columns[648],
                ConditionType: &self.columns[649],
                ConditionOperator: &self.columns[650],
                VisibleBool: &self.columns[651],
                ConditionBool: &self.columns[652],
                ItemBool: &self.columns[653],
                AnnounceBool: &self.columns[654],
                BehaviorBool: &self.columns[655],
                AcceptBool: &self.columns[656],
                QualifiedBool: &self.columns[657],
                CanTargetBool: &self.columns[658],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[659],
                ConditionValue: &self.columns[660],
                Behavior: &self.columns[661],
                ActorSpawnSeq: &self.columns[662],
                ActorDespawnSeq: &self.columns[663],
                Unknown0: &self.columns[664],
                Unknown1: &self.columns[665],
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
                Unknown0: &self.columns[682],
                Unknown1: &self.columns[683],
                QuestUInt8A: &self.columns[684],
                ConditionType: &self.columns[685],
                ConditionOperator: &self.columns[686],
                VisibleBool: &self.columns[687],
                ConditionBool: &self.columns[688],
                ItemBool: &self.columns[689],
                AnnounceBool: &self.columns[690],
                BehaviorBool: &self.columns[691],
                AcceptBool: &self.columns[692],
                QualifiedBool: &self.columns[693],
                CanTargetBool: &self.columns[694],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[695],
                ConditionValue: &self.columns[696],
                Behavior: &self.columns[697],
                ActorSpawnSeq: &self.columns[698],
                ActorDespawnSeq: &self.columns[699],
                Unknown0: &self.columns[700],
                Unknown1: &self.columns[701],
                QuestUInt8A: &self.columns[702],
                ConditionType: &self.columns[703],
                ConditionOperator: &self.columns[704],
                VisibleBool: &self.columns[705],
                ConditionBool: &self.columns[706],
                ItemBool: &self.columns[707],
                AnnounceBool: &self.columns[708],
                BehaviorBool: &self.columns[709],
                AcceptBool: &self.columns[710],
                QualifiedBool: &self.columns[711],
                CanTargetBool: &self.columns[712],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[713],
                ConditionValue: &self.columns[714],
                Behavior: &self.columns[715],
                ActorSpawnSeq: &self.columns[716],
                ActorDespawnSeq: &self.columns[717],
                Unknown0: &self.columns[718],
                Unknown1: &self.columns[719],
                QuestUInt8A: &self.columns[720],
                ConditionType: &self.columns[721],
                ConditionOperator: &self.columns[722],
                VisibleBool: &self.columns[723],
                ConditionBool: &self.columns[724],
                ItemBool: &self.columns[725],
                AnnounceBool: &self.columns[726],
                BehaviorBool: &self.columns[727],
                AcceptBool: &self.columns[728],
                QualifiedBool: &self.columns[729],
                CanTargetBool: &self.columns[730],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[731],
                ConditionValue: &self.columns[732],
                Behavior: &self.columns[733],
                ActorSpawnSeq: &self.columns[734],
                ActorDespawnSeq: &self.columns[735],
                Unknown0: &self.columns[736],
                Unknown1: &self.columns[737],
                QuestUInt8A: &self.columns[738],
                ConditionType: &self.columns[739],
                ConditionOperator: &self.columns[740],
                VisibleBool: &self.columns[741],
                ConditionBool: &self.columns[742],
                ItemBool: &self.columns[743],
                AnnounceBool: &self.columns[744],
                BehaviorBool: &self.columns[745],
                AcceptBool: &self.columns[746],
                QualifiedBool: &self.columns[747],
                CanTargetBool: &self.columns[748],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[749],
                ConditionValue: &self.columns[750],
                Behavior: &self.columns[751],
                ActorSpawnSeq: &self.columns[752],
                ActorDespawnSeq: &self.columns[753],
                Unknown0: &self.columns[754],
                Unknown1: &self.columns[755],
                QuestUInt8A: &self.columns[756],
                ConditionType: &self.columns[757],
                ConditionOperator: &self.columns[758],
                VisibleBool: &self.columns[759],
                ConditionBool: &self.columns[760],
                ItemBool: &self.columns[761],
                AnnounceBool: &self.columns[762],
                BehaviorBool: &self.columns[763],
                AcceptBool: &self.columns[764],
                QualifiedBool: &self.columns[765],
                CanTargetBool: &self.columns[766],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[767],
                ConditionValue: &self.columns[768],
                Behavior: &self.columns[769],
                ActorSpawnSeq: &self.columns[770],
                ActorDespawnSeq: &self.columns[771],
                Unknown0: &self.columns[772],
                Unknown1: &self.columns[773],
                QuestUInt8A: &self.columns[774],
                ConditionType: &self.columns[775],
                ConditionOperator: &self.columns[776],
                VisibleBool: &self.columns[777],
                ConditionBool: &self.columns[778],
                ItemBool: &self.columns[779],
                AnnounceBool: &self.columns[780],
                BehaviorBool: &self.columns[781],
                AcceptBool: &self.columns[782],
                QualifiedBool: &self.columns[783],
                CanTargetBool: &self.columns[784],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[785],
                ConditionValue: &self.columns[786],
                Behavior: &self.columns[787],
                ActorSpawnSeq: &self.columns[788],
                ActorDespawnSeq: &self.columns[789],
                Unknown0: &self.columns[790],
                Unknown1: &self.columns[791],
                QuestUInt8A: &self.columns[792],
                ConditionType: &self.columns[793],
                ConditionOperator: &self.columns[794],
                VisibleBool: &self.columns[795],
                ConditionBool: &self.columns[796],
                ItemBool: &self.columns[797],
                AnnounceBool: &self.columns[798],
                BehaviorBool: &self.columns[799],
                AcceptBool: &self.columns[800],
                QualifiedBool: &self.columns[801],
                CanTargetBool: &self.columns[802],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[803],
                ConditionValue: &self.columns[804],
                Behavior: &self.columns[805],
                ActorSpawnSeq: &self.columns[806],
                ActorDespawnSeq: &self.columns[807],
                Unknown0: &self.columns[808],
                Unknown1: &self.columns[809],
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
                Unknown0: &self.columns[826],
                Unknown1: &self.columns[827],
                QuestUInt8A: &self.columns[828],
                ConditionType: &self.columns[829],
                ConditionOperator: &self.columns[830],
                VisibleBool: &self.columns[831],
                ConditionBool: &self.columns[832],
                ItemBool: &self.columns[833],
                AnnounceBool: &self.columns[834],
                BehaviorBool: &self.columns[835],
                AcceptBool: &self.columns[836],
                QualifiedBool: &self.columns[837],
                CanTargetBool: &self.columns[838],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[839],
                ConditionValue: &self.columns[840],
                Behavior: &self.columns[841],
                ActorSpawnSeq: &self.columns[842],
                ActorDespawnSeq: &self.columns[843],
                Unknown0: &self.columns[844],
                Unknown1: &self.columns[845],
                QuestUInt8A: &self.columns[846],
                ConditionType: &self.columns[847],
                ConditionOperator: &self.columns[848],
                VisibleBool: &self.columns[849],
                ConditionBool: &self.columns[850],
                ItemBool: &self.columns[851],
                AnnounceBool: &self.columns[852],
                BehaviorBool: &self.columns[853],
                AcceptBool: &self.columns[854],
                QualifiedBool: &self.columns[855],
                CanTargetBool: &self.columns[856],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[857],
                ConditionValue: &self.columns[858],
                Behavior: &self.columns[859],
                ActorSpawnSeq: &self.columns[860],
                ActorDespawnSeq: &self.columns[861],
                Unknown0: &self.columns[862],
                Unknown1: &self.columns[863],
                QuestUInt8A: &self.columns[864],
                ConditionType: &self.columns[865],
                ConditionOperator: &self.columns[866],
                VisibleBool: &self.columns[867],
                ConditionBool: &self.columns[868],
                ItemBool: &self.columns[869],
                AnnounceBool: &self.columns[870],
                BehaviorBool: &self.columns[871],
                AcceptBool: &self.columns[872],
                QualifiedBool: &self.columns[873],
                CanTargetBool: &self.columns[874],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[875],
                ConditionValue: &self.columns[876],
                Behavior: &self.columns[877],
                ActorSpawnSeq: &self.columns[878],
                ActorDespawnSeq: &self.columns[879],
                Unknown0: &self.columns[880],
                Unknown1: &self.columns[881],
                QuestUInt8A: &self.columns[882],
                ConditionType: &self.columns[883],
                ConditionOperator: &self.columns[884],
                VisibleBool: &self.columns[885],
                ConditionBool: &self.columns[886],
                ItemBool: &self.columns[887],
                AnnounceBool: &self.columns[888],
                BehaviorBool: &self.columns[889],
                AcceptBool: &self.columns[890],
                QualifiedBool: &self.columns[891],
                CanTargetBool: &self.columns[892],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[893],
                ConditionValue: &self.columns[894],
                Behavior: &self.columns[895],
                ActorSpawnSeq: &self.columns[896],
                ActorDespawnSeq: &self.columns[897],
                Unknown0: &self.columns[898],
                Unknown1: &self.columns[899],
                QuestUInt8A: &self.columns[900],
                ConditionType: &self.columns[901],
                ConditionOperator: &self.columns[902],
                VisibleBool: &self.columns[903],
                ConditionBool: &self.columns[904],
                ItemBool: &self.columns[905],
                AnnounceBool: &self.columns[906],
                BehaviorBool: &self.columns[907],
                AcceptBool: &self.columns[908],
                QualifiedBool: &self.columns[909],
                CanTargetBool: &self.columns[910],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[911],
                ConditionValue: &self.columns[912],
                Behavior: &self.columns[913],
                ActorSpawnSeq: &self.columns[914],
                ActorDespawnSeq: &self.columns[915],
                Unknown0: &self.columns[916],
                Unknown1: &self.columns[917],
                QuestUInt8A: &self.columns[918],
                ConditionType: &self.columns[919],
                ConditionOperator: &self.columns[920],
                VisibleBool: &self.columns[921],
                ConditionBool: &self.columns[922],
                ItemBool: &self.columns[923],
                AnnounceBool: &self.columns[924],
                BehaviorBool: &self.columns[925],
                AcceptBool: &self.columns[926],
                QualifiedBool: &self.columns[927],
                CanTargetBool: &self.columns[928],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[929],
                ConditionValue: &self.columns[930],
                Behavior: &self.columns[931],
                ActorSpawnSeq: &self.columns[932],
                ActorDespawnSeq: &self.columns[933],
                Unknown0: &self.columns[934],
                Unknown1: &self.columns[935],
                QuestUInt8A: &self.columns[936],
                ConditionType: &self.columns[937],
                ConditionOperator: &self.columns[938],
                VisibleBool: &self.columns[939],
                ConditionBool: &self.columns[940],
                ItemBool: &self.columns[941],
                AnnounceBool: &self.columns[942],
                BehaviorBool: &self.columns[943],
                AcceptBool: &self.columns[944],
                QualifiedBool: &self.columns[945],
                CanTargetBool: &self.columns[946],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[947],
                ConditionValue: &self.columns[948],
                Behavior: &self.columns[949],
                ActorSpawnSeq: &self.columns[950],
                ActorDespawnSeq: &self.columns[951],
                Unknown0: &self.columns[952],
                Unknown1: &self.columns[953],
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
                Unknown0: &self.columns[970],
                Unknown1: &self.columns[971],
                QuestUInt8A: &self.columns[972],
                ConditionType: &self.columns[973],
                ConditionOperator: &self.columns[974],
                VisibleBool: &self.columns[975],
                ConditionBool: &self.columns[976],
                ItemBool: &self.columns[977],
                AnnounceBool: &self.columns[978],
                BehaviorBool: &self.columns[979],
                AcceptBool: &self.columns[980],
                QualifiedBool: &self.columns[981],
                CanTargetBool: &self.columns[982],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[983],
                ConditionValue: &self.columns[984],
                Behavior: &self.columns[985],
                ActorSpawnSeq: &self.columns[986],
                ActorDespawnSeq: &self.columns[987],
                Unknown0: &self.columns[988],
                Unknown1: &self.columns[989],
                QuestUInt8A: &self.columns[990],
                ConditionType: &self.columns[991],
                ConditionOperator: &self.columns[992],
                VisibleBool: &self.columns[993],
                ConditionBool: &self.columns[994],
                ItemBool: &self.columns[995],
                AnnounceBool: &self.columns[996],
                BehaviorBool: &self.columns[997],
                AcceptBool: &self.columns[998],
                QualifiedBool: &self.columns[999],
                CanTargetBool: &self.columns[1000],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1001],
                ConditionValue: &self.columns[1002],
                Behavior: &self.columns[1003],
                ActorSpawnSeq: &self.columns[1004],
                ActorDespawnSeq: &self.columns[1005],
                Unknown0: &self.columns[1006],
                Unknown1: &self.columns[1007],
                QuestUInt8A: &self.columns[1008],
                ConditionType: &self.columns[1009],
                ConditionOperator: &self.columns[1010],
                VisibleBool: &self.columns[1011],
                ConditionBool: &self.columns[1012],
                ItemBool: &self.columns[1013],
                AnnounceBool: &self.columns[1014],
                BehaviorBool: &self.columns[1015],
                AcceptBool: &self.columns[1016],
                QualifiedBool: &self.columns[1017],
                CanTargetBool: &self.columns[1018],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1019],
                ConditionValue: &self.columns[1020],
                Behavior: &self.columns[1021],
                ActorSpawnSeq: &self.columns[1022],
                ActorDespawnSeq: &self.columns[1023],
                Unknown0: &self.columns[1024],
                Unknown1: &self.columns[1025],
                QuestUInt8A: &self.columns[1026],
                ConditionType: &self.columns[1027],
                ConditionOperator: &self.columns[1028],
                VisibleBool: &self.columns[1029],
                ConditionBool: &self.columns[1030],
                ItemBool: &self.columns[1031],
                AnnounceBool: &self.columns[1032],
                BehaviorBool: &self.columns[1033],
                AcceptBool: &self.columns[1034],
                QualifiedBool: &self.columns[1035],
                CanTargetBool: &self.columns[1036],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1037],
                ConditionValue: &self.columns[1038],
                Behavior: &self.columns[1039],
                ActorSpawnSeq: &self.columns[1040],
                ActorDespawnSeq: &self.columns[1041],
                Unknown0: &self.columns[1042],
                Unknown1: &self.columns[1043],
                QuestUInt8A: &self.columns[1044],
                ConditionType: &self.columns[1045],
                ConditionOperator: &self.columns[1046],
                VisibleBool: &self.columns[1047],
                ConditionBool: &self.columns[1048],
                ItemBool: &self.columns[1049],
                AnnounceBool: &self.columns[1050],
                BehaviorBool: &self.columns[1051],
                AcceptBool: &self.columns[1052],
                QualifiedBool: &self.columns[1053],
                CanTargetBool: &self.columns[1054],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1055],
                ConditionValue: &self.columns[1056],
                Behavior: &self.columns[1057],
                ActorSpawnSeq: &self.columns[1058],
                ActorDespawnSeq: &self.columns[1059],
                Unknown0: &self.columns[1060],
                Unknown1: &self.columns[1061],
                QuestUInt8A: &self.columns[1062],
                ConditionType: &self.columns[1063],
                ConditionOperator: &self.columns[1064],
                VisibleBool: &self.columns[1065],
                ConditionBool: &self.columns[1066],
                ItemBool: &self.columns[1067],
                AnnounceBool: &self.columns[1068],
                BehaviorBool: &self.columns[1069],
                AcceptBool: &self.columns[1070],
                QualifiedBool: &self.columns[1071],
                CanTargetBool: &self.columns[1072],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1073],
                ConditionValue: &self.columns[1074],
                Behavior: &self.columns[1075],
                ActorSpawnSeq: &self.columns[1076],
                ActorDespawnSeq: &self.columns[1077],
                Unknown0: &self.columns[1078],
                Unknown1: &self.columns[1079],
                QuestUInt8A: &self.columns[1080],
                ConditionType: &self.columns[1081],
                ConditionOperator: &self.columns[1082],
                VisibleBool: &self.columns[1083],
                ConditionBool: &self.columns[1084],
                ItemBool: &self.columns[1085],
                AnnounceBool: &self.columns[1086],
                BehaviorBool: &self.columns[1087],
                AcceptBool: &self.columns[1088],
                QualifiedBool: &self.columns[1089],
                CanTargetBool: &self.columns[1090],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1091],
                ConditionValue: &self.columns[1092],
                Behavior: &self.columns[1093],
                ActorSpawnSeq: &self.columns[1094],
                ActorDespawnSeq: &self.columns[1095],
                Unknown0: &self.columns[1096],
                Unknown1: &self.columns[1097],
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
                Unknown0: &self.columns[1114],
                Unknown1: &self.columns[1115],
                QuestUInt8A: &self.columns[1116],
                ConditionType: &self.columns[1117],
                ConditionOperator: &self.columns[1118],
                VisibleBool: &self.columns[1119],
                ConditionBool: &self.columns[1120],
                ItemBool: &self.columns[1121],
                AnnounceBool: &self.columns[1122],
                BehaviorBool: &self.columns[1123],
                AcceptBool: &self.columns[1124],
                QualifiedBool: &self.columns[1125],
                CanTargetBool: &self.columns[1126],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1127],
                ConditionValue: &self.columns[1128],
                Behavior: &self.columns[1129],
                ActorSpawnSeq: &self.columns[1130],
                ActorDespawnSeq: &self.columns[1131],
                Unknown0: &self.columns[1132],
                Unknown1: &self.columns[1133],
                QuestUInt8A: &self.columns[1134],
                ConditionType: &self.columns[1135],
                ConditionOperator: &self.columns[1136],
                VisibleBool: &self.columns[1137],
                ConditionBool: &self.columns[1138],
                ItemBool: &self.columns[1139],
                AnnounceBool: &self.columns[1140],
                BehaviorBool: &self.columns[1141],
                AcceptBool: &self.columns[1142],
                QualifiedBool: &self.columns[1143],
                CanTargetBool: &self.columns[1144],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1145],
                ConditionValue: &self.columns[1146],
                Behavior: &self.columns[1147],
                ActorSpawnSeq: &self.columns[1148],
                ActorDespawnSeq: &self.columns[1149],
                Unknown0: &self.columns[1150],
                Unknown1: &self.columns[1151],
                QuestUInt8A: &self.columns[1152],
                ConditionType: &self.columns[1153],
                ConditionOperator: &self.columns[1154],
                VisibleBool: &self.columns[1155],
                ConditionBool: &self.columns[1156],
                ItemBool: &self.columns[1157],
                AnnounceBool: &self.columns[1158],
                BehaviorBool: &self.columns[1159],
                AcceptBool: &self.columns[1160],
                QualifiedBool: &self.columns[1161],
                CanTargetBool: &self.columns[1162],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1163],
                ConditionValue: &self.columns[1164],
                Behavior: &self.columns[1165],
                ActorSpawnSeq: &self.columns[1166],
                ActorDespawnSeq: &self.columns[1167],
                Unknown0: &self.columns[1168],
                Unknown1: &self.columns[1169],
                QuestUInt8A: &self.columns[1170],
                ConditionType: &self.columns[1171],
                ConditionOperator: &self.columns[1172],
                VisibleBool: &self.columns[1173],
                ConditionBool: &self.columns[1174],
                ItemBool: &self.columns[1175],
                AnnounceBool: &self.columns[1176],
                BehaviorBool: &self.columns[1177],
                AcceptBool: &self.columns[1178],
                QualifiedBool: &self.columns[1179],
                CanTargetBool: &self.columns[1180],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1181],
                ConditionValue: &self.columns[1182],
                Behavior: &self.columns[1183],
                ActorSpawnSeq: &self.columns[1184],
                ActorDespawnSeq: &self.columns[1185],
                Unknown0: &self.columns[1186],
                Unknown1: &self.columns[1187],
                QuestUInt8A: &self.columns[1188],
                ConditionType: &self.columns[1189],
                ConditionOperator: &self.columns[1190],
                VisibleBool: &self.columns[1191],
                ConditionBool: &self.columns[1192],
                ItemBool: &self.columns[1193],
                AnnounceBool: &self.columns[1194],
                BehaviorBool: &self.columns[1195],
                AcceptBool: &self.columns[1196],
                QualifiedBool: &self.columns[1197],
                CanTargetBool: &self.columns[1198],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1199],
                ConditionValue: &self.columns[1200],
                Behavior: &self.columns[1201],
                ActorSpawnSeq: &self.columns[1202],
                ActorDespawnSeq: &self.columns[1203],
                Unknown0: &self.columns[1204],
                Unknown1: &self.columns[1205],
                QuestUInt8A: &self.columns[1206],
                ConditionType: &self.columns[1207],
                ConditionOperator: &self.columns[1208],
                VisibleBool: &self.columns[1209],
                ConditionBool: &self.columns[1210],
                ItemBool: &self.columns[1211],
                AnnounceBool: &self.columns[1212],
                BehaviorBool: &self.columns[1213],
                AcceptBool: &self.columns[1214],
                QualifiedBool: &self.columns[1215],
                CanTargetBool: &self.columns[1216],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1217],
                ConditionValue: &self.columns[1218],
                Behavior: &self.columns[1219],
                ActorSpawnSeq: &self.columns[1220],
                ActorDespawnSeq: &self.columns[1221],
                Unknown0: &self.columns[1222],
                Unknown1: &self.columns[1223],
                QuestUInt8A: &self.columns[1224],
                ConditionType: &self.columns[1225],
                ConditionOperator: &self.columns[1226],
                VisibleBool: &self.columns[1227],
                ConditionBool: &self.columns[1228],
                ItemBool: &self.columns[1229],
                AnnounceBool: &self.columns[1230],
                BehaviorBool: &self.columns[1231],
                AcceptBool: &self.columns[1232],
                QualifiedBool: &self.columns[1233],
                CanTargetBool: &self.columns[1234],
            },
            QuestListenerParamsElement {
                Listener: &self.columns[1235],
                ConditionValue: &self.columns[1236],
                Behavior: &self.columns[1237],
                ActorSpawnSeq: &self.columns[1238],
                ActorDespawnSeq: &self.columns[1239],
                Unknown0: &self.columns[1240],
                Unknown1: &self.columns[1241],
                QuestUInt8A: &self.columns[1242],
                ConditionType: &self.columns[1243],
                ConditionOperator: &self.columns[1244],
                VisibleBool: &self.columns[1245],
                ConditionBool: &self.columns[1246],
                ItemBool: &self.columns[1247],
                AnnounceBool: &self.columns[1248],
                BehaviorBool: &self.columns[1249],
                AcceptBool: &self.columns[1250],
                QualifiedBool: &self.columns[1251],
                CanTargetBool: &self.columns[1252],
            },
        ]
    }
    pub fn TodoParams<'a>(&'a self) -> [TodoParamsElement<'a>; 24] {
        [
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1253],
                    &self.columns[1254],
                    &self.columns[1255],
                    &self.columns[1256],
                    &self.columns[1257],
                    &self.columns[1258],
                    &self.columns[1259],
                    &self.columns[1260],
                ],
                ToDoCompleteSeq: &self.columns[1261],
                ToDoQty: &self.columns[1262],
                CountableNum: &self.columns[1263],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1264],
                    &self.columns[1265],
                    &self.columns[1266],
                    &self.columns[1267],
                    &self.columns[1268],
                    &self.columns[1269],
                    &self.columns[1270],
                    &self.columns[1271],
                ],
                ToDoCompleteSeq: &self.columns[1272],
                ToDoQty: &self.columns[1273],
                CountableNum: &self.columns[1274],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1275],
                    &self.columns[1276],
                    &self.columns[1277],
                    &self.columns[1278],
                    &self.columns[1279],
                    &self.columns[1280],
                    &self.columns[1281],
                    &self.columns[1282],
                ],
                ToDoCompleteSeq: &self.columns[1283],
                ToDoQty: &self.columns[1284],
                CountableNum: &self.columns[1285],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1286],
                    &self.columns[1287],
                    &self.columns[1288],
                    &self.columns[1289],
                    &self.columns[1290],
                    &self.columns[1291],
                    &self.columns[1292],
                    &self.columns[1293],
                ],
                ToDoCompleteSeq: &self.columns[1294],
                ToDoQty: &self.columns[1295],
                CountableNum: &self.columns[1296],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1297],
                    &self.columns[1298],
                    &self.columns[1299],
                    &self.columns[1300],
                    &self.columns[1301],
                    &self.columns[1302],
                    &self.columns[1303],
                    &self.columns[1304],
                ],
                ToDoCompleteSeq: &self.columns[1305],
                ToDoQty: &self.columns[1306],
                CountableNum: &self.columns[1307],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1308],
                    &self.columns[1309],
                    &self.columns[1310],
                    &self.columns[1311],
                    &self.columns[1312],
                    &self.columns[1313],
                    &self.columns[1314],
                    &self.columns[1315],
                ],
                ToDoCompleteSeq: &self.columns[1316],
                ToDoQty: &self.columns[1317],
                CountableNum: &self.columns[1318],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1319],
                    &self.columns[1320],
                    &self.columns[1321],
                    &self.columns[1322],
                    &self.columns[1323],
                    &self.columns[1324],
                    &self.columns[1325],
                    &self.columns[1326],
                ],
                ToDoCompleteSeq: &self.columns[1327],
                ToDoQty: &self.columns[1328],
                CountableNum: &self.columns[1329],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1330],
                    &self.columns[1331],
                    &self.columns[1332],
                    &self.columns[1333],
                    &self.columns[1334],
                    &self.columns[1335],
                    &self.columns[1336],
                    &self.columns[1337],
                ],
                ToDoCompleteSeq: &self.columns[1338],
                ToDoQty: &self.columns[1339],
                CountableNum: &self.columns[1340],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1341],
                    &self.columns[1342],
                    &self.columns[1343],
                    &self.columns[1344],
                    &self.columns[1345],
                    &self.columns[1346],
                    &self.columns[1347],
                    &self.columns[1348],
                ],
                ToDoCompleteSeq: &self.columns[1349],
                ToDoQty: &self.columns[1350],
                CountableNum: &self.columns[1351],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1352],
                    &self.columns[1353],
                    &self.columns[1354],
                    &self.columns[1355],
                    &self.columns[1356],
                    &self.columns[1357],
                    &self.columns[1358],
                    &self.columns[1359],
                ],
                ToDoCompleteSeq: &self.columns[1360],
                ToDoQty: &self.columns[1361],
                CountableNum: &self.columns[1362],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1363],
                    &self.columns[1364],
                    &self.columns[1365],
                    &self.columns[1366],
                    &self.columns[1367],
                    &self.columns[1368],
                    &self.columns[1369],
                    &self.columns[1370],
                ],
                ToDoCompleteSeq: &self.columns[1371],
                ToDoQty: &self.columns[1372],
                CountableNum: &self.columns[1373],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1374],
                    &self.columns[1375],
                    &self.columns[1376],
                    &self.columns[1377],
                    &self.columns[1378],
                    &self.columns[1379],
                    &self.columns[1380],
                    &self.columns[1381],
                ],
                ToDoCompleteSeq: &self.columns[1382],
                ToDoQty: &self.columns[1383],
                CountableNum: &self.columns[1384],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1385],
                    &self.columns[1386],
                    &self.columns[1387],
                    &self.columns[1388],
                    &self.columns[1389],
                    &self.columns[1390],
                    &self.columns[1391],
                    &self.columns[1392],
                ],
                ToDoCompleteSeq: &self.columns[1393],
                ToDoQty: &self.columns[1394],
                CountableNum: &self.columns[1395],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1396],
                    &self.columns[1397],
                    &self.columns[1398],
                    &self.columns[1399],
                    &self.columns[1400],
                    &self.columns[1401],
                    &self.columns[1402],
                    &self.columns[1403],
                ],
                ToDoCompleteSeq: &self.columns[1404],
                ToDoQty: &self.columns[1405],
                CountableNum: &self.columns[1406],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1407],
                    &self.columns[1408],
                    &self.columns[1409],
                    &self.columns[1410],
                    &self.columns[1411],
                    &self.columns[1412],
                    &self.columns[1413],
                    &self.columns[1414],
                ],
                ToDoCompleteSeq: &self.columns[1415],
                ToDoQty: &self.columns[1416],
                CountableNum: &self.columns[1417],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1418],
                    &self.columns[1419],
                    &self.columns[1420],
                    &self.columns[1421],
                    &self.columns[1422],
                    &self.columns[1423],
                    &self.columns[1424],
                    &self.columns[1425],
                ],
                ToDoCompleteSeq: &self.columns[1426],
                ToDoQty: &self.columns[1427],
                CountableNum: &self.columns[1428],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1429],
                    &self.columns[1430],
                    &self.columns[1431],
                    &self.columns[1432],
                    &self.columns[1433],
                    &self.columns[1434],
                    &self.columns[1435],
                    &self.columns[1436],
                ],
                ToDoCompleteSeq: &self.columns[1437],
                ToDoQty: &self.columns[1438],
                CountableNum: &self.columns[1439],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1440],
                    &self.columns[1441],
                    &self.columns[1442],
                    &self.columns[1443],
                    &self.columns[1444],
                    &self.columns[1445],
                    &self.columns[1446],
                    &self.columns[1447],
                ],
                ToDoCompleteSeq: &self.columns[1448],
                ToDoQty: &self.columns[1449],
                CountableNum: &self.columns[1450],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1451],
                    &self.columns[1452],
                    &self.columns[1453],
                    &self.columns[1454],
                    &self.columns[1455],
                    &self.columns[1456],
                    &self.columns[1457],
                    &self.columns[1458],
                ],
                ToDoCompleteSeq: &self.columns[1459],
                ToDoQty: &self.columns[1460],
                CountableNum: &self.columns[1461],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1462],
                    &self.columns[1463],
                    &self.columns[1464],
                    &self.columns[1465],
                    &self.columns[1466],
                    &self.columns[1467],
                    &self.columns[1468],
                    &self.columns[1469],
                ],
                ToDoCompleteSeq: &self.columns[1470],
                ToDoQty: &self.columns[1471],
                CountableNum: &self.columns[1472],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1473],
                    &self.columns[1474],
                    &self.columns[1475],
                    &self.columns[1476],
                    &self.columns[1477],
                    &self.columns[1478],
                    &self.columns[1479],
                    &self.columns[1480],
                ],
                ToDoCompleteSeq: &self.columns[1481],
                ToDoQty: &self.columns[1482],
                CountableNum: &self.columns[1483],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1484],
                    &self.columns[1485],
                    &self.columns[1486],
                    &self.columns[1487],
                    &self.columns[1488],
                    &self.columns[1489],
                    &self.columns[1490],
                    &self.columns[1491],
                ],
                ToDoCompleteSeq: &self.columns[1492],
                ToDoQty: &self.columns[1493],
                CountableNum: &self.columns[1494],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1495],
                    &self.columns[1496],
                    &self.columns[1497],
                    &self.columns[1498],
                    &self.columns[1499],
                    &self.columns[1500],
                    &self.columns[1501],
                    &self.columns[1502],
                ],
                ToDoCompleteSeq: &self.columns[1503],
                ToDoQty: &self.columns[1504],
                CountableNum: &self.columns[1505],
            },
            TodoParamsElement {
                ToDoLocation: [
                    &self.columns[1506],
                    &self.columns[1507],
                    &self.columns[1508],
                    &self.columns[1509],
                    &self.columns[1510],
                    &self.columns[1511],
                    &self.columns[1512],
                    &self.columns[1513],
                ],
                ToDoCompleteSeq: &self.columns[1514],
                ToDoQty: &self.columns[1515],
                CountableNum: &self.columns[1516],
            },
        ]
    }
    pub fn GilReward<'a>(&'a self) -> &'a Field {
        &self.columns[1517]
    }
    pub fn CurrencyReward<'a>(&'a self) -> &'a Field {
        &self.columns[1518]
    }
    pub fn CurrencyRewardCount<'a>(&'a self) -> &'a Field {
        &self.columns[1519]
    }
    pub fn Reward<'a>(&'a self) -> [&'a Field; 7] {
        [
            &self.columns[1520],
            &self.columns[1521],
            &self.columns[1522],
            &self.columns[1523],
            &self.columns[1524],
            &self.columns[1525],
            &self.columns[1526],
        ]
    }
    pub fn OptionalItemReward<'a>(&'a self) -> [&'a Field; 5] {
        [
            &self.columns[1527],
            &self.columns[1528],
            &self.columns[1529],
            &self.columns[1530],
            &self.columns[1531],
        ]
    }
    pub fn InstanceContentUnlock<'a>(&'a self) -> &'a Field {
        &self.columns[1532]
    }
    pub fn ExpFactor<'a>(&'a self) -> &'a Field {
        &self.columns[1533]
    }
    pub fn EmoteReward<'a>(&'a self) -> &'a Field {
        &self.columns[1534]
    }
    pub fn ActionReward<'a>(&'a self) -> &'a Field {
        &self.columns[1535]
    }
    pub fn SystemReward<'a>(&'a self) -> [&'a Field; 2] {
        [&self.columns[1536], &self.columns[1537]]
    }
    pub fn GCTypeReward<'a>(&'a self) -> &'a Field {
        &self.columns[1538]
    }
    pub fn ItemCatalyst<'a>(&'a self) -> [&'a Field; 3] {
        [&self.columns[1539], &self.columns[1540], &self.columns[1541]]
    }
    pub fn ItemCountCatalyst<'a>(&'a self) -> [&'a Field; 3] {
        [&self.columns[1542], &self.columns[1543], &self.columns[1544]]
    }
    pub fn ItemRewardType<'a>(&'a self) -> &'a Field {
        &self.columns[1545]
    }
    pub fn ItemCountReward<'a>(&'a self) -> [&'a Field; 7] {
        [
            &self.columns[1546],
            &self.columns[1547],
            &self.columns[1548],
            &self.columns[1549],
            &self.columns[1550],
            &self.columns[1551],
            &self.columns[1552],
        ]
    }
    pub fn RewardStain<'a>(&'a self) -> [&'a Field; 7] {
        [
            &self.columns[1553],
            &self.columns[1554],
            &self.columns[1555],
            &self.columns[1556],
            &self.columns[1557],
            &self.columns[1558],
            &self.columns[1559],
        ]
    }
    pub fn OptionalItemCountReward<'a>(&'a self) -> [&'a Field; 5] {
        [
            &self.columns[1560],
            &self.columns[1561],
            &self.columns[1562],
            &self.columns[1563],
            &self.columns[1564],
        ]
    }
    pub fn OptionalItemStainReward<'a>(&'a self) -> [&'a Field; 5] {
        [
            &self.columns[1565],
            &self.columns[1566],
            &self.columns[1567],
            &self.columns[1568],
            &self.columns[1569],
        ]
    }
    pub fn GeneralActionReward<'a>(&'a self) -> [&'a Field; 2] {
        [&self.columns[1570], &self.columns[1571]]
    }
    pub fn OtherReward<'a>(&'a self) -> &'a Field {
        &self.columns[1572]
    }
    pub fn Tomestone<'a>(&'a self) -> &'a Field {
        &self.columns[1573]
    }
    pub fn TomestoneReward<'a>(&'a self) -> &'a Field {
        &self.columns[1574]
    }
    pub fn TomestoneCountReward<'a>(&'a self) -> &'a Field {
        &self.columns[1575]
    }
    pub fn ReputationReward<'a>(&'a self) -> &'a Field {
        &self.columns[1576]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[1577]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[1578]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[1579]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a Field {
        &self.columns[1580]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a Field {
        &self.columns[1581]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a Field {
        &self.columns[1582]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a Field {
        &self.columns[1583]
    }
    pub fn OptionalItemIsHQReward<'a>(&'a self) -> [&'a Field; 5] {
        [
            &self.columns[1584],
            &self.columns[1585],
            &self.columns[1586],
            &self.columns[1587],
            &self.columns[1588],
        ]
    }
    pub fn Id<'a>(&'a self) -> &'a Field {
        &self.columns[1589]
    }
    pub fn PreviousQuest<'a>(&'a self) -> [&'a Field; 3] {
        [&self.columns[1590], &self.columns[1591], &self.columns[1592]]
    }
    pub fn QuestLock<'a>(&'a self) -> [&'a Field; 2] {
        [&self.columns[1593], &self.columns[1594]]
    }
    pub fn InstanceContent<'a>(&'a self) -> [&'a Field; 3] {
        [&self.columns[1595], &self.columns[1596], &self.columns[1597]]
    }
    pub fn IssuerStart<'a>(&'a self) -> &'a Field {
        &self.columns[1598]
    }
    pub fn IssuerLocation<'a>(&'a self) -> &'a Field {
        &self.columns[1599]
    }
    pub fn TargetEnd<'a>(&'a self) -> &'a Field {
        &self.columns[1600]
    }
    pub fn JournalGenre<'a>(&'a self) -> &'a Field {
        &self.columns[1601]
    }
    pub fn Icon<'a>(&'a self) -> &'a Field {
        &self.columns[1602]
    }
    pub fn IconSpecial<'a>(&'a self) -> &'a Field {
        &self.columns[1603]
    }
    pub fn MountRequired<'a>(&'a self) -> &'a Field {
        &self.columns[1604]
    }
    pub fn ClassJobLevel<'a>(&'a self) -> [&'a Field; 2] {
        [&self.columns[1605], &self.columns[1606]]
    }
    pub fn Header<'a>(&'a self) -> &'a Field {
        &self.columns[1607]
    }
    pub fn Festival<'a>(&'a self) -> &'a Field {
        &self.columns[1608]
    }
    pub fn BellStart<'a>(&'a self) -> &'a Field {
        &self.columns[1609]
    }
    pub fn BellEnd<'a>(&'a self) -> &'a Field {
        &self.columns[1610]
    }
    pub fn BeastReputationValue<'a>(&'a self) -> &'a Field {
        &self.columns[1611]
    }
    pub fn ClientBehavior<'a>(&'a self) -> &'a Field {
        &self.columns[1612]
    }
    pub fn QuestClassJobSupply<'a>(&'a self) -> &'a Field {
        &self.columns[1613]
    }
    pub fn PlaceName<'a>(&'a self) -> &'a Field {
        &self.columns[1614]
    }
    pub fn SortKey<'a>(&'a self) -> &'a Field {
        &self.columns[1615]
    }
    pub fn Expansion<'a>(&'a self) -> &'a Field {
        &self.columns[1616]
    }
    pub fn ClassJobCategory0<'a>(&'a self) -> &'a Field {
        &self.columns[1617]
    }
    pub fn QuestLevelOffset<'a>(&'a self) -> &'a Field {
        &self.columns[1618]
    }
    pub fn ClassJobCategory1<'a>(&'a self) -> &'a Field {
        &self.columns[1619]
    }
    pub fn PreviousQuestJoin<'a>(&'a self) -> &'a Field {
        &self.columns[1620]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a Field {
        &self.columns[1621]
    }
    pub fn QuestLockJoin<'a>(&'a self) -> &'a Field {
        &self.columns[1622]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a Field {
        &self.columns[1623]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a Field {
        &self.columns[1624]
    }
    pub fn ClassJobUnlock<'a>(&'a self) -> &'a Field {
        &self.columns[1625]
    }
    pub fn GrandCompany<'a>(&'a self) -> &'a Field {
        &self.columns[1626]
    }
    pub fn GrandCompanyRank<'a>(&'a self) -> &'a Field {
        &self.columns[1627]
    }
    pub fn InstanceContentJoin<'a>(&'a self) -> &'a Field {
        &self.columns[1628]
    }
    pub fn FestivalBegin<'a>(&'a self) -> &'a Field {
        &self.columns[1629]
    }
    pub fn FestivalEnd<'a>(&'a self) -> &'a Field {
        &self.columns[1630]
    }
    pub fn BeastTribe<'a>(&'a self) -> &'a Field {
        &self.columns[1631]
    }
    pub fn BeastReputationRank<'a>(&'a self) -> &'a Field {
        &self.columns[1632]
    }
    pub fn SatisfactionNpc<'a>(&'a self) -> &'a Field {
        &self.columns[1633]
    }
    pub fn SatisfactionLevel<'a>(&'a self) -> &'a Field {
        &self.columns[1634]
    }
    pub fn DeliveryQuest<'a>(&'a self) -> &'a Field {
        &self.columns[1635]
    }
    pub fn RepeatIntervalType<'a>(&'a self) -> &'a Field {
        &self.columns[1636]
    }
    pub fn QuestRepeatFlag<'a>(&'a self) -> &'a Field {
        &self.columns[1637]
    }
    pub fn Type<'a>(&'a self) -> &'a Field {
        &self.columns[1638]
    }
    pub fn Unknown_70<'a>(&'a self) -> &'a Field {
        &self.columns[1639]
    }
    pub fn LevelMax<'a>(&'a self) -> &'a Field {
        &self.columns[1640]
    }
    pub fn ClassJobRequired<'a>(&'a self) -> &'a Field {
        &self.columns[1641]
    }
    pub fn QuestRewardOtherDisplay<'a>(&'a self) -> &'a Field {
        &self.columns[1642]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a Field {
        &self.columns[1643]
    }
    pub fn EventIconType<'a>(&'a self) -> &'a Field {
        &self.columns[1644]
    }
    /// 1/2 - normal daily beast tribe quests, 3 - 'exclusive' (if player's rank is not greater than max rank requirement of quests offered by npc, exactly one of the available quests will be from this pool)
    pub fn DailyQuestPool<'a>(&'a self) -> &'a Field {
        &self.columns[1645]
    }
    pub fn IsHouseRequired<'a>(&'a self) -> &'a Field {
        &self.columns[1646]
    }
    pub fn IsRepeatable<'a>(&'a self) -> &'a Field {
        &self.columns[1647]
    }
    pub fn CanCancel<'a>(&'a self) -> &'a Field {
        &self.columns[1648]
    }
    pub fn Introduction<'a>(&'a self) -> &'a Field {
        &self.columns[1649]
    }
    pub fn HideOfferIcon<'a>(&'a self) -> &'a Field {
        &self.columns[1650]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a Field {
        &self.columns[1651]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a Field {
        &self.columns[1652]
    }
}
