from typing import Mapping, NewType
from pathlib import Path


YmlPath = NewType('YmlPath', Path)
SrcPath = NewType('SrcPath', Path)



Paths = list[Path]
SrcPaths = list[SrcPath]
YmlPaths = list[YmlPath]
MatchedStringsToPaths = dict[str, SrcPaths]
KeysToPaths = NewType('KeysToPaths', MatchedStringsToPaths)
KeysToPathsMapping = Mapping[str, SrcPaths]

PathsToLines = dict[SrcPath, list[str]]
