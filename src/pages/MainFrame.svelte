<script>
  import { basicStore } from "../stores/basicStore";
  import InfoBar from "../components/InfoBar.svelte";

  import LeftArrow from "$svgIcon/left-arrow.svelte";
  import RightArrow from "$svgIcon/right-arrow.svelte";
  import MoveButton from "../components/MoveButton.svelte";
  import Game from "$svgIcon/game.svelte";

  let saveFromGameFolders = $state([]);

  const asyncModeText = $derived(
    $basicStore.async_mode == null
      ? "未能获取到"
      : $basicStore.async_mode == "mobile"
        ? "可移动存储同步"
        : "网络同步"
  );

  const asyncFolderText = $derived(
    $basicStore.async_folder == null ? "未能获取到" : $basicStore.async_folder
  );

</script>

<div class="p-8 flex flex-col grow">
  <div class="flex justify-between rounded-md border-2 border-2nd p-4">
    <div class="flex gap-4">
      <InfoBar label="同步模式" value={asyncModeText} />
      <InfoBar label="同步目录" value={asyncFolderText} />
    </div>
    <InfoBar right={true} label="当前设备名" value={$basicStore.device_name} />
  </div>
  <div class="mt-4 grow text-4th grid grid-cols-[1fr_160px_1fr]">
    <div class="flex flex-col gap-[12px]">
      <p class="text-xl">同步文件夹存档</p>
      <div class="bg-2nd/50 rounded-[10px] grow mb-[20px]">
        
      </div>
    </div>
    <div class="w-full pt-[24px] flex flex-col justify-center gap-[24px] items-center">
      <MoveButton>
        <LeftArrow />
      </MoveButton>
      <MoveButton>
        <RightArrow />
      </MoveButton>
    </div>
    <div class="flex flex-col gap-[12px]">
      <p class="text-xl text-right">游戏目录存档</p>
      <div class="bg-2nd rounded-[10px] grow mb-[20px] flex justify-center items-center">
        {#if saveFromGameFolders.length == 0}
          <button class="h-[50px] w-[230px] bg-3rd rounded-[10px] text-1st 
          flex items-center justify-center gap-[6px] 
          hover:outline outline-offset-1 outline-3rd-light">
            <Game />
            添加游戏
          </button>
        {/if}
      </div>
    </div>
  </div>
</div>
