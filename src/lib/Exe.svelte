<script lang="ts">
    import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
    import { appDir } from '@tauri-apps/api/path';

    export let filePath: string;
    let exeName = filePath
        .split(String.fromCharCode(92))
        .at(-1)
        .split(".")
        .at(0);
    function openExe() {
        invoke("run_exe", { filePath });
        invoke("get_file_name", { filePath });
    }
    async function getFileName() {
        let name = await invoke("get_file_name", { filePath });
        return name;
    }
    async function getAppDir() {
        const appDirPath = await appDir();
        return appDirPath;
    }
    async function get_icon(){
        const appDirPath = await appDir();
        const icon_path: string = await invoke("get_icon", { appDirPath,filePath });
         const assetUrl = convertFileSrc(icon_path);
        console.log(appDirPath+icon_path);
        console.log(assetUrl);

        return assetUrl;
    }

    </script>

<div class="container">
    {#await get_icon() then icon}
        <img src={icon} alt="icon" on:click={openExe} />
    {/await}
    {#await getFileName()}
        <div>{exeName}</div>
    {:then name}
        <div>{name}</div>
    {:catch error}
        <dir>{error}</dir>
        <div>{exeName}</div>
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
