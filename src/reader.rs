use std::io;
use quick_xml::events::*;
use quick_xml::{Result, Error};

pub struct Reader<B: io::BufRead> {
    pub reader: quick_xml::Reader<B>,
    pub buffer: Vec<u8>,
    buffer2: Vec<u8>,
}

impl<B: io::BufRead> Reader<B> {
    pub fn new(reader: quick_xml::Reader<B>) -> Self {
        Reader {
            reader: reader,
            buffer: Vec::new(),
            buffer2: Vec::new(),
        }
    }

    pub fn read_event(&mut self) -> Result<Event<'_>> {
        self.reader.read_event(&mut self.buffer)
    }

    /*pub fn next_start(&mut self) -> Result<Event<'_>> {
        let ret = loop {
            self.buffer.clear();

            match self.read_event() {
                Ok(Event::Start(e)) => break Ok(e.into_owned()),
                Ok(_) => (),
                Err(e) => break Err(e),
            }
        };

        match ret {
            Ok(x) => Ok(Event::Start(x)),
            Err(x) => Err(x),
        }
    }*/

    pub fn next(&mut self) -> Result<Option<BytesStart<'_>>> {
        let len = loop {
            self.buffer.clear();

            match self.read_event() {
                Ok(Event::Start(ref e)) => break e.local_name().len(),
                Ok(Event::Empty(ref e)) => break e.local_name().len(),
                Ok(Event::Eof) => return Ok(None),
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        };

        Ok(Some(BytesStart::borrowed(&self.buffer, len)))
    }

    //pub fn next_start_helper(&mut self) -> Result<Option<Event<'_>>> {
    //    
    //}

    pub fn find<'a, K: AsRef<[u8]>>(&mut self, start: K) -> Result<Option<BytesStart<'_>>>{
        let start = start.as_ref();

        let len = loop {
            self.buffer.clear();

            match self.read_event() {
                Ok(Event::Start(ref e)) if e.name() == start => break e.local_name().len(),
                Ok(Event::Empty(ref e)) if e.name() == start => break e.local_name().len(),
                Ok(Event::Eof) => return Ok(None),
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        };

        Ok(Some(BytesStart::borrowed(&self.buffer, len)))
    }

    fn read_until_helper<K: AsRef<[u8]>>(&mut self, end: K) -> Result<()> {
        let mut depth = 0;
        let end = end.as_ref();

        self.buffer2.clear();
        let mut writer = quick_xml::Writer::new(&mut self.buffer2);

        loop {
            self.buffer.clear();

            match self.reader.read_event(&mut self.buffer) {
                Ok(ref x) => {
                    match x {
                        Event::End(ref e) if e.name() == end => {
                            if depth == 0 {
                                break;
                            }
                            depth -= 1;
                        }
                        Event::Start(ref e) if e.name() == end => depth += 1,
                        Event::Eof => {
                            return Err(Error::UnexpectedEof(format!("</{:?}>", std::str::from_utf8(end))));
                        }
                        _ => (),
                    }
                    writer.write_event(x).unwrap();
                }
                Err(e) => return Err(e),
            }
        };

        self.buffer.clear();

        Ok(())
    }

    pub fn read_until_u8<K: AsRef<[u8]>>(&mut self, end: K) -> Result<&[u8]> {
        self.read_until_helper(end)?;
        Ok(&self.buffer2)
    }

    pub fn read_until_str<K: AsRef<[u8]>>(&mut self, end: K) -> Result<&str> {
        self.read_until_helper(end)?;
        Ok(std::str::from_utf8(&self.buffer2).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const XML: &str = 
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>

    <settings>
        <colors>
            <default><Rgb r=\"255\" g=\"255\" b=\"255\"/></default>
            <background><Rgb r=\"0\" g=\"0\" b=\"0\"/></background>
            <pointer><Rgb r=\"0\" g=\"255\" b=\"255\"/></pointer>
            <selected><Rgb r=\"255\" g=\"0\" b=\"255\"/></selected>
        </colors>
    </settings>";

    #[test]
    fn test_1() {
        let mut reader = Reader::new(quick_xml::Reader::from_str(XML));

        println!("{:?}", reader.find(b"settings"));
        println!("{:?}", reader.next());

        if let Ok(Some(temp)) = reader.find(b"Rgb"){
            println!("{}", std::str::from_utf8(temp.name()).unwrap());
            let attr: Vec<_> = temp.attributes().map(|attr| attr.unwrap()).collect();
            println!("{:?}", attr);
        }
    }
}
