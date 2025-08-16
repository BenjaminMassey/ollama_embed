# ollama_embed
Copyright &copy; 2025 Benjamin Massey (Version 0.1.0)

`ollama_embed`: a library for bundling ollama runtime into rust projects

## Usage

Add this project into your project's `Cargo.toml` dependencies:

```toml
ollama_embed = { git = "https://www.github.com/BenjaminMassey/ollama_embed" }
```

The next `cargo build` for your project may take quite a while: `ollama_embed` will download runnable binaries into your project. It will place the ollama runtime in a folder `ollama-win` or `ollama-lin`, depending on your system. It will also create a folder `ollama-model` with an empty file `ModelFile` in it.

That `ModelFile` is where you can define which LLM your project will use: [see ollama's documentation on how to set this up](https://github.com/ollama/ollama/blob/main/docs/modelfile.md). However, at it's most basic, you could start with one of two options:

- Simply copy an existing model. For Qwen's Qwen3 llm model, you could simply have your `ModelFile` only contain the text `FROM qwen3`. If it is not already loaded into the user's computer via ollama, then your first `ollama_embed:chat(..)` attempt will involve downloading this model: note that this is very slow. If going this route, you may want to do some situation such as a loading screen/message and a manual `./ollama-win/ollama.exe pull qwen3` (or similar). Or this might be fine for personal developer testing!

- Add a GGUF file into your `ollama-model` directory. This is in many ways the intended purpose, since `ollama_embed` seeks to provide a way to build an LLM into a Rust deployment. Get a licensing friendly model from the internet, place it at `ollama-model/model.gguf`, and start your `ModelFile` with `FROM ./model.gguf`. Do note that it generally appears that you will need to provide some simple settings in this scenario, such as a system prompt and a template string. If you are, for example, using a model that is also available on ollama, then you could start with `./ollama-win/ollama.exe pull <MODEL>`, and then use `./ollama-win/ollama.exe show <MODEL> --system` and `./ollama-win/ollama.exe show <MODEL> --template`. These might be all you need!

## Additional Notes

The model created by your `ModelFile` will be named `RustGGUF` in ollama, even if all you did was `FROM qwen3`. If you ever want to do anything with it, such as run it for testing, then you should refer to this naming.

If you ever change your `ModelFile`, then you will need to first remove it from ollama before it will properly remake it. This can be done via `./ollama-win/ollama.exe rm RustGGUF`.

## Future Plans

There needs to be a system via this library where the project using `ollama_embed` will properly package up its own project build alongside copies of its `ollama-win`/`ollama-lin` and `ollama-model` directories. Unclear if an injected `build.rs` or copied shell scripts would be better.

There should maybe be injection of useful ignore lines to the resulting projects `.gitignore` file.