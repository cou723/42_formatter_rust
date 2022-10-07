# norm formatter

## ~~Development has been completed~~

- ~~You must have an existing formatter~~
- ~~No problems or dissatisfaction with existing formatter~~
- ~~Because it is written in Rust, the execution speed can be differentiated, but in reality, it only reads a file of about 150 lines at most, so it is not expected to make a big difference.~~

~~For these three reasons, I am terminating development.~~

## Resume development
The existing formatter does not work on windows alone, so development will continue.

## Notes on this document

This text was written by a person who cannot read or write English, using a translation.
So, please understand if there are some strange parts.

## Summary of norm formatter

This is a command-line formatter that formats code written in C into a form suitable for norminette.

## Usage
Specify the `.c` or `.h` file to be formatted in <path>.

```shell
norm-formatter.exe <path>
````

In VSCode, this can be combined with Run on Save to automatically run on save.

### Usage examples

This is an example of use on Windows + VSCode.

You can format the file on save by following the steps below.

1. pass the path to the downloaded executable file (norm-formatter). 
Add the following setting1 to `.vscode/setting.json`. 
3. add VSCode extension Clang-Format/xaver. 
Place the configuration file `c_formatter_42/c_formatter_42/data/.clang-format` for [c_formatter_42](https://github.com/dawnbeen/c_formatter_42) in the root of your workspace. clang-format` to the root of your workspace. 
5. enable formatOnSave in the configuration on VSCode

setting1
```json
  "emeraldwalk.runonsave": {
    "commands": [
      {
        "match": ".c",
        "isAsync": true,
        "cmd": "norm-formatter ${file}"
      },
    ]
  }
```

## Supported norminette error messages

Not all norminette error messages are supported, but it is developed for use with [dawnbeen's c_formatter_42](https://github.com/dawnbeen/c_formatter_42)

| error message        | Countermeasures                                                                     |
|----------------------|-------------------------------------------------------------------------------------|
| NEWLINE_PROCESS_FUNC | If there are no blank lines between functions, add them there                       |
| RETURN_PARENTHESIS   | If the return value is not enclosed in parentheses, enclose it in parentheses       |
| NO_ARGS_VOID         | If there is no argument when defining a function, add void.                         |
| SPACE_REPLACE_TAB    | If there is a space between the variable type and the variable name in the variable |
| SPACE_BEFORE_FUNC    | If there is a space between the function name and the function return value in the  |
| BRACE_SHOULD_EOL     | If the last line is not blank, insert it.                                           |
| SPACE_AFTER_KW       | `break;` to `break ;`.                                                              |

### NEWLINE_PROCESS_FUNC

before
```c
int	x(void)
{
	return 1;
}
int	y(void)
{
	return 2;
}
```

after
```c
int	x(void)
{
	return 1;
}

int	y(void)
{
	return 2;
}
```

### RETURN_PARENTHESIS

before
```c
int	x(void)
{
	return 1;
}
```

after
```c
int	x(void)
{
	return (1);
}
```

### NO_ARGS_VOID

before
```c
int	x(void)
{
	return (1);
}
```

after
```c
int	x(void)
{
	return (1);
}
```

### SPACE_REPLACE_TAB

before
```c
void	x(void)
{
	int x;
	double y;
}
```

after
```c
void	x(void)
{
	int		x;
	double	y;
}
```

### SPACE_BEFORE_FUNC

before
```c
int x(void)
{
	return (1);
}
```

after
```c
int	x(void)
{
	return (1);
}
```

### BRACE_SHOULD_EOL

before
```c
int	x(void)
{
	return (1);
}
```

after
```c
int	x(void)
{
	return (1);
}

```
	
	Translated with www.DeepL.com/Translator (free version)
