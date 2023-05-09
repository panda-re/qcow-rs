use std::io::Write;

use super::*;

pub struct FileCfg {
    pub no_page: bool,
    pub force_fancy: bool,
    pub language: Option<String>,
}

pub fn output_file<R: ReadAt>(superblock: &ext4::SuperBlock<R>, path: &str, cfg: FileCfg) {
    let target_inode_number = superblock.resolve_path(path).unwrap().inode;
    let inode = superblock.load_inode(target_inode_number).unwrap();
    let mut file_reader = superblock.open(&inode).unwrap();

    let mut buf = Vec::new();
    file_reader.read_to_end(&mut buf).unwrap();

    // get set of syntax highlighting options from bat's default assets
    let assets = bat::assets::HighlightingAssets::from_binary();
    let mut syntax_set = SyntaxSet::new().into_builder();
    for syntax in assets.syntaxes() {
        syntax_set.add(SyntaxDefinition {
            name: syntax.name.clone(),
            file_extensions: syntax.file_extensions.clone(),
            scope: syntax.scope,
            first_line_match: syntax.first_line_match.clone(),
            hidden: syntax.hidden,
            variables: syntax.variables.clone(),
            contexts: Default::default(),
        });
    }
    syntax_set.add_plain_text_syntax();
    let syntax_set = syntax_set.build();

    // get syntax for file
    let syntax = syntax_set
        .find_syntax_by_path(path)
        .or_else(|| syntax_set.find_syntax_by_first_line(&String::from_utf8_lossy(&buf)))
        .or_else(|| {
            if path.ends_with("vimrc") {
                Some(syntax_set.find_syntax_by_name("VimL").unwrap())
            } else {
                None
            }
        })
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text())
        .name
        .clone();

    let is_binary = std::str::from_utf8(&buf).is_err();
    let if_is_tty = atty::is(atty::Stream::Stdout) || cfg.force_fancy;

    if if_is_tty && !is_binary {
        bat::PrettyPrinter::new()
            .input(bat::Input::from_bytes(&buf).title(path))
            .language(cfg.language.as_ref().unwrap_or(&syntax))
            .colored_output(true)
            .line_numbers(true)
            .grid(true)
            .header(true)
            .paging_mode(if cfg.no_page {
                bat::PagingMode::Never
            } else {
                bat::PagingMode::QuitIfOneScreen
            })
            .print()
            .unwrap();
    } else {
        std::io::stdout().lock().write_all(&buf).unwrap();
    }
}
