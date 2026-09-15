#!/usr/bin/env bash
set -e

echo "=== تثبيت لغة Luu المترجمة والتكامل مع النظام ==="

echo "--> جاري بناء وتثبيت المترجم عالمياً..."
cargo install --path .

if [ -d "src/vscode-luu" ]; then
    echo "--> حزم وتثبيت امتداد VS Code..."
    cd src/vscode-luu
    npx @vscode/vsce package --no-dependencies --allow-missing-repository
    code --install-extension luu-language-0.0.1.vsix --force
    cd ../..
fi

echo "=== اكتمل التثبيت بنجاح! ==="
echo "يمكنك الآن تشغيل أي ملف عبر الأمر: luu اسم_الملف.luu"
