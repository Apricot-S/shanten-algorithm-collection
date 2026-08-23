# Third-Party Notices

This project uses third-party libraries and source material distributed under licenses different from the project license. Resolved Rust dependency versions are recorded in `Cargo.lock`.

## rand

- Source: <https://github.com/rust-random/rand>
- Resolved version: 0.10.2
- License: MIT OR Apache-2.0
- Copyright 2018 Developers of the Rand project
- Copyright (c) 2014 The Rust Project Developers

## xiangting

- Source: <https://github.com/Apricot-S/xiangting>
- Pinned version: 6.0.1
- License: MIT
- Copyright 2024 Apricot S.

`algorithms/lib_xiangting` is a benchmark adapter for this library.

## shanten-dp

- Source: <https://github.com/tomohxx/shanten-dp-rust>
- Pinned version: 0.3.2
- License: MIT
- Copyright 2026 tomohxx

`algorithms/lib_shanten_dp` is a benchmark adapter for this library.

## mjai-manue-go

- Source: <https://github.com/Apricot-S/mjai-manue-go/tree/v0.3.0-beta.5>
- Version: 0.3.0-beta.5
- Commit: `1ead84275f75d1b4aafe68a6c6c6867e107379cb`
- License: BSD-3-Clause
- Copyright 2024 Apricot S.

`algorithms/pruning_dfs_mjai_manue_go` is a shanten-only Rust derivative. Its lineage is Gimite's `mjai-manue`, the corrected and optimized Go implementation, and this Rust derivative. Goal enumeration and related allocation behavior were not ported.

## MIT License text

```text
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## BSD 3-Clause License text

```text
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice,
   this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
```

Apache-2.0 alternatives and the licenses of transitive crates remain available in the corresponding crate distributions resolved by `Cargo.lock`.
