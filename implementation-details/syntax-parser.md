# Syntax Parser

The syntax parser is modelled as follows.

The exact match that passes the Backus-Naur-Form made is stored at each level.

```
<program> ::= <optws> <usings> <optws> <definitions> <optws>;
<usings> ::= <using> | <usings> <optws> <using>;
<using> ::= 'using' <ws> <definepath> ';';
<definepath> ::= <ident> | <ident> '::' <definepath>;
<definitions> ::= <definition> | <definitions> <optws> <definition>;
<definition> ::= 'define' <ws> <ident> <define_block>;
<define_block> ::= <optws> '{' <define_items> '}' <optws>;
<define_items> ::= <optws> <define_item> | <optws> <define_item> <optws> <define_items>;
<define_item> ::= <constant> | <mapping>;
<constant> ::= <optws> <optpub> 'constant' <ws> <typed_ident> <optws> '=' <optws> <ival>;
<mapping> ::= <optws> <optpub> 'mapping' <ws> <ident> <optmapping_types> <mapping_params> <optws> <mapping_block>;
<optmapping_types> ::= '::' <mapping_types> | '';
<mapping_types> ::= '<' <identlist> '>';
<identlist> ::= <ident> | <ident> ',' <identlist>;
<mapping_params> ::= '(' <typed_identlist> ')';
<typed_identlist> ::= <typed_ident> | <typed_ident> ',' <typed_identlist>;

<typed_ident> ::= <ident> <optws> ':' <optws> <ident>
<ival> ::= <ident> | <int> | <str>
<optpub> ::= 'public' <ws> | '';
<int> ::= <optsgn> <nums>;
<nums> ::= <num> | <nums> <num>;
<optsgn> ::= '-' | '+' | '';
<ident> ::= 'A' | 'B' | 'C' | ... | 'Y' | 'Z' | 'a' | 'b' | 'c' | ... | 'y' | 'z' |  <ident> <identnum>;
<num> ::= '0' | '1' | '2' | ... | '8' | '9';
<identnum> ::= <ident> | <int> | <identnum> <identnum>;

<optws> ::= <ws> | '';
<ws> ::= ' ', '\t', '\n', '\r';
```