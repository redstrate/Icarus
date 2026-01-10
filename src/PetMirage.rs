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
pub struct PetMirageSheet {
    sheet: Sheet,
}
impl PetMirageSheet {
    /// Read the sheet from a `ResourceResolver`.
    pub fn read_from(
        resolver: &mut ResourceResolver,
        language: Language,
    ) -> Result<Self, Error> {
        let exh = resolver.read_excel_sheet_header("PetMirage")?;
        let sheet = resolver.read_excel_sheet(&exh, "PetMirage", language)?;
        Ok(Self { sheet })
    }
    /// Fetches a single row from the sheet. If the row contains subrows, it returns the first one.
    pub fn row(&self, row_id: u32) -> Option<PetMirageRow> {
        let row = &self.sheet.row(row_id)?;
        self.read_row(row)
    }
    /// Fetches the specified subrow from the sheet.
    pub fn subrow(&self, row_id: u32, subrow_id: u16) -> Option<PetMirageRow> {
        let row = &self.sheet.subrow(row_id, subrow_id)?;
        self.read_row(row)
    }
    /// Returns the number of rows in this sheet.
    pub fn row_count(&self) -> u32 {
        self.sheet.exh.header.row_count
    }
}
impl StructuredSheet for PetMirageSheet {
    type Row = PetMirageRow;
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
impl<'a> IntoIterator for &'a PetMirageSheet {
    type Item = (u32, Vec<(u16, PetMirageRow)>);
    type IntoIter = StructuredSheetIterator<'a, PetMirageSheet>;
    fn into_iter(self) -> StructuredSheetIterator<'a, PetMirageSheet> {
        StructuredSheetIterator {
            sheet: self,
            iterator: (&self.sheet).into_iter(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PetMirageRow {
    columns: Vec<Field>,
}
impl PetMirageRow {
    pub fn Name<'a>(&'a self) -> &'a Field {
        &self.columns[0]
    }
    pub fn Unknown0<'a>(&'a self) -> &'a Field {
        &self.columns[1]
    }
    pub fn Unknown1<'a>(&'a self) -> &'a Field {
        &self.columns[2]
    }
    pub fn Unknown2<'a>(&'a self) -> &'a Field {
        &self.columns[3]
    }
    pub fn Unknown3<'a>(&'a self) -> &'a Field {
        &self.columns[4]
    }
    pub fn Unknown4<'a>(&'a self) -> &'a Field {
        &self.columns[5]
    }
    pub fn Unknown5<'a>(&'a self) -> &'a Field {
        &self.columns[6]
    }
    pub fn Unknown6<'a>(&'a self) -> &'a Field {
        &self.columns[7]
    }
    pub fn Unknown7<'a>(&'a self) -> &'a Field {
        &self.columns[8]
    }
    pub fn Unknown8<'a>(&'a self) -> &'a Field {
        &self.columns[9]
    }
    pub fn Unknown9<'a>(&'a self) -> &'a Field {
        &self.columns[10]
    }
    pub fn Unknown10<'a>(&'a self) -> &'a Field {
        &self.columns[11]
    }
    pub fn Unknown11<'a>(&'a self) -> &'a Field {
        &self.columns[12]
    }
    pub fn Unknown12<'a>(&'a self) -> &'a Field {
        &self.columns[13]
    }
    pub fn Unknown13<'a>(&'a self) -> &'a Field {
        &self.columns[14]
    }
    pub fn Unknown14<'a>(&'a self) -> &'a Field {
        &self.columns[15]
    }
    pub fn Unknown15<'a>(&'a self) -> &'a Field {
        &self.columns[16]
    }
    pub fn Unknown16<'a>(&'a self) -> &'a Field {
        &self.columns[17]
    }
    pub fn Unknown17<'a>(&'a self) -> &'a Field {
        &self.columns[18]
    }
    pub fn Unknown18<'a>(&'a self) -> &'a Field {
        &self.columns[19]
    }
    pub fn Unknown19<'a>(&'a self) -> &'a Field {
        &self.columns[20]
    }
    pub fn Unknown20<'a>(&'a self) -> &'a Field {
        &self.columns[21]
    }
    pub fn Unknown21<'a>(&'a self) -> &'a Field {
        &self.columns[22]
    }
    pub fn Unknown22<'a>(&'a self) -> &'a Field {
        &self.columns[23]
    }
    pub fn Unknown23<'a>(&'a self) -> &'a Field {
        &self.columns[24]
    }
    pub fn Unknown24<'a>(&'a self) -> &'a Field {
        &self.columns[25]
    }
    pub fn Unknown25<'a>(&'a self) -> &'a Field {
        &self.columns[26]
    }
    pub fn Unknown26<'a>(&'a self) -> &'a Field {
        &self.columns[27]
    }
    pub fn Unknown27<'a>(&'a self) -> &'a Field {
        &self.columns[28]
    }
    pub fn Unknown28<'a>(&'a self) -> &'a Field {
        &self.columns[29]
    }
    pub fn Unknown29<'a>(&'a self) -> &'a Field {
        &self.columns[30]
    }
    pub fn Unknown30<'a>(&'a self) -> &'a Field {
        &self.columns[31]
    }
    pub fn Unknown31<'a>(&'a self) -> &'a Field {
        &self.columns[32]
    }
    pub fn Unknown32<'a>(&'a self) -> &'a Field {
        &self.columns[33]
    }
    pub fn Unknown33<'a>(&'a self) -> &'a Field {
        &self.columns[34]
    }
    pub fn Unknown34<'a>(&'a self) -> &'a Field {
        &self.columns[35]
    }
    pub fn Unknown35<'a>(&'a self) -> &'a Field {
        &self.columns[36]
    }
    pub fn Unknown36<'a>(&'a self) -> &'a Field {
        &self.columns[37]
    }
    pub fn Unknown37<'a>(&'a self) -> &'a Field {
        &self.columns[38]
    }
    pub fn Unknown38<'a>(&'a self) -> &'a Field {
        &self.columns[39]
    }
    pub fn Unknown39<'a>(&'a self) -> &'a Field {
        &self.columns[40]
    }
    pub fn Unknown40<'a>(&'a self) -> &'a Field {
        &self.columns[41]
    }
    pub fn Unknown41<'a>(&'a self) -> &'a Field {
        &self.columns[42]
    }
    pub fn Unknown42<'a>(&'a self) -> &'a Field {
        &self.columns[43]
    }
    pub fn Unknown43<'a>(&'a self) -> &'a Field {
        &self.columns[44]
    }
    pub fn Unknown44<'a>(&'a self) -> &'a Field {
        &self.columns[45]
    }
    pub fn Unknown45<'a>(&'a self) -> &'a Field {
        &self.columns[46]
    }
    pub fn Unknown46<'a>(&'a self) -> &'a Field {
        &self.columns[47]
    }
    pub fn Unknown47<'a>(&'a self) -> &'a Field {
        &self.columns[48]
    }
    pub fn Unknown48<'a>(&'a self) -> &'a Field {
        &self.columns[49]
    }
    pub fn Unknown49<'a>(&'a self) -> &'a Field {
        &self.columns[50]
    }
    pub fn Unknown50<'a>(&'a self) -> &'a Field {
        &self.columns[51]
    }
    pub fn Unknown51<'a>(&'a self) -> &'a Field {
        &self.columns[52]
    }
    pub fn Unknown52<'a>(&'a self) -> &'a Field {
        &self.columns[53]
    }
    pub fn Unknown53<'a>(&'a self) -> &'a Field {
        &self.columns[54]
    }
    pub fn Unknown54<'a>(&'a self) -> &'a Field {
        &self.columns[55]
    }
    pub fn Unknown55<'a>(&'a self) -> &'a Field {
        &self.columns[56]
    }
    pub fn Unknown56<'a>(&'a self) -> &'a Field {
        &self.columns[57]
    }
    pub fn Unknown57<'a>(&'a self) -> &'a Field {
        &self.columns[58]
    }
    pub fn Unknown58<'a>(&'a self) -> &'a Field {
        &self.columns[59]
    }
    pub fn Unknown59<'a>(&'a self) -> &'a Field {
        &self.columns[60]
    }
    pub fn Scale<'a>(&'a self) -> &'a Field {
        &self.columns[61]
    }
    pub fn ModelChara<'a>(&'a self) -> &'a Field {
        &self.columns[62]
    }
}
