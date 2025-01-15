use std::io::{Read, Write};

use crate::traits::{ReadExt, WriteExt};

impl<W: Write> WriteExt for W {}

impl<R: Read> ReadExt for R {}
