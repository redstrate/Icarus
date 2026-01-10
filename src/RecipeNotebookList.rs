//! This file is auto-generated, do not edit it manually! This is generated based on the schema from https://github.com/xivdev/EXDSchema.
#![allow(warnings)]
use crate::{StructuredSheet, StructuredSheetIterator};
use physis::{
    Error, resource::{Resource, ResourceResolver},
    exd::EXD, exh::{EXH, ExcelColumnDefinition},
    excel::{Sheet, Field, Row},
    Language,
};
#[derive(Debug, Clone)]
pub struct RecipeNotebookListSheet {
    sheet: Sheet,
}
impl RecipeNotebookListSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("RecipeNotebookList")?;
        let sheet = resolver.read_excel_sheet(&exh, "RecipeNotebookList", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<RecipeNotebookListRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<RecipeNotebookListRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for RecipeNotebookListSheet {
    type Row = RecipeNotebookListRow;
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
impl<'a> IntoIterator for &'a RecipeNotebookListSheet {
    type Item = (u32, Vec<(u16, RecipeNotebookListRow)>);
    type IntoIter = StructuredSheetIterator<'a, RecipeNotebookListSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, RecipeNotebookListSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RecipeNotebookListRow {
    columns: Vec<Field>,
}
impl RecipeNotebookListRow {
    pub fn Recipe<'a>(&'a self) -> [&'a Field; 160] {
        [
            &self.columns[0],
            &self.columns[1],
            &self.columns[2],
            &self.columns[3],
            &self.columns[4],
            &self.columns[5],
            &self.columns[6],
            &self.columns[7],
            &self.columns[8],
            &self.columns[9],
            &self.columns[10],
            &self.columns[11],
            &self.columns[12],
            &self.columns[13],
            &self.columns[14],
            &self.columns[15],
            &self.columns[16],
            &self.columns[17],
            &self.columns[18],
            &self.columns[19],
            &self.columns[20],
            &self.columns[21],
            &self.columns[22],
            &self.columns[23],
            &self.columns[24],
            &self.columns[25],
            &self.columns[26],
            &self.columns[27],
            &self.columns[28],
            &self.columns[29],
            &self.columns[30],
            &self.columns[31],
            &self.columns[32],
            &self.columns[33],
            &self.columns[34],
            &self.columns[35],
            &self.columns[36],
            &self.columns[37],
            &self.columns[38],
            &self.columns[39],
            &self.columns[40],
            &self.columns[41],
            &self.columns[42],
            &self.columns[43],
            &self.columns[44],
            &self.columns[45],
            &self.columns[46],
            &self.columns[47],
            &self.columns[48],
            &self.columns[49],
            &self.columns[50],
            &self.columns[51],
            &self.columns[52],
            &self.columns[53],
            &self.columns[54],
            &self.columns[55],
            &self.columns[56],
            &self.columns[57],
            &self.columns[58],
            &self.columns[59],
            &self.columns[60],
            &self.columns[61],
            &self.columns[62],
            &self.columns[63],
            &self.columns[64],
            &self.columns[65],
            &self.columns[66],
            &self.columns[67],
            &self.columns[68],
            &self.columns[69],
            &self.columns[70],
            &self.columns[71],
            &self.columns[72],
            &self.columns[73],
            &self.columns[74],
            &self.columns[75],
            &self.columns[76],
            &self.columns[77],
            &self.columns[78],
            &self.columns[79],
            &self.columns[80],
            &self.columns[81],
            &self.columns[82],
            &self.columns[83],
            &self.columns[84],
            &self.columns[85],
            &self.columns[86],
            &self.columns[87],
            &self.columns[88],
            &self.columns[89],
            &self.columns[90],
            &self.columns[91],
            &self.columns[92],
            &self.columns[93],
            &self.columns[94],
            &self.columns[95],
            &self.columns[96],
            &self.columns[97],
            &self.columns[98],
            &self.columns[99],
            &self.columns[100],
            &self.columns[101],
            &self.columns[102],
            &self.columns[103],
            &self.columns[104],
            &self.columns[105],
            &self.columns[106],
            &self.columns[107],
            &self.columns[108],
            &self.columns[109],
            &self.columns[110],
            &self.columns[111],
            &self.columns[112],
            &self.columns[113],
            &self.columns[114],
            &self.columns[115],
            &self.columns[116],
            &self.columns[117],
            &self.columns[118],
            &self.columns[119],
            &self.columns[120],
            &self.columns[121],
            &self.columns[122],
            &self.columns[123],
            &self.columns[124],
            &self.columns[125],
            &self.columns[126],
            &self.columns[127],
            &self.columns[128],
            &self.columns[129],
            &self.columns[130],
            &self.columns[131],
            &self.columns[132],
            &self.columns[133],
            &self.columns[134],
            &self.columns[135],
            &self.columns[136],
            &self.columns[137],
            &self.columns[138],
            &self.columns[139],
            &self.columns[140],
            &self.columns[141],
            &self.columns[142],
            &self.columns[143],
            &self.columns[144],
            &self.columns[145],
            &self.columns[146],
            &self.columns[147],
            &self.columns[148],
            &self.columns[149],
            &self.columns[150],
            &self.columns[151],
            &self.columns[152],
            &self.columns[153],
            &self.columns[154],
            &self.columns[155],
            &self.columns[156],
            &self.columns[157],
            &self.columns[158],
            &self.columns[159],
        ]
    }
    pub fn Count<'a>(&'a self) -> &'a Field {
        &self.columns[160]
    }
}
