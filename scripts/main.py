from collections import defaultdict
import re

from .vendor.validate.api import extract_regex_matches, PathsToLines, KeysToPaths, MatchedStringsToPaths, run

from .patterns import inside_menu_option_pattern, macro_menu_option_pattern



class Extractor:
    def execute(self, pattern: re.Pattern[str], paths_to_lines: PathsToLines) -> KeysToPaths:
        return self._extract_used_keys_macro_menu_option(pattern, paths_to_lines)

    def _extract_used_keys_macro_menu_option(self, pattern: re.Pattern[str], paths_to_lines: PathsToLines) -> KeysToPaths:
        # Same that general one but searching the content of create_menu_options! macro
        # And after that, search the translation keys inside these contents
        contents: MatchedStringsToPaths = extract_regex_matches(pattern, paths_to_lines)

        keys_used = KeysToPaths(defaultdict(list))
        # Note: different contents could have an identical key
        for content, new_src_paths in contents.items():
            matches = inside_menu_option_pattern.findall(content)
            for key in matches:
                src_paths = keys_used[key]
                src_paths.extend([path for path in new_src_paths if path not in src_paths])
                assert keys_used[key] == src_paths, f'These two lists should be still the same object'

        return keys_used


def main() -> None:

    run('src', 'locales', [dict(name='macro_menu_option_pattern', regex=macro_menu_option_pattern, extractor=Extractor())])


if __name__ == "__main__":
    main()
