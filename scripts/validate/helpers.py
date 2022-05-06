from pathlib import Path
from typing import Sequence, TypeVar

GenericPath = TypeVar('GenericPath', bound=Path)

def get_paths_with_extension(base_dir: Path, ext: str) -> list[Path]:
    assert ext.startswith("."), ext
    all_paths = base_dir.glob('**/*' + ext)
    return [p for p in all_paths if p.is_file()]


def get_path_to_contents(paths: Sequence[GenericPath]) -> dict[GenericPath, str]:
    files_to_contents = {}
    for path in paths:
        with open(path, "r") as file:
            files_to_contents[path] = file.read()
    return files_to_contents
