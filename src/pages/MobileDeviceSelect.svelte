<script>
  import AddFolderIcon from "$svgIcon/add-folder.svelte";
  import BackBtn from "../components/BackBtn.svelte";

  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";

  const { toStage } = $props();

  // 路径
  let savePath = $state(null);
  // 可用大小
  let device = $state({
    avaliable_size: 0,
    removable: false,
  });

  const selectSavePath = async () => {
    const getSavePath = await open({
      multiple: false,
      directory: true,
    });

    // 点取消按钮什么都不做
    if (getSavePath == null) return;

    savePath = getSavePath;
    invoke("get_free_space", { file_path: getSavePath }).then(
      (json_string) => {
        const resultObj = JSON.parse(json_string);
        device = {
          avaliable_size: (resultObj.size / (1024 * 1024)).toFixed(3),
          removable: resultObj.removable,
        };
      }
    );
  };
</script>

<div class="relative flex flex-col justify-center items-center gap-[40px]">
  <BackBtn onclick={() => toStage("selectSaveMethod")} />
  <h1 class="text-[24px] mt-[90px] text-4th">选择存档保存目录</h1>
  {#if savePath == null}
    <button
      class="w-[640px] h-[300px] rounded-[30px]
      border-[3px] border-3rd border-dashed bg-2nd
      flex justify-center items-center gap-[8px]"
      onclick={selectSavePath}
    >
      <AddFolderIcon />
      <span class="text-4th">点击选择</span>
    </button>
  {:else}
    <div class="w-[640px] h-[300px] flex justify-between">
      <button
        class="w-[150px] h-full
      border-[3px] border-dashed border-3rd bg-2nd rounded-[30px]"
        onclick={selectSavePath}>重新选择</button
      >
      <div
        class="w-[300px] h-full border-[3px] border-2nd rounded-[30px]
      px-[24px] py-[40px] space-y-4"
      >
        <div>
          <h1 class="text-[24px] text-4th">已选目录</h1>
          <span class="text-4th">{savePath}</span>
        </div>
        <div>
          <h1 class="text-[24px] text-4th">可用空间</h1>
          <span class="text-4th">{device.avaliable_size} MiB</span>
        </div>
        <div>
          <h1 class="text-[24px] text-4th">可移动设备</h1>
          <span class="text-4th">{device.removable ? "是" : "否"}</span>
        </div>
      </div>
      <button
        class="w-[150px] h-full
      border-[3px] border-3rd bg-2nd rounded-[30px]">确认选择</button
      >
    </div>
  {/if}
</div>
