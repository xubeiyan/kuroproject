<script>
  import { basicStore } from "../stores/basicStore";
  import InfoBar from "../components/InfoBar.svelte";

  import LeftArrow from "$svgIcon/left-arrow.svelte";
  import RightArrow from "$svgIcon/right-arrow.svelte";
  import MoveButton from "../components/MoveButton.svelte";
  import Game from "$svgIcon/game.svelte";

  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import Refresh from "$svgIcon/refresh.svelte";

  let saveFromGameFolders = $state([]);
  let saveFromAsyncFolders = $state([]);

  let selectedGame = $state({
    left: null,
    right: null,
  });

  const selectGame = ({ side = "left", id }) => {
    if (side == "left") {
      selectedGame.left = selectedGame.left == id ? null : id;
    } else {
      selectedGame.right = selectedGame.right == id ? null : id;
    }
  };

  let leftSelectStyle = $derived((id) => {
    if (selectedGame.left == id) return "bg-3rd";
    return "";
  });

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

  // 没有游戏图标的时候提供替代图标
  const handleError = (ev) => {
    ev.target.src = "/gameIcons/default.png";
  };

  const load_conf_file = () => {
    if ($basicStore.async_folder == null) return;

    // 读取同步文件夹中的目录与配置
    invoke("get_all_save_and_config", { path: $basicStore.async_folder }).then(
      (json_string) => {
        let content = JSON.parse(json_string);
        saveFromAsyncFolders = content.conf_file_content.game_save;
        console.log('loaded!')
      }
    );
  };

  onMount(() => {
    load_conf_file();
  });
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
      <div
        class="bg-2nd/50 rounded-[10px] overflow-y-auto max-h-[300px] grow mb-[20px] py-[4px] pl-[12px] pr-[7px] flex flex-col gap-[4px]"
      >
        <div class="flex justify-end">
          <button onclick={load_conf_file}>
            <Refresh />
          </button>
        </div>
        {#each saveFromAsyncFolders as one, id}
          <button
            class="w-full flex items-center gap-[16px]
          border-2 border-3rd rounded-[10px] p-[8px] {leftSelectStyle(id)}"
            onclick={() => selectGame({ side: "left", id })}
          >
            <img
              class="size-[75px] rounded-[7px]"
              src="/gameIcons/{one.db_id}.jpg"
              alt={one.name}
              onerror={handleError}
            />
            <div class="flex flex-col items-start text-sm">
              <div class="h-[2.5rem] text-left">
                <span class="font-bold">{one.name}</span>
              </div>
              <span class="text-2nd-text">最后更新时间</span>
              <span class="text-2nd-text">{one.last_update_time}</span>
            </div>
          </button>
        {/each}
      </div>
    </div>
    <div
      class="w-full pt-[24px] flex flex-col justify-center gap-[24px] items-center"
    >
      <MoveButton>
        <LeftArrow />
      </MoveButton>
      <MoveButton>
        <RightArrow />
      </MoveButton>
    </div>
    <div class="flex flex-col gap-[12px]">
      <p class="text-xl text-right">游戏目录存档</p>
      <div
        class="bg-2nd rounded-[10px] grow mb-[20px] flex justify-center items-center"
      >
        {#if saveFromGameFolders.length == 0}
          <button
            class="h-[50px] w-[230px] bg-3rd rounded-[10px] text-1st
          flex items-center justify-center gap-[6px]
          hover:outline outline-offset-1 outline-3rd-light"
          >
            <Game />
            添加游戏
          </button>
        {/if}
      </div>
    </div>
  </div>
</div>
