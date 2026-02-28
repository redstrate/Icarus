//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct LandSetElement<'a> {
    pub UnknownRange1: &'a Field,
    pub PlacardId: &'a Field,
    pub UnknownRange2: &'a Field,
    pub InitialPrice: &'a Field,
    pub PlotSize: &'a Field,
}
#[derive(Debug, Clone)]
pub struct HousingLandSetSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl HousingLandSetSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HousingLandSet")?;
        let sheet = resolver.read_excel_sheet(&exh, "HousingLandSet", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<HousingLandSetRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<HousingLandSetRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for HousingLandSetSheet {
    type Row = HousingLandSetRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a HousingLandSetSheet {
    type Item = (u32, Vec<(u16, HousingLandSetRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, HousingLandSetSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HousingLandSetSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HousingLandSetRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> HousingLandSetRow<'a> {
    pub fn LandSet(&'a self) -> [LandSetElement<'a>; 60] {
        [
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[0]],
                PlacardId: &self.row.columns[self.index_mapping[1]],
                UnknownRange2: &self.row.columns[self.index_mapping[2]],
                InitialPrice: &self.row.columns[self.index_mapping[3]],
                PlotSize: &self.row.columns[self.index_mapping[4]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[5]],
                PlacardId: &self.row.columns[self.index_mapping[6]],
                UnknownRange2: &self.row.columns[self.index_mapping[7]],
                InitialPrice: &self.row.columns[self.index_mapping[8]],
                PlotSize: &self.row.columns[self.index_mapping[9]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[10]],
                PlacardId: &self.row.columns[self.index_mapping[11]],
                UnknownRange2: &self.row.columns[self.index_mapping[12]],
                InitialPrice: &self.row.columns[self.index_mapping[13]],
                PlotSize: &self.row.columns[self.index_mapping[14]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[15]],
                PlacardId: &self.row.columns[self.index_mapping[16]],
                UnknownRange2: &self.row.columns[self.index_mapping[17]],
                InitialPrice: &self.row.columns[self.index_mapping[18]],
                PlotSize: &self.row.columns[self.index_mapping[19]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[20]],
                PlacardId: &self.row.columns[self.index_mapping[21]],
                UnknownRange2: &self.row.columns[self.index_mapping[22]],
                InitialPrice: &self.row.columns[self.index_mapping[23]],
                PlotSize: &self.row.columns[self.index_mapping[24]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[25]],
                PlacardId: &self.row.columns[self.index_mapping[26]],
                UnknownRange2: &self.row.columns[self.index_mapping[27]],
                InitialPrice: &self.row.columns[self.index_mapping[28]],
                PlotSize: &self.row.columns[self.index_mapping[29]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[30]],
                PlacardId: &self.row.columns[self.index_mapping[31]],
                UnknownRange2: &self.row.columns[self.index_mapping[32]],
                InitialPrice: &self.row.columns[self.index_mapping[33]],
                PlotSize: &self.row.columns[self.index_mapping[34]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[35]],
                PlacardId: &self.row.columns[self.index_mapping[36]],
                UnknownRange2: &self.row.columns[self.index_mapping[37]],
                InitialPrice: &self.row.columns[self.index_mapping[38]],
                PlotSize: &self.row.columns[self.index_mapping[39]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[40]],
                PlacardId: &self.row.columns[self.index_mapping[41]],
                UnknownRange2: &self.row.columns[self.index_mapping[42]],
                InitialPrice: &self.row.columns[self.index_mapping[43]],
                PlotSize: &self.row.columns[self.index_mapping[44]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[45]],
                PlacardId: &self.row.columns[self.index_mapping[46]],
                UnknownRange2: &self.row.columns[self.index_mapping[47]],
                InitialPrice: &self.row.columns[self.index_mapping[48]],
                PlotSize: &self.row.columns[self.index_mapping[49]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[50]],
                PlacardId: &self.row.columns[self.index_mapping[51]],
                UnknownRange2: &self.row.columns[self.index_mapping[52]],
                InitialPrice: &self.row.columns[self.index_mapping[53]],
                PlotSize: &self.row.columns[self.index_mapping[54]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[55]],
                PlacardId: &self.row.columns[self.index_mapping[56]],
                UnknownRange2: &self.row.columns[self.index_mapping[57]],
                InitialPrice: &self.row.columns[self.index_mapping[58]],
                PlotSize: &self.row.columns[self.index_mapping[59]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[60]],
                PlacardId: &self.row.columns[self.index_mapping[61]],
                UnknownRange2: &self.row.columns[self.index_mapping[62]],
                InitialPrice: &self.row.columns[self.index_mapping[63]],
                PlotSize: &self.row.columns[self.index_mapping[64]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[65]],
                PlacardId: &self.row.columns[self.index_mapping[66]],
                UnknownRange2: &self.row.columns[self.index_mapping[67]],
                InitialPrice: &self.row.columns[self.index_mapping[68]],
                PlotSize: &self.row.columns[self.index_mapping[69]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[70]],
                PlacardId: &self.row.columns[self.index_mapping[71]],
                UnknownRange2: &self.row.columns[self.index_mapping[72]],
                InitialPrice: &self.row.columns[self.index_mapping[73]],
                PlotSize: &self.row.columns[self.index_mapping[74]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[75]],
                PlacardId: &self.row.columns[self.index_mapping[76]],
                UnknownRange2: &self.row.columns[self.index_mapping[77]],
                InitialPrice: &self.row.columns[self.index_mapping[78]],
                PlotSize: &self.row.columns[self.index_mapping[79]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[80]],
                PlacardId: &self.row.columns[self.index_mapping[81]],
                UnknownRange2: &self.row.columns[self.index_mapping[82]],
                InitialPrice: &self.row.columns[self.index_mapping[83]],
                PlotSize: &self.row.columns[self.index_mapping[84]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[85]],
                PlacardId: &self.row.columns[self.index_mapping[86]],
                UnknownRange2: &self.row.columns[self.index_mapping[87]],
                InitialPrice: &self.row.columns[self.index_mapping[88]],
                PlotSize: &self.row.columns[self.index_mapping[89]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[90]],
                PlacardId: &self.row.columns[self.index_mapping[91]],
                UnknownRange2: &self.row.columns[self.index_mapping[92]],
                InitialPrice: &self.row.columns[self.index_mapping[93]],
                PlotSize: &self.row.columns[self.index_mapping[94]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[95]],
                PlacardId: &self.row.columns[self.index_mapping[96]],
                UnknownRange2: &self.row.columns[self.index_mapping[97]],
                InitialPrice: &self.row.columns[self.index_mapping[98]],
                PlotSize: &self.row.columns[self.index_mapping[99]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[100]],
                PlacardId: &self.row.columns[self.index_mapping[101]],
                UnknownRange2: &self.row.columns[self.index_mapping[102]],
                InitialPrice: &self.row.columns[self.index_mapping[103]],
                PlotSize: &self.row.columns[self.index_mapping[104]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[105]],
                PlacardId: &self.row.columns[self.index_mapping[106]],
                UnknownRange2: &self.row.columns[self.index_mapping[107]],
                InitialPrice: &self.row.columns[self.index_mapping[108]],
                PlotSize: &self.row.columns[self.index_mapping[109]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[110]],
                PlacardId: &self.row.columns[self.index_mapping[111]],
                UnknownRange2: &self.row.columns[self.index_mapping[112]],
                InitialPrice: &self.row.columns[self.index_mapping[113]],
                PlotSize: &self.row.columns[self.index_mapping[114]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[115]],
                PlacardId: &self.row.columns[self.index_mapping[116]],
                UnknownRange2: &self.row.columns[self.index_mapping[117]],
                InitialPrice: &self.row.columns[self.index_mapping[118]],
                PlotSize: &self.row.columns[self.index_mapping[119]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[120]],
                PlacardId: &self.row.columns[self.index_mapping[121]],
                UnknownRange2: &self.row.columns[self.index_mapping[122]],
                InitialPrice: &self.row.columns[self.index_mapping[123]],
                PlotSize: &self.row.columns[self.index_mapping[124]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[125]],
                PlacardId: &self.row.columns[self.index_mapping[126]],
                UnknownRange2: &self.row.columns[self.index_mapping[127]],
                InitialPrice: &self.row.columns[self.index_mapping[128]],
                PlotSize: &self.row.columns[self.index_mapping[129]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[130]],
                PlacardId: &self.row.columns[self.index_mapping[131]],
                UnknownRange2: &self.row.columns[self.index_mapping[132]],
                InitialPrice: &self.row.columns[self.index_mapping[133]],
                PlotSize: &self.row.columns[self.index_mapping[134]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[135]],
                PlacardId: &self.row.columns[self.index_mapping[136]],
                UnknownRange2: &self.row.columns[self.index_mapping[137]],
                InitialPrice: &self.row.columns[self.index_mapping[138]],
                PlotSize: &self.row.columns[self.index_mapping[139]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[140]],
                PlacardId: &self.row.columns[self.index_mapping[141]],
                UnknownRange2: &self.row.columns[self.index_mapping[142]],
                InitialPrice: &self.row.columns[self.index_mapping[143]],
                PlotSize: &self.row.columns[self.index_mapping[144]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[145]],
                PlacardId: &self.row.columns[self.index_mapping[146]],
                UnknownRange2: &self.row.columns[self.index_mapping[147]],
                InitialPrice: &self.row.columns[self.index_mapping[148]],
                PlotSize: &self.row.columns[self.index_mapping[149]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[150]],
                PlacardId: &self.row.columns[self.index_mapping[151]],
                UnknownRange2: &self.row.columns[self.index_mapping[152]],
                InitialPrice: &self.row.columns[self.index_mapping[153]],
                PlotSize: &self.row.columns[self.index_mapping[154]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[155]],
                PlacardId: &self.row.columns[self.index_mapping[156]],
                UnknownRange2: &self.row.columns[self.index_mapping[157]],
                InitialPrice: &self.row.columns[self.index_mapping[158]],
                PlotSize: &self.row.columns[self.index_mapping[159]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[160]],
                PlacardId: &self.row.columns[self.index_mapping[161]],
                UnknownRange2: &self.row.columns[self.index_mapping[162]],
                InitialPrice: &self.row.columns[self.index_mapping[163]],
                PlotSize: &self.row.columns[self.index_mapping[164]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[165]],
                PlacardId: &self.row.columns[self.index_mapping[166]],
                UnknownRange2: &self.row.columns[self.index_mapping[167]],
                InitialPrice: &self.row.columns[self.index_mapping[168]],
                PlotSize: &self.row.columns[self.index_mapping[169]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[170]],
                PlacardId: &self.row.columns[self.index_mapping[171]],
                UnknownRange2: &self.row.columns[self.index_mapping[172]],
                InitialPrice: &self.row.columns[self.index_mapping[173]],
                PlotSize: &self.row.columns[self.index_mapping[174]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[175]],
                PlacardId: &self.row.columns[self.index_mapping[176]],
                UnknownRange2: &self.row.columns[self.index_mapping[177]],
                InitialPrice: &self.row.columns[self.index_mapping[178]],
                PlotSize: &self.row.columns[self.index_mapping[179]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[180]],
                PlacardId: &self.row.columns[self.index_mapping[181]],
                UnknownRange2: &self.row.columns[self.index_mapping[182]],
                InitialPrice: &self.row.columns[self.index_mapping[183]],
                PlotSize: &self.row.columns[self.index_mapping[184]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[185]],
                PlacardId: &self.row.columns[self.index_mapping[186]],
                UnknownRange2: &self.row.columns[self.index_mapping[187]],
                InitialPrice: &self.row.columns[self.index_mapping[188]],
                PlotSize: &self.row.columns[self.index_mapping[189]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[190]],
                PlacardId: &self.row.columns[self.index_mapping[191]],
                UnknownRange2: &self.row.columns[self.index_mapping[192]],
                InitialPrice: &self.row.columns[self.index_mapping[193]],
                PlotSize: &self.row.columns[self.index_mapping[194]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[195]],
                PlacardId: &self.row.columns[self.index_mapping[196]],
                UnknownRange2: &self.row.columns[self.index_mapping[197]],
                InitialPrice: &self.row.columns[self.index_mapping[198]],
                PlotSize: &self.row.columns[self.index_mapping[199]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[200]],
                PlacardId: &self.row.columns[self.index_mapping[201]],
                UnknownRange2: &self.row.columns[self.index_mapping[202]],
                InitialPrice: &self.row.columns[self.index_mapping[203]],
                PlotSize: &self.row.columns[self.index_mapping[204]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[205]],
                PlacardId: &self.row.columns[self.index_mapping[206]],
                UnknownRange2: &self.row.columns[self.index_mapping[207]],
                InitialPrice: &self.row.columns[self.index_mapping[208]],
                PlotSize: &self.row.columns[self.index_mapping[209]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[210]],
                PlacardId: &self.row.columns[self.index_mapping[211]],
                UnknownRange2: &self.row.columns[self.index_mapping[212]],
                InitialPrice: &self.row.columns[self.index_mapping[213]],
                PlotSize: &self.row.columns[self.index_mapping[214]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[215]],
                PlacardId: &self.row.columns[self.index_mapping[216]],
                UnknownRange2: &self.row.columns[self.index_mapping[217]],
                InitialPrice: &self.row.columns[self.index_mapping[218]],
                PlotSize: &self.row.columns[self.index_mapping[219]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[220]],
                PlacardId: &self.row.columns[self.index_mapping[221]],
                UnknownRange2: &self.row.columns[self.index_mapping[222]],
                InitialPrice: &self.row.columns[self.index_mapping[223]],
                PlotSize: &self.row.columns[self.index_mapping[224]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[225]],
                PlacardId: &self.row.columns[self.index_mapping[226]],
                UnknownRange2: &self.row.columns[self.index_mapping[227]],
                InitialPrice: &self.row.columns[self.index_mapping[228]],
                PlotSize: &self.row.columns[self.index_mapping[229]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[230]],
                PlacardId: &self.row.columns[self.index_mapping[231]],
                UnknownRange2: &self.row.columns[self.index_mapping[232]],
                InitialPrice: &self.row.columns[self.index_mapping[233]],
                PlotSize: &self.row.columns[self.index_mapping[234]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[235]],
                PlacardId: &self.row.columns[self.index_mapping[236]],
                UnknownRange2: &self.row.columns[self.index_mapping[237]],
                InitialPrice: &self.row.columns[self.index_mapping[238]],
                PlotSize: &self.row.columns[self.index_mapping[239]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[240]],
                PlacardId: &self.row.columns[self.index_mapping[241]],
                UnknownRange2: &self.row.columns[self.index_mapping[242]],
                InitialPrice: &self.row.columns[self.index_mapping[243]],
                PlotSize: &self.row.columns[self.index_mapping[244]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[245]],
                PlacardId: &self.row.columns[self.index_mapping[246]],
                UnknownRange2: &self.row.columns[self.index_mapping[247]],
                InitialPrice: &self.row.columns[self.index_mapping[248]],
                PlotSize: &self.row.columns[self.index_mapping[249]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[250]],
                PlacardId: &self.row.columns[self.index_mapping[251]],
                UnknownRange2: &self.row.columns[self.index_mapping[252]],
                InitialPrice: &self.row.columns[self.index_mapping[253]],
                PlotSize: &self.row.columns[self.index_mapping[254]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[255]],
                PlacardId: &self.row.columns[self.index_mapping[256]],
                UnknownRange2: &self.row.columns[self.index_mapping[257]],
                InitialPrice: &self.row.columns[self.index_mapping[258]],
                PlotSize: &self.row.columns[self.index_mapping[259]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[260]],
                PlacardId: &self.row.columns[self.index_mapping[261]],
                UnknownRange2: &self.row.columns[self.index_mapping[262]],
                InitialPrice: &self.row.columns[self.index_mapping[263]],
                PlotSize: &self.row.columns[self.index_mapping[264]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[265]],
                PlacardId: &self.row.columns[self.index_mapping[266]],
                UnknownRange2: &self.row.columns[self.index_mapping[267]],
                InitialPrice: &self.row.columns[self.index_mapping[268]],
                PlotSize: &self.row.columns[self.index_mapping[269]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[270]],
                PlacardId: &self.row.columns[self.index_mapping[271]],
                UnknownRange2: &self.row.columns[self.index_mapping[272]],
                InitialPrice: &self.row.columns[self.index_mapping[273]],
                PlotSize: &self.row.columns[self.index_mapping[274]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[275]],
                PlacardId: &self.row.columns[self.index_mapping[276]],
                UnknownRange2: &self.row.columns[self.index_mapping[277]],
                InitialPrice: &self.row.columns[self.index_mapping[278]],
                PlotSize: &self.row.columns[self.index_mapping[279]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[280]],
                PlacardId: &self.row.columns[self.index_mapping[281]],
                UnknownRange2: &self.row.columns[self.index_mapping[282]],
                InitialPrice: &self.row.columns[self.index_mapping[283]],
                PlotSize: &self.row.columns[self.index_mapping[284]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[285]],
                PlacardId: &self.row.columns[self.index_mapping[286]],
                UnknownRange2: &self.row.columns[self.index_mapping[287]],
                InitialPrice: &self.row.columns[self.index_mapping[288]],
                PlotSize: &self.row.columns[self.index_mapping[289]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[290]],
                PlacardId: &self.row.columns[self.index_mapping[291]],
                UnknownRange2: &self.row.columns[self.index_mapping[292]],
                InitialPrice: &self.row.columns[self.index_mapping[293]],
                PlotSize: &self.row.columns[self.index_mapping[294]],
            },
            LandSetElement {
                UnknownRange1: &self.row.columns[self.index_mapping[295]],
                PlacardId: &self.row.columns[self.index_mapping[296]],
                UnknownRange2: &self.row.columns[self.index_mapping[297]],
                InitialPrice: &self.row.columns[self.index_mapping[298]],
                PlotSize: &self.row.columns[self.index_mapping[299]],
            },
        ]
    }
    pub fn UnknownRange1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[300]]
    }
    pub fn UnknownRange2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[301]]
    }
}
