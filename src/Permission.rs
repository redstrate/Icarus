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
}
impl PermissionSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("Permission")?;
        let sheet = resolver.read_excel_sheet(&exh, "Permission", language)?;
        Ok(Self { sheet })
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
        Some(Self::Row { row })
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
}
impl<'a> PermissionRow<'a> {
    pub fn Unknown0(&'a self) -> bool {
        self.row.columns[0].into_bool().copied().unwrap()
    }
    pub fn Unknown1(&'a self) -> bool {
        self.row.columns[1].into_bool().copied().unwrap()
    }
    pub fn Unknown2(&'a self) -> bool {
        self.row.columns[2].into_bool().copied().unwrap()
    }
    pub fn Unknown3(&'a self) -> bool {
        self.row.columns[3].into_bool().copied().unwrap()
    }
    pub fn Unknown4(&'a self) -> bool {
        self.row.columns[4].into_bool().copied().unwrap()
    }
    pub fn Unknown5(&'a self) -> bool {
        self.row.columns[5].into_bool().copied().unwrap()
    }
    pub fn Unknown6(&'a self) -> bool {
        self.row.columns[6].into_bool().copied().unwrap()
    }
    pub fn Unknown7(&'a self) -> bool {
        self.row.columns[7].into_bool().copied().unwrap()
    }
    pub fn Unknown8(&'a self) -> bool {
        self.row.columns[8].into_bool().copied().unwrap()
    }
    pub fn Unknown9(&'a self) -> bool {
        self.row.columns[9].into_bool().copied().unwrap()
    }
    pub fn Unknown10(&'a self) -> bool {
        self.row.columns[10].into_bool().copied().unwrap()
    }
    pub fn Unknown11(&'a self) -> bool {
        self.row.columns[11].into_bool().copied().unwrap()
    }
    pub fn Unknown12(&'a self) -> bool {
        self.row.columns[12].into_bool().copied().unwrap()
    }
    pub fn Unknown13(&'a self) -> bool {
        self.row.columns[13].into_bool().copied().unwrap()
    }
    pub fn Unknown14(&'a self) -> bool {
        self.row.columns[14].into_bool().copied().unwrap()
    }
    pub fn Unknown15(&'a self) -> bool {
        self.row.columns[15].into_bool().copied().unwrap()
    }
    pub fn Unknown16(&'a self) -> bool {
        self.row.columns[16].into_bool().copied().unwrap()
    }
    pub fn Unknown17(&'a self) -> bool {
        self.row.columns[17].into_bool().copied().unwrap()
    }
    pub fn Unknown18(&'a self) -> bool {
        self.row.columns[18].into_bool().copied().unwrap()
    }
    pub fn Unknown19(&'a self) -> bool {
        self.row.columns[19].into_bool().copied().unwrap()
    }
    pub fn Unknown20(&'a self) -> bool {
        self.row.columns[20].into_bool().copied().unwrap()
    }
    pub fn Unknown21(&'a self) -> bool {
        self.row.columns[21].into_bool().copied().unwrap()
    }
    pub fn Unknown22(&'a self) -> bool {
        self.row.columns[22].into_bool().copied().unwrap()
    }
    pub fn Unknown23(&'a self) -> bool {
        self.row.columns[23].into_bool().copied().unwrap()
    }
    pub fn Unknown24(&'a self) -> bool {
        self.row.columns[24].into_bool().copied().unwrap()
    }
    pub fn Unknown25(&'a self) -> bool {
        self.row.columns[25].into_bool().copied().unwrap()
    }
    pub fn Unknown26(&'a self) -> bool {
        self.row.columns[26].into_bool().copied().unwrap()
    }
    pub fn Unknown27(&'a self) -> bool {
        self.row.columns[27].into_bool().copied().unwrap()
    }
    pub fn Unknown28(&'a self) -> bool {
        self.row.columns[28].into_bool().copied().unwrap()
    }
    pub fn Unknown29(&'a self) -> bool {
        self.row.columns[29].into_bool().copied().unwrap()
    }
    pub fn Unknown30(&'a self) -> bool {
        self.row.columns[30].into_bool().copied().unwrap()
    }
    pub fn Unknown31(&'a self) -> bool {
        self.row.columns[31].into_bool().copied().unwrap()
    }
    pub fn Unknown32(&'a self) -> bool {
        self.row.columns[32].into_bool().copied().unwrap()
    }
    pub fn Unknown33(&'a self) -> bool {
        self.row.columns[33].into_bool().copied().unwrap()
    }
    pub fn Unknown34(&'a self) -> bool {
        self.row.columns[34].into_bool().copied().unwrap()
    }
    pub fn Unknown35(&'a self) -> bool {
        self.row.columns[35].into_bool().copied().unwrap()
    }
    pub fn Unknown36(&'a self) -> bool {
        self.row.columns[36].into_bool().copied().unwrap()
    }
    pub fn Unknown37(&'a self) -> bool {
        self.row.columns[37].into_bool().copied().unwrap()
    }
    pub fn Unknown38(&'a self) -> bool {
        self.row.columns[38].into_bool().copied().unwrap()
    }
    pub fn Unknown39(&'a self) -> bool {
        self.row.columns[39].into_bool().copied().unwrap()
    }
    pub fn Unknown40(&'a self) -> bool {
        self.row.columns[40].into_bool().copied().unwrap()
    }
    pub fn Unknown41(&'a self) -> bool {
        self.row.columns[41].into_bool().copied().unwrap()
    }
    pub fn Unknown42(&'a self) -> bool {
        self.row.columns[42].into_bool().copied().unwrap()
    }
    pub fn Unknown43(&'a self) -> bool {
        self.row.columns[43].into_bool().copied().unwrap()
    }
    pub fn Unknown44(&'a self) -> bool {
        self.row.columns[44].into_bool().copied().unwrap()
    }
    pub fn Unknown45(&'a self) -> bool {
        self.row.columns[45].into_bool().copied().unwrap()
    }
    pub fn Unknown46(&'a self) -> bool {
        self.row.columns[46].into_bool().copied().unwrap()
    }
    pub fn Unknown47(&'a self) -> bool {
        self.row.columns[47].into_bool().copied().unwrap()
    }
    pub fn Unknown48(&'a self) -> bool {
        self.row.columns[48].into_bool().copied().unwrap()
    }
    pub fn Unknown49(&'a self) -> bool {
        self.row.columns[49].into_bool().copied().unwrap()
    }
    pub fn Unknown50(&'a self) -> bool {
        self.row.columns[50].into_bool().copied().unwrap()
    }
    pub fn Unknown51(&'a self) -> bool {
        self.row.columns[51].into_bool().copied().unwrap()
    }
    pub fn Unknown52(&'a self) -> bool {
        self.row.columns[52].into_bool().copied().unwrap()
    }
    pub fn Unknown53(&'a self) -> bool {
        self.row.columns[53].into_bool().copied().unwrap()
    }
    pub fn Unknown54(&'a self) -> bool {
        self.row.columns[54].into_bool().copied().unwrap()
    }
    pub fn Unknown55(&'a self) -> bool {
        self.row.columns[55].into_bool().copied().unwrap()
    }
    pub fn Unknown56(&'a self) -> bool {
        self.row.columns[56].into_bool().copied().unwrap()
    }
    pub fn Unknown57(&'a self) -> bool {
        self.row.columns[57].into_bool().copied().unwrap()
    }
    pub fn Unknown58(&'a self) -> bool {
        self.row.columns[58].into_bool().copied().unwrap()
    }
    pub fn Unknown59(&'a self) -> bool {
        self.row.columns[59].into_bool().copied().unwrap()
    }
    pub fn Unknown60(&'a self) -> bool {
        self.row.columns[60].into_bool().copied().unwrap()
    }
    pub fn Unknown61(&'a self) -> bool {
        self.row.columns[61].into_bool().copied().unwrap()
    }
    pub fn Unknown62(&'a self) -> bool {
        self.row.columns[62].into_bool().copied().unwrap()
    }
    pub fn Unknown63(&'a self) -> bool {
        self.row.columns[63].into_bool().copied().unwrap()
    }
    pub fn Unknown64(&'a self) -> bool {
        self.row.columns[64].into_bool().copied().unwrap()
    }
    pub fn Unknown65(&'a self) -> bool {
        self.row.columns[65].into_bool().copied().unwrap()
    }
    pub fn Unknown66(&'a self) -> bool {
        self.row.columns[66].into_bool().copied().unwrap()
    }
    pub fn Unknown67(&'a self) -> bool {
        self.row.columns[67].into_bool().copied().unwrap()
    }
    pub fn Unknown68(&'a self) -> bool {
        self.row.columns[68].into_bool().copied().unwrap()
    }
    pub fn Unknown69(&'a self) -> bool {
        self.row.columns[69].into_bool().copied().unwrap()
    }
    pub fn Unknown70(&'a self) -> bool {
        self.row.columns[70].into_bool().copied().unwrap()
    }
    pub fn Unknown71(&'a self) -> bool {
        self.row.columns[71].into_bool().copied().unwrap()
    }
    pub fn Unknown72(&'a self) -> bool {
        self.row.columns[72].into_bool().copied().unwrap()
    }
    pub fn Unknown73(&'a self) -> bool {
        self.row.columns[73].into_bool().copied().unwrap()
    }
    pub fn Unknown74(&'a self) -> bool {
        self.row.columns[74].into_bool().copied().unwrap()
    }
    pub fn Unknown75(&'a self) -> bool {
        self.row.columns[75].into_bool().copied().unwrap()
    }
    pub fn Unknown76(&'a self) -> bool {
        self.row.columns[76].into_bool().copied().unwrap()
    }
    pub fn Unknown77(&'a self) -> bool {
        self.row.columns[77].into_bool().copied().unwrap()
    }
    pub fn Unknown78(&'a self) -> bool {
        self.row.columns[78].into_bool().copied().unwrap()
    }
    pub fn Unknown79(&'a self) -> bool {
        self.row.columns[79].into_bool().copied().unwrap()
    }
    pub fn Unknown80(&'a self) -> bool {
        self.row.columns[80].into_bool().copied().unwrap()
    }
    pub fn Unknown81(&'a self) -> bool {
        self.row.columns[81].into_bool().copied().unwrap()
    }
    pub fn Unknown82(&'a self) -> bool {
        self.row.columns[82].into_bool().copied().unwrap()
    }
    pub fn Unknown83(&'a self) -> bool {
        self.row.columns[83].into_bool().copied().unwrap()
    }
    pub fn Unknown84(&'a self) -> bool {
        self.row.columns[84].into_bool().copied().unwrap()
    }
    pub fn Unknown85(&'a self) -> bool {
        self.row.columns[85].into_bool().copied().unwrap()
    }
    pub fn Unknown86(&'a self) -> bool {
        self.row.columns[86].into_bool().copied().unwrap()
    }
    pub fn Unknown87(&'a self) -> bool {
        self.row.columns[87].into_bool().copied().unwrap()
    }
    pub fn Unknown88(&'a self) -> bool {
        self.row.columns[88].into_bool().copied().unwrap()
    }
    pub fn Unknown89(&'a self) -> bool {
        self.row.columns[89].into_bool().copied().unwrap()
    }
    pub fn Unknown90(&'a self) -> bool {
        self.row.columns[90].into_bool().copied().unwrap()
    }
    pub fn Unknown91(&'a self) -> bool {
        self.row.columns[91].into_bool().copied().unwrap()
    }
    pub fn Unknown92(&'a self) -> bool {
        self.row.columns[92].into_bool().copied().unwrap()
    }
    pub fn Unknown93(&'a self) -> bool {
        self.row.columns[93].into_bool().copied().unwrap()
    }
    pub fn Unknown94(&'a self) -> bool {
        self.row.columns[94].into_bool().copied().unwrap()
    }
    pub fn Unknown95(&'a self) -> bool {
        self.row.columns[95].into_bool().copied().unwrap()
    }
    pub fn Unknown96(&'a self) -> bool {
        self.row.columns[96].into_bool().copied().unwrap()
    }
    pub fn Unknown97(&'a self) -> bool {
        self.row.columns[97].into_bool().copied().unwrap()
    }
    pub fn Unknown98(&'a self) -> bool {
        self.row.columns[98].into_bool().copied().unwrap()
    }
    pub fn Unknown99(&'a self) -> bool {
        self.row.columns[99].into_bool().copied().unwrap()
    }
    pub fn Unknown100(&'a self) -> bool {
        self.row.columns[100].into_bool().copied().unwrap()
    }
    pub fn Unknown101(&'a self) -> bool {
        self.row.columns[101].into_bool().copied().unwrap()
    }
    pub fn Unknown102(&'a self) -> bool {
        self.row.columns[102].into_bool().copied().unwrap()
    }
    pub fn Unknown104(&'a self) -> bool {
        self.row.columns[103].into_bool().copied().unwrap()
    }
    pub fn Unknown105(&'a self) -> bool {
        self.row.columns[104].into_bool().copied().unwrap()
    }
    pub fn Unknown106(&'a self) -> bool {
        self.row.columns[105].into_bool().copied().unwrap()
    }
    pub fn Unknown107(&'a self) -> bool {
        self.row.columns[106].into_bool().copied().unwrap()
    }
    pub fn Unknown108(&'a self) -> bool {
        self.row.columns[107].into_bool().copied().unwrap()
    }
    pub fn Unknown109(&'a self) -> bool {
        self.row.columns[108].into_bool().copied().unwrap()
    }
    pub fn Unknown110(&'a self) -> bool {
        self.row.columns[109].into_bool().copied().unwrap()
    }
    pub fn Unknown111(&'a self) -> bool {
        self.row.columns[110].into_bool().copied().unwrap()
    }
    pub fn Unknown103(&'a self) -> bool {
        self.row.columns[111].into_bool().copied().unwrap()
    }
}
