use std::io;

use quick_xml::events::*;
use quick_xml::{Writer, Result, se::to_string};



pub trait MoreWriter {
    fn named_value<K: AsRef<[u8]>, S: serde::Serialize>(&mut self, name: K, value: &S) -> Result<()>;
}

impl <W: io::Write>MoreWriter for Writer<W> {
    fn named_value<K: AsRef<[u8]>, S: serde::Serialize>(&mut self, name: K, value: &S) -> Result<()>{
        let name = name.as_ref();

        let temp = BytesStart::borrowed_name(name);
        self.write_event(Event::Start(temp.to_borrowed()))?;
            self.write_event(Event::Text(BytesText::from_escaped_str(&to_string(value).unwrap())))?;
        self.write_event(Event::End(temp.to_end()))?;

        Ok(())
    }
}