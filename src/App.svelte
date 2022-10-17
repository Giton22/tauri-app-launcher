<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { Executable } from "./types/executable.type";
    import Tray from "./lib/Tray.svelte";
    import Exe from "./lib/Exe.svelte";
    let executables: Executable[] = [];
    $: errMsg = "";

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
        if (!Array.isArray(selected)) {
            let exe: Executable = {
                name: selected,
                path: selected,
                iconLocation: selected,
            };
            executables = [...executables, exe];
        } else if (selected === null) {
            errMsg = "You did not select any file or the file is not supported";
        }
    }
</script>

<Tray />
{errMsg}

<button on:click={addExe}> Add application </button>

<div class="container">
    {#each executables as exe}
        <Exe filePath={exe.path} />
    {/each}
</div>

<style>
    .container {
        display: flex;
        flex-wrap: nowrap;
        flex-direction: row;
    }
</style>
