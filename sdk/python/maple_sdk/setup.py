from setuptools import setup

setup(
    name="maple-sdk",
    version="0.1.0",
    description="Python SDK for interacting with the MAPLE ecosystem",
    author="Finalverse Inc.",
    author_email="maple@finalverse.com",
    url="https://github.com/finalverse/mapleai.git",
    license="© 2025 Finalverse Inc. All rights reserved",
    py_modules=["maple_sdk"],
    install_requires=["pyo3"],
    classifiers=[
        "Programming Language :: Python :: 3",
        "Operating System :: OS Independent",
    ],
)