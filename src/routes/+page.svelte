<script>
  import SelectSaveMethod from "../pages/SelectSaveMethod.svelte";
  import CloudLogin from "../pages/CloudLogin.svelte";
  import MobileDeviceSelect from "../pages/MobileDeviceSelect.svelte";

  import FooterLogo from "../components/FooterLogo.svelte";
  import MainFrame from "../pages/MainFrame.svelte";

  import { getConfigFile } from "$lib/configFile";
  import { getHostname } from "$lib/systemInfo";

  import { onMount } from "svelte";
  import { basicStore } from "../stores/basicStore";

  let stage = $state("main_frame");

  const handleToStage = (stageName) => {
    if (stageName == "selectSaveMethod") {
      stage = "select_save_method";
    } else if (stageName == "cloud") {
      stage = "cloud_login";
    } else if (stageName == "usb") {
      stage = "mobile_device_select";
    } else if (stageName == "mainStage") {
      stage = "main_frame";
    }
  };

  onMount(async () => {
    const config = await getConfigFile();

    if (config != false) {
      const { asyncMode, asyncFolder, deviceName } = config;

      basicStore.update((b) => ({
        async_folder: asyncFolder,
        async_mode: asyncMode,
        device_name: deviceName,
      }));
      return;
    }

    // 不知为何在onMount函数中无法直接调用 hostname() 获取本机名称，在库函数中就可以（
    const hostname = await getHostname();
    basicStore.update((b) => ({
      ...b,
      device_name: hostname,
    }));

    stage = "select_save_method";
  });
</script>

<div class="bg-1st h-full">
  {#if stage == "select_save_method"}
    <SelectSaveMethod toStage={handleToStage} />
  {:else if stage == "cloud_login"}
    <CloudLogin toStage={handleToStage} />
  {:else if stage == "mobile_device_select"}
    <MobileDeviceSelect toStage={handleToStage} />
  {:else if stage == "main_frame"}
    <MainFrame />
  {/if}
  <FooterLogo />
</div>
