#!/bin/bash

parsers=(
  # "https://github.com/briot/tree-sitter-ada"
  # "https://github.com/PowerShell/tree-sitter-PowerShell"
  # "https://github.com/ikatyang/tree-sitter-toml"
  # "https://github.com/wasm-lsp/tree-sitter-wasm"
  # "https://github.com/FallenAngel97/tree-sitter-rego"
  "https://github.com/tree-sitter/tree-sitter-agda"
  "https://github.com/aheber/tree-sitter-sfapex"
  "https://github.com/tree-sitter/tree-sitter-bash"
  "https://github.com/zwpaper/tree-sitter-beancount"
  "https://github.com/amaanq/tree-sitter-capnp"
  "https://github.com/tree-sitter/tree-sitter-c"
  "https://github.com/tree-sitter/tree-sitter-cpp"
  "https://github.com/tree-sitter/tree-sitter-c-sharp"
  "https://github.com/sogaiu/tree-sitter-clojure"
  "https://github.com/uyha/tree-sitter-cmake"
  "https://github.com/stsewd/tree-sitter-comment"
  "https://github.com/theHamsta/tree-sitter-commonlisp"
  "https://github.com/tree-sitter/tree-sitter-css"
  "https://github.com/theHamsta/tree-sitter-cuda"
  "https://github.com/UserNobody14/tree-sitter-dart"
  "https://github.com/gdamore/tree-sitter-d"
  "https://github.com/camdencheek/tree-sitter-dockerfile"
  "https://github.com/rydesun/tree-sitter-dot"
  "https://github.com/elixir-lang/tree-sitter-elixir"
  "https://github.com/elm-tooling/tree-sitter-elm"
  "https://github.com/Wilfred/tree-sitter-elisp"
  "https://github.com/eno-lang/tree-sitter-eno"
  "https://github.com/tree-sitter/tree-sitter-embedded-template"
  "https://github.com/WhatsApp/tree-sitter-erlang"
  "https://github.com/travonted/tree-sitter-fennel"
  "https://github.com/ram02z/tree-sitter-fish"
  "https://github.com/siraben/tree-sitter-formula"
  "https://github.com/stadelmanma/tree-sitter-fortran"
  "https://github.com/ObserverOfTime/tree-sitter-gitattributes"
  "https://github.com/shunsambongi/tree-sitter-gitignore"
  "https://github.com/gleam-lang/tree-sitter-gleam"
  "https://github.com/theHamsta/tree-sitter-glsl"
  "https://github.com/tree-sitter/tree-sitter-go"
  "https://github.com/camdencheek/tree-sitter-go-mod"
  "https://github.com/omertuc/tree-sitter-go-work"
  "https://github.com/bkegley/tree-sitter-graphql"
  "https://github.com/slackhq/tree-sitter-hack"
  "https://github.com/tree-sitter/tree-sitter-haskell"
  "https://github.com/MichaHoffmann/tree-sitter-hcl"
  "https://github.com/tree-sitter/tree-sitter-html"
  "https://github.com/tree-sitter/tree-sitter-java"
  "https://github.com/tree-sitter/tree-sitter-javascript"
  "https://github.com/flurie/tree-sitter-jq"
  "https://github.com/Joakker/tree-sitter-json5"
  "https://github.com/tree-sitter/tree-sitter-json"
  "https://github.com/tree-sitter/tree-sitter-julia"
  "https://github.com/fwcd/tree-sitter-kotlin"
  "https://github.com/traxys/tree-sitter-lalrpop"
  "https://github.com/latex-lsp/tree-sitter-latex"
  "https://github.com/Julian/tree-sitter-lean"
  "https://github.com/benwilliamgraham/tree-sitter-llvm"
  "https://github.com/Flakebi/tree-sitter-llvm-mir"
  "https://github.com/Flakebi/tree-sitter-tablegen"
  "https://github.com/Azganoth/tree-sitter-lua"
  "https://github.com/alemuller/tree-sitter-make"
  "https://github.com/MDeiml/tree-sitter-markdown"
  "https://github.com/staysail/tree-sitter-meson"
  "https://github.com/grahambates/tree-sitter-m68k"
  "https://github.com/cstrahan/tree-sitter-nix"
  "https://github.com/jiyee/tree-sitter-objc"
  "https://github.com/tree-sitter/tree-sitter-ocaml"
  "https://github.com/milisims/tree-sitter-org"
  "https://github.com/Isopod/tree-sitter-pascal"
  "https://github.com/ganezdragon/tree-sitter-perl"
  "https://github.com/tree-sitter-perl/tree-sitter-pod"
  "https://github.com/tree-sitter/tree-sitter-php"
  "https://github.com/rolandwalker/tree-sitter-pgn"
  "https://github.com/mitchellh/tree-sitter-proto"
  "https://github.com/tree-sitter/tree-sitter-python"
  "https://github.com/yuja/tree-sitter-qmljs"
  "https://github.com/6cdh/tree-sitter-racket"
  "https://github.com/Fymyte/tree-sitter-rasi"
  "https://github.com/alemuller/tree-sitter-re2c"
  "https://github.com/tree-sitter/tree-sitter-regex"
  "https://github.com/stsewd/tree-sitter-rst"
  "https://github.com/r-lib/tree-sitter-r"
  "https://github.com/tree-sitter/tree-sitter-ruby"
  "https://github.com/tree-sitter/tree-sitter-rust"
  "https://github.com/tree-sitter/tree-sitter-scala"
  "https://github.com/6cdh/tree-sitter-scheme"
  "https://github.com/serenadeai/tree-sitter-scss"
  "https://github.com/AbstractMachinesLab/tree-sitter-sexp"
  "https://github.com/amaanq/tree-sitter-smali"
  "https://github.com/nilshelmig/tree-sitter-sourcepawn"
  "https://github.com/BonaBeavis/tree-sitter-sparql"
  "https://github.com/takegue/tree-sitter-sql-bigquery"
  "https://github.com/m-novikov/tree-sitter-sql"
  "https://github.com/dhcmrlchtdj/tree-sitter-sqlite"
  "https://github.com/metio/tree-sitter-ssh-client-config"
  "https://github.com/Himujjal/tree-sitter-svelte"
  "https://github.com/alex-pinkus/tree-sitter-swift"
  "https://github.com/SystemRDL/tree-sitter-systemrdl"
  "https://github.com/duskmoon314/tree-sitter-thrift"
  "https://github.com/nvim-treesitter/tree-sitter-query"
  "https://github.com/BonaBeavis/tree-sitter-turtle"
  "https://github.com/gbprod/tree-sitter-twig"
  "https://github.com/tree-sitter/tree-sitter-typescript"
  "https://github.com/tree-sitter/tree-sitter-verilog"
  "https://github.com/alemuller/tree-sitter-vhdl"
  "https://github.com/ikatyang/tree-sitter-vue"
  "https://github.com/mehmetoguzderin/tree-sitter-wgsl"
  "https://github.com/ikatyang/tree-sitter-yaml"
  "https://github.com/Hubro/tree-sitter-yang"
  "https://github.com/maxxnino/tree-sitter-zig"
)
mkdir -p languages
cd languages

git_clone(){
  git clone $1 > /dev/null
  echo "finish installing $1"
}

# clone parsers
for language in "${parsers[@]}"; do
  git_clone $language &
done
wait


echo "Generating rust binds"
# crete rust bindings
folders=$(ls -1 | sort)
for folder in $folders; do
  if [ -f "$folder/Cargo.toml" ]; then
    echo "bindings exist for: $folder"
  else
    echo "generating rust bindings for: $folder"
    cd "$folder"
    tree-sitter generate || echo "failed to generate bindings for: $folder"
    cd ..
  fi
done

cargo install --path .
