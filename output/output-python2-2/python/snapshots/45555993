from setuptools import setup, find_packages

setup(
    name='listerhtml',
    version="0.0.1",
    description="HTML output plugin for lister",
    author="Yury Yurevich",
    author_email="the.pythy@gmail.com",
    url="http://gorod-omsk.ru/blog/pythy/2007/02/18/understanding-eggs-entrypoints/",
    packages=find_packages(),
    zip_safe=True,
    install_requires=['lister'],
    entry_points={
        'lister.output': ['html=listerhtml:html_list'],
    }
)
