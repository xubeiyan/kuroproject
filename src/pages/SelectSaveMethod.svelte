<script>
  import CloudUploadIcon from "$svgIcon/cloud-upload.svelte";
  import SsdSquareIcon from "$svgIcon/ssd-square.svelte";

  const { toStage } = $props();
</script>

<div class="w-full h-full flex flex-col items-center justify-center gap-[60px]">
  <h1 class="text-[24px] text-4th">选择一个存档保存方式</h1>
  <div class="flex gap-[50px] justify-center">
    <button
      class="size-[300px] flex flex-col items-center justify-center bg-2nd rounded-[10px]"
      onclick={() => toStage("cloud")}
    >
      <CloudUploadIcon />
      <h2 class="text-[24px] mt-[14px]">云端</h2>
      <p class="text-[18px] text-2nd-text">需要云端部署存储和互联网连接</p>
    </button>
    <button
      class="size-[300px] flex flex-col items-center justify-center bg-2nd rounded-[10px]"
      onclick={() => toStage("usb")}
    >
      <SsdSquareIcon />
      <h2 class="text-[24px] mt-[14px]">可移动存储</h2>
      <p class="text-[18px] text-2nd-text">需要可移动存储来转移数据</p>
    </button>
  </div>
</div>
