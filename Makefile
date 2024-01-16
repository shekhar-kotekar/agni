PYTHON_VERSION := 3.12
CONDA_ENV_NAME := agni

.PHONY conda-env:
conda-env:
	conda create -n $(CONDA_ENV_NAME) python=$(PYTHON_VERSION) -y
	$(shell conda info --base)/envs/$(CONDA_ENV_NAME)/bin/pip install --upgrade pip
	$(shell conda info --base)/envs/$(CONDA_ENV_NAME)/bin/pip install -r ./pythons/requirements.txt -r ./pythons/test_requirements.txt
	$(shell conda info --base)/envs/$(CONDA_ENV_NAME)/bin/pre-commit install
	$(shell conda info --base)/envs/$(CONDA_ENV_NAME)/bin/pre-commit run -a
	conda activate $(CONDA_ENV_NAME)
