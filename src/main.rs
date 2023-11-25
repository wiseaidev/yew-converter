use web_sys::HtmlInputElement;
use yew::prelude::*;

fn convert_values(input: &str, base: u32) -> (String, String, String, String) {
    let decimal_value = u64::from_str_radix(input, base).unwrap_or(0);
    let hex_value = format!("{:x}", decimal_value);
    let binary_value = format!("{:b}", decimal_value);
    let ascii_value = match decimal_value as u8 {
        v if v.is_ascii() => v as char,
        _ => ' ',
    }
    .to_string();

    (
        decimal_value.to_string(),
        hex_value,
        binary_value,
        ascii_value,
    )
}

#[function_component(FormModel)]
pub fn form_model() -> Html {
    let input_decimal_ref = use_node_ref();
    let input_decimal_handle = use_state(String::default);
    let input_decimal = (*input_decimal_handle).clone();

    let input_binary_ref = use_node_ref();
    let input_binary_handle = use_state(String::default);
    let input_binary = (*input_binary_handle).clone();

    let input_hex_ref = use_node_ref();
    let input_hex_handle = use_state(String::default);
    let input_hex = (*input_hex_handle).clone();

    let input_ascii_ref = use_node_ref();
    let input_ascii_handle = use_state(String::default);
    let input_ascii = (*input_ascii_handle).clone();

    let on_decimal_change = {
        let input_decimal_ref = input_decimal_ref.clone();
        let input_decimal_handle = input_decimal_handle.clone();
        let input_binary_handle = input_binary_handle.clone();
        let input_hex_handle = input_hex_handle.clone();
        let input_ascii_handle = input_ascii_handle.clone();

        Callback::from(move |_| {
            let input = input_decimal_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_decimal_handle.set(value.clone());

                let (_, h, b, a) = convert_values(&value.clone(), 10);
                input_binary_handle.set(b);
                input_hex_handle.set(h);
                input_ascii_handle.set(a);
            }
        })
    };

    let on_binary_change = {
        let input_binary_ref = input_binary_ref.clone();
        let input_decimal_handle = input_decimal_handle.clone();
        let input_hex_handle = input_hex_handle.clone();
        let input_binary_handle = input_binary_handle.clone();
        let input_ascii_handle = input_ascii_handle.clone();

        Callback::from(move |_| {
            let input = input_binary_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_binary_handle.set(value.clone());

                let (d, h, _, a) = convert_values(&value.clone(), 2);
                input_decimal_handle.set(d);
                input_hex_handle.set(h);
                input_ascii_handle.set(a);
            }
        })
    };

    let on_hex_change = {
        let input_hex_ref = input_hex_ref.clone();
        let input_decimal_handle = input_decimal_handle.clone();
        let input_binary_handle = input_binary_handle.clone();
        let input_hex_handle = input_hex_handle.clone();
        let input_ascii_handle = input_ascii_handle.clone();

        Callback::from(move |_| {
            let input = input_hex_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_hex_handle.set(value.clone());

                let (d, _, b, a) = convert_values(&value.clone(), 16);
                input_decimal_handle.set(d);
                input_binary_handle.set(b);
                input_ascii_handle.set(a);
            }
        })
    };

    let on_ascii_change = {
        let input_ascii_ref = input_ascii_ref.clone();
        let input_decimal_handle = input_decimal_handle.clone();
        let input_binary_handle = input_binary_handle.clone();
        let input_ascii_handle = input_ascii_handle.clone();

        Callback::from(move |_| {
            let input = input_ascii_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_ascii_handle.set(value.clone());

                let (d, h, b, _) = convert_values(&value.clone(), 10);
                input_decimal_handle.set(d);
                input_binary_handle.set(b);
                input_hex_handle.set(h);
            }
        })
    };

    html! {
        <section
        >
            <div>
                <h2>
                  {"Dope Number Converter!"}
                </h2>
                <div>
                  <label for="decimal"
                    >{"Decimal"}</label
                  >
                    <input
                      type="text"
                      id="decimal"
                      name="decimal"
                      placeholder="Decimal"
                      required={true}
                      value={input_decimal}
                      ref={input_decimal_ref}
                      oninput={on_decimal_change}
                    />
                </div>
                <div>
                  <label for="binary"
                    >{"Binary"}</label
                  >
                  <input
                    type="text"
                    id="binary"
                    name="binary"
                    placeholder="Binary"
                    required={true}
                    value={input_binary}
                    ref={input_binary_ref}
                    oninput={on_binary_change}
                  />
                </div>
                <div>
                  <label for="hex"
                    >{"Hex"}</label
                  >
                  <input
                    type="text"
                    id="hex"
                    name="hex"
                    placeholder="Hex"
                    required={true}
                    value={input_hex}
                    ref={input_hex_ref}
                    oninput={on_hex_change}
                  />
                </div>
                <div>
                  <label for="ascii"
                    >{"Ascii"}</label
                  >
                  <input
                    type="text"
                    id="ascii"
                    name="ascii"
                    placeholder="Ascii"
                    required={true}
                    value={input_ascii}
                    ref={input_ascii_ref}
                    oninput={on_ascii_change}
                  />
                </div>
            </div>
        </section>
    }
}

fn main() {
    yew::Renderer::<FormModel>::new().render();
}
