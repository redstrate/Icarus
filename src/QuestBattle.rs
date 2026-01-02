//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{ExcelSheet, ColumnData, ExcelRowKind, ExcelSingleRow},
    common::Language,
};
pub struct QuestBattleParamsElement<'a> {
    pub ScriptInstruction: &'a ColumnData,
    pub ScriptValue: &'a ColumnData,
}
pub struct QuestBattleSheet {
    sheet: ExcelSheet,
}
impl QuestBattleSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("QuestBattle")?;
        let sheet = resolver.read_excel_sheet(exh, "QuestBattle", language)?;
        Ok(Self { sheet })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<QuestBattleRow> {
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
        Some(QuestBattleRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<QuestBattleRow> {
        let row = &self.sheet.get_row(row_id)?;
        let row = match row {
            ExcelRowKind::SingleRow(row) => row,
            ExcelRowKind::SubRows(rows) => &rows.first()?.1,
        };
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn get_subrow(&self, row_id: u32, subrow_id: u16) -> Option<QuestBattleRow> {
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
pub struct QuestBattleRow {
    columns: Vec<ColumnData>,
}
impl QuestBattleRow {
    pub fn QuestBattleParams<'a>(&'a self) -> [QuestBattleParamsElement<'a>; 220] {
        [
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[0],
                ScriptValue: &self.columns[1],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[2],
                ScriptValue: &self.columns[3],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[4],
                ScriptValue: &self.columns[5],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[6],
                ScriptValue: &self.columns[7],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[8],
                ScriptValue: &self.columns[9],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[10],
                ScriptValue: &self.columns[11],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[12],
                ScriptValue: &self.columns[13],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[14],
                ScriptValue: &self.columns[15],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[16],
                ScriptValue: &self.columns[17],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[18],
                ScriptValue: &self.columns[19],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[20],
                ScriptValue: &self.columns[21],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[22],
                ScriptValue: &self.columns[23],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[24],
                ScriptValue: &self.columns[25],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[26],
                ScriptValue: &self.columns[27],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[28],
                ScriptValue: &self.columns[29],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[30],
                ScriptValue: &self.columns[31],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[32],
                ScriptValue: &self.columns[33],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[34],
                ScriptValue: &self.columns[35],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[36],
                ScriptValue: &self.columns[37],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[38],
                ScriptValue: &self.columns[39],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[40],
                ScriptValue: &self.columns[41],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[42],
                ScriptValue: &self.columns[43],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[44],
                ScriptValue: &self.columns[45],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[46],
                ScriptValue: &self.columns[47],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[48],
                ScriptValue: &self.columns[49],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[50],
                ScriptValue: &self.columns[51],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[52],
                ScriptValue: &self.columns[53],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[54],
                ScriptValue: &self.columns[55],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[56],
                ScriptValue: &self.columns[57],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[58],
                ScriptValue: &self.columns[59],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[60],
                ScriptValue: &self.columns[61],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[62],
                ScriptValue: &self.columns[63],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[64],
                ScriptValue: &self.columns[65],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[66],
                ScriptValue: &self.columns[67],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[68],
                ScriptValue: &self.columns[69],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[70],
                ScriptValue: &self.columns[71],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[72],
                ScriptValue: &self.columns[73],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[74],
                ScriptValue: &self.columns[75],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[76],
                ScriptValue: &self.columns[77],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[78],
                ScriptValue: &self.columns[79],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[80],
                ScriptValue: &self.columns[81],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[82],
                ScriptValue: &self.columns[83],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[84],
                ScriptValue: &self.columns[85],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[86],
                ScriptValue: &self.columns[87],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[88],
                ScriptValue: &self.columns[89],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[90],
                ScriptValue: &self.columns[91],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[92],
                ScriptValue: &self.columns[93],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[94],
                ScriptValue: &self.columns[95],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[96],
                ScriptValue: &self.columns[97],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[98],
                ScriptValue: &self.columns[99],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[100],
                ScriptValue: &self.columns[101],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[102],
                ScriptValue: &self.columns[103],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[104],
                ScriptValue: &self.columns[105],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[106],
                ScriptValue: &self.columns[107],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[108],
                ScriptValue: &self.columns[109],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[110],
                ScriptValue: &self.columns[111],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[112],
                ScriptValue: &self.columns[113],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[114],
                ScriptValue: &self.columns[115],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[116],
                ScriptValue: &self.columns[117],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[118],
                ScriptValue: &self.columns[119],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[120],
                ScriptValue: &self.columns[121],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[122],
                ScriptValue: &self.columns[123],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[124],
                ScriptValue: &self.columns[125],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[126],
                ScriptValue: &self.columns[127],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[128],
                ScriptValue: &self.columns[129],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[130],
                ScriptValue: &self.columns[131],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[132],
                ScriptValue: &self.columns[133],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[134],
                ScriptValue: &self.columns[135],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[136],
                ScriptValue: &self.columns[137],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[138],
                ScriptValue: &self.columns[139],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[140],
                ScriptValue: &self.columns[141],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[142],
                ScriptValue: &self.columns[143],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[144],
                ScriptValue: &self.columns[145],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[146],
                ScriptValue: &self.columns[147],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[148],
                ScriptValue: &self.columns[149],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[150],
                ScriptValue: &self.columns[151],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[152],
                ScriptValue: &self.columns[153],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[154],
                ScriptValue: &self.columns[155],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[156],
                ScriptValue: &self.columns[157],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[158],
                ScriptValue: &self.columns[159],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[160],
                ScriptValue: &self.columns[161],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[162],
                ScriptValue: &self.columns[163],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[164],
                ScriptValue: &self.columns[165],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[166],
                ScriptValue: &self.columns[167],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[168],
                ScriptValue: &self.columns[169],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[170],
                ScriptValue: &self.columns[171],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[172],
                ScriptValue: &self.columns[173],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[174],
                ScriptValue: &self.columns[175],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[176],
                ScriptValue: &self.columns[177],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[178],
                ScriptValue: &self.columns[179],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[180],
                ScriptValue: &self.columns[181],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[182],
                ScriptValue: &self.columns[183],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[184],
                ScriptValue: &self.columns[185],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[186],
                ScriptValue: &self.columns[187],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[188],
                ScriptValue: &self.columns[189],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[190],
                ScriptValue: &self.columns[191],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[192],
                ScriptValue: &self.columns[193],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[194],
                ScriptValue: &self.columns[195],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[196],
                ScriptValue: &self.columns[197],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[198],
                ScriptValue: &self.columns[199],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[200],
                ScriptValue: &self.columns[201],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[202],
                ScriptValue: &self.columns[203],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[204],
                ScriptValue: &self.columns[205],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[206],
                ScriptValue: &self.columns[207],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[208],
                ScriptValue: &self.columns[209],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[210],
                ScriptValue: &self.columns[211],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[212],
                ScriptValue: &self.columns[213],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[214],
                ScriptValue: &self.columns[215],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[216],
                ScriptValue: &self.columns[217],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[218],
                ScriptValue: &self.columns[219],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[220],
                ScriptValue: &self.columns[221],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[222],
                ScriptValue: &self.columns[223],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[224],
                ScriptValue: &self.columns[225],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[226],
                ScriptValue: &self.columns[227],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[228],
                ScriptValue: &self.columns[229],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[230],
                ScriptValue: &self.columns[231],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[232],
                ScriptValue: &self.columns[233],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[234],
                ScriptValue: &self.columns[235],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[236],
                ScriptValue: &self.columns[237],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[238],
                ScriptValue: &self.columns[239],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[240],
                ScriptValue: &self.columns[241],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[242],
                ScriptValue: &self.columns[243],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[244],
                ScriptValue: &self.columns[245],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[246],
                ScriptValue: &self.columns[247],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[248],
                ScriptValue: &self.columns[249],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[250],
                ScriptValue: &self.columns[251],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[252],
                ScriptValue: &self.columns[253],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[254],
                ScriptValue: &self.columns[255],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[256],
                ScriptValue: &self.columns[257],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[258],
                ScriptValue: &self.columns[259],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[260],
                ScriptValue: &self.columns[261],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[262],
                ScriptValue: &self.columns[263],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[264],
                ScriptValue: &self.columns[265],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[266],
                ScriptValue: &self.columns[267],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[268],
                ScriptValue: &self.columns[269],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[270],
                ScriptValue: &self.columns[271],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[272],
                ScriptValue: &self.columns[273],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[274],
                ScriptValue: &self.columns[275],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[276],
                ScriptValue: &self.columns[277],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[278],
                ScriptValue: &self.columns[279],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[280],
                ScriptValue: &self.columns[281],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[282],
                ScriptValue: &self.columns[283],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[284],
                ScriptValue: &self.columns[285],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[286],
                ScriptValue: &self.columns[287],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[288],
                ScriptValue: &self.columns[289],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[290],
                ScriptValue: &self.columns[291],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[292],
                ScriptValue: &self.columns[293],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[294],
                ScriptValue: &self.columns[295],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[296],
                ScriptValue: &self.columns[297],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[298],
                ScriptValue: &self.columns[299],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[300],
                ScriptValue: &self.columns[301],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[302],
                ScriptValue: &self.columns[303],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[304],
                ScriptValue: &self.columns[305],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[306],
                ScriptValue: &self.columns[307],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[308],
                ScriptValue: &self.columns[309],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[310],
                ScriptValue: &self.columns[311],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[312],
                ScriptValue: &self.columns[313],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[314],
                ScriptValue: &self.columns[315],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[316],
                ScriptValue: &self.columns[317],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[318],
                ScriptValue: &self.columns[319],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[320],
                ScriptValue: &self.columns[321],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[322],
                ScriptValue: &self.columns[323],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[324],
                ScriptValue: &self.columns[325],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[326],
                ScriptValue: &self.columns[327],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[328],
                ScriptValue: &self.columns[329],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[330],
                ScriptValue: &self.columns[331],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[332],
                ScriptValue: &self.columns[333],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[334],
                ScriptValue: &self.columns[335],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[336],
                ScriptValue: &self.columns[337],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[338],
                ScriptValue: &self.columns[339],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[340],
                ScriptValue: &self.columns[341],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[342],
                ScriptValue: &self.columns[343],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[344],
                ScriptValue: &self.columns[345],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[346],
                ScriptValue: &self.columns[347],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[348],
                ScriptValue: &self.columns[349],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[350],
                ScriptValue: &self.columns[351],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[352],
                ScriptValue: &self.columns[353],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[354],
                ScriptValue: &self.columns[355],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[356],
                ScriptValue: &self.columns[357],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[358],
                ScriptValue: &self.columns[359],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[360],
                ScriptValue: &self.columns[361],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[362],
                ScriptValue: &self.columns[363],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[364],
                ScriptValue: &self.columns[365],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[366],
                ScriptValue: &self.columns[367],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[368],
                ScriptValue: &self.columns[369],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[370],
                ScriptValue: &self.columns[371],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[372],
                ScriptValue: &self.columns[373],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[374],
                ScriptValue: &self.columns[375],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[376],
                ScriptValue: &self.columns[377],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[378],
                ScriptValue: &self.columns[379],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[380],
                ScriptValue: &self.columns[381],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[382],
                ScriptValue: &self.columns[383],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[384],
                ScriptValue: &self.columns[385],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[386],
                ScriptValue: &self.columns[387],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[388],
                ScriptValue: &self.columns[389],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[390],
                ScriptValue: &self.columns[391],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[392],
                ScriptValue: &self.columns[393],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[394],
                ScriptValue: &self.columns[395],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[396],
                ScriptValue: &self.columns[397],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[398],
                ScriptValue: &self.columns[399],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[400],
                ScriptValue: &self.columns[401],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[402],
                ScriptValue: &self.columns[403],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[404],
                ScriptValue: &self.columns[405],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[406],
                ScriptValue: &self.columns[407],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[408],
                ScriptValue: &self.columns[409],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[410],
                ScriptValue: &self.columns[411],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[412],
                ScriptValue: &self.columns[413],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[414],
                ScriptValue: &self.columns[415],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[416],
                ScriptValue: &self.columns[417],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[418],
                ScriptValue: &self.columns[419],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[420],
                ScriptValue: &self.columns[421],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[422],
                ScriptValue: &self.columns[423],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[424],
                ScriptValue: &self.columns[425],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[426],
                ScriptValue: &self.columns[427],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[428],
                ScriptValue: &self.columns[429],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[430],
                ScriptValue: &self.columns[431],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[432],
                ScriptValue: &self.columns[433],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[434],
                ScriptValue: &self.columns[435],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[436],
                ScriptValue: &self.columns[437],
            },
            QuestBattleParamsElement {
                ScriptInstruction: &self.columns[438],
                ScriptValue: &self.columns[439],
            },
        ]
    }
    pub fn Quest<'a>(&'a self) -> &'a ColumnData {
        &self.columns[440]
    }
    pub fn TimeLimit<'a>(&'a self) -> &'a ColumnData {
        &self.columns[441]
    }
    pub fn LevelSync<'a>(&'a self) -> &'a ColumnData {
        &self.columns[442]
    }
    pub fn QuestBattleScene<'a>(&'a self) -> &'a ColumnData {
        &self.columns[443]
    }
}
