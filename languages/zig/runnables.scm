; main function
(
  (function_declaration
    "pub"? @_pub
    [
      "extern"
      "export"
      "inline"
      "noinline"
    ]? @_modifier
    "fn" @_fn
    name: (identifier) @run @main_fn) @function
  (#eq? @run "main")
  (#set! "tag" "zig-run")
)

; test functions
(
  (test_declaration
    "test" @_test
    [
      (string) @run @test_name
      (identifier) @run @test_name
    ]) @function
  (#set! "tag" "zig-test")
)
