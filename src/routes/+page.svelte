<script>
    import { invoke } from "@tauri-apps/api";
    import { listen } from "@tauri-apps/api/event";

    let programs = [];
    const program_names = { ...localStorage };

    let today = new Date();
    today.setHours(0, 0, 0, 0);
    invoke("compare_timestamps", {today: today.getTime()}).then(same_day => {
        if (!same_day) {
            for (const name in program_names) {
                localStorage.removeItem(name);
                let stats = JSON.parse(program_names[name]);
                stats.today = 0;
                stats.todays_sesh = 0;

                let json_data = JSON.stringify(stats);
                localStorage.setItem(name, json_data);
            }
        }
    });

    for (const name in program_names) {
        invoke("insert_new_program", {newProgram: {name, stats: JSON.parse(program_names[name])}})
        programs = [...programs, {name, stats: JSON.parse(program_names[name])}];
    }

    listen("update-longest-sesh", update => {
        for (const program of programs) {
            if (program.name === update.payload.name) {
                program.stats.longest_sesh = update.payload.sesh;
                localStorage.removeItem(program.name);
                localStorage.setItem(program.name, JSON.stringify(program.stats));
            }
        }
    });

    listen("update-program-data", updated => {
        for (const program of programs) {
            for (const updated_program of updated.payload) {
                if (program.name !== updated_program.name) {
                    continue;
                }

                let changed = false;
                if (program.stats.total < updated_program.stats.total) {
                    program.stats.total = updated_program.stats.total;
                    changed = true;
                }
                if (program.stats.today < updated_program.stats.today) {
                    program.stats.today = updated_program.stats.today;
                    changed = true;
                }

                if (changed) {
                    localStorage.removeItem(program.name);
                    localStorage.setItem(program.name, JSON.stringify(program.stats));
                }
            }
        }

        programs = programs;
    });

    /** @type {HTMLDialogElement} */
    let add_program_dialog;
    let is_showing_add_program_dialog = false;
    function toggle_add_program_dialog() {
        is_showing_add_program_dialog = !is_showing_add_program_dialog;

        if (is_showing_add_program_dialog) {
            add_program_dialog.showModal();
            return
        }

        add_program_dialog.close();
    }

    let new_program = "";
    /** @type {number} */
    let new_program_total_hours;
    async function add_program() {
        if (localStorage.getItem(new_program) == null) {
            let data = {
                total: Number(new_program_total_hours),
                longest_sesh: 0,
                todays_sesh: 0,
                today: 0,
            };
            programs = [...programs, {name: new_program, stats: data}];

            let json_data = JSON.stringify(data);
            localStorage.setItem(new_program, json_data);

            invoke("insert_new_program", {newProgram: {name: new_program, stats: data}});
            toggle_add_program_dialog();
        }
    }

    $: programs_data = programs;
</script>

<div class="container">
    <h1 class="logo">hourglass</h1>
    <button on:click={toggle_add_program_dialog} style="position: absolute; top: 0; right: 0; margin: 15px; cursor: pointer;">+</button>

    <dialog bind:this={add_program_dialog}>
        <div style="margin: 15px;">
            <label>name: </label><input bind:value={new_program} placeholder="program name" style="cursor: text; margin: 10px;"/><br>
            <label>total hours: </label><input type="number" bind:value={new_program_total_hours} placeholder="0.5 (30 minutes)" style="cursor: text; margin: 10px; margin-bottom: 20px;"/><br>
            <button on:click={toggle_add_program_dialog} style="float: left; margin-bottom: 20px; cursor: pointer;">cancel</button>
            <button on:click={add_program} style="float: right; cursor: pointer;">add</button>
        </div>
    </dialog>

    {#each programs_data as program}
        <div class="stats">
            <div class="stat-item" title="name">{program.name}</div>
            <div class="vl"></div>
            <div class="stat-item" title="total">{Math.round(program.stats.total * 10) / 10}</div>
            <div class="vl"></div>
            <div class="stat-item" title="longest session">{Math.round(program.stats.longest_sesh * 10) / 10}</div>
            <div class="vl"></div>
            <div class="stat-item" title="today">{Math.round(program.stats.today * 10) / 10}</div>
        </div>
    {/each}
</div>

<style>
    :root {
        --text: whitesmoke;
        --primary: #857eab;
        --secondary: #3c336d;
        --accent: #6555c1;
        --bg: #08080b;

        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 16px;
        line-height: 24px;
        font-weight: 400;

        color: var(--text);
        background-color: var(--bg);

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
    }

    ::placeholder {
       text-align: center; 
    }

    ::-webkit-input-placeholder {
       text-align: center;
    }

    :-moz-placeholder {
       text-align: center;  
    }

    ::-moz-placeholder {
       text-align: center;  
    }

    :-ms-input-placeholder {  
       text-align: center; 
    }

    * {
        margin: 0;
        padding: 0;
       -moz-user-select: -moz-none;
       -khtml-user-select: none;
       -webkit-user-select: none;
       -ms-user-select: none;
       user-select: none;
    }

    input, button {
        border-radius: 6px;
        border: 2px solid var(--secondary);
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        color: var(--text);
        background-color: var(--bg);
        transition: border-color 0.25s;
        box-shadow: 0 2px 2px #3c336d10;
        outline: none;
    }

    input:hover, button:hover {
        border: 2px solid var(--accent);
    }

    input:focus {
        outline-width: 0;
    }

    dialog {
        max-width: 100%;
        margin: auto;
        background-color: var(--bg);
        color: var(--text);
        border: 2px solid black;
        border-radius: 5px;
        z-index: 10;
    }

    dialog[open] {
        animation: fade-in 150ms cubic-bezier(0, 0, 0.2, 1);
    }

    .container {
        margin: 0;
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
    }

    .logo {
        margin: 15px auto 40px auto;
        color: var(--primary);
        transition: font-size 0.25s;
    }

    .logo:hover {
        text-transform: uppercase;
        font-size: 60px;
    }

    .stats {
        margin: auto auto 15px;
        border: 2px solid var(--secondary);
        border-radius: 5px;
        max-width: 100%;
        cursor: default;
    }

    .stats .stat-item {
        display: inline-block;
        padding: 5px;
        margin: 5px;
        font-size: 20px;
        transition: font-size 0.25s;
    }

    .stats .stat-item:hover {
        font-size: 22px;
    }

    .stats .vl {
        display: inline;
        font-size: 22px;
        border-left: 2px solid var(--accent);
    }
</style>
