import os
from pathlib import Path

project_root = Path(__file__).parents[1].absolute()

pyproject_toml_path = project_root / "pyproject.toml"

with open(pyproject_toml_path, "r") as f:
    contents = f.read()

with open(pyproject_toml_path, "w") as f:
    if (github_ref := os.getenv("GITHUB_REF")) and github_ref.startswith("refs/tags/"):
        version = github_ref.replace("refs/tags/v", "")
        contents = contents.replace('version = "0.0.0"', f'version = "{version}"')
    else:
        version = "0.0.0"

    f.write(contents)

print(f"TOMBI_VERSION={version}")
