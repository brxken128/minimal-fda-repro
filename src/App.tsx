import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
    const check = () => {
        invoke("has_full_disk").then((message) => {
            alert(message);
        });
    };

    return (
        <div>
            <button
                onClick={() => {
                    check();
                }}
            >
                Click
            </button>
        </div>
    );
}

export default App;
