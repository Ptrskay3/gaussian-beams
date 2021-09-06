import sys

if sys.version_info[:2] < (3, 6):
    raise RuntimeError("Python version >= 3.6 required.")


from setuptools import setup, find_packages


try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension, Binding

setup(
    name="beams",
    version="0.1.0",
    author="Péter Leéh",
    author_email="leeh123peter@gmail.com",
    description="Gaussian beams",
    packages=find_packages(),
    include_package_data=True,
    classifiers=[
        "Programming Language :: Python :: 3",
        "Programming Language :: Rust",
        "Development Status :: 4 - Beta",
        "Intended Audience :: Education",
        "Intended Audience :: Science/Research",
        "Topic :: Scientific/Engineering :: Physics",
    ],
    setup_requires=["setuptools-rust>=0.11.4", "wheel"],
    rust_extensions=[
        RustExtension("beams.beams", "Cargo.toml", debug=False, binding=Binding.PyO3, py_limited_api=True),
    ],
    zip_safe=False,
)