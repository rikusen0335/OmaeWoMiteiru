# How to update translations

Have `xtr` installed: `cargo install xtr`

When strings in main.rs change:
 - run `xtr src/main.rs -o i18n/OmaeWoMiteiru.pot`
 - Open i18n/ja/OmaeWoMiteiru.po in Poedit (https://poedit.net/)
   - Select Translation -> Update from .POT file
   - Select the generated `i18n/OmaeWoMiteiru.pot`
   - Change or update translation
   - Save & Exit