from setuptools import setup
from Cython.Build import cythonize

setup(
    name='SeaSnake library',
    ext_modules=cythonize("seasnake.py"),
    zip_safe=False,
)