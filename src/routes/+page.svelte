<script>
  import SelectSaveMethod from "../pages/SelectSaveMethod.svelte";
  import CloudLogin from "../pages/CloudLogin.svelte";
  import MobileDeviceSelect from "../pages/MobileDeviceSelect.svelte";
  import MainFrame from "../pages/MainFrame.svelte";

  import TitleBar from "../components/TitleBar.svelte";
  import FooterLogo from "../components/FooterLogo.svelte";

  import { getConfigFile } from "$lib/configFile";
  import { getHostname } from "$lib/systemInfo";

  import { basicStore } from "../stores/basicStore";
  import { onMount } from "svelte";

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
    // 不知为何在onMount函数中无法直接调用 hostname() 获取本机名称，在库函数中就可以（
    const hostname = await getHostname();

    // 如果没有配置文件或者当前设备名和配置中的设备名不符则重新生成配置文件
    if (config == false || hostname != config.deviceName) {
      basicStore.update((b) => ({
        ...b,
        device_name: hostname,
      }));
      stage = "select_save_method";

      return;
    }

    const { asyncMode, asyncFolder, deviceName } = config;

    basicStore.update((b) => ({
      async_folder: asyncFolder,
      async_mode: asyncMode,
      device_name: deviceName,
    }));
  });
</script>

<div class="bg-1st h-full flex flex-col">
  <TitleBar />
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
