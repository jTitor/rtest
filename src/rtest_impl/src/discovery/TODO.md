* As mentioned in ```test_run/TODO.md```, need to generate two lists of tests:
  * Normal tests that can be run on parallel threads
  * Main tests that must run on the main thread
* Need some way to get the tests:
  * Looks like you can do this with custom attributes? Crate ```syn``` handles this, you make attributes by tagging a fn with ```#[proc_macro_attribute]```, while the function follows the format ```fn name(args: TokenStream, input: TokenStream) -> TokenStream```, where ```args``` are anything passed in parenthesis to the attribute (i.e., ```#[attribute(args_here)]```), while ```input``` is whatever you tagged with the attribute, parsed into a TokenStream.
  * Your attribute gets run at compile time, so it seems like this is the perfect place to put discovery code - the attribute checks that it's on a function, and if so adds the function to a list. If the attribute args contain something like ```is_main```, then the test function's added to an alternate list for main-only tests.
  * And since this is happening on a TokenStream, the code's already been parsed, so hopefully we can group by module.
* How do you handle ```#[ignore]```?
  * Probably need a separate attribute just like with ```#[test]```, as there's no way to get inside the attribute handler for either existing attribute.