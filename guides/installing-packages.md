# What are packages?
Packages are external collections of code that you can use in your own projects. Common examples of packages that you will use are `numpy`, `matplotlib`, and `pandas`, for example. To use these packages, you must first install them into your project. 

Python packages are organized by **Virtual Environments,** or projects. Typically, you separate projects by having different folders for each project. While you can install packages to your entire computer, it is *strongly* recommended to not do this! Make sure you are only ever installing packages when you are in a project folder. 

# Installing Packages
To install packages to your project, first open a terminal window. If you're using VS Code, using the shortcut `Ctrl`+`` ` `` on Windows or `Command` + `` ` `` on macOS works. Then, run either one of these commands to install a package:

```
uv pip install <package name>
pip install <package name>
```

**Note:** for the pip command to work in your project, you must first run the command ``uv pip install pip``.

**Important**: these commands will only work when you are in a project folder! The easiest way to make sure of this is by running these commands in a VS Code terminal in your project. 

For example, to install the `numpy` package, run the following command in your VS Code terminal:

```
uv pip install numpy
```