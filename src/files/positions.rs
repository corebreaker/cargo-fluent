use std::{ops::Range, path::Path, io::{Result, BufReader}, fs::File, fmt::{Display, Formatter, Result as FmtResult}};
use std::io::BufRead;

struct PositionDesc {
    line: usize,
    range: Range<usize>,
    text: String,
}

impl PositionDesc {
    fn new(line: usize, start: usize, size: usize, text: String) -> Self {
        Self {
            text,
            line,
            range: Range {
                start,
                end: start + size
            }
        }
    }
}

pub(crate) struct FilePosition {
    line: usize,
    column: usize,
}

impl FilePosition {
    fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub(crate) fn line(&self) -> usize {
        self.line
    }

    pub(crate) fn column(&self) -> usize {
        self.column
    }
}

impl Display for FilePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}:{}", self.line, self.column)
    }
}

pub(crate) struct FilePositions {
    last: usize,
    lines: Vec<PositionDesc>,
}

impl FilePositions {
    pub(crate) fn read(path: &Path) -> Result<Self> {
        let mut input = BufReader::new(File::open(path)?);

        let mut lines = vec![];
        let mut line = 1;
        let mut last = 0;
        let mut n = 0;

        loop {
            let mut text = String::new();

            match input.read_line(&mut text)? {
                0 => { break; },
                size => {
                    let start = last + n;

                    lines.push(PositionDesc::new(line, start, size, text));

                    last = start;
                    n = size;
                }
            }

            line += 1;
        }

        Ok(Self { lines, last })
    }

    pub(crate) fn get_position_from_offset(&self, offset: usize) -> FilePosition {
        let mut line = 1usize;

        for desc in &self.lines {
            if desc.range.contains(&offset) {
                let subtext = &desc.text[0..(offset - desc.range.start + 1)];

                return FilePosition::new(line, subtext.chars().count() + 1);
            }

            line += 1;
        }

        FilePosition::new(self.lines.len(), offset - self.last + 1)
    }
}
