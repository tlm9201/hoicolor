pub struct Converter(pub String);

impl Converter {

    // parses a hoi colored string
    // returns a similar colored ansi string
    pub fn to_ansi(&self) -> String {
        let mut formatted = String::new();
        let mut escaped = false; // have we escaped (u8 17)
        for b in self.0.as_bytes() {
            match b {
                // 17 signifies the start of a color code
                17 => {escaped = true},
                _ => {
                    // if we havent escaped we can safely push this char
                    if !escaped {
                        formatted.push( char::from(b.to_owned()));
                        continue
                    }

                    // else we need to add the color if valid, or end the escape 
                    match b {
                        87 => formatted.push_str("\x1b[37m"), // white
                        98 => formatted.push_str("\x1b[30m"), // black
                        82 => formatted.push_str("\x1b[31m"), // red
                        103 => formatted.push_str("\x1b[38;5;248m"), // gray,
                        89 => formatted.push_str("\x1b[33m"), // yellow
                        71 => formatted.push_str("\x1b[32m"), // green,
                        66 => formatted.push_str("\x1b[34m"), // blue,
                        33 => formatted.push_str("\x1b[39m"), // reset to default color
                        _ => {}, // do nothing if invalid color code (hoi ignores the character)
                    }

                    escaped = false;
                },
            }
        }

        formatted.push_str("\x1b[39m"); // reset to default color one more time

        return formatted;
    }

}
