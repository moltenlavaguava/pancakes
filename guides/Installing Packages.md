# What are packages?
Packages are external collections of code that you can use in your own projects. Common examples of packages that you will use are `numpy`, `matplotlib`, and `pandas`, for example. To use these packages, you must first install them into your project. 

# Installing Packages
To install packages to your project, follow one of these two options:

1. Open a terminal window in your project. If you're using VS Code, using the shortcut `Ctrl`+`Shift`+`` ` ``

* Note that using uv to install packages is going to be faster (3-4x) and less prone to errors so installing the pip package to bypass it is not required.
## Intalling pip Package (not required)

1. Open the project folder created by Pancakes in VS Code which you want to install packages on.
2. Click `Terminal` then `New Terminal` or do the keyboard shortcut: ``Ctrl + Shift + ` ``
3. Run: `uv pip install pip` This will install the pip package so that, just like normal, you can use `pip install <package>`

## Intalling Other Packages w/ uv 
### (required if pip package was not installed)
1. Open the project folder created by Pancakes in VS Code which you want to install packages on.
2. Open the VS Code terminal using the keyboard shortcut: ``Ctrl + Shift + ` ``
3. To install any package such as `pandas`, `matplotlib`, `numpy`, `scipy`, etc. Run the command `uv pip install <package>`
    * Example command installing `pandas` using uv: `uv pip install pandas`  

## Intalling Other Packages w/o uv 
### (only works if pip package was installed)

1. Open the project folder created by Pancakes in VS Code which you want to install packages on.
2. Open the VS Code terminal using the keyboard shortcut: ``Ctrl + Shift + ` ``
3. To install any package such as `pandas`, `matplotlib`, `numpy`, `scipy`, etc. Run the command `pip install <package>`
    * Example command installing `pandas` using uv: `pip install pandas`  

