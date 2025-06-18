from dataclasses import dataclass
from typing import Dict, Any, Optional
import json
from pathlib import Path
import project_dirs_py._project_dirs_rust as _base

from project_dirs_py._project_dirs_rust import xdg_config_dirs, xdg_data_dirs


@dataclass
class ProjectDirs:
    bin: Optional[Path]
    cache: Optional[Path]
    config: Optional[Path]
    data: Optional[Path]
    include: Optional[Path]
    lib: Optional[Path]
    log: Optional[Path]
    project_root: Optional[Path]
    runtime: Optional[Path]
    state: Optional[Path]

    @classmethod
    def _from_str_dict(cls, d: Dict[str, str]) -> "ProjectDirs":
        return ProjectDirs(
            bin=Path(d["bin"]) if "bin" in d else None,
            cache=Path(d["cache"]) if "cache" in d else None,
            config=Path(d["config"]) if "config" in d else None,
            data=Path(d["data"]) if "data" in d else None,
            include=Path(d["include"]) if "include" in d else None,
            lib=Path(d["lib"]) if "lib" in d else None,
            log=Path(d["log"]) if "log" in d else None,
            project_root=Path(d["project_root"]) if "project_root" in d else None,
            runtime=Path(d["runtime"]) if "runtime" in d else None,
            state=Path(d["state"]) if "state" in d else None,
        )

@dataclass
class BuilderResult:
    application_name: str
    dirs: Dict[str, "ProjectDirs"]

    @classmethod
    def _from_str_dict(cls, d: Dict[str, Any]) -> "BuilderResult":
        return BuilderResult(
            application_name=d["application_name"],
            dirs={k: ProjectDirs._from_str_dict(v) for k, v in d["dirs"].items()},
        )

    @classmethod
    def from_default(
        cls, application: str, organization: str, qualifier: str
    ) -> "BuilderResult":
        """Use default manifest to evaluate project directories"""
        default_builder = json.dumps(
            {
                "qualifier": qualifier,
                "organization": organization,
                "application": application,
                "spec": "system-default",
            }
        )
        return cls._from_str_dict(json.loads(_base.from_manifest(default_builder)))

    @classmethod
    def from_builder(cls, string: str) -> "BuilderResult":
        """Evaluate project directories from the manifest"""
        return cls._from_str_dict(json.loads(_base.from_manifest(string)))


__all__ = ["BuilderResult", "ProjectDirs", "xdg_config_dirs", "xdg_data_dirs"]
