# colorschemify
A blazingly fast commandline tool written in rust that includes a mix of utilities to play with colorschemes and images.

Current feature list:
| feature | action command |
| ------- | -------------- |
| Apply a colorscheme to an image | `apply` |
| Extract a colorscheme from an image | `extract` |
| blur an image | `blur` |


I will update this tool with more features in the future and based on what i need and possibly suggestions from whoever uses it.

## Compiling from source
You will need to have rust and cargo installed on your system, you can find instructions on how to do that [here](https://www.rust-lang.org/tools/install).
Once you have rust and cargo installed, you can clone this repository and compile the code with the following commands:

```bash
git clone https://github.com/ciocapiat02/colorschemify.git
cargo build --release
```

## Usage
Running the executable with the `-h` flag will get you this list of possible flags:

```bash
Usage: colorschemify [OPTIONS] <ACTION> <INPUT> <OUTPUT>

Arguments:
  <ACTION>  Action to perform
  <INPUT>   Input file
  <OUTPUT>  Output file

Options:
  -c, --colorscheme <COLORSCHEME>
          Colorscheme chosen for the apply function
  -l, --length <LENGTH>
          Number of colors to extract from the image [default: 16]
  -g, --generate-image
          generate an image of the colors extracted from the input image
  -i, --iteration-number <ITERATION_NUMBER>
          number of iteration for the kmeans algorithm [default: 1000]
  -s, --show
          Show results in a window
  -b, --blur-amount <BLUR_AMOUNT>
          Amount of blur filter [default: 1]
  -h, --help
          Print help
  -V, --version
          Print version
```


## Colorscheme file
The colorscheme file required by the `-c` flag is simply a yaml file containing the list of colors under the `colorscheme` field, an example is provided with gruvbox in the file `colorschemes/gruvbox.yaml`.
