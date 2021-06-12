# argus DB #
![Go test](https://github.com/cmj0121/argusdb/workflows/ci/badge.svg)

The argus is the multi-layer data store. It is based on the [log-structured merge-tree (LSM tree)][0]
with 128-bit [ULID][1] like primary key and support few data types.

## Interface ##
The argus provides two access interfaces: command-line and web API. The command-line interface
provides simple [REPL (read-eval-print loop)][2] with few grammar support, and the web API is based
on the [RESTFul][3].


## Data Types ##
The primary key in the argus is the ULID-like 128-bit fixed length binary. The key would be the
unique and searchable in the database, order by timestamp by-default.

It also supports many data types and can be divided into three categories: digital, symbol and object.
The digital can be encoded to binary format with few fixed-length bytes, the symbol is used to store
the binary data with maximal length. The symbol can be used to store raw binary, or null-end printable
string. The object is used to store large file and the key is the hashed value with fixed length.


| type   | size | description                                 |
|--------|------|---------------------------------------------|
| PKey   |  16  | the ULID-like primary key                   |
| INT    |   8  | 64-bit signed integer                       |
|   RAT  |  16  | rational number with two INT                |
| BLOB   | 512  | 510-bytes binary format with 2-bytes meta   |
|   STR  | 256  | 256-bytes null-end string                   |
|   TYPE |  32  | 32-bytes null-end string                    |
| OBJECT |  20  | the hashed value with fixed 20-bytes        |
|   REV  |  20  | the revision of object via two-layer object |


[0]: https://en.wikipedia.org/wiki/Log-structured_merge-tree
[1]: https://github.com/ulid/spec
[2]: https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop
[3]: https://en.wikipedia.org/wiki/Representational_state_transfer

