# pathjoin-relabs - joining non-absolute paths to make an absolute path

This effect is specific to Windows. On Windows:

- A path like `C:` or `C:foo` is not absolute, because while it specifies a drive, it is relative to the drive-specific current directory on that drive.
- A path like `/` or `\`, or `/bar` or `\bar`, is (unlike on other operating systems) not absolute, because while it specifies an absolute location on a drive, it does not specify that drive.

Such paths are relative paths in the [usual terminology](https://learn.microsoft.com/en-us/dotnet/standard/io/file-path-formats#traditional-dos-paths). Yet joining these non-absolute (i.e. relative) paths produces an absolute path. For example, neither `C:` nor `\` is absolute, but joining them produces `C:\`, which is absolute.

In one sense, this may seem very obvious: of course joining `C:` and `\` produces `C:\`. But the effect that joining paths all of which are relative can produce a path that is absolute is moderately unintuitive, and for users coming from a Unix perspective, very unintuitive.

This repository contains [a Rust program](src/main.rs) that demonstrates the effect.

## Output

Running the program on Windows with `cargo run` produces:

```text
C:   relative? true   absolute? false
/    relative? true   absolute? false
\    relative? true   absolute? false
C:/  relative? false  absolute? true
C:\  relative? false  absolute? true
```

## Intuitive explanation

Speaking informally, what is happening is that `C:` is relative because only the drive it specifies is absolute, and `/` and `\` are relative on Windows because only the drive-relative location they specify is absolute. So `C:\` (or `C:/`) is absolute because the relative piece in each of `C:` and `\` (or `/`) has been resolved using corresponding absolute information from the other.

## A note on semantics

This is no mere trick. It pertains to the semantics of joining paths *as paths*. It happens to be that, for example, `C:` and `\` also concatenate, as strings, to produce `C:\`. But that this is so is not relevant; what matters is that the operation of joining them as paths produces `C:\`.

This is therefore dissimilar to non-examples such as how *string concatenation* of the valid relative paths `\` and `\.\` would produce the valid absolute path `\\.\` (which is valid because it designates the local [device namespace](https://learn.microsoft.com/en-us/dotnet/standard/io/file-path-formats#dos-device-paths)). Although string concatenation on `\` and `\.\` produce the string `\\.\`, joining `\` and `\.\` as paths just produces `\.\` itself, which is still relative.

## License

[0BSD](LICENSE)
