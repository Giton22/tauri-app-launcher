<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { invoke } from '@tauri-apps/api/tauri';
    import type { Executable } from "./types/executable.type";
    import Tray from "./lib/Tray.svelte";
    import Exe from "./lib/Exe.svelte";
    import { onMount } from "svelte";
    let executables: Executable[] = [];
    $: errMsg = "";
    let path;
    onMount(()=>{
        downloadImage()
    })
    function downloadImage() {
        const options = {
            method: "GET",
            headers: {
                'Access-Control-Allow-Origin':'*',
                accept: "application/json",
                Authorization:
                    "Bearer hnQTW4n0CCSize4pyJDqIdyeUkkj3DHaS89C9GbcdBM1a1fVsy5N2Aaf3nsAMGdO",
            },
        };

        fetch(
            "https://api.iconfinder.com/v4/icons/search?query=discord&count=1&offset=0&premium=0",
            options
        )
            .then((response) => response.json())
            .then((response) => console.log(response))
            .catch((err) => console.error(err));
    }
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
        if (Array.isArray(selected)) {
            selected.forEach((item) => {
                let exe: Executable = {
                    name: item,
                    path: item,
                    iconLocation: "src/assets/unknown.png",
                };
                executables = [...executables, exe];
            });
        } else if (selected === null) {
            errMsg = "You did not select any file or the file is not supported";
        } else {
            let exe: Executable = {
                name: selected,
                path: selected,
                iconLocation: "src/assets/unknown.png",
            };
            executables = [...executables, exe];
        }
    }
    async function messagebox() {
        await invoke("message_box")
    }
</script>

<Tray />
{errMsg}
<button on:click={messagebox}></button>
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
