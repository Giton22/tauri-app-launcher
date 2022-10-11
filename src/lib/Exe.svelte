<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'

    export let filePath:string;
    let exeName = filePath.split(String.fromCharCode(92)).at(-1).split('.').at(0);
    let iconPath = "src/assets/unknown.png"
    let appName="App"

    async function runExe(filePath:string) {
    invoke("run_exe",{filePath});
}
</script>

<div class="container" on:click={()=>runExe(filePath)}>
<img src="{iconPath}" alt="icon">
<div>{exeName}</div>
<div>{filePath}</div>
</div>

<style>
    img{
        height: 100px;
        width: 100px;
        border-radius: 5px;
        margin-bottom: 10px;
    }
    .container{
        margin: 10px;
        display: flex;
        flex-direction:column;
        text-align: center;
        width: 100px;
    }
</style>