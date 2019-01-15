# Tests Needed
* ```discovery``` works
	1. ```#[rtest]``` works
		1. All tests marked ```#[rtest]``` OR ```#[rtest(main)]``` are in one of the test lists
			* You'll need a pre-marked list of what tests go where for this.
		2. No test is in multiple test lists. This includes the list for ignored tests.
		3. Only the ```#[rtest]```-tagged tests are in the parallel test list, AND only the ```#[rtest(main)]```-tagged tests are in the main test list
			* As with (1), you'll need a pre-marked list of what tests go where for this.
		4. The test lists don't have any functions that are NOT tagged with either ```#[rtest]``` OR ```#[rtest(main)]```.
			* Use the list from (1) and (3) for this - if the function's not on that list it's invalid.
		5. The test lists only contain functions that take no parameters AND return nothing.
			* Depending on how you handle the attribute parsing, this may be redundant.
			* An alternate test may be: ```#[rtest]``` doesn't add anything that's not a Fn<()>.
				* Whether or not it must raise an error on failure is a separate issue.
	2. ```#[rignore]``` works
		1. When a test is ignored:
			1. It is removed from any list it was originally in.
			2. It is added to an "ignored test" list.
		2. Ignored tests are not in any other test list once ignored.
		3. Only Fn<()> are ignored when tagged, all other elements have no effect.
			* Need to decide what to do when an invalid element is tagged (Error/warning message? Silent?)
* ```frontend``` works
	* ???
* ```test_run``` works
	* ???

# Scenarios That Need Manual Testing
* Macro-time errors
  * When a test is marked as both ```#[test]``` and ```#[test(main)]```, it must report an error.
  But since this is a macro-time error it'd cause compilation to fail.