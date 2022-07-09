use std::{collections::HashMap, path::Path};

use crate::{
    document::{Paragraph, Run},
    Docx, DocxResult,
};

pub enum Part<'a> {
    Paragraph(&'a mut Paragraph<'a>),
    Run(&'a mut Run<'a>),
}

pub struct MergeGroups<'a> {
    pub name: String,
    pub contents: Vec<Part<'a>>,
}

pub fn mail_merge<P>(template: &Docx, _map: HashMap<String, String>, path: P) -> DocxResult<()>
where
    P: AsRef<Path>,
{
    let mut docx = template.clone();
    let _dmap: HashMap<String, Vec<Part>> = HashMap::new();
    let is_merge_field = false;
    for c in docx.document.body.content.iter() {
        match c {
            crate::document::BodyContent::Paragraph(p) => {
                if !is_merge_field {
                    let mut iter = p.content.iter().skip_while(|pc| {
                        if let crate::document::ParagraphContent::Run(r) = pc {
                            let mut v = true;
                            for rc in r.content.iter() {
                                if let crate::document::RunContent::FieldChar(fc) = rc {
                                    if let Some(ct) = &fc.ty {
                                        if let crate::document::CharType::Begin = ct {
                                            v = false;
                                            break;
                                        }
                                    }
                                };
                            }
                            v
                        } else {
                            true
                        }
                    });

                    if let Some(_pc) = iter.next() {}
                } else {
                    // if it's close part, add merged content. Otherwise, ignore it.
                }
            }
            crate::document::BodyContent::Table(_t) => {}
            crate::document::BodyContent::SectionProperty(_) => {}
            crate::document::BodyContent::Sdt(_) => {}
        }
    }
    let _f = docx.write_file(path)?;
    Ok(())
}
