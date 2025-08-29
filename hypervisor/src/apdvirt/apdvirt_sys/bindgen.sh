#!/usr/bin/env bash
# Copyright 2023 The ChromiumOS Authors
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

# Regenerate geniezone_sys bindgen bindings.

set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/../../../.."

source tools/impl/bindgen-common.sh

AVTVM_SYS_BASE="hypervisor/src/apdvirt/apdvirt_sys"
AVTVM_BINDINGS="${AVTVM_SYS_BASE}/aarch64/bindings.rs"

AVTVM_HEADER_FILE="${BINDGEN_LINUX_ARM64_HEADERS}/include/linux/apdvirt_interface.h"

bindgen_generate \
    --blocklist-item='__kernel.*' \
    --blocklist-item='__BITS_PER_LONG' \
    --blocklist-item='__FD_SETSIZE' \
    --blocklist-item='_?IOC.*' \
    ${AVTVM_HEADER_FILE} \
    -- \
    -isystem "${BINDGEN_LINUX_ARM64_HEADERS}/include" \
    | replace_linux_int_types \
    > ${AVTVM_BINDINGS}
