async function run() {
    await wasm_bindgen(browser.runtime.getURL('resources/popup_bg.wasm'));
}

run();