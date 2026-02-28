//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct MemberParamsElement<'a> {
    pub EquipPreset: &'a Field,
    pub Physical: &'a Field,
    pub Mental: &'a Field,
    pub Tactical: &'a Field,
}
#[derive(Debug, Clone)]
pub struct GcArmyMemberGrowSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl GcArmyMemberGrowSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("GcArmyMemberGrow")?;
        let sheet = resolver.read_excel_sheet(&exh, "GcArmyMemberGrow", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<GcArmyMemberGrowRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<GcArmyMemberGrowRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for GcArmyMemberGrowSheet {
    type Row = GcArmyMemberGrowRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a GcArmyMemberGrowSheet {
    type Item = (u32, Vec<(u16, GcArmyMemberGrowRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, GcArmyMemberGrowSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, GcArmyMemberGrowSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GcArmyMemberGrowRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> GcArmyMemberGrowRow<'a> {
    pub fn MemberParams(&'a self) -> [MemberParamsElement<'a>; 60] {
        [
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[0]],
                Physical: &self.row.columns[self.index_mapping[1]],
                Mental: &self.row.columns[self.index_mapping[2]],
                Tactical: &self.row.columns[self.index_mapping[3]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[4]],
                Physical: &self.row.columns[self.index_mapping[5]],
                Mental: &self.row.columns[self.index_mapping[6]],
                Tactical: &self.row.columns[self.index_mapping[7]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[8]],
                Physical: &self.row.columns[self.index_mapping[9]],
                Mental: &self.row.columns[self.index_mapping[10]],
                Tactical: &self.row.columns[self.index_mapping[11]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[12]],
                Physical: &self.row.columns[self.index_mapping[13]],
                Mental: &self.row.columns[self.index_mapping[14]],
                Tactical: &self.row.columns[self.index_mapping[15]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[16]],
                Physical: &self.row.columns[self.index_mapping[17]],
                Mental: &self.row.columns[self.index_mapping[18]],
                Tactical: &self.row.columns[self.index_mapping[19]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[20]],
                Physical: &self.row.columns[self.index_mapping[21]],
                Mental: &self.row.columns[self.index_mapping[22]],
                Tactical: &self.row.columns[self.index_mapping[23]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[24]],
                Physical: &self.row.columns[self.index_mapping[25]],
                Mental: &self.row.columns[self.index_mapping[26]],
                Tactical: &self.row.columns[self.index_mapping[27]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[28]],
                Physical: &self.row.columns[self.index_mapping[29]],
                Mental: &self.row.columns[self.index_mapping[30]],
                Tactical: &self.row.columns[self.index_mapping[31]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[32]],
                Physical: &self.row.columns[self.index_mapping[33]],
                Mental: &self.row.columns[self.index_mapping[34]],
                Tactical: &self.row.columns[self.index_mapping[35]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[36]],
                Physical: &self.row.columns[self.index_mapping[37]],
                Mental: &self.row.columns[self.index_mapping[38]],
                Tactical: &self.row.columns[self.index_mapping[39]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[40]],
                Physical: &self.row.columns[self.index_mapping[41]],
                Mental: &self.row.columns[self.index_mapping[42]],
                Tactical: &self.row.columns[self.index_mapping[43]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[44]],
                Physical: &self.row.columns[self.index_mapping[45]],
                Mental: &self.row.columns[self.index_mapping[46]],
                Tactical: &self.row.columns[self.index_mapping[47]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[48]],
                Physical: &self.row.columns[self.index_mapping[49]],
                Mental: &self.row.columns[self.index_mapping[50]],
                Tactical: &self.row.columns[self.index_mapping[51]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[52]],
                Physical: &self.row.columns[self.index_mapping[53]],
                Mental: &self.row.columns[self.index_mapping[54]],
                Tactical: &self.row.columns[self.index_mapping[55]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[56]],
                Physical: &self.row.columns[self.index_mapping[57]],
                Mental: &self.row.columns[self.index_mapping[58]],
                Tactical: &self.row.columns[self.index_mapping[59]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[60]],
                Physical: &self.row.columns[self.index_mapping[61]],
                Mental: &self.row.columns[self.index_mapping[62]],
                Tactical: &self.row.columns[self.index_mapping[63]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[64]],
                Physical: &self.row.columns[self.index_mapping[65]],
                Mental: &self.row.columns[self.index_mapping[66]],
                Tactical: &self.row.columns[self.index_mapping[67]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[68]],
                Physical: &self.row.columns[self.index_mapping[69]],
                Mental: &self.row.columns[self.index_mapping[70]],
                Tactical: &self.row.columns[self.index_mapping[71]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[72]],
                Physical: &self.row.columns[self.index_mapping[73]],
                Mental: &self.row.columns[self.index_mapping[74]],
                Tactical: &self.row.columns[self.index_mapping[75]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[76]],
                Physical: &self.row.columns[self.index_mapping[77]],
                Mental: &self.row.columns[self.index_mapping[78]],
                Tactical: &self.row.columns[self.index_mapping[79]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[80]],
                Physical: &self.row.columns[self.index_mapping[81]],
                Mental: &self.row.columns[self.index_mapping[82]],
                Tactical: &self.row.columns[self.index_mapping[83]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[84]],
                Physical: &self.row.columns[self.index_mapping[85]],
                Mental: &self.row.columns[self.index_mapping[86]],
                Tactical: &self.row.columns[self.index_mapping[87]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[88]],
                Physical: &self.row.columns[self.index_mapping[89]],
                Mental: &self.row.columns[self.index_mapping[90]],
                Tactical: &self.row.columns[self.index_mapping[91]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[92]],
                Physical: &self.row.columns[self.index_mapping[93]],
                Mental: &self.row.columns[self.index_mapping[94]],
                Tactical: &self.row.columns[self.index_mapping[95]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[96]],
                Physical: &self.row.columns[self.index_mapping[97]],
                Mental: &self.row.columns[self.index_mapping[98]],
                Tactical: &self.row.columns[self.index_mapping[99]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[100]],
                Physical: &self.row.columns[self.index_mapping[101]],
                Mental: &self.row.columns[self.index_mapping[102]],
                Tactical: &self.row.columns[self.index_mapping[103]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[104]],
                Physical: &self.row.columns[self.index_mapping[105]],
                Mental: &self.row.columns[self.index_mapping[106]],
                Tactical: &self.row.columns[self.index_mapping[107]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[108]],
                Physical: &self.row.columns[self.index_mapping[109]],
                Mental: &self.row.columns[self.index_mapping[110]],
                Tactical: &self.row.columns[self.index_mapping[111]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[112]],
                Physical: &self.row.columns[self.index_mapping[113]],
                Mental: &self.row.columns[self.index_mapping[114]],
                Tactical: &self.row.columns[self.index_mapping[115]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[116]],
                Physical: &self.row.columns[self.index_mapping[117]],
                Mental: &self.row.columns[self.index_mapping[118]],
                Tactical: &self.row.columns[self.index_mapping[119]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[120]],
                Physical: &self.row.columns[self.index_mapping[121]],
                Mental: &self.row.columns[self.index_mapping[122]],
                Tactical: &self.row.columns[self.index_mapping[123]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[124]],
                Physical: &self.row.columns[self.index_mapping[125]],
                Mental: &self.row.columns[self.index_mapping[126]],
                Tactical: &self.row.columns[self.index_mapping[127]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[128]],
                Physical: &self.row.columns[self.index_mapping[129]],
                Mental: &self.row.columns[self.index_mapping[130]],
                Tactical: &self.row.columns[self.index_mapping[131]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[132]],
                Physical: &self.row.columns[self.index_mapping[133]],
                Mental: &self.row.columns[self.index_mapping[134]],
                Tactical: &self.row.columns[self.index_mapping[135]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[136]],
                Physical: &self.row.columns[self.index_mapping[137]],
                Mental: &self.row.columns[self.index_mapping[138]],
                Tactical: &self.row.columns[self.index_mapping[139]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[140]],
                Physical: &self.row.columns[self.index_mapping[141]],
                Mental: &self.row.columns[self.index_mapping[142]],
                Tactical: &self.row.columns[self.index_mapping[143]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[144]],
                Physical: &self.row.columns[self.index_mapping[145]],
                Mental: &self.row.columns[self.index_mapping[146]],
                Tactical: &self.row.columns[self.index_mapping[147]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[148]],
                Physical: &self.row.columns[self.index_mapping[149]],
                Mental: &self.row.columns[self.index_mapping[150]],
                Tactical: &self.row.columns[self.index_mapping[151]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[152]],
                Physical: &self.row.columns[self.index_mapping[153]],
                Mental: &self.row.columns[self.index_mapping[154]],
                Tactical: &self.row.columns[self.index_mapping[155]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[156]],
                Physical: &self.row.columns[self.index_mapping[157]],
                Mental: &self.row.columns[self.index_mapping[158]],
                Tactical: &self.row.columns[self.index_mapping[159]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[160]],
                Physical: &self.row.columns[self.index_mapping[161]],
                Mental: &self.row.columns[self.index_mapping[162]],
                Tactical: &self.row.columns[self.index_mapping[163]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[164]],
                Physical: &self.row.columns[self.index_mapping[165]],
                Mental: &self.row.columns[self.index_mapping[166]],
                Tactical: &self.row.columns[self.index_mapping[167]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[168]],
                Physical: &self.row.columns[self.index_mapping[169]],
                Mental: &self.row.columns[self.index_mapping[170]],
                Tactical: &self.row.columns[self.index_mapping[171]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[172]],
                Physical: &self.row.columns[self.index_mapping[173]],
                Mental: &self.row.columns[self.index_mapping[174]],
                Tactical: &self.row.columns[self.index_mapping[175]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[176]],
                Physical: &self.row.columns[self.index_mapping[177]],
                Mental: &self.row.columns[self.index_mapping[178]],
                Tactical: &self.row.columns[self.index_mapping[179]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[180]],
                Physical: &self.row.columns[self.index_mapping[181]],
                Mental: &self.row.columns[self.index_mapping[182]],
                Tactical: &self.row.columns[self.index_mapping[183]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[184]],
                Physical: &self.row.columns[self.index_mapping[185]],
                Mental: &self.row.columns[self.index_mapping[186]],
                Tactical: &self.row.columns[self.index_mapping[187]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[188]],
                Physical: &self.row.columns[self.index_mapping[189]],
                Mental: &self.row.columns[self.index_mapping[190]],
                Tactical: &self.row.columns[self.index_mapping[191]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[192]],
                Physical: &self.row.columns[self.index_mapping[193]],
                Mental: &self.row.columns[self.index_mapping[194]],
                Tactical: &self.row.columns[self.index_mapping[195]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[196]],
                Physical: &self.row.columns[self.index_mapping[197]],
                Mental: &self.row.columns[self.index_mapping[198]],
                Tactical: &self.row.columns[self.index_mapping[199]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[200]],
                Physical: &self.row.columns[self.index_mapping[201]],
                Mental: &self.row.columns[self.index_mapping[202]],
                Tactical: &self.row.columns[self.index_mapping[203]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[204]],
                Physical: &self.row.columns[self.index_mapping[205]],
                Mental: &self.row.columns[self.index_mapping[206]],
                Tactical: &self.row.columns[self.index_mapping[207]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[208]],
                Physical: &self.row.columns[self.index_mapping[209]],
                Mental: &self.row.columns[self.index_mapping[210]],
                Tactical: &self.row.columns[self.index_mapping[211]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[212]],
                Physical: &self.row.columns[self.index_mapping[213]],
                Mental: &self.row.columns[self.index_mapping[214]],
                Tactical: &self.row.columns[self.index_mapping[215]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[216]],
                Physical: &self.row.columns[self.index_mapping[217]],
                Mental: &self.row.columns[self.index_mapping[218]],
                Tactical: &self.row.columns[self.index_mapping[219]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[220]],
                Physical: &self.row.columns[self.index_mapping[221]],
                Mental: &self.row.columns[self.index_mapping[222]],
                Tactical: &self.row.columns[self.index_mapping[223]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[224]],
                Physical: &self.row.columns[self.index_mapping[225]],
                Mental: &self.row.columns[self.index_mapping[226]],
                Tactical: &self.row.columns[self.index_mapping[227]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[228]],
                Physical: &self.row.columns[self.index_mapping[229]],
                Mental: &self.row.columns[self.index_mapping[230]],
                Tactical: &self.row.columns[self.index_mapping[231]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[232]],
                Physical: &self.row.columns[self.index_mapping[233]],
                Mental: &self.row.columns[self.index_mapping[234]],
                Tactical: &self.row.columns[self.index_mapping[235]],
            },
            MemberParamsElement {
                EquipPreset: &self.row.columns[self.index_mapping[236]],
                Physical: &self.row.columns[self.index_mapping[237]],
                Mental: &self.row.columns[self.index_mapping[238]],
                Tactical: &self.row.columns[self.index_mapping[239]],
            },
        ]
    }
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[240]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[241]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[242]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[243]]
    }
    pub fn ClassBook(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[244]]
    }
    pub fn ClassJob(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[245]]
    }
}
