<script>
    import { invoke } from "@tauri-apps/api";

    let games = [];
    const game_names = { ...localStorage };
    for (const name in game_names) {
        invoke("insert_new_game", {newGame: name})
        games.push({name, stats: JSON.parse(game_names[name])})
    }
    console.log(games);

    /** @type {HTMLDialogElement} */
    let add_game_dialog;
    let is_showing_add_game_dialog = false;
    function toggle_add_game_dialog() {
        is_showing_add_game_dialog = !is_showing_add_game_dialog;

        if (is_showing_add_game_dialog) {
            add_game_dialog.showModal();
            return
        }

        add_game_dialog.close();
    }

    let new_game = "";
    /** @type {number} */
    let new_game_total_hours;
    async function add_game() {
        if (localStorage.getItem(new_game) == null) {
            let data = {
                total: Number(new_game_total_hours),
                longest_sesh: 0,
                today: 0,
            };
            let json_data = JSON.stringify(data);
            localStorage.setItem(new_game, json_data);
            invoke("insert_new_game", {newGame: new_game});
        }
    }
</script>

<div class="container">
    <h1 class="logo">hourglass</h1>
    <button on:click={toggle_add_game_dialog} style="position: absolute; top: 0; right: 0; margin: 15px; cursor: pointer;">+</button>

    <dialog bind:this={add_game_dialog}>
        <div style="margin: 15px;">
            <label>name: </label><input bind:value={new_game} placeholder="game name" style="cursor: text; margin: 10px;"/><br>
            <label>total hours: </label><input bind:value={new_game_total_hours} placeholder="0.5 (30 minutes)" style="cursor: text; margin: 10px; margin-bottom: 20px;"/><br>
            <button on:click={toggle_add_game_dialog} style="float: left; margin-bottom: 20px; cursor: pointer;">cancel</button>
            <button on:click={add_game} style="float: right; cursor: pointer;">add</button>
        </div>
    </dialog>

    {#each games as game}
        <div class="stats">
            <div class="stat-item" title="name">{game.name}</div>
            <div class="vl"></div>
            <div class="stat-item" title="total">{game.stats.total}</div>
            <div class="vl"></div>
            <div class="stat-item" title="longest session">{game.stats.longest_sesh}</div>
            <div class="vl"></div>
            <div class="stat-item" title="today">{game.stats.today}</div>
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
