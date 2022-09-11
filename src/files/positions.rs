use std::{ops::Range, path::Path, io::{Result, BufReader}, fs::File, fmt::{Display, Formatter, Result as FmtResult}};
use std::io::BufRead;

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
    lines: Vec<(Range<usize>, String)>,
}

impl FilePositions {
    pub(crate) fn read(path: &Path) -> Result<Self> {
        let mut input = BufReader::new(File::open(path)?);

        let mut lines = vec![];
        let mut line = 1;
        let mut last = 0;
        let mut size = 0;

        loop {
            let mut buf = String::new();

            match input.read_line(&mut buf)? {
                0 => { break; },
                n => {
                    let start = last + size;

                    lines.push((Range { start, end: start + n }, buf));

                    last = start;
                    size = n;
                }
            }

            line += 1;
        }

        Ok(Self { lines, last })
    }

    pub(crate) fn get_position_from_offset(&self, offset: usize) -> FilePosition {
        let mut line = 1usize;

        for (range, text) in &self.lines {
            if range.contains(&offset) {
                let subtext = &text[0..(offset - range.start + 1)];

                return FilePosition::new(line, subtext.chars().count() + 1);
            }

            line += 1;
        }

        FilePosition::new(self.lines.len(), offset - self.last + 1)
    }
}
