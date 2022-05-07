
import yaml
from typing import  Union, cast
from dataclasses import dataclass
from pathlib import Path

from .helpers import get_path_to_contents, get_paths_with_extension
from .types import  YmlPath

LangKey = str  # ex. 'en'
Translation = str
TranslationKey = str
SimpleTranslations = dict[TranslationKey, Translation]
TranslationValue = Union[Translation, SimpleTranslations]  # because there are nested keys
TranslationMapping = dict[TranslationKey, TranslationValue]
Parsed = dict[LangKey, dict[TranslationKey, TranslationValue]]


@dataclass(frozen=True)
class DefinedKeys:
    by_lang: dict[str, set[TranslationKey]]
    all_keys: set[TranslationKey]

    def get_missing_langs_for_key(self, key: TranslationKey) -> list[str]:
        missing = []
        for lang in self.by_lang.keys():
            if key not in self.by_lang[lang]:
                missing.append(lang)
        return missing


class TranslationsExplorer:
    def get_defined_keys(self, locale_dir: Path) -> DefinedKeys:
        paths_to_parsed = self._get_paths_to_parsed_yml(locale_dir)
        return self._get_defined_keys(paths_to_parsed)

    def _get_defined_keys(self, paths_to_parsed: dict[YmlPath, Parsed]) -> DefinedKeys:
        """Return a set with all the defined keys in locale files"""
        by_lang = {}
        all_keys = set()
        for _yml_path, parsed in paths_to_parsed.items():
            for lang, translations in parsed.items():
                lang_keys = set()
                for k, v in translations.items():
                    if isinstance(v, str):
                        lang_keys.add(k)
                    else:
                        assert isinstance(v, dict)
                        key_1 = k
                        for key_2, translation in v.items():
                            assert isinstance(translation, str), translation
                            double_key = ".".join([key_1, key_2])
                            lang_keys.add(double_key)
                by_lang[lang] = lang_keys
                all_keys.update(lang_keys)
        return DefinedKeys(by_lang, all_keys)

    def _get_paths_to_parsed_yml(self, base_dir: Path) -> dict[YmlPath, Parsed]:
        """Builds a dict mapping translations files paths to parsed content"""
        yml_paths = cast(list[YmlPath], get_paths_with_extension(base_dir, ".yml"))
        files_to_contents = get_path_to_contents(yml_paths)
        paths_to_parsed = {}
        for path, content in files_to_contents.items():
            if path.parts[-1].startswith("TODO"):
                continue
            try:
                parsed = yaml.load(content, yaml.Loader)
            except yaml.parser.ParserError as err:
                raise self._convert_parser_error(err, path) from err
            assert isinstance(parsed, dict)
            paths_to_parsed[path] = parsed
        return paths_to_parsed

    def _convert_parser_error(self, err: yaml.parser.ParserError, path: Path)-> RuntimeError:
        info_line = self._get_info_line(err)
        info = f"Error while parsing {path}, {info_line}"
        return RuntimeError(info)

    def _get_info_line(self, err: yaml.parser.ParserError):
        if err.problem_mark:
            return "line " + str(err.problem_mark.line + 1)
        return "unknown"
