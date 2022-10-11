<script lang="ts">
import Tray from './lib/Tray.svelte';
import { invoke } from '@tauri-apps/api/tauri'
    import Exe from './lib/Exe.svelte';
let executables:string[] =[];
$: PickExe ='Lmao'
async function exePicker(){

    PickExe = await invoke("exe_picker")
    if(!executables.includes(PickExe)&&PickExe!="None"){
        executables = [...executables,PickExe]

    }
}



</script>

<Tray></Tray>
<button on:click={exePicker}>
Browse
</button>
<div class="container">

{#each executables as exe}
    <Exe filePath={exe}></Exe>
    {/each}
</div>


<style>
.container{
    display: flex;
    flex-wrap: nowrap;
    flex-direction: row;
    width: 1080px;
}
</style>
