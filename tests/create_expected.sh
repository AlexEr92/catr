#!/bin/bash

INPUT_DIR=inputs/
EXPECTED_DIR=expected/

if [[ ! -d "${EXPECTED_DIR}" ]]; then
  mkdir -p "${EXPECTED_DIR}"
fi

HELLO_WORLD_WITHOUT_NEWLINE="${INPUT_DIR}/hello_world_without_newline.txt"

BASENAME=$(basename "${HELLO_WORLD_WITHOUT_NEWLINE}")
cat "${HELLO_WORLD_WITHOUT_NEWLINE}" >"${EXPECTED_DIR}/${BASENAME}.out"
