/*!
 * Defines actual functionality of
 * the #![test_harness_root] attribute.
 */
use proc_macro::TokenStream;
use quote::ToTokens;
use std::File;
use syn;

use failure::Error;

use super::discover_tree::DiscoverTree;
use crate::discovery::rls_common::*;

const PARSE_FAILED_TEXT: &'static str = "Failed to parse crate root; no tests will be run";
const MODIFY_FAILED_TEXT: &'static str = "Failed to parse modules in file; no tests will be run";

/**
 * TODO
 */
pub fn do_test_harness_root(_attr: TokenStream, item: TokenStream) -> TokenStream {
	//Convert the item token stream to a syn::File...
	//also make it mutable.
	let item_raw = item.clone();
	let parse_result = syn::parse::<syn::File>(item_raw);

	match parse_result {
		Ok(mut file_tokens) => {
			//Iterate over the mutable file with syn::VisitMut.
			let modify_result = DiscoverTree::new().discover(&mut file_tokens);
			match modify_result {
				Ok(_) => {
					//If we didn't panic
					//through the iteration,
					//we should be good;
					//return the modified file as a proc_macro::TokenStream
					return file_tokens.into_token_stream().into();
				}
				_ => {
					//Otherwise we messed up again,
					//report failure to RLS
					post_rls_error(&MODIFY_FAILED_TEXT);
					//and return the unmodified tokens
					return item;
				}
			}
		}
		_ => {
			//We couldn't parse the initial file;
			//return an RLS error
			post_rls_error(&PARSE_FAILED_TEXT);
			//and return the unmodified tokens
			return item;
		}
	}

	unreachable!();
}

pub fn discover_file(file_path: &str) -> Result<TokenStream, Error> {
	let mut file = File::open(file_path)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;

	let mut file_tokens = syn::parse_file(&content)?;

	let modify_result = DiscoverTree::new().discover(&mut file_tokens)?;

	Ok(modify_result.into_token_stream().into())
}
