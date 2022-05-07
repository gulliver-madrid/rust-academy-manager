import re
import yaml

target = r"locales\es.yml"
source = r"src\texts.rs"

ENABLED = False

def main() -> None:
    if not ENABLED:
        print("Script not enabled")
        return
    with open(source, "r") as file:
        lines = file.readlines()
    pattern = re.compile(r'pub const ([\w]+): &str = "([^"]+)";')
    translations: dict[str, str] = {}
    for line in lines:
        if m := pattern.search(line):
            groups = m.groups()
            assert len(groups) == 2, groups
            key, value = groups
            translations[key] = value
    with open(target, "r") as file:
            content = file.read()

    data = yaml.load(content, yaml.Loader)
    es_translations = data['es']
    for key, value in translations.items():
        es_translations[key.lower()] = value

    output = yaml.dump(data, allow_unicode=True)
    with open(target, "w") as file:
        file.write(output)


if __name__ == '__main__':
    main()
