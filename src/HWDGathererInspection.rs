//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
pub struct HWDGathererInspectionDataElement<'a> {
    pub RequiredItem: &'a Field,
    pub FishParameter: &'a Field,
    pub ItemReceived: &'a Field,
    pub Reward: [&'a Field; 2],
    pub AmountRequired: &'a Field,
    pub Phase: &'a Field,
}
#[derive(Debug, Clone)]
pub struct HWDGathererInspectionSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl HWDGathererInspectionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("HWDGathererInspection")?;
        let sheet = resolver.read_excel_sheet(&exh, "HWDGathererInspection", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<HWDGathererInspectionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(
        &self,
        row_id: u32,
        subrow_id: u16,
    ) -> Option<HWDGathererInspectionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for HWDGathererInspectionSheet {
    type Row = HWDGathererInspectionRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a HWDGathererInspectionSheet {
    type Item = (u32, Vec<(u16, HWDGathererInspectionRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, HWDGathererInspectionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, HWDGathererInspectionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HWDGathererInspectionRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> HWDGathererInspectionRow<'a> {
    pub fn HWDGathererInspectionData(
        &'a self,
    ) -> [HWDGathererInspectionDataElement<'a>; 79] {
        [
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[0]],
                FishParameter: &self.row.columns[self.index_mapping[1]],
                ItemReceived: &self.row.columns[self.index_mapping[2]],
                Reward: [
                    &self.row.columns[self.index_mapping[3]],
                    &self.row.columns[self.index_mapping[4]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[5]],
                Phase: &self.row.columns[self.index_mapping[6]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[7]],
                FishParameter: &self.row.columns[self.index_mapping[8]],
                ItemReceived: &self.row.columns[self.index_mapping[9]],
                Reward: [
                    &self.row.columns[self.index_mapping[10]],
                    &self.row.columns[self.index_mapping[11]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[12]],
                Phase: &self.row.columns[self.index_mapping[13]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[14]],
                FishParameter: &self.row.columns[self.index_mapping[15]],
                ItemReceived: &self.row.columns[self.index_mapping[16]],
                Reward: [
                    &self.row.columns[self.index_mapping[17]],
                    &self.row.columns[self.index_mapping[18]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[19]],
                Phase: &self.row.columns[self.index_mapping[20]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[21]],
                FishParameter: &self.row.columns[self.index_mapping[22]],
                ItemReceived: &self.row.columns[self.index_mapping[23]],
                Reward: [
                    &self.row.columns[self.index_mapping[24]],
                    &self.row.columns[self.index_mapping[25]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[26]],
                Phase: &self.row.columns[self.index_mapping[27]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[28]],
                FishParameter: &self.row.columns[self.index_mapping[29]],
                ItemReceived: &self.row.columns[self.index_mapping[30]],
                Reward: [
                    &self.row.columns[self.index_mapping[31]],
                    &self.row.columns[self.index_mapping[32]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[33]],
                Phase: &self.row.columns[self.index_mapping[34]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[35]],
                FishParameter: &self.row.columns[self.index_mapping[36]],
                ItemReceived: &self.row.columns[self.index_mapping[37]],
                Reward: [
                    &self.row.columns[self.index_mapping[38]],
                    &self.row.columns[self.index_mapping[39]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[40]],
                Phase: &self.row.columns[self.index_mapping[41]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[42]],
                FishParameter: &self.row.columns[self.index_mapping[43]],
                ItemReceived: &self.row.columns[self.index_mapping[44]],
                Reward: [
                    &self.row.columns[self.index_mapping[45]],
                    &self.row.columns[self.index_mapping[46]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[47]],
                Phase: &self.row.columns[self.index_mapping[48]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[49]],
                FishParameter: &self.row.columns[self.index_mapping[50]],
                ItemReceived: &self.row.columns[self.index_mapping[51]],
                Reward: [
                    &self.row.columns[self.index_mapping[52]],
                    &self.row.columns[self.index_mapping[53]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[54]],
                Phase: &self.row.columns[self.index_mapping[55]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[56]],
                FishParameter: &self.row.columns[self.index_mapping[57]],
                ItemReceived: &self.row.columns[self.index_mapping[58]],
                Reward: [
                    &self.row.columns[self.index_mapping[59]],
                    &self.row.columns[self.index_mapping[60]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[61]],
                Phase: &self.row.columns[self.index_mapping[62]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[63]],
                FishParameter: &self.row.columns[self.index_mapping[64]],
                ItemReceived: &self.row.columns[self.index_mapping[65]],
                Reward: [
                    &self.row.columns[self.index_mapping[66]],
                    &self.row.columns[self.index_mapping[67]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[68]],
                Phase: &self.row.columns[self.index_mapping[69]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[70]],
                FishParameter: &self.row.columns[self.index_mapping[71]],
                ItemReceived: &self.row.columns[self.index_mapping[72]],
                Reward: [
                    &self.row.columns[self.index_mapping[73]],
                    &self.row.columns[self.index_mapping[74]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[75]],
                Phase: &self.row.columns[self.index_mapping[76]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[77]],
                FishParameter: &self.row.columns[self.index_mapping[78]],
                ItemReceived: &self.row.columns[self.index_mapping[79]],
                Reward: [
                    &self.row.columns[self.index_mapping[80]],
                    &self.row.columns[self.index_mapping[81]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[82]],
                Phase: &self.row.columns[self.index_mapping[83]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[84]],
                FishParameter: &self.row.columns[self.index_mapping[85]],
                ItemReceived: &self.row.columns[self.index_mapping[86]],
                Reward: [
                    &self.row.columns[self.index_mapping[87]],
                    &self.row.columns[self.index_mapping[88]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[89]],
                Phase: &self.row.columns[self.index_mapping[90]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[91]],
                FishParameter: &self.row.columns[self.index_mapping[92]],
                ItemReceived: &self.row.columns[self.index_mapping[93]],
                Reward: [
                    &self.row.columns[self.index_mapping[94]],
                    &self.row.columns[self.index_mapping[95]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[96]],
                Phase: &self.row.columns[self.index_mapping[97]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[98]],
                FishParameter: &self.row.columns[self.index_mapping[99]],
                ItemReceived: &self.row.columns[self.index_mapping[100]],
                Reward: [
                    &self.row.columns[self.index_mapping[101]],
                    &self.row.columns[self.index_mapping[102]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[103]],
                Phase: &self.row.columns[self.index_mapping[104]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[105]],
                FishParameter: &self.row.columns[self.index_mapping[106]],
                ItemReceived: &self.row.columns[self.index_mapping[107]],
                Reward: [
                    &self.row.columns[self.index_mapping[108]],
                    &self.row.columns[self.index_mapping[109]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[110]],
                Phase: &self.row.columns[self.index_mapping[111]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[112]],
                FishParameter: &self.row.columns[self.index_mapping[113]],
                ItemReceived: &self.row.columns[self.index_mapping[114]],
                Reward: [
                    &self.row.columns[self.index_mapping[115]],
                    &self.row.columns[self.index_mapping[116]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[117]],
                Phase: &self.row.columns[self.index_mapping[118]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[119]],
                FishParameter: &self.row.columns[self.index_mapping[120]],
                ItemReceived: &self.row.columns[self.index_mapping[121]],
                Reward: [
                    &self.row.columns[self.index_mapping[122]],
                    &self.row.columns[self.index_mapping[123]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[124]],
                Phase: &self.row.columns[self.index_mapping[125]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[126]],
                FishParameter: &self.row.columns[self.index_mapping[127]],
                ItemReceived: &self.row.columns[self.index_mapping[128]],
                Reward: [
                    &self.row.columns[self.index_mapping[129]],
                    &self.row.columns[self.index_mapping[130]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[131]],
                Phase: &self.row.columns[self.index_mapping[132]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[133]],
                FishParameter: &self.row.columns[self.index_mapping[134]],
                ItemReceived: &self.row.columns[self.index_mapping[135]],
                Reward: [
                    &self.row.columns[self.index_mapping[136]],
                    &self.row.columns[self.index_mapping[137]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[138]],
                Phase: &self.row.columns[self.index_mapping[139]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[140]],
                FishParameter: &self.row.columns[self.index_mapping[141]],
                ItemReceived: &self.row.columns[self.index_mapping[142]],
                Reward: [
                    &self.row.columns[self.index_mapping[143]],
                    &self.row.columns[self.index_mapping[144]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[145]],
                Phase: &self.row.columns[self.index_mapping[146]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[147]],
                FishParameter: &self.row.columns[self.index_mapping[148]],
                ItemReceived: &self.row.columns[self.index_mapping[149]],
                Reward: [
                    &self.row.columns[self.index_mapping[150]],
                    &self.row.columns[self.index_mapping[151]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[152]],
                Phase: &self.row.columns[self.index_mapping[153]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[154]],
                FishParameter: &self.row.columns[self.index_mapping[155]],
                ItemReceived: &self.row.columns[self.index_mapping[156]],
                Reward: [
                    &self.row.columns[self.index_mapping[157]],
                    &self.row.columns[self.index_mapping[158]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[159]],
                Phase: &self.row.columns[self.index_mapping[160]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[161]],
                FishParameter: &self.row.columns[self.index_mapping[162]],
                ItemReceived: &self.row.columns[self.index_mapping[163]],
                Reward: [
                    &self.row.columns[self.index_mapping[164]],
                    &self.row.columns[self.index_mapping[165]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[166]],
                Phase: &self.row.columns[self.index_mapping[167]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[168]],
                FishParameter: &self.row.columns[self.index_mapping[169]],
                ItemReceived: &self.row.columns[self.index_mapping[170]],
                Reward: [
                    &self.row.columns[self.index_mapping[171]],
                    &self.row.columns[self.index_mapping[172]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[173]],
                Phase: &self.row.columns[self.index_mapping[174]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[175]],
                FishParameter: &self.row.columns[self.index_mapping[176]],
                ItemReceived: &self.row.columns[self.index_mapping[177]],
                Reward: [
                    &self.row.columns[self.index_mapping[178]],
                    &self.row.columns[self.index_mapping[179]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[180]],
                Phase: &self.row.columns[self.index_mapping[181]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[182]],
                FishParameter: &self.row.columns[self.index_mapping[183]],
                ItemReceived: &self.row.columns[self.index_mapping[184]],
                Reward: [
                    &self.row.columns[self.index_mapping[185]],
                    &self.row.columns[self.index_mapping[186]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[187]],
                Phase: &self.row.columns[self.index_mapping[188]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[189]],
                FishParameter: &self.row.columns[self.index_mapping[190]],
                ItemReceived: &self.row.columns[self.index_mapping[191]],
                Reward: [
                    &self.row.columns[self.index_mapping[192]],
                    &self.row.columns[self.index_mapping[193]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[194]],
                Phase: &self.row.columns[self.index_mapping[195]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[196]],
                FishParameter: &self.row.columns[self.index_mapping[197]],
                ItemReceived: &self.row.columns[self.index_mapping[198]],
                Reward: [
                    &self.row.columns[self.index_mapping[199]],
                    &self.row.columns[self.index_mapping[200]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[201]],
                Phase: &self.row.columns[self.index_mapping[202]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[203]],
                FishParameter: &self.row.columns[self.index_mapping[204]],
                ItemReceived: &self.row.columns[self.index_mapping[205]],
                Reward: [
                    &self.row.columns[self.index_mapping[206]],
                    &self.row.columns[self.index_mapping[207]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[208]],
                Phase: &self.row.columns[self.index_mapping[209]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[210]],
                FishParameter: &self.row.columns[self.index_mapping[211]],
                ItemReceived: &self.row.columns[self.index_mapping[212]],
                Reward: [
                    &self.row.columns[self.index_mapping[213]],
                    &self.row.columns[self.index_mapping[214]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[215]],
                Phase: &self.row.columns[self.index_mapping[216]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[217]],
                FishParameter: &self.row.columns[self.index_mapping[218]],
                ItemReceived: &self.row.columns[self.index_mapping[219]],
                Reward: [
                    &self.row.columns[self.index_mapping[220]],
                    &self.row.columns[self.index_mapping[221]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[222]],
                Phase: &self.row.columns[self.index_mapping[223]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[224]],
                FishParameter: &self.row.columns[self.index_mapping[225]],
                ItemReceived: &self.row.columns[self.index_mapping[226]],
                Reward: [
                    &self.row.columns[self.index_mapping[227]],
                    &self.row.columns[self.index_mapping[228]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[229]],
                Phase: &self.row.columns[self.index_mapping[230]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[231]],
                FishParameter: &self.row.columns[self.index_mapping[232]],
                ItemReceived: &self.row.columns[self.index_mapping[233]],
                Reward: [
                    &self.row.columns[self.index_mapping[234]],
                    &self.row.columns[self.index_mapping[235]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[236]],
                Phase: &self.row.columns[self.index_mapping[237]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[238]],
                FishParameter: &self.row.columns[self.index_mapping[239]],
                ItemReceived: &self.row.columns[self.index_mapping[240]],
                Reward: [
                    &self.row.columns[self.index_mapping[241]],
                    &self.row.columns[self.index_mapping[242]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[243]],
                Phase: &self.row.columns[self.index_mapping[244]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[245]],
                FishParameter: &self.row.columns[self.index_mapping[246]],
                ItemReceived: &self.row.columns[self.index_mapping[247]],
                Reward: [
                    &self.row.columns[self.index_mapping[248]],
                    &self.row.columns[self.index_mapping[249]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[250]],
                Phase: &self.row.columns[self.index_mapping[251]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[252]],
                FishParameter: &self.row.columns[self.index_mapping[253]],
                ItemReceived: &self.row.columns[self.index_mapping[254]],
                Reward: [
                    &self.row.columns[self.index_mapping[255]],
                    &self.row.columns[self.index_mapping[256]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[257]],
                Phase: &self.row.columns[self.index_mapping[258]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[259]],
                FishParameter: &self.row.columns[self.index_mapping[260]],
                ItemReceived: &self.row.columns[self.index_mapping[261]],
                Reward: [
                    &self.row.columns[self.index_mapping[262]],
                    &self.row.columns[self.index_mapping[263]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[264]],
                Phase: &self.row.columns[self.index_mapping[265]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[266]],
                FishParameter: &self.row.columns[self.index_mapping[267]],
                ItemReceived: &self.row.columns[self.index_mapping[268]],
                Reward: [
                    &self.row.columns[self.index_mapping[269]],
                    &self.row.columns[self.index_mapping[270]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[271]],
                Phase: &self.row.columns[self.index_mapping[272]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[273]],
                FishParameter: &self.row.columns[self.index_mapping[274]],
                ItemReceived: &self.row.columns[self.index_mapping[275]],
                Reward: [
                    &self.row.columns[self.index_mapping[276]],
                    &self.row.columns[self.index_mapping[277]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[278]],
                Phase: &self.row.columns[self.index_mapping[279]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[280]],
                FishParameter: &self.row.columns[self.index_mapping[281]],
                ItemReceived: &self.row.columns[self.index_mapping[282]],
                Reward: [
                    &self.row.columns[self.index_mapping[283]],
                    &self.row.columns[self.index_mapping[284]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[285]],
                Phase: &self.row.columns[self.index_mapping[286]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[287]],
                FishParameter: &self.row.columns[self.index_mapping[288]],
                ItemReceived: &self.row.columns[self.index_mapping[289]],
                Reward: [
                    &self.row.columns[self.index_mapping[290]],
                    &self.row.columns[self.index_mapping[291]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[292]],
                Phase: &self.row.columns[self.index_mapping[293]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[294]],
                FishParameter: &self.row.columns[self.index_mapping[295]],
                ItemReceived: &self.row.columns[self.index_mapping[296]],
                Reward: [
                    &self.row.columns[self.index_mapping[297]],
                    &self.row.columns[self.index_mapping[298]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[299]],
                Phase: &self.row.columns[self.index_mapping[300]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[301]],
                FishParameter: &self.row.columns[self.index_mapping[302]],
                ItemReceived: &self.row.columns[self.index_mapping[303]],
                Reward: [
                    &self.row.columns[self.index_mapping[304]],
                    &self.row.columns[self.index_mapping[305]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[306]],
                Phase: &self.row.columns[self.index_mapping[307]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[308]],
                FishParameter: &self.row.columns[self.index_mapping[309]],
                ItemReceived: &self.row.columns[self.index_mapping[310]],
                Reward: [
                    &self.row.columns[self.index_mapping[311]],
                    &self.row.columns[self.index_mapping[312]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[313]],
                Phase: &self.row.columns[self.index_mapping[314]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[315]],
                FishParameter: &self.row.columns[self.index_mapping[316]],
                ItemReceived: &self.row.columns[self.index_mapping[317]],
                Reward: [
                    &self.row.columns[self.index_mapping[318]],
                    &self.row.columns[self.index_mapping[319]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[320]],
                Phase: &self.row.columns[self.index_mapping[321]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[322]],
                FishParameter: &self.row.columns[self.index_mapping[323]],
                ItemReceived: &self.row.columns[self.index_mapping[324]],
                Reward: [
                    &self.row.columns[self.index_mapping[325]],
                    &self.row.columns[self.index_mapping[326]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[327]],
                Phase: &self.row.columns[self.index_mapping[328]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[329]],
                FishParameter: &self.row.columns[self.index_mapping[330]],
                ItemReceived: &self.row.columns[self.index_mapping[331]],
                Reward: [
                    &self.row.columns[self.index_mapping[332]],
                    &self.row.columns[self.index_mapping[333]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[334]],
                Phase: &self.row.columns[self.index_mapping[335]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[336]],
                FishParameter: &self.row.columns[self.index_mapping[337]],
                ItemReceived: &self.row.columns[self.index_mapping[338]],
                Reward: [
                    &self.row.columns[self.index_mapping[339]],
                    &self.row.columns[self.index_mapping[340]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[341]],
                Phase: &self.row.columns[self.index_mapping[342]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[343]],
                FishParameter: &self.row.columns[self.index_mapping[344]],
                ItemReceived: &self.row.columns[self.index_mapping[345]],
                Reward: [
                    &self.row.columns[self.index_mapping[346]],
                    &self.row.columns[self.index_mapping[347]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[348]],
                Phase: &self.row.columns[self.index_mapping[349]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[350]],
                FishParameter: &self.row.columns[self.index_mapping[351]],
                ItemReceived: &self.row.columns[self.index_mapping[352]],
                Reward: [
                    &self.row.columns[self.index_mapping[353]],
                    &self.row.columns[self.index_mapping[354]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[355]],
                Phase: &self.row.columns[self.index_mapping[356]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[357]],
                FishParameter: &self.row.columns[self.index_mapping[358]],
                ItemReceived: &self.row.columns[self.index_mapping[359]],
                Reward: [
                    &self.row.columns[self.index_mapping[360]],
                    &self.row.columns[self.index_mapping[361]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[362]],
                Phase: &self.row.columns[self.index_mapping[363]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[364]],
                FishParameter: &self.row.columns[self.index_mapping[365]],
                ItemReceived: &self.row.columns[self.index_mapping[366]],
                Reward: [
                    &self.row.columns[self.index_mapping[367]],
                    &self.row.columns[self.index_mapping[368]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[369]],
                Phase: &self.row.columns[self.index_mapping[370]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[371]],
                FishParameter: &self.row.columns[self.index_mapping[372]],
                ItemReceived: &self.row.columns[self.index_mapping[373]],
                Reward: [
                    &self.row.columns[self.index_mapping[374]],
                    &self.row.columns[self.index_mapping[375]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[376]],
                Phase: &self.row.columns[self.index_mapping[377]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[378]],
                FishParameter: &self.row.columns[self.index_mapping[379]],
                ItemReceived: &self.row.columns[self.index_mapping[380]],
                Reward: [
                    &self.row.columns[self.index_mapping[381]],
                    &self.row.columns[self.index_mapping[382]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[383]],
                Phase: &self.row.columns[self.index_mapping[384]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[385]],
                FishParameter: &self.row.columns[self.index_mapping[386]],
                ItemReceived: &self.row.columns[self.index_mapping[387]],
                Reward: [
                    &self.row.columns[self.index_mapping[388]],
                    &self.row.columns[self.index_mapping[389]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[390]],
                Phase: &self.row.columns[self.index_mapping[391]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[392]],
                FishParameter: &self.row.columns[self.index_mapping[393]],
                ItemReceived: &self.row.columns[self.index_mapping[394]],
                Reward: [
                    &self.row.columns[self.index_mapping[395]],
                    &self.row.columns[self.index_mapping[396]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[397]],
                Phase: &self.row.columns[self.index_mapping[398]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[399]],
                FishParameter: &self.row.columns[self.index_mapping[400]],
                ItemReceived: &self.row.columns[self.index_mapping[401]],
                Reward: [
                    &self.row.columns[self.index_mapping[402]],
                    &self.row.columns[self.index_mapping[403]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[404]],
                Phase: &self.row.columns[self.index_mapping[405]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[406]],
                FishParameter: &self.row.columns[self.index_mapping[407]],
                ItemReceived: &self.row.columns[self.index_mapping[408]],
                Reward: [
                    &self.row.columns[self.index_mapping[409]],
                    &self.row.columns[self.index_mapping[410]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[411]],
                Phase: &self.row.columns[self.index_mapping[412]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[413]],
                FishParameter: &self.row.columns[self.index_mapping[414]],
                ItemReceived: &self.row.columns[self.index_mapping[415]],
                Reward: [
                    &self.row.columns[self.index_mapping[416]],
                    &self.row.columns[self.index_mapping[417]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[418]],
                Phase: &self.row.columns[self.index_mapping[419]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[420]],
                FishParameter: &self.row.columns[self.index_mapping[421]],
                ItemReceived: &self.row.columns[self.index_mapping[422]],
                Reward: [
                    &self.row.columns[self.index_mapping[423]],
                    &self.row.columns[self.index_mapping[424]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[425]],
                Phase: &self.row.columns[self.index_mapping[426]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[427]],
                FishParameter: &self.row.columns[self.index_mapping[428]],
                ItemReceived: &self.row.columns[self.index_mapping[429]],
                Reward: [
                    &self.row.columns[self.index_mapping[430]],
                    &self.row.columns[self.index_mapping[431]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[432]],
                Phase: &self.row.columns[self.index_mapping[433]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[434]],
                FishParameter: &self.row.columns[self.index_mapping[435]],
                ItemReceived: &self.row.columns[self.index_mapping[436]],
                Reward: [
                    &self.row.columns[self.index_mapping[437]],
                    &self.row.columns[self.index_mapping[438]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[439]],
                Phase: &self.row.columns[self.index_mapping[440]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[441]],
                FishParameter: &self.row.columns[self.index_mapping[442]],
                ItemReceived: &self.row.columns[self.index_mapping[443]],
                Reward: [
                    &self.row.columns[self.index_mapping[444]],
                    &self.row.columns[self.index_mapping[445]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[446]],
                Phase: &self.row.columns[self.index_mapping[447]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[448]],
                FishParameter: &self.row.columns[self.index_mapping[449]],
                ItemReceived: &self.row.columns[self.index_mapping[450]],
                Reward: [
                    &self.row.columns[self.index_mapping[451]],
                    &self.row.columns[self.index_mapping[452]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[453]],
                Phase: &self.row.columns[self.index_mapping[454]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[455]],
                FishParameter: &self.row.columns[self.index_mapping[456]],
                ItemReceived: &self.row.columns[self.index_mapping[457]],
                Reward: [
                    &self.row.columns[self.index_mapping[458]],
                    &self.row.columns[self.index_mapping[459]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[460]],
                Phase: &self.row.columns[self.index_mapping[461]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[462]],
                FishParameter: &self.row.columns[self.index_mapping[463]],
                ItemReceived: &self.row.columns[self.index_mapping[464]],
                Reward: [
                    &self.row.columns[self.index_mapping[465]],
                    &self.row.columns[self.index_mapping[466]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[467]],
                Phase: &self.row.columns[self.index_mapping[468]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[469]],
                FishParameter: &self.row.columns[self.index_mapping[470]],
                ItemReceived: &self.row.columns[self.index_mapping[471]],
                Reward: [
                    &self.row.columns[self.index_mapping[472]],
                    &self.row.columns[self.index_mapping[473]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[474]],
                Phase: &self.row.columns[self.index_mapping[475]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[476]],
                FishParameter: &self.row.columns[self.index_mapping[477]],
                ItemReceived: &self.row.columns[self.index_mapping[478]],
                Reward: [
                    &self.row.columns[self.index_mapping[479]],
                    &self.row.columns[self.index_mapping[480]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[481]],
                Phase: &self.row.columns[self.index_mapping[482]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[483]],
                FishParameter: &self.row.columns[self.index_mapping[484]],
                ItemReceived: &self.row.columns[self.index_mapping[485]],
                Reward: [
                    &self.row.columns[self.index_mapping[486]],
                    &self.row.columns[self.index_mapping[487]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[488]],
                Phase: &self.row.columns[self.index_mapping[489]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[490]],
                FishParameter: &self.row.columns[self.index_mapping[491]],
                ItemReceived: &self.row.columns[self.index_mapping[492]],
                Reward: [
                    &self.row.columns[self.index_mapping[493]],
                    &self.row.columns[self.index_mapping[494]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[495]],
                Phase: &self.row.columns[self.index_mapping[496]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[497]],
                FishParameter: &self.row.columns[self.index_mapping[498]],
                ItemReceived: &self.row.columns[self.index_mapping[499]],
                Reward: [
                    &self.row.columns[self.index_mapping[500]],
                    &self.row.columns[self.index_mapping[501]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[502]],
                Phase: &self.row.columns[self.index_mapping[503]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[504]],
                FishParameter: &self.row.columns[self.index_mapping[505]],
                ItemReceived: &self.row.columns[self.index_mapping[506]],
                Reward: [
                    &self.row.columns[self.index_mapping[507]],
                    &self.row.columns[self.index_mapping[508]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[509]],
                Phase: &self.row.columns[self.index_mapping[510]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[511]],
                FishParameter: &self.row.columns[self.index_mapping[512]],
                ItemReceived: &self.row.columns[self.index_mapping[513]],
                Reward: [
                    &self.row.columns[self.index_mapping[514]],
                    &self.row.columns[self.index_mapping[515]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[516]],
                Phase: &self.row.columns[self.index_mapping[517]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[518]],
                FishParameter: &self.row.columns[self.index_mapping[519]],
                ItemReceived: &self.row.columns[self.index_mapping[520]],
                Reward: [
                    &self.row.columns[self.index_mapping[521]],
                    &self.row.columns[self.index_mapping[522]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[523]],
                Phase: &self.row.columns[self.index_mapping[524]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[525]],
                FishParameter: &self.row.columns[self.index_mapping[526]],
                ItemReceived: &self.row.columns[self.index_mapping[527]],
                Reward: [
                    &self.row.columns[self.index_mapping[528]],
                    &self.row.columns[self.index_mapping[529]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[530]],
                Phase: &self.row.columns[self.index_mapping[531]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[532]],
                FishParameter: &self.row.columns[self.index_mapping[533]],
                ItemReceived: &self.row.columns[self.index_mapping[534]],
                Reward: [
                    &self.row.columns[self.index_mapping[535]],
                    &self.row.columns[self.index_mapping[536]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[537]],
                Phase: &self.row.columns[self.index_mapping[538]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[539]],
                FishParameter: &self.row.columns[self.index_mapping[540]],
                ItemReceived: &self.row.columns[self.index_mapping[541]],
                Reward: [
                    &self.row.columns[self.index_mapping[542]],
                    &self.row.columns[self.index_mapping[543]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[544]],
                Phase: &self.row.columns[self.index_mapping[545]],
            },
            HWDGathererInspectionDataElement {
                RequiredItem: &self.row.columns[self.index_mapping[546]],
                FishParameter: &self.row.columns[self.index_mapping[547]],
                ItemReceived: &self.row.columns[self.index_mapping[548]],
                Reward: [
                    &self.row.columns[self.index_mapping[549]],
                    &self.row.columns[self.index_mapping[550]],
                ],
                AmountRequired: &self.row.columns[self.index_mapping[551]],
                Phase: &self.row.columns[self.index_mapping[552]],
            },
        ]
    }
}
