export-env {
    if $env.FMOD_PATH? == null {
        $env.FMOD_PATH = (registry query --hkcu 'Software\FMOD Studio API Windows' | get 0.value)
    }
}

const FMOD_RS_SRC = path self ./crates/fmod-rs/src

def 'html select' [
    query: string,
] {
    let html = $in
    plugin add nu_plugin_query.exe
    $html | query web --as-html --query $query
}

def clean-links [] {
    $in
    | str replace -ar 'FMOD_DEBUG_([[:word:]]+)' {|it| $'`DebugFlags::($it | str pascal-case)`'}
    | str replace -ar 'FMOD_ERR_([[:word:]]+)' {|it| $'`Error::($it | str pascal-case)`'}
    | str replace -ar 'Debug_([[:word:]]+)' {|it| $'`debug::($it | str snake-case)`'}
    | str replace -ar 'Memory_([[:word:]]+)' {|it| $'`memory::($it | str snake-case)`'}
    | str replace -ar '\[`(.+?)`\]\(.*?\)\{.apilink\}' {|it| $'[`($it)`]'}
}

export def get-doc [
    page: string,
    id: string,
] {
    open ([$env.FMOD_PATH 'doc' 'FMOD API User Manual' $'($page).html'] | path join)
    | html select ([
        $"h2#($id) ~ :is\(p, ul)"          # after selected header
        $":not\(h2#($id) ~ h2 ~ *)"        # but before next header
        ':not(:has(> strong:first-child))' # and not the one with `**See Also**`
    ] | str join)
    | str join
    | pandoc --from html --to markdown
    | clean-links
}

export def docgen [] {
    const REGEX = '(?x)fmod_doc!\(
        \s* "(?<page>[[:word:]\-]+)"
        \s* ,
        \s* "(?<id>[[:word:]\-]+)"
        \s* ,?
        \s* \)'

    ls ...(glob -D 'crates/fmod-rs/src/**/*.rs')
    | get name
    | par-each {|src|
        let dir = $src | path dirname
        open $src
        | find --regex $REGEX --no-highlight
        | each {|m| $m | parse --regex $REGEX}
        | flatten
        | each {|it| get-doc $it.page $it.id | save -f $'($dir)/($it.id).md'}
    }

    cargo docs-rs --package fmod-rs
}
