# gsjf

get and set JSON quickly

## valid json format definition:
```
json: object | array

object: '{' members? '}'
    members: pair (',' pair)*
    pair:    string ':' value

array: '[' elements? ']'
    elements: value (',' value)*

value: string
     | number
     | object
     | array
     | 'true'
     | 'false'
     | 'null'

string: '"' char* '"'
    char: [^"\\\x00-\x1F]
        | '\' escape
    escape: ["\\/bfnrt]
          | u [0-9A-Fa-f]{4}

number: '-'? (0 | [1-9][0-9]*) ('.' [0-9]+)? ([Ee] [+-]? [0-9]+)?
```