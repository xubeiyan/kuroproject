<script>
  import ArrowLeftIcon from "$svgIcon/arrow-left.svelte";
  let props = $props();
</script>

<button
  class="size-[35px] rounded-full flex justify-center items-center bg-2nd 
  hover:outline outline-3rd"
  {...props}
>
  <ArrowLeftIcon />
</button>
