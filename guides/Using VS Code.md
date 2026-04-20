## Installing VS Code

1. Download VS Code here: [https://code.visualstudio.com/](https://code.visualstudio.com/)

![VS Code Download](windows-vscode.png)

## Opening Projects

1. Open VS Code
2. Click `File`
3. Click `Open Folder`
4. Navigate to the location of folder created by pancakes (default is in `Documents`) and click `Select Folder`

## Using VS Code

### Extensions:
**Important**: To use most file types on VS Code you need the extensions which support them.

To run python files, you need the python extensions such as: `Python`, `Pylance`, `Python Debugger`,and `Python Environments`.

Note: Installing the `Python` extension should install the other requirements along with it.

To run Jupyter Notebooks, you need the extensions: `Jupyter`, `Jupyter Cell Tags`, `Jupyter Keymap`, `Jupyter Notebook Renderers`, and `Jupyter Slide Show`. Note: Installing the `Jupyter` extension should install the other requirements along with it.

To use the serial monitor for a microcontroller, you need the extension: Serial Monitor.

Note: if there is any error running the `main.py` file, make sure to follow steps 2 and 3 to select the interpreter.
1. Attempt to run `main.py`; an error should pop up saying `An Invalid Python interpreter is selected…` If no errors pop up and "Hello world" is printed in the terminal then steps 2 and 3 can be skipped.
2. Click `Select Python Interpreter`
3. Click the option which says `Recommended`. If nothing is recommended select the option that contains `.venv`

![Python Interpreter Selection in VS Code](select-interpreter.png)