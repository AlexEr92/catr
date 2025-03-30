#!/bin/bash

INPUT_DIR=inputs/
EXPECTED_DIR=expected/

if [[ ! -d "${EXPECTED_DIR}" ]]; then
  mkdir -p "${EXPECTED_DIR}"
fi

HELLO_WORLD="${INPUT_DIR}/hello_world.txt"

BASENAME=$(basename "${HELLO_WORLD}")
cat "${HELLO_WORLD}" >"${EXPECTED_DIR}/${BASENAME}.out"
