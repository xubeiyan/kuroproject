<script>
  import { getCurrentWindow } from "@tauri-apps/api/window";
  const appWindow = getCurrentWindow();
  import CloseIcon from "../assets/svg-icon/close.svelte";
</script>

<div class="bg-3rd h-[40px] px-[15px] flex justify-end" data-tauri-drag-region>
  <button
    class="bg-alert w-[60px] h-[30px] flex justify-center items-center rounded-[5px]"
    onclick={() => appWindow.close()}
  >
    <CloseIcon />
  </button>
</div>
