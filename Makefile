PYTHON_VERSION := 3.12
CONDA_ENV_NAME := agni

.PHONY python-tests:
python-tests:
	python -m pytest -vvrP ./pythons/tests/test_main.py

.PHONY integration-tests:
integration-tests:
	cargo fmt --all
	maturin develop
	pre-commit run -a
	python -m pytest -vvrP ./pythons/tests/test_main.py

.PHONY conda-env:
conda-env:
	conda remove --name $(CONDA_ENV_NAME) --all
	conda create -n $(CONDA_ENV_NAME) python=$(PYTHON_VERSION) -y
	$(shell conda info --base)/envs/$(CONDA_ENV_NAME)/bin/pip install --upgrade pip
	$(shell conda info --base)/envs/$(CONDA_ENV_NAME)/bin/pip install -r ./pythons/requirements.txt -r ./pythons/test_requirements.txt
	$(shell conda info --base)/envs/$(CONDA_ENV_NAME)/bin/pre-commit install
	$(shell conda info --base)/envs/$(CONDA_ENV_NAME)/bin/pre-commit run -a
