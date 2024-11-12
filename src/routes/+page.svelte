<script>
  import SelectSaveMethod from "../pages/SelectSaveMethod.svelte";
  import FooterLogo from "../components/FooterLogo.svelte";

  let stage = $state("select_save_method");
</script>

<div class="bg-1st h-full">
  {#if stage == "select_save_method"}
    <SelectSaveMethod />
  {/if}
  <FooterLogo />
</div>
