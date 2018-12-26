# Required Display Elements
* Display should be similar to the existing ```cargo test```:
  * Log that you're doing pre-run setup
  * Log that you're running tests
    * Log that an individual test started
    * On the same line, log that the test passed or failed
      * As tests can run in parallel, it may be simpler to log on a separate line that the test passed/failed.
      * This needs colored text: green for passes, red for failures. Yellow for skips?
  * Report failure details:
    * For each failure:
      * The test that failed
      * The reason why the test failed (assert, unexpected panic)
      * File:line to the failure
      * Backtrace to the failure
        * Optional - only backtrace if the assert/panic area isn't in the failing test?
  * Report overall results:
    * Number of tests passed
    * Number of tests failed
    * Number of tests skipped
    * List of tests failed, if any

# Things That Would Be Nice
* Localized text!
  * This will be much easier if you stick to only using/formatting string constants.
  * Format strings need to have ordered parameters, since the text's grammatical structure
  may not be the same in different languages.
