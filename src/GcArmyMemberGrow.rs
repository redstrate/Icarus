//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use physis::{
    resource::{Resource, read_excel_sheet_header, read_excel_sheet},
    exd::{EXD, ColumnData, ExcelRowKind, ExcelSingleRow},
    exh::{EXH, ExcelColumnDefinition},
    common::Language,
};
pub struct MemberParamsElement<'a> {
    EquipPreset: &'a ColumnData,
    Physical: &'a ColumnData,
    Mental: &'a ColumnData,
    Tactical: &'a ColumnData,
}
pub struct GcArmyMemberGrowSheet {
    pages: Vec<EXD>,
    exh: EXH,
    row_count: u32,
}
impl GcArmyMemberGrowSheet {
    /// Read the sheet from a `Resource`.
    pub fn read_from<T: Resource>(resource: &mut T, language: Language) -> Option<Self> {
        let exh = read_excel_sheet_header(resource, "GcArmyMemberGrow")?;
        let mut pages = Vec::new();
        for (i, _) in exh.pages.iter().enumerate() {
            pages
                .push(
                    read_excel_sheet(resource, "GcArmyMemberGrow", &exh, language, i)?,
                );
        }
        let row_count = exh.header.row_count;
        Some(Self { exh, pages, row_count })
    }
    fn read_row(&self, row: &ExcelSingleRow) -> Option<GcArmyMemberGrowRow> {
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
        Some(GcArmyMemberGrowRow { columns })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn get_row(&self, row_id: u32) -> Option<GcArmyMemberGrowRow> {
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
    ) -> Option<GcArmyMemberGrowRow> {
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
pub struct GcArmyMemberGrowRow {
    columns: Vec<ColumnData>,
}
impl GcArmyMemberGrowRow {
    pub fn MemberParams<'a>(&'a self) -> [MemberParamsElement<'a>; 60] {
        [
            MemberParamsElement {
                EquipPreset: &self.columns[0],
                Physical: &self.columns[1],
                Mental: &self.columns[2],
                Tactical: &self.columns[3],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[4],
                Physical: &self.columns[5],
                Mental: &self.columns[6],
                Tactical: &self.columns[7],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[8],
                Physical: &self.columns[9],
                Mental: &self.columns[10],
                Tactical: &self.columns[11],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[12],
                Physical: &self.columns[13],
                Mental: &self.columns[14],
                Tactical: &self.columns[15],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[16],
                Physical: &self.columns[17],
                Mental: &self.columns[18],
                Tactical: &self.columns[19],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[20],
                Physical: &self.columns[21],
                Mental: &self.columns[22],
                Tactical: &self.columns[23],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[24],
                Physical: &self.columns[25],
                Mental: &self.columns[26],
                Tactical: &self.columns[27],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[28],
                Physical: &self.columns[29],
                Mental: &self.columns[30],
                Tactical: &self.columns[31],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[32],
                Physical: &self.columns[33],
                Mental: &self.columns[34],
                Tactical: &self.columns[35],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[36],
                Physical: &self.columns[37],
                Mental: &self.columns[38],
                Tactical: &self.columns[39],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[40],
                Physical: &self.columns[41],
                Mental: &self.columns[42],
                Tactical: &self.columns[43],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[44],
                Physical: &self.columns[45],
                Mental: &self.columns[46],
                Tactical: &self.columns[47],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[48],
                Physical: &self.columns[49],
                Mental: &self.columns[50],
                Tactical: &self.columns[51],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[52],
                Physical: &self.columns[53],
                Mental: &self.columns[54],
                Tactical: &self.columns[55],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[56],
                Physical: &self.columns[57],
                Mental: &self.columns[58],
                Tactical: &self.columns[59],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[60],
                Physical: &self.columns[61],
                Mental: &self.columns[62],
                Tactical: &self.columns[63],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[64],
                Physical: &self.columns[65],
                Mental: &self.columns[66],
                Tactical: &self.columns[67],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[68],
                Physical: &self.columns[69],
                Mental: &self.columns[70],
                Tactical: &self.columns[71],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[72],
                Physical: &self.columns[73],
                Mental: &self.columns[74],
                Tactical: &self.columns[75],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[76],
                Physical: &self.columns[77],
                Mental: &self.columns[78],
                Tactical: &self.columns[79],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[80],
                Physical: &self.columns[81],
                Mental: &self.columns[82],
                Tactical: &self.columns[83],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[84],
                Physical: &self.columns[85],
                Mental: &self.columns[86],
                Tactical: &self.columns[87],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[88],
                Physical: &self.columns[89],
                Mental: &self.columns[90],
                Tactical: &self.columns[91],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[92],
                Physical: &self.columns[93],
                Mental: &self.columns[94],
                Tactical: &self.columns[95],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[96],
                Physical: &self.columns[97],
                Mental: &self.columns[98],
                Tactical: &self.columns[99],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[100],
                Physical: &self.columns[101],
                Mental: &self.columns[102],
                Tactical: &self.columns[103],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[104],
                Physical: &self.columns[105],
                Mental: &self.columns[106],
                Tactical: &self.columns[107],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[108],
                Physical: &self.columns[109],
                Mental: &self.columns[110],
                Tactical: &self.columns[111],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[112],
                Physical: &self.columns[113],
                Mental: &self.columns[114],
                Tactical: &self.columns[115],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[116],
                Physical: &self.columns[117],
                Mental: &self.columns[118],
                Tactical: &self.columns[119],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[120],
                Physical: &self.columns[121],
                Mental: &self.columns[122],
                Tactical: &self.columns[123],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[124],
                Physical: &self.columns[125],
                Mental: &self.columns[126],
                Tactical: &self.columns[127],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[128],
                Physical: &self.columns[129],
                Mental: &self.columns[130],
                Tactical: &self.columns[131],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[132],
                Physical: &self.columns[133],
                Mental: &self.columns[134],
                Tactical: &self.columns[135],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[136],
                Physical: &self.columns[137],
                Mental: &self.columns[138],
                Tactical: &self.columns[139],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[140],
                Physical: &self.columns[141],
                Mental: &self.columns[142],
                Tactical: &self.columns[143],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[144],
                Physical: &self.columns[145],
                Mental: &self.columns[146],
                Tactical: &self.columns[147],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[148],
                Physical: &self.columns[149],
                Mental: &self.columns[150],
                Tactical: &self.columns[151],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[152],
                Physical: &self.columns[153],
                Mental: &self.columns[154],
                Tactical: &self.columns[155],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[156],
                Physical: &self.columns[157],
                Mental: &self.columns[158],
                Tactical: &self.columns[159],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[160],
                Physical: &self.columns[161],
                Mental: &self.columns[162],
                Tactical: &self.columns[163],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[164],
                Physical: &self.columns[165],
                Mental: &self.columns[166],
                Tactical: &self.columns[167],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[168],
                Physical: &self.columns[169],
                Mental: &self.columns[170],
                Tactical: &self.columns[171],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[172],
                Physical: &self.columns[173],
                Mental: &self.columns[174],
                Tactical: &self.columns[175],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[176],
                Physical: &self.columns[177],
                Mental: &self.columns[178],
                Tactical: &self.columns[179],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[180],
                Physical: &self.columns[181],
                Mental: &self.columns[182],
                Tactical: &self.columns[183],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[184],
                Physical: &self.columns[185],
                Mental: &self.columns[186],
                Tactical: &self.columns[187],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[188],
                Physical: &self.columns[189],
                Mental: &self.columns[190],
                Tactical: &self.columns[191],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[192],
                Physical: &self.columns[193],
                Mental: &self.columns[194],
                Tactical: &self.columns[195],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[196],
                Physical: &self.columns[197],
                Mental: &self.columns[198],
                Tactical: &self.columns[199],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[200],
                Physical: &self.columns[201],
                Mental: &self.columns[202],
                Tactical: &self.columns[203],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[204],
                Physical: &self.columns[205],
                Mental: &self.columns[206],
                Tactical: &self.columns[207],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[208],
                Physical: &self.columns[209],
                Mental: &self.columns[210],
                Tactical: &self.columns[211],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[212],
                Physical: &self.columns[213],
                Mental: &self.columns[214],
                Tactical: &self.columns[215],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[216],
                Physical: &self.columns[217],
                Mental: &self.columns[218],
                Tactical: &self.columns[219],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[220],
                Physical: &self.columns[221],
                Mental: &self.columns[222],
                Tactical: &self.columns[223],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[224],
                Physical: &self.columns[225],
                Mental: &self.columns[226],
                Tactical: &self.columns[227],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[228],
                Physical: &self.columns[229],
                Mental: &self.columns[230],
                Tactical: &self.columns[231],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[232],
                Physical: &self.columns[233],
                Mental: &self.columns[234],
                Tactical: &self.columns[235],
            },
            MemberParamsElement {
                EquipPreset: &self.columns[236],
                Physical: &self.columns[237],
                Mental: &self.columns[238],
                Tactical: &self.columns[239],
            },
        ]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a ColumnData {
        &self.columns[240]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a ColumnData {
        &self.columns[241]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a ColumnData {
        &self.columns[242]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a ColumnData {
        &self.columns[243]
    }
    pub fn ClassBook<'a>(&'a self) -> &'a ColumnData {
        &self.columns[244]
    }
    pub fn ClassJob<'a>(&'a self) -> &'a ColumnData {
        &self.columns[245]
    }
}
