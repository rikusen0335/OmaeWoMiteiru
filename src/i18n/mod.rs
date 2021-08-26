use gettextrs::{bindtextdomain, bind_textdomain_codeset, textdomain, setlocale, LocaleCategory};

pub fn i18n_init() {
    // This is either:
    //  - /usr[/local]/bin/exe
    //  - <projectDir>/target/<type>/exe
    let mut i18n_path = std::env::current_exe().unwrap();
    //remove file name
    i18n_path.pop();

    // If we're in /usr[/local]/bin, change to /usr[/local]/share/OmaeWoMiteiru/i18n
    i18n_path.pop();
    i18n_path.push("share");
    i18n_path.push("OmaeWoMiteiru");
    i18n_path.push("i18n");
    if !i18n_path.is_dir() {
        //try ../../, in case we're in <projectDir>/target/<type>/
        i18n_path.pop();
        i18n_path.pop();
        i18n_path.pop();
        i18n_path.pop();
        i18n_path.push("i18n");
    }
    if i18n_path.is_dir() {
        println!("i18n dir: {}", i18n_path.display());
        bindtextdomain("OmaeWoMiteiru", i18n_path.to_str().unwrap()).expect("bind text domain");
        bind_textdomain_codeset("OmaeWoMiteiru", "UTF-8").expect("set UTF-8 codeset");
        textdomain("OmaeWoMiteiru").expect("set text domain");
        setlocale(LocaleCategory::LcAll, "");
    }
}
