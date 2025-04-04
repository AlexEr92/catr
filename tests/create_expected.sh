#!/bin/bash

INPUT_DIR=inputs/
EXPECTED_DIR=expected/

if [[ ! -d "${EXPECTED_DIR}" ]]; then
  mkdir -p "${EXPECTED_DIR}"
fi

HELLO_WORLD_WITH_NEWLINE="${INPUT_DIR}/hello_world_with_newline.txt"
HELLO_WORLD_WITHOUT_NEWLINE="${INPUT_DIR}/hello_world_without_newline.txt"
COUNTING="${INPUT_DIR}/counting.txt"
INVICTUS="${INPUT_DIR}/invictus.txt"
EMPTY="${INPUT_DIR}/empty.txt"

declare -a test_files
test_files=(
  "${HELLO_WORLD_WITH_NEWLINE}"
  "${HELLO_WORLD_WITHOUT_NEWLINE}"
  "${COUNTING}"
  "${INVICTUS}"
  "${EMPTY}"
)

for file in "${test_files[@]}"; do
  file_basename=$(basename "${file}")
  cat "${file}" >"${EXPECTED_DIR}/${file_basename}.out"
  cat -n "${file}" >"${EXPECTED_DIR}/${file_basename}.n.out"
  cat -b "${file}" >"${EXPECTED_DIR}/${file_basename}.b.out"
done
