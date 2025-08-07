use json::codegen::Generator;
use json::{JsonValue, object::Object};

use std::io;

pub trait OrderedGenerator: Generator {
    #[inline(always)]
    fn write_object(&mut self, object: &json::object::Object) -> io::Result<()> {
        self.write_char(b'{')?;
        let iter_object = object.iter();

        let mut data: Vec<(&str, &json::JsonValue)> = Vec::new();
        for (key, value) in iter_object {
            data.push((key, value));
        }

        data.sort_by(|a, b| a.0.cmp(b.0));

        let mut iter = data.iter();

        if let Some((key, value)) = iter.next() {
            self.indent();
            self.new_line()?;
            self.write_string(key)?;
            self.write_min(b": ", b':')?;
            self.write_json(value)?;
        } else {
            self.write_char(b'}')?;
            return Ok(());
        }

        for (key, value) in iter {
            self.write_char(b',')?;
            self.new_line()?;
            self.write_string(key)?;
            self.write_min(b": ", b':')?;
            self.write_json(value)?;
        }

        self.dedent();
        self.new_line()?;
        self.write_char(b'}')
    }
}

pub struct PrettyGenerator {
    code: Vec<u8>,
    dent: u16,
    spaces_per_indent: u16,
    pretty: bool,
}

impl PrettyGenerator {
    pub fn new(spaces: u16, pretty: bool) -> Self {
        PrettyGenerator {
            code: Vec::with_capacity(1024),
            dent: 0,
            spaces_per_indent: spaces,
            pretty,
        }
    }

    pub fn consume(self) -> String {
        unsafe { String::from_utf8_unchecked(self.code) }
    }
}

impl OrderedGenerator for PrettyGenerator {}

impl Generator for PrettyGenerator
where
    Self: OrderedGenerator,
{
    type T = Vec<u8>;

    #[inline(always)]
    fn write_object(&mut self, object: &Object) -> io::Result<()> {
        OrderedGenerator::write_object(self, object)
    }

    #[inline(always)]
    fn write(&mut self, slice: &[u8]) -> io::Result<()> {
        self.code.extend_from_slice(slice);
        Ok(())
    }

    #[inline(always)]
    fn write_char(&mut self, ch: u8) -> io::Result<()> {
        self.code.push(ch);
        Ok(())
    }

    #[inline(always)]
    fn get_writer(&mut self) -> &mut Vec<u8> {
        &mut self.code
    }

    #[inline(always)]
    fn write_min(&mut self, slice: &[u8], _: u8) -> io::Result<()> {
        self.code.extend_from_slice(slice);
        Ok(())
    }

    fn new_line(&mut self) -> io::Result<()> {
        if self.pretty {
            self.code.push(b'\n');
            for _ in 0..(self.dent * self.spaces_per_indent) {
                self.code.push(b' ');
            }
        }
        Ok(())
    }

    fn indent(&mut self) {
        if self.pretty {
            self.dent += 1;
        }
    }

    fn dedent(&mut self) {
        if self.pretty && self.dent > 0 {
            self.dent -= 1;
        }
    }
}

pub fn stringify<T>(root: T) -> String
where
    T: Into<JsonValue>,
{
    let root: JsonValue = root.into();
    let mut generator = PrettyGenerator::new(0, false);
    generator.write_json(&root).expect("Can't fail");
    generator.consume()
}

/// Pretty prints out the value as JSON string. Second argument is a
/// number of spaces to indent new blocks with.
pub fn stringify_pretty<T>(root: T, spaces: u16) -> String
where
    T: Into<JsonValue>,
{
    let root: JsonValue = root.into();
    let mut generator = PrettyGenerator::new(spaces, true);
    generator.write_json(&root).expect("Can't fail");
    generator.consume()
}
