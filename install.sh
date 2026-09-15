#!/bin/bash
set -e

echo "=== بناء لغة luu ==="
cargo build --release

mkdir -p ~/.local/bin

cp target/release/luu_compiler ~/.local/bin/luu

echo "=== تم تثبيت لغة luu بنجاح! ==="
echo "تأكد من إضافة ~/.local/bin إلى مسار النظام (PATH) لديك."
