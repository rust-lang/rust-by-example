(function addThemePicker() {
    const rightButtonsElement = document.querySelector('.right-buttons');
    rightButtonsElement.insertAdjacentHTML("afterbegin", `
        <button id="language-toggle" class="icon-button" type="button"
                title="Change language" aria-label="Change language"
                aria-haspopup="true" aria-expanded="false"
                aria-controls="language-list">
            <i class="fa fa-globe"></i>
        </button>
        <ul id="language-list" class="theme-popup" aria-label="Languages" role="menu">
          <li role="none"><button role="menuitem" class="theme">
              <a id="en">English</a>
          </button></li>
          <li role="none"><button role="menuitem" class="theme">
              <a id="ja">日本語</a>
          </button></li>
          <li role="none"><button role="menuitem" class="theme">
              <a id="zh">中文</a>
          </button></li>
          <li role="none"><button role="menuitem" class="theme">
              <a id="es">Español</a>
          </button></li>
        </ul>
    `);

    const language = document.documentElement.getAttribute("lang");
    let langToggle = document.getElementById("language-toggle");
    let langList = document.getElementById("language-list");
    langToggle.addEventListener("click", (event) => {
        langList.style.display =
            langList.style.display == "block" ? "none" : "block";
    });
    let selectedLang = document.getElementById(language);
    if (selectedLang) {
        selectedLang.parentNode.classList.add("theme-selected");
    }

    // The path to the root, taking the current language into account.
    let full_path_to_root =
        language == "en" ? `${mdbookPathToRoot}` : `${mdbookPathToRoot}../`;
    // The page path (mdbook only gives us access to the path to the Markdown file).
    let path = mdbookPath.replace(/\.md$/, ".html");
    for (let lang of langList.querySelectorAll("a")) {
        if (lang.id == "en") {
            lang.href = `${full_path_to_root}${path}`;
        } else {
            lang.href = `${full_path_to_root}${lang.id}/${path}`;
        }
    }
})();
