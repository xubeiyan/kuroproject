<script>
  const { label = "大一号", value = "小一号", right = false } = $props();

  const textAlign = $derived(right ? "text-right" : "");
</script>

<div class="text-4th">
  <h1 class="text-3xl">{label}</h1>
  <h5 class={textAlign}>{value}</h5>
</div>
