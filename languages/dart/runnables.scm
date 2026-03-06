; Flutter main
(
    (
        (import_or_export
            (library_import
                (import_specification
                    ("import"
                        (configurable_uri
                            (uri
                                (string_literal) @_import
                                (#match? @_import "package:flutter/(material|widgets|cupertino).dart")
                                (#not-match? @_import "package:flutter_test/flutter_test.dart")
                                (#not-match? @_import "package:test/test.dart")
        ))))))
        (
            (function_signature
                name: (_) @run
            )
            (#eq? @run "main")
        )
        (#set! tag flutter-main)
    )
)

; Flutter test main
(
    (
        (import_or_export
            (library_import
                (import_specification
                    ("import"
                        (configurable_uri
                            (uri
                                (string_literal) @_import
                                (#match? @_import "package:flutter_test/flutter_test.dart")
        ))))))
        (
            (function_signature
                name: (_) @run
            )
            (#eq? @run "main")
        )
        (#set! tag flutter-test-main)
    )
)

; Flutter widget test (testWidgets)
(
    (
        (import_or_export
            (library_import
                (import_specification
                    ("import"
                        (configurable_uri
                            (uri
                                (string_literal) @_import
                                (#match? @_import "package:flutter_test/flutter_test.dart")
        ))))))
        (
            (function_body
                (block
                    (expression_statement
                        (
                            (identifier) @run (#eq? @run "testWidgets")
        )))))
        (#set! tag flutter-widget-test)
    )
)

; Flutter integration test
(
    (
        (import_or_export
            (library_import
                (import_specification
                    ("import"
                        (configurable_uri
                            (uri
                                (string_literal) @_import
                                (#match? @_import "package:integration_test/integration_test.dart")
        ))))))
        (
            (function_signature
                name: (_) @run
            )
            (#eq? @run "main")
        )
        (#set! tag flutter-integration-test)
    )
)

; Dart test file
(
    (
        (import_or_export
            (library_import
                (import_specification
                    ("import"
                        (configurable_uri
                            (uri
                                (string_literal) @_import
                                (#match? @_import "package:test/test.dart")
        ))))))
        (
            (function_signature
                name: (_) @run
            )
            (#eq? @run "main")
        )
        (#set! tag dart-test-file)
    )
)

; Dart test group
(
    (
        (import_or_export
            (library_import
                (import_specification
                    ("import"
                        (configurable_uri
                            (uri
                                (string_literal) @_import
                                (#match? @_import "package:test/test.dart")
        ))))))
        (
            (function_body
                (block
                    (expression_statement
                        (
                            (identifier) @run (#eq? @run "group")
        )))))
        (#set! tag dart-test-group)
    )
)

; Dart test single
(
    (
        (import_or_export
            (library_import
                (import_specification
                    ("import"
                        (configurable_uri
                            (uri
                                (string_literal) @_import
                                (#match? @_import "package:test/test.dart")
        ))))))
        (
            (function_body
                (block
                    (expression_statement
                        (selector
                            (argument_part
                                (arguments
                                    (argument
                                        (function_expression
                                            (function_expression_body
                                                (block
                                                    (expression_statement
                                                        (
                                                            (identifier) @run (#eq? @run "test")
        )))))))))))))
        (#set! tag dart-test-single)
    )
)

; Dart test single (top-level)
(
    (
        (import_or_export
            (library_import
                (import_specification
                    ("import"
                        (configurable_uri
                            (uri
                                (string_literal) @_import
                                (#match? @_import "package:test/test.dart")
        ))))))
        (
            (function_body
                (block
                    (expression_statement
                        (
                            (identifier) @run (#eq? @run "test")
        )))))
        (#set! tag dart-test-single)
    )
)
