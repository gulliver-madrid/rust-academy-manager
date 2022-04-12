
from pathlib import Path
import re
import yaml

from validate import get_paths_with_extension #type: ignore

ENABLED = False
translations = r"locales\es.yml"
src = r"src"


def main() -> None:
    if not ENABLED:
        print("Script not enabled")
        return
    paths = get_paths_with_extension(Path(src), ".rs")
    with open(translations, "r") as file:
        content = file.read()

    data = yaml.load(content, yaml.Loader)
    pattern = re.compile(r'(texts::(\w+))')
    es_translations = data['es']
    for path in paths:
        with open(path, "r") as file:
            content = file.read()
        lines = content.split("\n")
        modified = False
        for i, line in enumerate(lines):
            if m := pattern.search(line):
                groups = m.groups()
                assert len(groups) == 2, groups
                whole, key = groups
                print(whole)
                print(key)
                low_key = key.lower()
                if low_key in es_translations:
                    new = f'&t!("{low_key}")'
                    new_line = line.replace(whole, new)
                    print(new_line)
                    lines[i] = new_line
                    modified = True
        if modified:
            with open(path, "w") as file:
                file.write("\n".join(lines))




if __name__ == '__main__':
    main()
