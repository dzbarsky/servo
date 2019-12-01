extern crate cssparser;
extern crate parking_lot;
extern crate servo_arc;
extern crate style;
extern crate style_traits;
extern crate servo_url;
extern crate embedder_traits;

use std::io::{self, Read};
use style::context::QuirksMode;
use style::error_reporting::{ContextualParseError, ParseErrorReporter};
use style::parser::ParserContext;
use style::shared_lock::{SharedRwLock};
use style::stylesheets::rule_parser::{State, TopLevelRuleParser};
use style::stylesheets::{Namespaces, Origin};
use cssparser::{Parser, ParserInput, SourceLocation, RuleListParser};
use parking_lot::RwLock;
use style_traits::ParsingMode;
use servo_url::ServoUrl;

struct ErrorCollector {}
impl ParseErrorReporter for ErrorCollector {
    fn report_error(&self, _url: &ServoUrl, _location: SourceLocation, error: ContextualParseError) {
        println!("{}", error.to_string());
    }
}

fn main() {
       embedder_traits::resources::set_for_tests();
       let mut css = String::new();
       io::stdin().lock().read_to_string(&mut css);

       let mut input = ParserInput::new(&css);
       let mut input = Parser::new(&mut input);
       let namespaces = RwLock::new(Namespaces::default());
       let shared_lock = &SharedRwLock::new();
       let url_data = ServoUrl::parse("localhost:80").unwrap();

       let context = ParserContext::new(
            Origin::Author,
            &url_data,
            None,
            ParsingMode::DEFAULT,
            QuirksMode::NoQuirks,
            Some(&ErrorCollector{}),
            None,
        );

        let rule_parser = TopLevelRuleParser {
            shared_lock,
            loader: None,
            context,
            state: State::Start,
            dom_error: None,
            insert_rule_context: None,
            namespaces: &mut *namespaces.write(),
        };

        {
            let mut iter = RuleListParser::new_for_stylesheet(&mut input, rule_parser);

            while let Some(result) = iter.next() {
                match result {
                    Ok(_) => (),
                    Err((error, slice)) => {
			println!("{:?}, {:?}", error, slice)
                    },
                }
            }
        }
}
