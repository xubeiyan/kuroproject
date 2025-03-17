<script>
  import { basicStore } from "../stores/basicStore";

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

  import InfoBar from "../components/InfoBar.svelte";
</script>

<div class="p-8">
  <div class="flex justify-between rounded-md border-2 border-2nd p-4">
    <div class="flex gap-4">
      <InfoBar label="同步模式" value={asyncModeText} />
      <InfoBar label="同步目录" value={asyncFolderText} />
    </div>
    <InfoBar right={true} label="当前设备名" value={$basicStore.device_name} />
  </div>
</div>
