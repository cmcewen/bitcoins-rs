use js_sys;
use wasm_bindgen::prelude::*;

use riemann_core::{
    types::primitives::{PrefixVec},
    ser::{Ser}
};
use riemann_bitcoin::{
    types::{script, txout},
};

use crate::errors::WasmError;

wrap_struct!(txout::TxOut);
wrap_struct!(txout::Vout);

impl_simple_getter!(TxOut, value, u64);

#[wasm_bindgen]
impl TxOut {
    /// Instantiate a new TxOut.
    #[wasm_bindgen(constructor)]
    pub fn new(value: u64, script_pubkey: &[u8]) -> Self {
        txout::TxOut{
            value,
            script_pubkey: script_pubkey.into()
        }.into()
    }

    /// Instantiate the null TxOut, which is used in Legacy Sighash.
    pub fn null() -> Self {
        txout::TxOut{
            value: 0xffff_ffff_ffff_ffff,
            script_pubkey: script::ScriptPubkey::null()
        }.into()
    }

    /// Instantiate the null TxOut, which is used in Legacy Sighash.
    pub fn default() -> Self {
        txout::TxOut{
            value: 0xffff_ffff_ffff_ffff,
            script_pubkey: script::ScriptPubkey::null()
        }.into()
    }

    #[wasm_bindgen(method, getter)]
    pub fn script_pubkey(&self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(self.0.script_pubkey.items())
    }
}

#[wasm_bindgen]
impl Vout {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(txout::Vout::new(vec![]))
    }

    pub fn push(&mut self, input: &TxOut) {
         self.0.push(input.0.clone())
    }

    #[wasm_bindgen(method, getter)]
    pub fn items(&self) -> js_sys::Array {
        self.0.items()
            .into_iter()
            .map(|v| TxOut::from(v.clone()))
            .map(JsValue::from)
            .collect()
    }
}