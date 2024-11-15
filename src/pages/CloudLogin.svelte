<script>
  import BackBtn from "../components/BackBtn.svelte";
  import { createEventDispatcher } from "svelte";

  const { toStage } = $props();
</script>

<div
  class="relative h-full flex flex-col items-center justify-center gap-[10px]"
>
  <BackBtn onclick={() => toStage("selectSaveMethod")} />
  <h1 class="text-[24px] text-4th mb-[70px]">登录你的云端账号</h1>
  <label class="flex flex-col gap-[5px]">
    <span class="text-[14px] text-4th">用户名</span>
    <input
      class="border-[3px] border-3rd rounded-[5px]
        w-[240px] h-[30px] outline-none bg-transparent"
      type="text"
    />
  </label>
  <label class="flex flex-col gap-[5px]">
    <span class="text-[14px] text-4th">密码</span>
    <input
      class="border-[3px] border-3rd rounded-[5px]
        w-[240px] h-[30px] outline-none bg-transparent"
      type="password"
    />
  </label>
  <button class="w-[240px] h-[40px] rounded-[10px] bg-2nd text-4th mt-[30px]">
    登录
  </button>
  <a href="#1" class="text-4th mb-[45px]"> 没有账号？点击注册 </a>
</div>
