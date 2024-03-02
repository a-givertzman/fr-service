#!/bin/bash
llvm-cov report \
    $( \
      for file in \
        $( \
          RUSTFLAGS="-C instrument-coverage" \
            cargo test --tests --no-run --message-format=json \
              | jq -r "select(.profile.test == true) | .filenames[]" \
              | grep -v dSYM - \
        ); \
      do \
        printf "%s %s " -object $file; \
      done \
    ) \
  --instr-profile=json5format.profdata --summary-only # and/or other options
llvm-profdata merge -sparse default_8818858786087223891_0_40425.profraw -o 01.profdata
llvm-profdata merge -sparse default_14418867304261787254_0_40423.profraw -o 01.profdata
llvm-profdata merge -sparse default_17286758045204420752_0_40425.profraw -o 01.profdata
llvm-cov show ./foo -instr-profile=01.profdata