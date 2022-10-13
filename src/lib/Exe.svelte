<script lang="ts">
    import { Command, open } from "@tauri-apps/api/shell";
    import { invoke } from '@tauri-apps/api/tauri';


    export let filePath: string;
    let exeName = filePath
        .split(String.fromCharCode(92))
        .at(-1)
        .split(".")
        .at(0);
    export let iconPath;
    let appName = "App";
    function openExe() {
        invoke('run_exe',{filePath})
        invoke('get_file_name',{filePath})
}
async function getFileName() {
    let name = await invoke("get_file_name",{filePath});
    return name;
}
</script>

<div class="container">
    <img src={iconPath} alt="icon" on:click={openExe} />
    {#await getFileName() then name}
        <div>{name}</div>
    {/await}
</div>

<style>
    img {
        height: 100px;
        width: 100px;
        border-radius: 5px;
        margin-bottom: 10px;
    }
    .container {
        margin: 10px;
        display: flex;
        flex-direction: column;
        text-align: center;
        width: 100px;
    }
</style>
