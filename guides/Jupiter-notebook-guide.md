## What is a Jupyter Notebook file?

A Jupyter Notebook file is an interactive file made up of individual cells that can be run one at a time. These cells can be either markdown cells or Python code cells.

- Markdown cells allow the user to annotate their notebook with formatted text, LaTeX-style equations, images, and other styling options that provide more customization than raw text.
- Unlike raw Python (`.py`) files, Jupyter Notebook files (`.ipynb`) allow the user to create and run separate cells one at a time. This is useful for large multi-step programs that do not require the entire file to be rerun each time.

## Running a Jupyter Notebook for the First Time

To use a Jupyter Notebook file, a `kernel` must be selected. To achieve this, do the following:

1. Click `Select Kernel`.

![Select kernel](select-kernel)

2. If the `.venv` Python environment created by Pancakes does not immediately appear, click `Python Environments...`.

![Select Python Environments](python-environments)

3. Click the `.venv` created by Pancakes. It should be tagged as `recommended` and have a star. *To be super sure you have the right python, the path should contain `.venv`.  

![Select .venv](select-venv)

4. Run the first code cell. If you are prompted to install the `ipykernel` package, click `Install`.

![Download ipykernel](ipkernel-install)

5. Restart the kernel by clicking `Restart` near the top of the page, then run the code cell again. After this, clicking the `Run` button by code cells should execute the Python code within.