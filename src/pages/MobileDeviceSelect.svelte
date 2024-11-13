<script>
  import BackBtn from "../components/BackBtn.svelte";
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  const back = () => {
    dispatch("toStage", "selectSaveMethod");
  };
</script>

<div class="p-[20px]">
  <BackBtn onclick={back} />
</div>
