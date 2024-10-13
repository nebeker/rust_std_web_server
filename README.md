# rust_std_web_server
Rust exercise: a basic web server using only the standard library.

## An itneresting bug

Commit 31c5fd fixes an interesting bug: a typo in the HTTP header (missing /) caused the following issues in the following browsers:

* Firefox - loads the page, but displays the HTTP header and the content as plain text, instead of parsing the HTML;
* Safari - refuses to load the page for security reasons (out of date protocol);
* Chrome - renders HTML as expected.

The reason is that by not correctly including the HTTP version, standards will have it default to HTTP 0.9. Firefox accepts this and render the response starting at "1.1" and Safari refuses to accept HTTP 0.9. Chrome assumes there's a missing slash and works from there - very forgiving, while ignoring web standards. But that's a whole other topic.

## "Fixing" a bug with gen AI

Let's consider that instead of testing in multiple browsers, doing research, and learning a little more about web standards, we decided to get "AI" to help. Let's throw a GPU at the problem and ask llama3.2:1b for help:

```
my Rust std only web server builds and runs fine, but Firefox loads a weird pa
... ge. Here's the code that sends the HTTP response: fn read_file(filename: &str)
(...)
```

llama fails to spot the typo, ignores the std-only requirement and suggests a number possible issues that are unrelated and would lead to research in unproductive dirrections. It also offers code that may work, by doing away with the HTTP header all together, defeating the purpose of the exercise.

The full transcritp is on [llamma_test.txt](llamma_test.txt).

There are obvious limitations with this experiment, starting with the model selection, but those may be acceptable in the context of a beginner exercise.