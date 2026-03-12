#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'USAGE'
Usage: scripts/sync-gd32-rs.sh [--source-root PATH] [--prune]

Options:
  --source-root PATH   Path to gd32-rs repository.
                       Default: /Users/ashwin/Code/rust/gd32-rs
  --prune              Remove stale src/gd32* directories in this crate that
                       no longer exist in the source repository.
  -h, --help           Show this help.
USAGE
}

SOURCE_ROOT="/Users/ashwin/Code/rust/gd32-rs"
PRUNE=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --source-root)
            if [[ $# -lt 2 ]]; then
                echo "error: --source-root requires a value" >&2
                exit 1
            fi
            SOURCE_ROOT="$2"
            shift 2
            ;;
        --prune)
            PRUNE=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "error: unknown argument '$1'" >&2
            usage >&2
            exit 1
            ;;
    esac
done

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
DEST_ROOT="$(cd -- "$SCRIPT_DIR/.." && pwd)"
DEST_SRC="$DEST_ROOT/src"
DEST_LIB_RS="$DEST_SRC/lib.rs"
DEST_GENERIC_RS="$DEST_SRC/generic.rs"
DEST_CARGO_TOML="$DEST_ROOT/Cargo.toml"

if [[ ! -d "$SOURCE_ROOT" ]]; then
    echo "error: source root not found: $SOURCE_ROOT" >&2
    exit 1
fi

if [[ ! -f "$DEST_CARGO_TOML" ]]; then
    echo "error: destination Cargo.toml not found: $DEST_CARGO_TOML" >&2
    exit 1
fi

if [[ ! -d "$DEST_SRC" ]]; then
    mkdir -p "$DEST_SRC"
fi

source_crates=()
for dir in "$SOURCE_ROOT"/gd32*; do
    [[ -d "$dir" ]] || continue
    [[ -f "$dir/src/lib.rs" ]] || continue
    [[ -f "$dir/src/generic.rs" ]] || continue
    source_crates+=("$(basename "$dir")")
done

if [[ ${#source_crates[@]} -eq 0 ]]; then
    echo "error: no source crates found under $SOURCE_ROOT" >&2
    exit 1
fi

sorted_crates=()
while IFS= read -r item; do
    [[ -n "$item" ]] && sorted_crates+=("$item")
done < <(printf '%s\n' "${source_crates[@]}" | sort)
source_crates=("${sorted_crates[@]}")

canonical_generic="$SOURCE_ROOT/${source_crates[0]}/src/generic.rs"
for crate in "${source_crates[@]}"; do
    generic_path="$SOURCE_ROOT/$crate/src/generic.rs"
    if ! cmp -s "$canonical_generic" "$generic_path"; then
        echo "error: generic.rs mismatch between source crates ($crate differs)." >&2
        echo "       resolve the mismatch in gd32-rs before flattening." >&2
        exit 1
    fi
done
cp "$canonical_generic" "$DEST_GENERIC_RS"
# rust-analyzer in some editors can misdiagnose `FI::Ux::from(variant)` here.
perl -0pi -e 's/FI::Ux::from\(variant\)/variant.into()/g' "$DEST_GENERIC_RS"

sync_dir() {
    local src="$1"
    local dst="$2"

    if command -v rsync >/dev/null 2>&1; then
        mkdir -p "$dst"
        rsync -a --delete "$src/" "$dst/"
    else
        rm -rf "$dst"
        mkdir -p "$(dirname "$dst")"
        cp -R "$src" "$dst"
    fi
}

chips=()
for crate in "${source_crates[@]}"; do
    crate_src="$SOURCE_ROOT/$crate/src"
    for chip_dir in "$crate_src"/gd32*; do
        [[ -d "$chip_dir" ]] || continue
        chip="$(basename "$chip_dir")"
        chips+=("$chip")
        sync_dir "$chip_dir" "$DEST_SRC/$chip"
    done
done

sorted_chips=()
while IFS= read -r item; do
    [[ -n "$item" ]] && sorted_chips+=("$item")
done < <(printf '%s\n' "${chips[@]}" | sort -u)
chips=("${sorted_chips[@]}")

if [[ ${#chips[@]} -eq 0 ]]; then
    echo "error: no chip directories found under source crates" >&2
    exit 1
fi

if [[ "$PRUNE" == "true" ]]; then
    chip_index=$'\n'
    for chip in "${chips[@]}"; do
        chip_index+="$chip"$'\n'
    done

    for existing in "$DEST_SRC"/gd32*; do
        [[ -d "$existing" ]] || continue
        existing_name="$(basename "$existing")"
        if [[ "$chip_index" != *$'\n'"$existing_name"$'\n'* ]]; then
            rm -rf "$existing"
        fi
    done
fi

{
    cat <<'RS_HEADER'
#![allow(non_camel_case_types)]
#![no_std]

mod generic;
pub use self::generic::*;

RS_HEADER

    for chip in "${chips[@]}"; do
        echo "#[cfg(feature = \"$chip\")]"
        echo "pub mod $chip;"
        echo "#[cfg(feature = \"$chip\")]"
        echo "pub use self::$chip as pac;"
        echo
    done
} > "$DEST_LIB_RS"

cargo_tmp="$(mktemp)"
awk '
BEGIN { skip = 0 }
{
    if (skip == 1) {
        if ($0 ~ /^\[/) {
            skip = 0
        } else {
            next
        }
    }

    if ($0 ~ /^\[(dependencies|dependencies\.cortex-m-rt|dependencies\.critical-section|features)\]$/) {
        skip = 1
        next
    }

    print
}
' "$DEST_CARGO_TOML" > "$cargo_tmp"

{
    echo
    echo "[dependencies]"
    echo "vcell = \"0.1.3\""
    echo "cortex-m = \"0.7.7\""
    echo
    echo "[dependencies.cortex-m-rt]"
    echo "optional = true"
    echo "version = \"0.7.5\""
    echo
    echo "[dependencies.critical-section]"
    echo "optional = true"
    echo "version = \"1.1.2\""
    echo
    echo "[features]"
    echo "default = []"
    echo "rt = [\"cortex-m-rt/device\"]"

    for chip in "${chips[@]}"; do
        echo "$chip = []"
    done
} >> "$cargo_tmp"

mv "$cargo_tmp" "$DEST_CARGO_TOML"

echo "Synced ${#chips[@]} chip modules from ${#source_crates[@]} source crates."
