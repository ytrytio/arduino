((identifier) @function.builtin
  (#any-of? @function.builtin
    "digitalRead" "digitalWrite" "pinMode"
    "analogRead" "analogReference" "analogWrite"
    "analogReadResolution" "analogWriteResolution"
    "noTone" "pulseIn" "pulseInLong" "shiftIn" "shiftOut" "tone"
    "delay" "delayMicroseconds" "micros" "millis"
    "abs" "constrain" "map" "max" "min" "pow" "sq" "sqrt"
    "cos" "sin" "tan"
    "isAlpha" "isAlphaNumeric" "isAscii" "isControl" "isDigit" "isGraph" "isHexadecimalDigit"
    "isLowerCase" "isPrintable" "isPunct" "isSpace" "isUpperCase" "isWhitespace"
    "random" "randomSeed"
    "bit" "bitClear" "bitRead" "bitSet" "bitWrite" "highByte" "lowByte"
    "attachInterrupt" "detachInterrupt"
    "interrupts" "noInterrupts"))

((identifier) @type.builtin
  (#any-of? @type.builtin "Serial" "SPI" "Stream" "Wire" "Keyboard" "Mouse" "String"))

(preproc_include) @keyword.preproc
(preproc_def) @keyword.control
(preproc_call) @function.call

(comment) @comment
(string_literal) @string
(system_lib_string) @string
(number_literal) @number

(false) @constans.boolean
(true) @constans.boolean

((identifier) @constant.builtin
 (#match? @constant.builtin "^(HIGH|LOW|INPUT|OUTPUT|INPUT_PULLUP|INPUT_PULLDOWN|LED_BUILTIN|PI|HALF_PI|TWO_PI|DEG_TO_RAD|RAD_TO_DEG|EULER|HEX|DEC|OCT|BIN|LSBFIRST|MSBFIRST|CHANGE|FALLING|RISING|WIFI_STA|WIFI_AP)$"))

[
  "const" "static" "volatile" "extern" "virtual"
] @keyword

(function_declarator
  declarator: (identifier) @function)

(function_definition
  type: (primitive_type) @keyword.type)

(field_expression
  field: (field_identifier) @function.call)

(declaration
    type: (type_identifier) @keyword)

[
   (primitive_type) 
   (sized_type_specifier)
   (struct_specifier)
] @keyword.type

(parameter_declaration
    type: (type_identifier) @variable)

(call_expression
    function: (identifier) @function.call)

[
  "if" "else" "while" "for" "return" "break" 
  "continue" "switch" "case" "default" "do"
] @keyword.control

[
  "=" "==" "!=" ">" "<" ">=" "<="
  "+" "-" "*" "/" "&&" "||" "!" "&" "|"
] @operator

["(" ")" "{" "}" "[" "]"] @punctuation.bracket
[";" "," "."] @punctuation.delimiter
