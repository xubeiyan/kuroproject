<script>
  const { active = false, children } = $props();
  const bgStyle = $derived(
    active
      ? "bg-3rd cursor-pointer hover:outline"
      : "bg-3rd/50 cursor-not-allowed"
  );
</script>

<button
  type="button"
  class="size-[120px] rounded-[20px] flex justify-center items-center
         outline-2 outline-offset-1 outline-3rd {bgStyle}"
>
  {@render children()}
</button>
