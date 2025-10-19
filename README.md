# Bevy AI ChatBox

A simple AI ChatBox implementation built with Bevy, currently supporting the DeepSeek API.

> [!WARNING]
> Development is on hold for the near future, as Bevy does not yet have native support for text input with IME.

## Preview

<div align="center">
    <img src="./docs/preview.png" alt="Preview" style="width: 500px" >
</div>

## Build & Run

First, bundle the application using the provided script:

```shell
nu bundle.nu
```

Then, navigate to the `dist` directory and run the executable.

## Configuration

When you first run the application, it will generate `config.ron` and `dialog.ron` files in the same directory.

You must edit these files before the app will work:

1.  `config.ron`: Add your DeepSeek API key here.
2.  `dialog.ron`: Add your desired system prompt here.

You must restart the application after updating these files for the changes to take effect.