"""Build a wheel, install it into an isolated virtualenv, and run package smoke checks."""

from __future__ import annotations

import os
import pathlib
import shutil
import subprocess
import sys
import tempfile
import venv


REPO_ROOT = pathlib.Path(__file__).resolve().parents[1]
PACKAGE_SMOKE = REPO_ROOT / "tests" / "package_smoke.py"


def _run(command: list[str], *, cwd: pathlib.Path | None = None, env: dict[str, str] | None = None) -> None:
    subprocess.run(command, cwd=cwd, env=env, check=True)


def _python_in_venv(venv_dir: pathlib.Path) -> pathlib.Path:
    if sys.platform == "win32":
        return venv_dir / "Scripts" / "python.exe"
    return venv_dir / "bin" / "python"


def main() -> None:
    with tempfile.TemporaryDirectory() as tmp_dir:
        tmp_path = pathlib.Path(tmp_dir)
        dist_dir = tmp_path / "dist"
        venv_dir = tmp_path / "venv"
        smoke_script = tmp_path / "package_smoke.py"

        venv.EnvBuilder(with_pip=True).create(venv_dir)
        venv_python = _python_in_venv(venv_dir)

        _run([sys.executable, "-m", "maturin", "build", "--release", "--out", str(dist_dir)], cwd=REPO_ROOT)

        wheels = sorted(dist_dir.glob("cdg_python_client-*.whl"))
        if not wheels:
            raise RuntimeError(f"No wheel produced in {dist_dir}")

        _run([str(venv_python), "-m", "pip", "install", "--force-reinstall", str(wheels[-1])])

        shutil.copy2(PACKAGE_SMOKE, smoke_script)
        env = os.environ.copy()
        env["CDG_EXPECT_SITE_PACKAGES"] = "1"
        env["CDG_REPO_ROOT"] = str(REPO_ROOT)

        _run([str(venv_python), str(smoke_script)], cwd=tmp_path, env=env)


if __name__ == "__main__":
    main()
