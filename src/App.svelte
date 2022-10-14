<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Executable } from "./types/executable.type";
    import Tray from "./lib/Tray.svelte";
    import Exe from "./lib/Exe.svelte";
    import { onMount } from "svelte";
    let executables: Executable[] = [];
    $: errMsg = "";
    let path;

    async function addExe() {
        errMsg = "";
        const selected = await open({
            multiple: false,
            filters: [
                {
                    name: "Executable",
                    extensions: ["exe"],
                },
            ],
        });
        let icon:string = await invoke("get_icon",{selected});
        if (Array.isArray(selected)) {
            selected.forEach((item) => {
                let exe: Executable = {
                    name: item,
                    path: item,
                    iconLocation:icon,
                };
                icon=await invoke("get_icon",{exe.})
                executables = [...executables, exe];
                console.log(exe)

            });
        } else if (selected === null) {
            errMsg = "You did not select any file or the file is not supported";
        } else {
            let exe: Executable = {
                name: selected,
                path: selected,
                iconLocation: icon,
            };
            executables = [...executables, exe];
            console.log(exe.iconLocation)

        }
    }

</script>

<Tray />
{errMsg}

<button on:click={addExe}> Add application </button>

<div class="container">
    {#each executables as exe}
        <Exe filePath={exe.path} iconPath={exe.iconLocation} />
    {/each}
</div>

<style>
    .container {
        display: flex;
        flex-wrap: nowrap;
        flex-direction: row;
    }
</style>
