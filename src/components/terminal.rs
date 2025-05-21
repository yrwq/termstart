use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Default, Clone)]
struct TerminalHistory {
    commands: Vec<String>,
    outputs: Vec<String>,
}

#[function_component(Terminal)]
pub fn terminal() -> Html {
    let input_ref = use_node_ref();
    let history = use_state(TerminalHistory::default);
    let current_input = use_state(String::new);

    let onkeydown = {
        let input_ref = input_ref.clone();
        let history = history.clone();
        let current_input = current_input.clone();
        
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                let input = input_ref.cast::<HtmlInputElement>().unwrap();
                let command = input.value();
                
                // Process command
                let output = match command.trim() {
                    "help" => "Available commands: help, clear, version".to_string(),
                    "clear" => {
                        input.set_value("");
                        let mut new_history = (*history).clone();
                        new_history.commands.clear();
                        new_history.outputs.clear();
                        history.set(new_history);
                        return;
                    },
                    "version" => "termstart v0.1.0".to_string(),
                    "" => String::new(),
                    cmd => format!("Command not found: {}", cmd),
                };

                if !output.is_empty() {
                    let mut new_history = (*history).clone();
                    new_history.commands.push(command.clone());
                    new_history.outputs.push(output);
                    history.set(new_history);
                }

                input.set_value("");
                current_input.set(String::new());
            }
        })
    };

    let oninput = {
        let current_input = current_input.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            current_input.set(input.value());
        })
    };

    html! {
        <div class="w-full max-w-3xl mx-auto mt-8 p-4 bg-github-light-button dark:bg-github-dark-button rounded-lg shadow-lg font-mono">
            <div class="mb-4 overflow-y-auto max-h-96 whitespace-pre-wrap">
                <div class="text-github-light-text dark:text-github-dark-text mb-4">
                    {"Welcome to termstart v0.1.0\nType 'help' for available commands.\n"}
                </div>
                {
                    history.commands.iter().enumerate().map(|(i, cmd)| {
                        html! {
                            <div key={i} class="mb-2">
                                <div class="flex items-start text-github-light-text dark:text-github-dark-text">
                                    <span class="text-green-500 mr-2 select-none">{"$"}</span>
                                    <span class="font-bold">{cmd}</span>
                                </div>
                                <div class="text-github-light-text dark:text-github-dark-text ml-4 opacity-90 font-light">
                                    {&history.outputs[i]}
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="flex items-center text-github-light-text dark:text-github-dark-text border-t border-github-light-border dark:border-github-dark-border pt-4">
                <span class="text-green-500 mr-2 select-none">{"$"}</span>
                <input
                    type="text"
                    ref={input_ref}
                    {onkeydown}
                    {oninput}
                    autofocus=true
                    class="flex-1 bg-transparent outline-none border-none text-github-light-text dark:text-github-dark-text"
                    placeholder=" "
                    spellcheck="false"
                    autocomplete="off"
                />
            </div>
        </div>
    }
}