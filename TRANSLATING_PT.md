# Diretrizes de tradução do Rust by Example

Por favor, consulte o arquivo [CONTRIBUTING.md](https://github.com/rust-lang/rust-by-example/blob/master/CONTRIBUTING.md?utm_source=chatgpt.com) para as diretrizes gerais de contribuição.
Este arquivo descreve o fluxo de trabalho de tradução.

## Fluxo de trabalho de tradução

### Preparação

O RBE utiliza o [mdbook-i18n-helpers](https://github.com/google/mdbook-i18n-helpers?utm_source=chatgpt.com) como framework de tradução.
As seguintes ferramentas são necessárias:

* Utilitários GNU gettext (`msgmerge` e `msgcat`)
* mdbook-i18n-helpers (`cargo install mdbook-i18n-helpers`)

---

## Criando e Atualizando Traduções

Consulte o arquivo [USAGE do mdbook-i18n-helpers](https://github.com/google/mdbook-i18n-helpers/blob/main/i18n-helpers/USAGE.md?utm_source=chatgpt.com) para obter instruções detalhadas de uso do mdbook-i18n-helpers.
Abaixo está um resumo dos comandos principais:

### Gerando um template de mensagens

O template gerado `po/messages.pot` é necessário para criar ou atualizar traduções.

```bash
MDBOOK_OUTPUT='{"xgettext": {"pot-file": "messages.pot"}}' \
  mdbook build -d po
```

### Criando um novo recurso de tradução

`xx` é o código de idioma [ISO 639](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).

```bash
msginit -i po/messages.pot -l xx -o po/xx.po
```

### Atualizando um recurso de tradução existente

```bash
msgmerge --update po/xx.po po/messages.pot
```

### Acompanhando o progresso da tradução

```bash
msgfmt --statistics po/xx.po
```

---

## Editando recursos de tradução

Após gerar um recurso de tradução `po/xx.po`, você pode escrever as mensagens traduzidas na entrada `msgstr` do arquivo `po/xx.po`.

Para compilar um livro traduzido, utilize os seguintes comandos:

```bash
MDBOOK_BOOK__LANGUAGE=xx mdbook build
MDBOOK_BOOK__LANGUAGE=xx mdbook serve
```

---

## Adicionando uma entrada de idioma

Por favor, adicione uma entrada de idioma em:

* `.github/workflows/rbe.yml`
* `theme/head.hbs`
* `src/bootstrap/src/core/build_steps/doc.rs`

no repositório [rust-lang/rust](https://github.com/rust-lang/rust?utm_source=chatgpt.com), conforme abaixo:

### `rbe.yml`

```yml
env:
  # Atualize o seletor de idiomas em index.hbs para incluir novos idiomas.
  LANGUAGES: xx yy zz
```

### `head.hbs`

```html
<ul id="language-list" class="theme-popup" aria-label="Languages" role="menu">
  <li role="none"><button role="menuitem" class="theme">
      <a id="en">English</a>
  </button></li>
  <li role="none"><button role="menuitem" class="theme">
      <a id="xx">Idioma XX</a>
  </button></li>
  <li role="none"><button role="menuitem" class="theme">
      <a id="yy">Idioma YY</a>
  </button></li>
  <li role="none"><button role="menuitem" class="theme">
      <a id="zz">Idioma ZZ</a>
  </button></li>
</ul>
```

### `src/bootstrap/src/core/build_steps/doc.rs`

```rust
RustByExample, "src/doc/rust-by-example", "rust-by-example", &["xx", "yy", "zz"], submodule;
```
