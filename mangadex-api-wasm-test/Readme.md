# mangadex-api-wasm-test

This is experiment crate for testing the `mangadex-api` in a wasm environment.

To try it:

1. Install [`wasm-pack`](https://github.com/drager/wasm-pack)
2. Build the package

    ```bash
        wasm-pack build --release -d ./dist
    ```

3. Run the package

    _Note: I use [`deno`](https://deno.com) here, but you can use whatever JS runtime you like_

    ```bash
        deno --allow-net=api.mangadex.org --allow-read=. ./dist/mangadex_api_wasm_test.js 
    ```
