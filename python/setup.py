from setuptools import setup
from Cython.Build import cythonize

setup(
    name='SeaSnake library',
    ext_modules=cythonize(
        'seasnake.py',
        compiler_directives={"language_level": "3"}
    ),
    zip_safe=False
)
