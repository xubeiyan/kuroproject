<script>
  import { getCurrentWindow } from "@tauri-apps/api/window";
  const appWindow = getCurrentWindow();
  import CloseIcon from "../assets/svg-icon/close.svelte";
  import MinimizeIcon from "$svgIcon/minimize.svelte";
</script>

<div class="bg-3rd h-[40px] px-[15px] flex justify-end" data-tauri-drag-region>
  <button
    class="bg-2nd w-[60px] h-[30px] flex justify-center items-center rounded-l-[5px]"
    onclick={() => appWindow.minimize()}
  >
    <MinimizeIcon />
  </button>
  <button
    class="bg-alert hover:bg-alert-light
    w-[60px] h-[30px] flex justify-center items-center rounded-r-[5px]"
    onclick={() => appWindow.close()}
  >
    <CloseIcon />
  </button>
</div>
