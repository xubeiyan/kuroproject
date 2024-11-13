<script>
  import SelectSaveMethod from "../pages/SelectSaveMethod.svelte";
  import CloudLogin from "../pages/CloudLogin.svelte";
  import MobileDeviceSelect from "../pages/MobileDeviceSelect.svelte";

  import FooterLogo from "../components/FooterLogo.svelte";

  let stage = $state("select_save_method");

  const handleToStage = (e) => {
    if (e.detail == "selectSaveMethod") {
      stage = "select_save_method";
    } else if (e.detail == "cloud") {
      stage = "cloud_login";
    } else if (e.detail == "usb") {
      stage = "mobile_device_select";
    }
  };
</script>

<div class="bg-1st h-full">
  {#if stage == "select_save_method"}
    <SelectSaveMethod on:toStage={handleToStage} />
  {:else if stage == "cloud_login"}
    <CloudLogin on:toStage={handleToStage} />
  {:else if stage == "mobile_device_select"}
    <MobileDeviceSelect on:toStage={handleToStage} />
  {/if}
  <FooterLogo />
</div>
