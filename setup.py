from setuptools import setup
from setuptools_rust import Binding, RustExtension
import subprocess
import os

subprocess.run("curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y", shell=True)
os.environ['PATH'] = os.environ.get("HOME", "") + '/.cargo/bin:' + os.environ.get("PATH", "")


setup(
    name="fcm_async",
    version="0.1.0",
    rust_extensions=[RustExtension("fcm_async.fcm_async", binding=Binding.PyO3)],
    packages=["fcm_async"],
    include_package_data=True,
    zip_safe=False,
)
