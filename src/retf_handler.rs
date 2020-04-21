use std::error;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

fn is_printable(msg: &str) -> bool {
    for i in 0..msg.len() {
        let cur = msg.as_bytes()[i];
        if cur < 20 || cur > 127 {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone)]
pub struct RetfHandler {
    prefix: String,
    ascii_art: String,
    width: usize,
}

#[derive(Debug, Clone)]
struct SizeError;

impl EventHandler for RetfHandler {
    fn message(&self, ctx: Context, msg: Message) {
        println!("Received message: {:?}", msg.content);

        let resp: String;

        if msg.content == self.prefix {
            resp = self.box_me("I'm the Rust daddy UwU");
        }
        else if msg.content.len() > self.prefix.len() &&
                msg.content[..self.prefix.len()] == self.prefix
        {
            if !is_printable(msg.content.as_str()) {
                resp = self.box_me(
                    "Back in my day, we had to use printable characters!"
                );
            } else {
                resp = self.box_me(&msg.content[self.prefix.len() + 1..]);
            }
        } else {
            return;
        }

        println!("sending: {:?}", resp);

        if let Err(why) = msg.channel_id.say(&ctx.http, resp.as_str()) {
            println!("Error sending message {:?}", why);
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }
}

impl RetfHandler {
    const PADDING: usize = 5;

    pub fn new(prefix: &str, art: &str, text_width: usize)
        -> Result<RetfHandler, Box<dyn error::Error>>
    {
        if text_width <= RetfHandler::PADDING {
            return Err(Box::from(SizeError));
        }

        Ok(
            RetfHandler {
                prefix: String::from(prefix),
                ascii_art: String::from(art),
                width: text_width - RetfHandler::PADDING + 1,
            }
        )
    }


    /*
    +------------+
    | my message |
    | overflow   |
    +------------+
    */
    fn box_me(&self, msg: &str) -> String {
        let n_chars = msg.len();
        let req_width = match n_chars / self.width {
            0 => n_chars,
            _ => self.width,
        };
        let n_lines = match n_chars % req_width {
            0 => n_chars / req_width,
            _ => (n_chars / req_width) + 1,
        };

        let mut boundary = String::with_capacity(req_width + RetfHandler::PADDING);
        boundary.push('+');
        for _ in 0..(boundary.capacity() - 3) {
            boundary.push('-');
        }
        boundary.push('+');

        /* text box + ascii art + ticks */
        let ret_sz = (boundary.capacity() + 1) * (3 + n_lines)
            + self.ascii_art.len() + 1
            + 7;
        let mut ret = String::with_capacity(ret_sz);

        ret.push_str("```\n");
        ret.push_str(boundary.as_str());
        ret.push('\n');

        for line in 0..n_lines-1 {
            let pos = line * req_width;
            ret.push_str("| ");
            ret.push_str(&msg[pos..pos + req_width]);
            ret.push_str(" |\n");
        }

        ret.push_str("| ");
        ret.push_str(&msg[(req_width * (n_lines - 1))..]);

        let pad_sz = match msg.len() % req_width {
            0 => 0,
            l => req_width - l,
        };

        for _ in 0..pad_sz {
            ret.push(' ');
        }
        ret.push_str(" |\n");

        ret.push_str(boundary.as_str());
        ret.push('\n');

        ret.push_str(self.ascii_art.as_str());
        ret.push_str("```");

        ret
    }
}

impl std::fmt::Display for SizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "you dumbass")
    }
}

impl error::Error for SizeError {}

mod tests {
    use super::*;

    #[test]
    fn wrap_with_pad() {
        let test_str = "123456789012345";
        let res =
"+------------+
| 1234567890 |
| 12345      |
+------------+
art";
        assert_eq!(
            RetfHandler::new("", "art", 10).unwrap().box_me(test_str),
            res
        );
    }
}
