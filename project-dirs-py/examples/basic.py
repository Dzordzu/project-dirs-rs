import project_dirs_py

dirs = project_dirs_py.BuilderResult.from_default('app', 'mycorp', 'org')
__import__('pprint').pprint(dirs)
