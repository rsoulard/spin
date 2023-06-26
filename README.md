<!-- Intro -->
<br />
<div align="center">
  <img src="images/icon.png" alt="Logo" width="100" height="100"/>
  <h3 align="center">Spin</h3>
  <p align="center">A command line string generation tool</p>
</div>

<!-- Table of Contents -->
<details>
  <summary>Table of Contents</summary>
  <ul>
    <li>
      <a href="#about">About</a>
    </li>
    <li>
      <a href="#usage">Usage</a>
      <ul>
        <li>
          <a href="#echo-a-string">Echo a String</a>
        </li>
        <li>
          <a href="#generate-a-uuid">Generate a UUID</a>
        </li>
        <li>
          <a href="#postprocessing">Postprocessing</a>
        </li>
      </ul>
    </li>
    <li>
      <a href="#license">License</a>
    </li>
    <li>
      <a href="#acknowledgments">Acknowledgments</a>
    </li>
  </ul>
</details>

<!-- About -->
## About
Spin is a cross-platform string generation tool that is run from the command line. Instead of having to go to website when a string resource, such as a UUID, is needed, Spin can be used to generate it.

Additionally, there are tools for adding elements like quotation marks to a string after it has been generated.

<!-- Getting Started -->

<!-- Usage -->
## Usage
The following are a few examples of Spin in use. You can also use the `help` command to see a full list of commands and options. Generally, the format of usage is as follows:
```console
fizz@buzz~$ spin [OPTIONS] <COMMAND>
```

### Echo a String
User the `echo` command to print a string back to the command line. This is useful in conjunction with <a href="#postprocessing">postprocessing</a>.
```console
fizz@buzz~$ spin echo "Hello, World!"
Hello, World!
```

### Generate A UUID
Use the `uuid` command to generate a random UUID.
```console
fizz@buzz~$ spin uuid
e458f9a9-e625-4d2b-8886-22dd0c83ddba
```

### Postprocessing

#### Adding Quotation Marks to a String
Use the `-s` option to add single quotes around a string.
```console
fizz@buzz~$ spin -s echo "Hello, World!"
'Hello, World!'
```

The '-d' option can be used to add double quotes around a string.
```console
fizz@buzz~$ spin -d echo "Hello, World!"
"Hello, World!"
```

<!-- License -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<!-- Acknowledgments -->
## Acknowledgments

Thanks to the contributors and maintainers of the [clap](https://github.com/clap-rs/clap) package for Rust.
