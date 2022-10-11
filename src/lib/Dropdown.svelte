<script lang="ts">
    import { appWindow } from '@tauri-apps/api/window'

</script>

<div data-tauri-drag-region class="titlebar">
    <div on:click="{()=>appWindow.minimize()}" class="titlebar-button" id="titlebar-minimize">
      <img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
      />
    </div>
    <div on:click="{()=>appWindow.toggleMaximize()}" class="titlebar-button" id="titlebar-maximize">
      <img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize"
      />
    </div>
    <div on:click="{()=>appWindow.close()}" class="titlebar-button" id="titlebar-close">
      <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </div>
  </div>

<style>
    .titlebar{
    height: 30px;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
    }
    .titlebar-button{
        margin: 10px;
    }
    img{
        filter: drop-shadow(1 1 0.75rem crimson);

    }
</style>
