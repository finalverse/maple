from setuptools import setup, find_packages

setup(
    name="maple-sdk",
    version="0.1.0",
    description="Python SDK for the MAPLE ecosystem",
    author="Finalverse Inc.",
    author_email="maple@finalverse.com",
    url="https://mapleai.org",
    packages=find_packages(),
    install_requires=["requests"],
    license="© 2025 Finalverse Inc. All rights reserved",
    classifiers=[
        "Programming Language :: Python :: 3",
    ],
)