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
pub struct PermissionSheet {
    sheet: Sheet,
    index_mapping: Vec<usize>,
}
impl PermissionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Permission")?;
        let sheet = resolver.read_excel_sheet(&exh, "Permission", language)?;
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
    pub fn row(&self, row_id: u32) -> Option<PermissionRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PermissionRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl<'a> StructuredSheet<'a> for PermissionSheet {
    type Row = PermissionRow<'a>;
    fn read_row(&self, row: &'a Row) -> Option<Self::Row> {
        Some(Self::Row {
            row,
            index_mapping: self.index_mapping.clone(),
        })
    }
}
impl<'a> IntoIterator for &'a PermissionSheet {
    type Item = (u32, Vec<(u16, PermissionRow<'a>)>);
    type IntoIter = StructuredSheetIterator<'a, PermissionSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PermissionSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PermissionRow<'a> {
    row: &'a Row,
    index_mapping: Vec<usize>,
}
impl<'a> PermissionRow<'a> {
    pub fn Unknown0(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[0]]
    }
    pub fn Unknown1(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[1]]
    }
    pub fn Unknown2(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[2]]
    }
    pub fn Unknown3(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[3]]
    }
    pub fn Unknown4(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[4]]
    }
    pub fn Unknown5(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[5]]
    }
    pub fn Unknown6(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[6]]
    }
    pub fn Unknown7(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[7]]
    }
    pub fn Unknown8(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[8]]
    }
    pub fn Unknown9(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[9]]
    }
    pub fn Unknown10(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[10]]
    }
    pub fn Unknown11(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[11]]
    }
    pub fn Unknown12(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[12]]
    }
    pub fn Unknown13(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[13]]
    }
    pub fn Unknown14(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[14]]
    }
    pub fn Unknown15(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[15]]
    }
    pub fn Unknown16(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[16]]
    }
    pub fn Unknown17(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[17]]
    }
    pub fn Unknown18(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[18]]
    }
    pub fn Unknown19(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[19]]
    }
    pub fn Unknown20(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[20]]
    }
    pub fn Unknown21(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[21]]
    }
    pub fn Unknown22(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[22]]
    }
    pub fn Unknown23(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[23]]
    }
    pub fn Unknown24(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[24]]
    }
    pub fn Unknown25(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[25]]
    }
    pub fn Unknown26(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[26]]
    }
    pub fn Unknown27(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[27]]
    }
    pub fn Unknown28(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[28]]
    }
    pub fn Unknown29(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[29]]
    }
    pub fn Unknown30(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[30]]
    }
    pub fn Unknown31(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[31]]
    }
    pub fn Unknown32(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[32]]
    }
    pub fn Unknown33(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[33]]
    }
    pub fn Unknown34(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[34]]
    }
    pub fn Unknown35(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[35]]
    }
    pub fn Unknown36(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[36]]
    }
    pub fn Unknown37(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[37]]
    }
    pub fn Unknown38(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[38]]
    }
    pub fn Unknown39(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[39]]
    }
    pub fn Unknown40(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[40]]
    }
    pub fn Unknown41(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[41]]
    }
    pub fn Unknown42(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[42]]
    }
    pub fn Unknown43(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[43]]
    }
    pub fn Unknown44(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[44]]
    }
    pub fn Unknown45(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[45]]
    }
    pub fn Unknown46(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[46]]
    }
    pub fn Unknown47(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[47]]
    }
    pub fn Unknown48(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[48]]
    }
    pub fn Unknown49(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[49]]
    }
    pub fn Unknown50(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[50]]
    }
    pub fn Unknown51(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[51]]
    }
    pub fn Unknown52(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[52]]
    }
    pub fn Unknown53(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[53]]
    }
    pub fn Unknown54(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[54]]
    }
    pub fn Unknown55(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[55]]
    }
    pub fn Unknown56(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[56]]
    }
    pub fn Unknown57(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[57]]
    }
    pub fn Unknown58(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[58]]
    }
    pub fn Unknown59(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[59]]
    }
    pub fn Unknown60(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[60]]
    }
    pub fn Unknown61(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[61]]
    }
    pub fn Unknown62(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[62]]
    }
    pub fn Unknown63(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[63]]
    }
    pub fn Unknown64(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[64]]
    }
    pub fn Unknown65(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[65]]
    }
    pub fn Unknown66(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[66]]
    }
    pub fn Unknown67(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[67]]
    }
    pub fn Unknown68(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[68]]
    }
    pub fn Unknown69(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[69]]
    }
    pub fn Unknown70(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[70]]
    }
    pub fn Unknown71(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[71]]
    }
    pub fn Unknown72(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[72]]
    }
    pub fn Unknown73(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[73]]
    }
    pub fn Unknown74(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[74]]
    }
    pub fn Unknown75(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[75]]
    }
    pub fn Unknown76(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[76]]
    }
    pub fn Unknown77(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[77]]
    }
    pub fn Unknown78(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[78]]
    }
    pub fn Unknown79(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[79]]
    }
    pub fn Unknown80(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[80]]
    }
    pub fn Unknown81(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[81]]
    }
    pub fn Unknown82(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[82]]
    }
    pub fn Unknown83(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[83]]
    }
    pub fn Unknown84(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[84]]
    }
    pub fn Unknown85(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[85]]
    }
    pub fn Unknown86(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[86]]
    }
    pub fn Unknown87(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[87]]
    }
    pub fn Unknown88(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[88]]
    }
    pub fn Unknown89(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[89]]
    }
    pub fn Unknown90(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[90]]
    }
    pub fn Unknown91(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[91]]
    }
    pub fn Unknown92(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[92]]
    }
    pub fn Unknown93(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[93]]
    }
    pub fn Unknown94(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[94]]
    }
    pub fn Unknown95(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[95]]
    }
    pub fn Unknown96(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[96]]
    }
    pub fn Unknown97(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[97]]
    }
    pub fn Unknown98(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[98]]
    }
    pub fn Unknown99(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[99]]
    }
    pub fn Unknown100(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[100]]
    }
    pub fn Unknown101(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[101]]
    }
    pub fn Unknown102(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[102]]
    }
    pub fn Unknown104(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[103]]
    }
    pub fn Unknown105(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[104]]
    }
    pub fn Unknown106(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[105]]
    }
    pub fn Unknown107(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[106]]
    }
    pub fn Unknown108(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[107]]
    }
    pub fn Unknown109(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[108]]
    }
    pub fn Unknown110(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[109]]
    }
    pub fn Unknown111(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[110]]
    }
    pub fn Unknown103(&'a self) -> &'a Field {
        &self.row.columns[self.index_mapping[111]]
    }
}
